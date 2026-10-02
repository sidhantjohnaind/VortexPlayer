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

