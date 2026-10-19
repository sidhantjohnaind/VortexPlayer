pub mod commands;
pub mod bookmark;
pub mod capture;
pub mod config;
pub mod engine;
pub mod library;
pub mod network;
pub mod platform;
pub mod playlist;
pub mod plugin;
pub mod subtitles;
pub mod ui;

pub fn log_step(_msg: &str) {
    use std::io::Write;
    let path = std::env::temp_dir().join("vortex_startup.log");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "[{:?}] [PID {}] {}", std::time::SystemTime::now(), std::process::id(), _msg);
    }
}

pub fn log_playback_debug(msg: &str) {
    use std::io::Write;
    let local_ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let line = format!("[{}] [PID {}] {}", local_ts, std::process::id(), msg);

    let debug_path = dirs::data_local_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("VortexPlayer")
        .join("playback_debug.log");
    if let Some(parent) = debug_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&debug_path) {
        let _ = writeln!(f, "{}", line);
    }

    let startup_path = std::env::temp_dir().join("vortex_startup.log");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(startup_path) {
        let _ = writeln!(f, "{}", line);
    }
}

