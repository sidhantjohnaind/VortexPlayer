pub mod screen;
pub use screen::{ScreenCaptureManager, ScreenCaptureMode};
pub mod devices;
pub mod thumbnail_sheet;
pub use thumbnail_sheet::*;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub use devices::CaptureDeviceManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureFormat {
    Png,
    Jpeg,
    WebP,
}

impl CaptureFormat {
    pub const ALL: [CaptureFormat; 3] = [CaptureFormat::Png, CaptureFormat::Jpeg, CaptureFormat::WebP];

    pub fn display_name(&self) -> &'static str {
        match self {
            CaptureFormat::Png => "PNG (Lossless)",
            CaptureFormat::Jpeg => "JPEG (Standard)",
            CaptureFormat::WebP => "WebP (Modern)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConfig {
    pub format: CaptureFormat,
    pub output_dir: PathBuf,
    pub burst_interval_secs: f64,
    pub burst_interval_sec: f64,
    pub burst_count: u32,
    pub contact_sheet_cols: u32,
    pub contact_sheet_rows: u32,
    pub recording_codec: String,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        let dir = dirs::picture_dir().unwrap_or_else(|| PathBuf::from(".")).join("VertexPlayer Captures");
        Self {
            format: CaptureFormat::Png,
            output_dir: dir,
            burst_interval_secs: 1.0,
            burst_interval_sec: 1.0,
            burst_count: 10,
            contact_sheet_cols: 4,
            contact_sheet_rows: 4,
            recording_codec: "Direct Copy (Lossless)".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BurstCaptureEngine {
    pub is_running: bool,
    pub frame_counter: u64,
    pub interval_sec: f64,
    pub output_dir: PathBuf,
}

impl Default for BurstCaptureEngine {
    fn default() -> Self {
        Self {
            is_running: false,
            frame_counter: 0,
            interval_sec: 1.0,
            output_dir: PathBuf::from("."),
        }
    }
}

impl BurstCaptureEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.is_running = !self.is_running;
        if self.is_running {
            self.frame_counter = 0;
        }
    }
}
