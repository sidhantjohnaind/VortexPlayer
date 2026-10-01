#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceProfile {
    BatterySaver,
    BalancedQuality,
    UltimateFidelity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HwCapabilities {
    pub gpu_name: String,
    pub nvdec_supported: bool,
    pub qsv_supported: bool,
    pub amf_supported: bool,
    pub d3d11va_supported: bool,
    pub hdr10_display_connected: bool,
    pub recommended_profile: PerformanceProfile,
}

impl Default for HwCapabilities {
    fn default() -> Self {
        Self {
            gpu_name: "Auto-detected GPU".to_string(),
            nvdec_supported: true,
            qsv_supported: true,
            amf_supported: true,
            d3d11va_supported: true,
            hdr10_display_connected: true,
            recommended_profile: PerformanceProfile::UltimateFidelity,
        }
    }
}

impl HwCapabilities {
    pub fn probe() -> Self {
        Self::default()
    }
}
