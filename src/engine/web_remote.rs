#![allow(dead_code)]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebPlayerStatus {
    pub status: String,
    pub title: String,
    pub duration: f64,
    pub time_pos: f64,
    pub percent_pos: f64,
    pub is_paused: bool,
    pub volume: f64,
    pub is_muted: bool,
    pub speed: f64,
    pub audio_track: String,
    pub subtitle_track: String,
}

impl Default for WebPlayerStatus {
    fn default() -> Self {
        Self {
            status: "ok".to_string(),
            title: "VortexPlayer Ready".to_string(),
            duration: 0.0,
            time_pos: 0.0,
            percent_pos: 0.0,
            is_paused: true,
            volume: 100.0,
            is_muted: false,
            speed: 1.0,
            audio_track: "Default".to_string(),
            subtitle_track: "Off".to_string(),
        }
    }
}

static SHARED_STATUS: OnceLock<Arc<Mutex<WebPlayerStatus>>> = OnceLock::new();

pub fn get_shared_status() -> Arc<Mutex<WebPlayerStatus>> {
    SHARED_STATUS.get_or_init(|| Arc::new(Mutex::new(WebPlayerStatus::default()))).clone()
}

pub fn update_shared_status(status: WebPlayerStatus) {
    if let Ok(mut lock) = get_shared_status().lock() {
        *lock = status;
    }
}

static ACTIVE_CONNECTIONS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
const MAX_CONCURRENT_CONNECTIONS: usize = 32;

pub struct WebRemoteServer {
    running: Arc<AtomicBool>,
}

impl WebRemoteServer {
    pub fn start(port: u16, tx: Sender<crate::engine::IpcCommand>) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let thread_running = Arc::clone(&running);

        thread::spawn(move || {
            let candidate_ports = [port, 9876, 9877, 42121, 42122];
            let mut bound_listener = None;

            for &p in &candidate_ports {
                let addr = format!("0.0.0.0:{}", p);
                if let Ok(listener) = TcpListener::bind(&addr) {
                    let _ = listener.set_nonblocking(true);
                    eprintln!("🌌 VortexPlayer Web Remote active at http://localhost:{}", p);
                    bound_listener = Some((listener, p));
                    break;
                }
            }

            if let Some((listener, _)) = bound_listener {
                while thread_running.load(Ordering::Relaxed) {
                    if let Ok((mut stream, _)) = listener.accept() {
                        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(3)));
                        let _ = stream.set_write_timeout(Some(std::time::Duration::from_secs(3)));

                        if ACTIVE_CONNECTIONS.load(Ordering::Relaxed) < MAX_CONCURRENT_CONNECTIONS {
                            ACTIVE_CONNECTIONS.fetch_add(1, Ordering::SeqCst);
                            let tx_clone = tx.clone();
                            thread::spawn(move || {
                                Self::handle_connection(&mut stream, tx_clone);
                                ACTIVE_CONNECTIONS.fetch_sub(1, Ordering::SeqCst);
                            });
                        }
                    }
                    thread::sleep(std::time::Duration::from_millis(50));
                }
            }
        });

        Self { running }
    }

    fn is_allowed_origin(req: &str) -> bool {
        for line in req.lines() {
            let lower = line.to_ascii_lowercase();
            if lower.starts_with("origin:") || lower.starts_with("referer:") {
                let val = line.split_once(':').map(|(_, v)| v.trim()).unwrap_or("");
                if val.is_empty() {
                    continue;
                }
                // Allow localhost / local loopback and standard RFC1918 private subnets
                if val.starts_with("http://localhost")
                    || val.starts_with("https://localhost")
                    || val.starts_with("http://127.0.0.1")
                    || val.starts_with("http://[::1]")
                    || val.starts_with("http://192.168.")
                    || val.starts_with("http://10.")
                    || (16..=31).any(|b| val.starts_with(&format!("http://172.{}.", b)))
                {
                    return true;
                }
                // Disallow untrusted external public web origins
                return false;
            }
        }
        // Direct non-browser clients (native apps, curl) do not include Origin headers
        true
    }

    fn handle_connection(stream: &mut TcpStream, tx: Sender<crate::engine::IpcCommand>) {
        let mut buf = [0u8; 4096];
        if let Ok(n) = stream.read(&mut buf) {
            if n == 0 {
                return;
            }
            let req = String::from_utf8_lossy(&buf[..n]);

            // Validate origin to prevent CSRF attacks from external internet websites
            if !Self::is_allowed_origin(&req) {
                let forbidden = "HTTP/1.1 403 Forbidden\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nForbidden: Cross-Origin Request Blocked\r\n";
                let _ = stream.write_all(forbidden.as_bytes());
                return;
            }

            let first_line = req.lines().next().unwrap_or("");
            
            if first_line.starts_with("GET /api/status") {
                let status_obj = if let Ok(lock) = get_shared_status().lock() {
                    lock.clone()
                } else {
                    WebPlayerStatus::default()
                };
                let status_json = serde_json::to_string(&status_obj).unwrap_or_else(|_| r#"{"status":"ok"}"#.to_string());
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    status_json.len(),
                    status_json
                );
                let _ = stream.write_all(response.as_bytes());
                return;
            }

            if first_line.starts_with("GET /api/command") {
                if let Some(query_idx) = first_line.find('?') {
                    let query = first_line[query_idx + 1..].split_whitespace().next().unwrap_or("");
                    Self::process_command_query(query, &tx);
                }
                let resp_json = r#"{"result":"ok"}"#;
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    resp_json.len(),
                    resp_json
                );
                let _ = stream.write_all(response.as_bytes());
                return;
            }

            // Default: serve the high-end mobile touch web remote HTML
            let html = include_str!("web_remote_ui.html");
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                html.len(),
                html
            );
            let _ = stream.write_all(response.as_bytes());
        }
    }

    fn process_command_query(query: &str, tx: &Sender<crate::engine::IpcCommand>) {
        let mut action = "";
        let mut val_str = "";

        for part in query.split('&') {
            if let Some((k, v)) = part.split_once('=') {
                match k {
                    "action" => action = v,
                    "val" | "pos" | "speed" | "volume" | "pct" => val_str = v,
                    _ => {}
                }
            }
        }

        match action {
            "play" => { let _ = tx.send(crate::engine::IpcCommand::Play); }
            "pause" => { let _ = tx.send(crate::engine::IpcCommand::Pause); }
            "play_pause" | "toggle" => { let _ = tx.send(crate::engine::IpcCommand::TogglePause); }
            "stop" => { let _ = tx.send(crate::engine::IpcCommand::Stop); }
            "seek_fwd" => { let _ = tx.send(crate::engine::IpcCommand::SeekRelative(10.0)); }
            "seek_back" => { let _ = tx.send(crate::engine::IpcCommand::SeekRelative(-10.0)); }
            "seek_fwd_30" => { let _ = tx.send(crate::engine::IpcCommand::SeekRelative(30.0)); }
            "seek_back_30" => { let _ = tx.send(crate::engine::IpcCommand::SeekRelative(-30.0)); }
            "seek" | "seek_abs" => {
                if let Ok(pos) = val_str.parse::<f64>() {
                    let _ = tx.send(crate::engine::IpcCommand::SeekAbsolute(pos));
                }
            }
            "seek_pct" => {
                if let Ok(pct) = val_str.parse::<f64>() {
                    let _ = tx.send(crate::engine::IpcCommand::SeekPercent(pct));
                }
            }
            "volume" => {
                if let Ok(vol) = val_str.parse::<f64>() {
                    let _ = tx.send(crate::engine::IpcCommand::SetVolume(vol));
                }
            }
            "mute" | "toggle_mute" => { let _ = tx.send(crate::engine::IpcCommand::ToggleMute); }
            "speed" => {
                if let Ok(spd) = val_str.parse::<f64>() {
                    let _ = tx.send(crate::engine::IpcCommand::SetSpeed(spd));
                }
            }
            "prev" => { let _ = tx.send(crate::engine::IpcCommand::PrevTrack); }
            "next" => { let _ = tx.send(crate::engine::IpcCommand::NextTrack); }
            "cycle_audio" => { let _ = tx.send(crate::engine::IpcCommand::CycleAudio); }
            "cycle_sub" => { let _ = tx.send(crate::engine::IpcCommand::CycleSubtitle); }
            "fullscreen" | "toggle_fullscreen" => { let _ = tx.send(crate::engine::IpcCommand::ToggleFullscreen); }
            "screenshot" => { let _ = tx.send(crate::engine::IpcCommand::TakeScreenshot); }
            _ => {}
        }
    }
}
