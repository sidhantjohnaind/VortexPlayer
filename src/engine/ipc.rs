#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: Option<String>,
    pub id: Option<u64>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum IpcCommand {
    Play,
    Pause,
    TogglePause,
    Stop,
    SeekRelative(f64),
    SeekAbsolute(f64),
    SeekPercent(f64),
    SetVolume(f64),
    ToggleMute,
    SetSpeed(f64),
    OpenFile(String),
    OpenUrl(String),
    NextTrack,
    PrevTrack,
    CycleAudio,
    CycleSubtitle,
    ToggleFullscreen,
    TakeScreenshot,
    StartRecording(String),
    StopRecording,
}

pub struct IpcServer {
    pub receiver: Receiver<IpcCommand>,
    running: Arc<AtomicBool>,
}

impl IpcServer {
    pub fn start() -> Self {
        let (tx, rx) = channel::<IpcCommand>();
        let running = Arc::new(AtomicBool::new(true));

        #[cfg(windows)]
        {
            let tx_pipe = tx.clone();
            let running_pipe = Arc::clone(&running);
            thread::spawn(move || {
                Self::listen_named_pipe(tx_pipe, running_pipe);
            });
        }

        let tx_tcp = tx.clone();
        let running_tcp = Arc::clone(&running);
        thread::spawn(move || {
            Self::listen_tcp(tx_tcp, running_tcp);
        });

        let tx_web = tx;
        crate::engine::web_remote::WebRemoteServer::start(9876, tx_web);

        Self {
            receiver: rx,
            running,
        }
    }

    pub fn poll_command(&self) -> Option<IpcCommand> {
        self.receiver.try_recv().ok()
    }

    pub fn handle_json_rpc(raw: &str, tx: &Sender<IpcCommand>) -> String {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(trimmed) {
            let method = req.method.as_str();
            let cmd = match method {
                "play" => Some(IpcCommand::Play),
                "pause" => Some(IpcCommand::Pause),
                "toggle_pause" | "play_pause" => Some(IpcCommand::TogglePause),
                "stop" => Some(IpcCommand::Stop),
                "seek_relative" => {
                    let offset = req.params.as_ref().and_then(|p| {
                        p.get("offset")
                            .and_then(|v| v.as_f64())
                            .or_else(|| p.get(0).and_then(|v| v.as_f64()))
                    }).unwrap_or(5.0);
                    Some(IpcCommand::SeekRelative(offset))
                }
                "seek" | "seek_absolute" => {
                    let pos = req.params.as_ref().and_then(|p| {
                        p.get("position")
                            .and_then(|v| v.as_f64())
                            .or_else(|| p.get(0).and_then(|v| v.as_f64()))
                    }).unwrap_or(0.0);
                    Some(IpcCommand::SeekAbsolute(pos))
                }
                "seek_percent" | "seek_pct" => {
                    let pct = req.params.as_ref().and_then(|p| {
                        p.get("percent")
                            .and_then(|v| v.as_f64())
                            .or_else(|| p.get(0).and_then(|v| v.as_f64()))
                    }).unwrap_or(0.0);
                    Some(IpcCommand::SeekPercent(pct))
                }
                "set_volume" => {
                    let vol = req.params.as_ref().and_then(|p| {
                        p.get("volume")
                            .and_then(|v| v.as_f64())
                            .or_else(|| p.get(0).and_then(|v| v.as_f64()))
                    }).unwrap_or(100.0);
                    Some(IpcCommand::SetVolume(vol))
                }
                "toggle_mute" | "mute" => Some(IpcCommand::ToggleMute),
                "toggle_fullscreen" | "fullscreen" => Some(IpcCommand::ToggleFullscreen),
                "set_speed" => {
                    let speed = req.params.as_ref().and_then(|p| {
                        p.get("speed")
                            .and_then(|v| v.as_f64())
                            .or_else(|| p.get(0).and_then(|v| v.as_f64()))
                    }).unwrap_or(1.0);
                    Some(IpcCommand::SetSpeed(speed))
                }
                "load_file" | "open_file" => {
                    req.params.as_ref().and_then(|p| {
                        p.get("path")
                            .and_then(|v| v.as_str())
                            .or_else(|| p.get(0).and_then(|v| v.as_str()))
                    }).map(|s| IpcCommand::OpenFile(s.to_string()))
                }
                "load_url" | "open_url" => {
                    req.params.as_ref().and_then(|p| {
                        p.get("url")
                            .and_then(|v| v.as_str())
                            .or_else(|| p.get(0).and_then(|v| v.as_str()))
                    }).map(|s| IpcCommand::OpenUrl(s.to_string()))
                }
                "next" | "next_track" => Some(IpcCommand::NextTrack),
                "prev" | "prev_track" | "previous" => Some(IpcCommand::PrevTrack),
                "cycle_audio" => Some(IpcCommand::CycleAudio),
                "cycle_subtitle" | "cycle_sub" => Some(IpcCommand::CycleSubtitle),
                "take_screenshot" | "screenshot" => Some(IpcCommand::TakeScreenshot),
                "start_recording" => {
                    req.params.as_ref().and_then(|p| {
                        p.get("path")
                            .and_then(|v| v.as_str())
                            .or_else(|| p.get(0).and_then(|v| v.as_str()))
                    }).map(|s| IpcCommand::StartRecording(s.to_string()))
                }
                "stop_recording" => Some(IpcCommand::StopRecording),
                _ => None,
            };

            if let Some(c) = cmd {
                let _ = tx.send(c);
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(serde_json::json!({
                        "status": "ok",
                        "method": req.method
                    })),
                    error: None,
                };
                serde_json::to_string(&resp).unwrap_or_default()
            } else {
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: None,
                    error: Some(format!("Method not supported: {}", req.method)),
                };
                serde_json::to_string(&resp).unwrap_or_default()
            }
        } else {
            let resp = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: None,
                result: None,
                error: Some("Invalid JSON-RPC 2.0 payload".to_string()),
            };
            serde_json::to_string(&resp).unwrap_or_default()
        }
    }

    #[cfg(windows)]
    fn listen_named_pipe(tx: Sender<IpcCommand>, running: Arc<AtomicBool>) {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        const PIPE_ACCESS_DUPLEX: u32 = 0x00000003;
        const PIPE_TYPE_MESSAGE: u32 = 0x00000004;
        const PIPE_READMODE_MESSAGE: u32 = 0x00000002;
        const PIPE_WAIT: u32 = 0x00000000;
        const PIPE_UNLIMITED_INSTANCES: u32 = 255;
        const INVALID_HANDLE_VALUE: isize = -1;
        const ERROR_PIPE_CONNECTED: u32 = 535;

        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn CreateNamedPipeW(
                lpName: *const u16,
                dwOpenMode: u32,
                dwPipeMode: u32,
                nMaxInstances: u32,
                nOutBufferSize: u32,
                nInBufferSize: u32,
                nDefaultTimeOut: u32,
                lpSecurityAttributes: *const std::ffi::c_void,
            ) -> isize;

            fn ConnectNamedPipe(
                hNamedPipe: isize,
                lpOverlapped: *mut std::ffi::c_void,
            ) -> i32;

            fn DisconnectNamedPipe(
                hNamedPipe: isize,
            ) -> i32;

            fn ReadFile(
                hFile: isize,
                lpBuffer: *mut std::ffi::c_void,
                nNumberOfBytesToRead: u32,
                lpNumberOfBytesRead: *mut u32,
                lpOverlapped: *mut std::ffi::c_void,
            ) -> i32;

            fn WriteFile(
                hFile: isize,
                lpBuffer: *const std::ffi::c_void,
                nNumberOfBytesToWrite: u32,
                lpNumberOfBytesWritten: *mut u32,
                lpOverlapped: *mut std::ffi::c_void,
            ) -> i32;

            fn CloseHandle(hObject: isize) -> i32;
            fn GetLastError() -> u32;
        }

        let pipe_name: Vec<u16> = OsStr::new(r"\\.\pipe\vertexplayer-ipc")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        while running.load(Ordering::Relaxed) {
            unsafe {
                let handle = CreateNamedPipeW(
                    pipe_name.as_ptr(),
                    PIPE_ACCESS_DUPLEX,
                    PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                    PIPE_UNLIMITED_INSTANCES,
                    4096,
                    4096,
                    50,
                    std::ptr::null(),
                );

                if handle == INVALID_HANDLE_VALUE {
                    thread::sleep(std::time::Duration::from_millis(500));
                    continue;
                }

                let connected = ConnectNamedPipe(handle, std::ptr::null_mut());
                if connected != 0 || GetLastError() == ERROR_PIPE_CONNECTED {
                    let mut buffer = [0u8; 4096];
                    let mut bytes_read = 0u32;
                    let success = ReadFile(
                        handle,
                        buffer.as_mut_ptr() as *mut _,
                        buffer.len() as u32,
                        &mut bytes_read,
                        std::ptr::null_mut(),
                    );

                    if success != 0 && bytes_read > 0 {
                        if let Ok(text) = std::str::from_utf8(&buffer[..bytes_read as usize]) {
                            let response_json = Self::handle_json_rpc(text, &tx);
                            if !response_json.is_empty() {
                                let mut bytes_written = 0u32;
                                WriteFile(
                                    handle,
                                    response_json.as_ptr() as *const _,
                                    response_json.len() as u32,
                                    &mut bytes_written,
                                    std::ptr::null_mut(),
                                );
                            }
                        }
                    }
                    DisconnectNamedPipe(handle);
                }
                CloseHandle(handle);
            }
        }
    }

    fn listen_tcp(tx: Sender<IpcCommand>, running: Arc<AtomicBool>) {
        use std::io::{Read, Write};
        use std::net::TcpListener;

        if let Ok(listener) = TcpListener::bind("127.0.0.1:42120") {
            let _ = listener.set_nonblocking(true);
            while running.load(Ordering::Relaxed) {
                if let Ok((mut stream, _)) = listener.accept() {
                    let mut buf = [0u8; 4096];
                    if let Ok(n) = stream.read(&mut buf) {
                        if n > 0 {
                            if let Ok(text) = std::str::from_utf8(&buf[..n]) {
                                let resp = Self::handle_json_rpc(text, &tx);
                                let _ = stream.write_all(resp.as_bytes());
                            }
                        }
                    }
                }
                thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }
}
