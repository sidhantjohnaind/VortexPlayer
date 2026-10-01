#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SilenceSkipConfig {
    pub enabled: bool,
    pub threshold_db: f64,     // e.g. -45.0 dB
    pub min_duration_sec: f64, // e.g. 0.5 sec
    pub speed_boost_factor: f64, // e.g. 2.0x
}

impl Default for SilenceSkipConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            threshold_db: -45.0,
            min_duration_sec: 0.5,
            speed_boost_factor: 2.0,
        }
    }
}

impl SilenceSkipConfig {
    pub fn build_filter_string(&self) -> String {
        if !self.enabled {
            return String::new();
        }
        format!(
            "silenceremove=stop_periods=-1:stop_duration={:.2}:stop_threshold={:.1}dB",
            self.min_duration_sec, self.threshold_db
        )
    }
}
