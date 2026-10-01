#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScreenCaptureMode {
    PrimaryMonitor,
    SecondaryMonitor,
    SpecificWindow(String),
    Region { x: i32, y: i32, w: i32, h: i32 },
}

pub struct ScreenCaptureManager;

impl ScreenCaptureManager {
    pub fn build_gdigrab_url(fps: u32, draw_mouse: bool) -> String {
        format!("avdevice://gdigrab:desktop?framerate={}&draw_mouse={}", fps, if draw_mouse { 1 } else { 0 })
    }
}
