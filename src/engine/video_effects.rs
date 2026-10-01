#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// ── Deinterlace mode ─────────────────────────────────────────────────────────

/// Selects the deinterlacing algorithm passed to the video filter graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeinterlaceMode {
    /// Deinterlacing disabled.
    Off,
    /// Yadif (Yet Another DeInterlacing Filter) — good quality, low CPU.
    Yadif,
    /// W3FDIF — Weston 3-Field Deinterlacing Filter, high quality.
    W3FDIF,
    /// Bwdif — Bob Weaver Deinterlacing Filter, best quality.
    Bwdif,
}

impl Default for DeinterlaceMode {
    fn default() -> Self {
        Self::Off
    }
}

impl DeinterlaceMode {
    /// Returns the ffmpeg filter string fragment for this mode, or `None` when
    /// deinterlacing is disabled.
    pub fn filter_str(&self) -> Option<&'static str> {
        match self {
            Self::Off => None,
            Self::Yadif => Some("yadif=mode=0"),
            Self::W3FDIF => Some("w3fdif"),
            Self::Bwdif => Some("bwdif=mode=0"),
        }
    }
}

// ── VideoRotation ────────────────────────────────────────────────────────────

/// Discrete rotation steps expressed in degrees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoRotation {
    Deg0 = 0,
    Deg90 = 90,
    Deg180 = 180,
    Deg270 = 270,
}

impl Default for VideoRotation {
    fn default() -> Self {
        Self::Deg0
    }
}

// ── AspectRatioMode ──────────────────────────────────────────────────────────

/// Display aspect-ratio override modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectRatioMode {
    Auto,
    Ratio16x9,
    Ratio4x3,
    Ratio1_85x1,
    Ratio2_35x1,
    FitWindow,
    Stretch,
}

impl Default for AspectRatioMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl AspectRatioMode {
    /// Returns the mpv `--video-aspect-override` value for this mode.
    pub fn as_mpv_str(&self) -> &'static str {
        match self {
            Self::Auto => "no",
            Self::Ratio16x9 => "16:9",
            Self::Ratio4x3 => "4:3",
            Self::Ratio1_85x1 => "1.85:1",
            Self::Ratio2_35x1 => "2.35:1",
            Self::FitWindow => "no",
            Self::Stretch => "-1",
        }
    }
}

// ── Stereoscopic 3D ─────────────────────────────────────────────────────────

/// 3D video decoding and anaglyph presentation modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stereo3dMode {
    Off,
    SideBySideToAnaglyphRedCyan,
    TopBottomToAnaglyphRedCyan,
    SideBySideToAnaglyphGreenMagenta,
    SideBySideTo2D,
    TopBottomTo2D,
    SwapEyes,
}

impl Default for Stereo3dMode {
    fn default() -> Self {
        Self::Off
    }
}

impl Stereo3dMode {
    /// Returns the ffmpeg stereo3d filter string for this mode.
    pub fn filter_str(&self) -> Option<&'static str> {
        match self {
            Self::Off => None,
            Self::SideBySideToAnaglyphRedCyan => Some("stereo3d=sbsl:arcd"),
            Self::TopBottomToAnaglyphRedCyan => Some("stereo3d=abl:arcd"),
            Self::SideBySideToAnaglyphGreenMagenta => Some("stereo3d=sbsl:al"),
            Self::SideBySideTo2D => Some("stereo3d=sbsl:mono"),
            Self::TopBottomTo2D => Some("stereo3d=abl:mono"),
            Self::SwapEyes => Some("stereo3d=sbsl:sbsr"),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Off => "Disabled (2D)",
            Self::SideBySideToAnaglyphRedCyan => "Side-by-Side → Red/Cyan 3D",
            Self::TopBottomToAnaglyphRedCyan => "Top-Bottom → Red/Cyan 3D",
            Self::SideBySideToAnaglyphGreenMagenta => "Side-by-Side → Green/Magenta 3D",
            Self::SideBySideTo2D => "Side-by-Side → 2D (Left Eye)",
            Self::TopBottomTo2D => "Top-Bottom → 2D (Top Eye)",
            Self::SwapEyes => "Side-by-Side Swap Eyes (R/L)",
        }
    }
}

// ── VideoEffectsConfig ───────────────────────────────────────────────────────

/// All video post-processing settings for a single playback session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoEffectsConfig {
    // ── Colour adjustments ────────────────────────────────────────────────
    /// Brightness offset.  Range: -100.0 to +100.0.  Default: 0.0.
    pub brightness: f64,
    /// Contrast factor.  Range: -100.0 to +100.0.  Default: 0.0.
    pub contrast: f64,
    /// Colour saturation.  Range: -100.0 to +100.0.  Default: 0.0.
    pub saturation: f64,
    /// Hue rotation in degrees.  Range: -100.0 to +100.0.  Default: 0.0.
    pub hue: f64,
    /// Gamma correction.  Range: -100.0 to +100.0.  Default: 0.0.
    pub gamma: f64,

    // ── Geometry ──────────────────────────────────────────────────────────
    /// Clockwise rotation step applied to the video.
    pub rotation: i32,
    /// Zoom multiplier applied around the frame centre.  Range: 0.5 to 4.0.  Default: 1.0.
    pub zoom: f64,
    /// Pan-and-scan expansion factor.  Range: 0.0 to 1.0.  Default: 0.0.
    pub pan_scan: f64,

    // ── Deinterlacing ─────────────────────────────────────────────────────
    /// Deinterlace algorithm to apply.
    pub deinterlace: DeinterlaceMode,

    // ── Sharpening ────────────────────────────────────────────────────────
    /// Unsharp mask intensity.  Range: 0.0 to 1.5.  Default: 0.0.
    /// Maps to the `unsharp` ffmpeg filter.
    pub sharpen: f64,

    // ── Noise reduction ───────────────────────────────────────────────────
    /// Non-local means denoising strength.  Range: 0.0 to 1.0.  Default: 0.0.
    /// Maps to the `nlmeans` ffmpeg filter.
    pub denoise: f64,

    // ── Colour temperature ────────────────────────────────────────────────
    /// Colour temperature shift.  Negative = cooler (blueish), positive = warmer (orange).
    /// Range: -50.0 to +50.0.  Default: 0.0.
    /// Implemented via `colorchannelmixer` to tint R/B channels.
    pub color_temp: f64,

    // ── Vignette & Flip & Deband ───────────────────────────────────────
    /// Overlay a natural lens vignette at π/4 angle.  Default: false.
    pub vignette: bool,
    /// Flip video horizontally (mirror effect). Default: false.
    pub flip_horizontal: bool,
    /// Flip video vertically (upside down). Default: false.
    pub flip_vertical: bool,
    /// Deband filter to reduce 8-bit color banding artifacts. Default: false.
    pub deband: bool,

    // ── 3D LUT & Cropping ───────────────────────────────────────────────
    /// External 3D LUT file (.cube) for cinematic color grading/calibration.
    pub lut_file: Option<String>,
    /// Cropping margins (pixels).
    pub crop_left: u32,
    pub crop_right: u32,
    pub crop_top: u32,
    pub crop_bottom: u32,
    /// Auto-detect and crop black letterbox bars.
    pub auto_crop_black_bars: bool,

    // ── Stereoscopic 3D ─────────────────────────────────────────────────
    /// Stereoscopic 3D decoding/presentation mode.
    pub stereo3d: Stereo3dMode,

    // ── Motion Smoothing (Interpolation) ─────────────────────────────────
    /// Enable smooth frame interpolation matching monitor refresh rate.
    pub motion_interpolation: bool,
    /// Motion interpolation algorithm (`oversample`, `mitchell`, `bicubic`, `catmull_rom`).
    pub interpolation_tscale: String,
}

impl Default for VideoEffectsConfig {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 0.0,
            saturation: 0.0,
            hue: 0.0,
            gamma: 0.0,
            rotation: 0,
            zoom: 1.0,
            pan_scan: 0.0,
            deinterlace: DeinterlaceMode::Off,
            sharpen: 0.0,
            denoise: 0.0,
            color_temp: 0.0,
            vignette: false,
            flip_horizontal: false,
            flip_vertical: false,
            deband: false,
            lut_file: None,
            crop_left: 0,
            crop_right: 0,
            crop_top: 0,
            crop_bottom: 0,
            auto_crop_black_bars: false,
            stereo3d: Stereo3dMode::Off,
            motion_interpolation: false,
            interpolation_tscale: "oversample".to_string(),
        }
    }
}

impl VideoEffectsConfig {
    /// Build an `ffmpeg`/`mpv` compatible video filter chain string.
    ///
    /// Returns an empty string when no filters are active so the caller can
    /// skip passing `--vf` to mpv entirely.
    pub fn build_video_filter_string(&self) -> String {
        let mut filters: Vec<String> = Vec::new();

        // ── Cropping ────────────────────────────────────────────────────
        if self.auto_crop_black_bars {
            filters.push("cropdetect=24:16:0".to_string());
        } else if self.crop_left > 0 || self.crop_right > 0 || self.crop_top > 0 || self.crop_bottom > 0 {
            filters.push(format!(
                "crop=in_w-{l}-{r}:in_h-{t}-{b}:{l}:{t}",
                l = self.crop_left,
                r = self.crop_right,
                t = self.crop_top,
                b = self.crop_bottom
            ));
        }

        // ── Deinterlace ──────────────────────────────────────────────────
        if let Some(f) = self.deinterlace.filter_str() {
            filters.push(f.to_string());
        }

        // ── Sharpening (unsharp mask) ────────────────────────────────────
        if self.sharpen > 0.01 {
            let s = (self.sharpen * 1.5).clamp(0.01, 2.5);
            filters.push(format!("unsharp=la={s:.2}:ca={s:.2}", s = s));
        }

        // ── Denoise (nlmeans) ────────────────────────────────────────────
        if self.denoise > 0.01 {
            let d = (self.denoise * 3.0 + 1.0).clamp(1.0, 4.0);
            filters.push(format!("nlmeans=s={d:.2}:p=5:r=9", d = d));
        }

        // ── Colour temperature ───────────────────────────────────────────
        if self.color_temp.abs() > 0.5 {
            let t = self.color_temp.clamp(-50.0, 50.0) / 50.0; // normalise to -1..1
            let r = (1.0 + t * 0.15).clamp(0.0, 2.0);           // +15 % at max warm
            let b = (1.0 - t * 0.15).clamp(0.0, 2.0);           // -15 % at max warm
            filters.push(format!(
                "colorchannelmixer=rr={r:.4}:bb={b:.4}",
                r = r,
                b = b
            ));
        }

        // ── Debanding ───────────────────────────────────────────────────
        if self.deband {
            filters.push("gradfun=1.2:16".to_string());
        }

        // ── 3D LUT Color Calibration ─────────────────────────────────────
        if let Some(ref lut) = self.lut_file {
            if !lut.is_empty() {
                let clean_path = lut.replace('\\', "/");
                filters.push(format!("lut3d=file='{}'", clean_path));
            }
        }

        // ── Vignette ────────────────────────────────────────────────────
        if self.vignette {
            filters.push("vignette=PI/4".to_string());
        }

        // ── Flips ───────────────────────────────────────────────────────
        if self.flip_horizontal {
            filters.push("hflip".to_string());
        }
        if self.flip_vertical {
            filters.push("vflip".to_string());
        }

        filters.join(",")
    }

    // ── Convenience reset helpers ────────────────────────────────────────────

    /// Reset all colour-related parameters to their defaults.
    pub fn reset_colors(&mut self) {
        self.brightness = 0.0;
        self.contrast = 0.0;
        self.saturation = 0.0;
        self.hue = 0.0;
        self.gamma = 0.0;
        self.color_temp = 0.0;
    }

    /// Reset all geometry-related parameters to their defaults.
    pub fn reset_geometry(&mut self) {
        self.rotation = 0;
        self.zoom = 1.0;
        self.pan_scan = 0.0;
    }

    /// Reset all post-processing (sharpen, denoise, vignette) to defaults.
    pub fn reset_processing(&mut self) {
        self.sharpen = 0.0;
        self.denoise = 0.0;
        self.vignette = false;
        self.deinterlace = DeinterlaceMode::Off;
    }

    /// Reset every field to its default value.
    pub fn reset_all(&mut self) {
        *self = Self::default();
    }
}
