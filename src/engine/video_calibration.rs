#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCalibrationConfig {
    pub lut_3d_path: Option<PathBuf>,
    pub icc_profile_path: Option<PathBuf>,
    pub icc_auto: bool,
    pub auto_crop_black_bars: bool,
    pub deband_enabled: bool,
    pub deband_iterations: u32,
    pub denoise_nlmeans: f64,
}

impl Default for VideoCalibrationConfig {
    fn default() -> Self {
        Self {
            lut_3d_path: None,
            icc_profile_path: None,
            icc_auto: true,
            auto_crop_black_bars: false,
            deband_enabled: false,
            deband_iterations: 4,
            denoise_nlmeans: 0.0,
        }
    }
}

impl VideoCalibrationConfig {
    pub fn build_video_filter_string(&self) -> String {
        let mut filters = Vec::new();
        if self.auto_crop_black_bars {
            filters.push("cropdetect=24:16:0".to_string());
        }
        if self.deband_enabled {
            filters.push(format!("deband=iterations={}:threshold=48:range=16", self.deband_iterations));
        }
        if self.denoise_nlmeans > 0.01 {
            filters.push(format!("nlmeans=s={:.2}:p=5:r=9", self.denoise_nlmeans * 3.0 + 1.0));
        }
        filters.join(",")
    }
}
