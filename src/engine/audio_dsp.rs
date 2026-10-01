#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelDownmixMode {
    Auto,
    Stereo,
    Surround51,
    Surround71,
    Mono,
}

impl Default for ChannelDownmixMode {
    fn default() -> Self {
        Self::Auto
    }
}

pub struct EqPresetItem {
    pub name: &'static str,
    pub bands: &'static [f64],
}

/// The 18 canonical EQ centre frequencies in Hz.
pub const FREQS: [f64; 18] = [
    20.0, 31.0, 50.0, 80.0, 125.0, 200.0, 315.0, 500.0, 800.0, 1250.0, 2000.0, 3150.0, 5000.0,
    8000.0, 12500.0, 16000.0, 18000.0, 20000.0,
];

pub const POT_EQ_PRESETS: &[EqPresetItem] = &[
    EqPresetItem { name: "Flat", bands: &[0.0; 18] },
    EqPresetItem { name: "Rock", bands: &[2.0, 3.0, 4.0, 2.0, 0.0, -1.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0, 1.0, 0.0, 0.0] },
    EqPresetItem { name: "Pop", bands: &[-1.0, 0.0, 2.0, 3.0, 3.0, 2.0, 0.0, -1.0, -1.0, 0.0, 1.0, 2.0, 2.0, 1.0, 0.0, -1.0, 0.0, 0.0] },
    EqPresetItem { name: "Classical", bands: &[3.0, 4.0, 4.0, 3.0, 2.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0] },
    EqPresetItem { name: "Movie", bands: &[2.0, 3.0, 3.0, 2.0, 1.0, 0.0, 0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0] },
    EqPresetItem { name: "Bass Boost", bands: &[6.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] },
    EqPresetItem { name: "Voice Enhance", bands: &[-2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 4.0, 5.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -2.0, -2.0] },
    EqPresetItem { name: "Techno", bands: &[4.0, 4.0, 3.0, 2.0, 0.0, -1.0, -1.0, 0.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0] },
    EqPresetItem { name: "Night Mode", bands: &[3.0, 3.0, 2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, -2.0, -3.0, -4.0, -4.0, -4.0, -3.0, -2.0] },
];

/// Built-in EQ presets.  Each entry is `(name, [gain_per_band; 18])` where gains
/// are in dB, clamped to -12.0 … +12.0 by `build_audio_filter_string`.
pub const EQ_PRESETS: &[(&str, [f64; 18])] = &[
    (
        "Flat",
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    ),
    (
        "Rock",
        [2.0, 3.0, 4.0, 2.0, 0.0, -1.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0, 1.0, 0.0, 0.0],
    ),
    (
        "Pop",
        [-1.0, 0.0, 2.0, 3.0, 3.0, 2.0, 0.0, -1.0, -1.0, 0.0, 1.0, 2.0, 2.0, 1.0, 0.0, -1.0, 0.0, 0.0],
    ),
    (
        "Classical",
        [3.0, 4.0, 4.0, 3.0, 2.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0],
    ),
    (
        "Movie",
        [2.0, 3.0, 3.0, 2.0, 1.0, 0.0, 0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0],
    ),
    (
        "Bass Boost",
        [6.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    ),
    (
        "Voice Enhance",
        [-2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 4.0, 5.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -2.0, -2.0],
    ),
    (
        "Techno",
        [4.0, 4.0, 3.0, 2.0, 0.0, -1.0, -1.0, 0.0, 2.0, 3.0, 4.0, 4.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0],
    ),
    (
        "Night Mode",
        [3.0, 3.0, 2.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, -2.0, -3.0, -4.0, -4.0, -4.0, -3.0, -2.0],
    ),
];

/// Runtime DSP configuration for the audio pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDspConfig {
    /// Master enable switch for the equalizer.
    pub enabled: bool,

    /// Per-band gain values in dB (-12.0 to +12.0).  Must have exactly 18 elements
    /// matching the `FREQS` constant.
    pub bands: Vec<f64>,

    /// Enable dynamic audio normalisation (`dynaudnorm`).
    pub normalize: bool,

    /// Stereo widening factor (0.0 = no effect, 2.0 = maximum widening).
    pub stereo_widen: f64,

    /// Crossfeed for headphone listening (`bs2b`).
    pub crossfeed: bool,

    /// Audio track delay in seconds (positive = delay audio, negative = advance audio).
    pub audio_delay: f64,

    /// Karaoke Center-Channel Vocal Remover filter.
    pub vocal_remover: bool,

    /// Dialogue & Speech Clarity Enhancer filter.
    pub voice_enhance: bool,

    /// Pitch shift in semitones (-12 to +12).
    pub pitch_semitones: f64,

    /// Audiophile Soxr Resampler rate (0 = disabled, 48000, 96000, 192000).
    pub resample_rate: u32,

    /// ReplayGain normalization mode ("no", "track", "album").
    pub replaygain_mode: String,

    /// ReplayGain pre-amp gain in dB.
    pub replaygain_preamp: f64,

    /// Name of the currently active preset (informational – does not drive DSP directly).
    pub preset: String,
}

impl Default for AudioDspConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bands: vec![0.0; 18],
            normalize: false,
            stereo_widen: 0.0,
            crossfeed: false,
            audio_delay: 0.0,
            vocal_remover: false,
            voice_enhance: false,
            pitch_semitones: 0.0,
            resample_rate: 0,
            replaygain_mode: "no".to_string(),
            replaygain_preamp: 0.0,
            preset: "Flat".to_string(),
        }
    }
}

impl AudioDspConfig {
    /// Build an `ffmpeg`/`mpv` compatible audio filter chain string.
    ///
    /// Returns an empty string when no processing is needed so the caller can
    /// skip passing `--af` to mpv entirely.
    pub fn build_audio_filter_string(&self) -> String {
        let mut filters: Vec<String> = Vec::new();

        // ── 18-band parametric equalizer ────────────────────────────────────
        if self.enabled {
            for (i, &gain) in self.bands.iter().enumerate().take(FREQS.len()) {
                if gain.abs() > 0.05 {
                    let freq = FREQS[i];
                    let g = gain.clamp(-12.0, 12.0);
                    filters.push(format!(
                        "equalizer=f={freq:.1}:width_type=o:w=1:g={g:.1}",
                        freq = freq,
                        g = g
                    ));
                }
            }
        }

        // ── Pitch Shifter (Independent of tempo) ─────────────────────────────
        if self.pitch_semitones.abs() > 0.05 {
            let scale = (2.0f64).powf(self.pitch_semitones / 12.0);
            filters.push(format!("rubberband=pitch-scale={:.4}", scale));
        }

        // ── Audiophile Soxr Resampler ────────────────────────────────────────
        if self.resample_rate > 0 {
            filters.push(format!("aresample={}:resampler=soxr", self.resample_rate));
        }

        // ── ReplayGain Normalization ─────────────────────────────────────────
        if self.replaygain_mode != "no" && !self.replaygain_mode.is_empty() {
            filters.push(format!("replaygain={}", self.replaygain_mode));
        }

        // ── Dynamic audio normalisation ──────────────────────────────────────
        if self.normalize {
            filters.push("dynaudnorm=p=0.9:s=5".to_string());
        }

        // ── Stereo widening ──────────────────────────────────────────────────
        if self.stereo_widen > 0.01 {
            let val = self.stereo_widen.clamp(0.0, 2.0);
            filters.push(format!("extrastereo=m={val:.2}", val = val));
        }

        // ── Headphone Crossfeed (bs2b) ────────────────────────────────────────
        if self.crossfeed {
            filters.push("bs2b=profile=default".to_string());
        }

        // ── Voice / Speech Enhancement ──────────────────────────────────────
        if self.voice_enhance {
            filters.push("highpass=f=120,equalizer=f=3000:width_type=o:w=1.5:g=4.0,dynaudnorm=p=0.9:s=3".to_string());
        }

        // ── Karaoke Center-Channel Vocal Remover ─────────────────────────────
        if self.vocal_remover {
            filters.push("stereotools=mlev=0:slev=1.5".to_string());
        }

        filters.join(",")
    }

    /// Apply a named preset by copying its band gains into `self.bands`.
    ///
    /// If `name` does not match any known preset the bands are left unchanged.
    pub fn apply_preset(&mut self, name: &str) {
        if let Some(&(_, ref gains)) = EQ_PRESETS
            .iter()
            .find(|(preset_name, _)| preset_name.eq_ignore_ascii_case(name))
        {
            self.bands = gains.to_vec();
            self.preset = name.to_string();
        }
    }
}
