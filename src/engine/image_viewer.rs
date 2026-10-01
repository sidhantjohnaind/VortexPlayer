#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageViewerConfig {
    pub slideshow_interval_sec: f64,
    pub auto_rotate_exif: bool,
    pub loop_slideshow: bool,
}

impl Default for ImageViewerConfig {
    fn default() -> Self {
        Self {
            slideshow_interval_sec: 5.0,
            auto_rotate_exif: true,
            loop_slideshow: true,
        }
    }
}

pub struct ImageViewerEngine;

impl ImageViewerEngine {
    pub const IMAGE_EXTENSIONS: [&'static str; 10] = [
        "jpg", "jpeg", "png", "webp", "avif", "heic", "gif", "apng", "bmp", "svg",
    ];

    pub fn is_image_file(path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            Self::IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
        } else {
            false
        }
    }
}
