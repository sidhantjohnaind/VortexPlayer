#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleLayerState {
    pub track_id: Option<i64>,
    pub font_size: f32,
    pub vertical_offset: f32,
    pub delay_sec: f64,
    pub visible: bool,
}

impl Default for SubtitleLayerState {
    fn default() -> Self {
        Self {
            track_id: None,
            font_size: 28.0,
            vertical_offset: 92.0,
            delay_sec: 0.0,
            visible: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubtitleController {
    pub primary: SubtitleLayerState,
    pub secondary: SubtitleLayerState,
    pub tertiary: SubtitleLayerState,
}

impl SubtitleController {
    pub fn new() -> Self {
        Self::default()
    }
}
