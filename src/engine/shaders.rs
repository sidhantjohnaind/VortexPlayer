#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoScaler {
    Lanczos,
    Spline36,
    Mitchell,
    EwaLanczos, // Jinc
    Bicubic,
    Bilinear,
}

impl Default for VideoScaler {
    fn default() -> Self {
        Self::Lanczos
    }
}

impl VideoScaler {
    pub const ALL: [VideoScaler; 6] = [
        VideoScaler::Lanczos,
        VideoScaler::Spline36,
        VideoScaler::Mitchell,
        VideoScaler::EwaLanczos,
        VideoScaler::Bicubic,
        VideoScaler::Bilinear,
    ];

    pub fn to_mpv_str(&self) -> &'static str {
        match self {
            Self::Lanczos => "lanczos",
            Self::Spline36 => "spline36",
            Self::Mitchell => "mitchell",
            Self::EwaLanczos => "ewa_lanczos",
            Self::Bicubic => "bicubic",
            Self::Bilinear => "bilinear",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Lanczos => "Lanczos (3-tap High Sharpness)",
            Self::Spline36 => "Spline36 (Artifact-Free Balanced)",
            Self::Mitchell => "Mitchell-Netravali (Smooth Film)",
            Self::EwaLanczos => "EWA Lanczos / Jinc (Ultra Quality)",
            Self::Bicubic => "Bicubic (Standard)",
            Self::Bilinear => "Bilinear (Fast / Low GPU)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Anime4kPreset {
    Off,
    ModeA, // Fast Upscale & Reconstruct
    ModeB, // Soft Blur Reduction
    ModeC, // High Fidelity Line Art Restoration
}

impl Anime4kPreset {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::ModeA => "Anime4K Mode A (Fast)",
            Self::ModeB => "Anime4K Mode B (HQ)",
            Self::ModeC => "Anime4K Mode C (Ultra)",
        }
    }
}

impl Default for Anime4kPreset {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderConfig {
    pub scale: VideoScaler,
    pub cscale: VideoScaler, // Chroma scaler
    pub dscale: VideoScaler, // Downscaler
    pub anime4k: Anime4kPreset,
    pub cas_sharpen: bool,   // Contrast Adaptive Sharpening
    pub custom_glsl: Vec<String>,
}

impl Default for ShaderConfig {
    fn default() -> Self {
        Self {
            scale: VideoScaler::Lanczos,
            cscale: VideoScaler::Spline36,
            dscale: VideoScaler::Mitchell,
            anime4k: Anime4kPreset::Off,
            cas_sharpen: false,
            custom_glsl: Vec::new(),
        }
    }
}
