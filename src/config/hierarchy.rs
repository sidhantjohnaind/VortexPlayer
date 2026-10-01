#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingSource {
    Global,
    Profile,
    Folder,
    File,
}

impl Default for SettingSource {
    fn default() -> Self {
        SettingSource::Global
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedSetting<T> {
    pub value: T,
    pub source: SettingSource,
    pub source_name: String,
}

impl<T: Default> Default for ResolvedSetting<T> {
    fn default() -> Self {
        Self {
            value: T::default(),
            source: SettingSource::Global,
            source_name: "Default Config".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayeredConfigState {
    pub subtitle_delay: ResolvedSetting<f64>,
    pub audio_delay: ResolvedSetting<f64>,
    pub playback_speed: ResolvedSetting<f64>,
    pub aspect_ratio: ResolvedSetting<String>,
    pub eq_preset: ResolvedSetting<String>,
    pub zoom_level: ResolvedSetting<f64>,
    pub hwdec_mode: ResolvedSetting<String>,
}

impl Default for LayeredConfigState {
    fn default() -> Self {
        Self::new()
    }
}

impl LayeredConfigState {
    pub fn new() -> Self {
        Self {
            subtitle_delay: ResolvedSetting { value: 0.0, source: SettingSource::Global, source_name: "Default Config".to_string() },
            audio_delay: ResolvedSetting { value: 0.0, source: SettingSource::Global, source_name: "Default Config".to_string() },
            playback_speed: ResolvedSetting { value: 1.0, source: SettingSource::Global, source_name: "Default Config".to_string() },
            aspect_ratio: ResolvedSetting { value: "auto".to_string(), source: SettingSource::Global, source_name: "Default Config".to_string() },
            eq_preset: ResolvedSetting { value: "Flat".to_string(), source: SettingSource::Global, source_name: "Default Config".to_string() },
            zoom_level: ResolvedSetting { value: 1.0, source: SettingSource::Global, source_name: "Default Config".to_string() },
            hwdec_mode: ResolvedSetting { value: "auto-safe".to_string(), source: SettingSource::Global, source_name: "Default Config".to_string() },
        }
    }
}
