#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailSheetConfig {
    pub rows: u32,               // e.g. 4
    pub cols: u32,               // e.g. 4
    pub output_format: String,   // "jpg", "png", "webp"
    pub quality: u32,            // 1-100 (90 default)
    pub output_dir: PathBuf,
    pub include_timestamp_badge: bool,
    pub include_file_info_header: bool,
    pub width: u32,              // Sheet total width (e.g. 1920)
}

impl Default for ThumbnailSheetConfig {
    fn default() -> Self {
        let pic_dir = dirs::picture_dir().unwrap_or_else(|| PathBuf::from("."));
        Self {
            rows: 4,
            cols: 4,
            output_format: "jpg".to_string(),
            quality: 90,
            output_dir: pic_dir,
            include_timestamp_badge: true,
            include_file_info_header: true,
            width: 1920,
        }
    }
}

impl ThumbnailSheetConfig {
    pub fn calculate_timestamps(&self, duration_secs: f64) -> Vec<f64> {
        let total_frames = self.rows * self.cols;
        if total_frames == 0 || duration_secs <= 0.0 {
            return Vec::new();
        }

        let step = duration_secs / (total_frames as f64 + 1.0);
        (1..=total_frames)
            .map(|i| (i as f64) * step)
            .collect()
    }
}
