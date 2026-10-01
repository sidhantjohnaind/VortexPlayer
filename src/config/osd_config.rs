#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OsdPosition {
    TopLeft,
    TopCenter,
    Center,
    BottomCenter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsdConfig {
    pub enabled: bool,
    pub position: OsdPosition,
    pub font_size: f32,
    pub duration_ms: u64,
    pub show_volume: bool,
    pub show_tracks: bool,
    pub show_speed: bool,
    pub show_zoom: bool,
    pub show_chapters: bool,
    pub show_bookmarks: bool,
}

impl Default for OsdConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            position: OsdPosition::TopLeft,
            font_size: 16.0,
            duration_ms: 2200,
            show_volume: true,
            show_tracks: true,
            show_speed: true,
            show_zoom: true,
            show_chapters: true,
            show_bookmarks: true,
        }
    }
}
