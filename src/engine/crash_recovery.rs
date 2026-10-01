#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionState {
    pub last_file: String,
    pub last_position: f64,
    pub playlist_items: Vec<String>,
    pub crashed_cleanly: bool,
    pub failed_launch_count: u32,
    pub safe_mode_active: bool,
}

pub struct CrashRecoverySentinel {
    state_file: PathBuf,
}

impl Default for CrashRecoverySentinel {
    fn default() -> Self {
        let dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")).join("VertexPlayer");
        let _ = fs::create_dir_all(&dir);
        Self {
            state_file: dir.join("session_state.json"),
        }
    }
}

impl CrashRecoverySentinel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_state(&self) -> SessionState {
        if let Ok(data) = fs::read_to_string(&self.state_file) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            SessionState::default()
        }
    }

    pub fn mark_session_start(&self, current_file: &str, position: f64, playlist: &[String]) {
        let mut state = self.load_state();
        state.last_file = current_file.to_string();
        state.last_position = position;
        state.playlist_items = playlist.to_vec();
        state.crashed_cleanly = false;
        state.failed_launch_count += 1;
        if state.failed_launch_count >= 2 {
            state.safe_mode_active = true;
        }
        let _ = fs::write(&self.state_file, serde_json::to_string_pretty(&state).unwrap_or_default());
    }

    pub fn mark_clean_shutdown(&self) {
        let mut state = self.load_state();
        state.crashed_cleanly = true;
        state.failed_launch_count = 0;
        state.safe_mode_active = false;
        let _ = fs::write(&self.state_file, serde_json::to_string_pretty(&state).unwrap_or_default());
    }
}
