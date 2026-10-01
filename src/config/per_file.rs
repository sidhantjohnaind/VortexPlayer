#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerItemProfile {
    pub audio_track_id: Option<i64>,
    pub subtitle_track_id: Option<i64>,
    pub audio_delay_sec: Option<f64>,
    pub subtitle_delay_sec: Option<f64>,
    pub aspect_ratio: Option<String>,
    pub playback_speed: Option<f64>,
    pub eq_preset: Option<String>,
    pub zoom_level: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerItemMemoryStore {
    pub files: HashMap<String, PerItemProfile>,
    pub folders: HashMap<String, PerItemProfile>,
}

impl PerItemMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save_for_file(&mut self, path: &str, profile: PerItemProfile) {
        self.files.insert(path.to_string(), profile);
    }

    pub fn get_for_file(&self, path: &str) -> Option<&PerItemProfile> {
        self.files.get(path)
    }
}
