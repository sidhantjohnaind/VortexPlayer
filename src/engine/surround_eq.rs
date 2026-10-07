use serde::{Deserialize, Serialize};

pub const SURROUND_EQ_FREQS: [f64; 10] = [
    31.25, 62.5, 125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0, 16000.0,
];

pub const SURROUND_EQ_LABELS: [&str; 10] = [
    "31Hz", "63Hz", "125Hz", "250Hz", "500Hz", "1kHz", "2kHz", "4kHz", "8kHz", "16kHz",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelId {
    FL = 0,
    FR = 1,
    FC = 2,
    LFE = 3,
    SL = 4,
    SR = 5,
    BL = 6,
    BR = 7,
}

impl ChannelId {
    pub const ALL: [ChannelId; 8] = [
        ChannelId::FL,
        ChannelId::FR,
        ChannelId::FC,
        ChannelId::LFE,
        ChannelId::SL,
        ChannelId::SR,
        ChannelId::BL,
        ChannelId::BR,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::FL => "Front Left (FL)",
            Self::FR => "Front Right (FR)",
            Self::FC => "Center (FC / Speech)",
            Self::LFE => "Subwoofer (LFE / Bass)",
            Self::SL => "Surround Left (SL)",
            Self::SR => "Surround Right (SR)",
            Self::BL => "Rear Left (BL)",
            Self::BR => "Rear Right (BR)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::FL => "FL",
            Self::FR => "FR",
            Self::FC => "FC",
            Self::LFE => "LFE",
            Self::SL => "SL",
            Self::SR => "SR",
            Self::BL => "BL",
            Self::BR => "BR",
        }
    }

    pub fn ffmpeg_code(&self) -> &'static str {
        match self {
            Self::FL => "FL",
            Self::FR => "FR",
            Self::FC => "FC",
            Self::LFE => "LFE",
            Self::SL => "SL",
            Self::SR => "SR",
            Self::BL => "BL",
            Self::BR => "BR",
        }
    }

    pub fn stereo_pair_index(&self) -> Option<usize> {
        match self {
            Self::FL => Some(ChannelId::FR as usize),
            Self::FR => Some(ChannelId::FL as usize),
            Self::SL => Some(ChannelId::SR as usize),
            Self::SR => Some(ChannelId::SL as usize),
            Self::BL => Some(ChannelId::BR as usize),
            Self::BR => Some(ChannelId::BL as usize),
            Self::FC | Self::LFE => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelEq {
    pub enabled: bool,
    pub gain_offset_db: f64,
    pub bands: [f64; 10],
}

impl Default for ChannelEq {
    fn default() -> Self {
        Self {
            enabled: true,
            gain_offset_db: 0.0,
            bands: [0.0; 10],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurroundEqConfig {
    pub enabled: bool,
    pub link_stereo_pairs: bool,
    pub channels: [ChannelEq; 8],
    pub profile_name: String,
}

impl Default for SurroundEqConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            link_stereo_pairs: true,
            channels: [
                ChannelEq::default(), // FL
                ChannelEq::default(), // FR
                ChannelEq::default(), // FC
                ChannelEq::default(), // LFE
                ChannelEq::default(), // SL
                ChannelEq::default(), // SR
                ChannelEq::default(), // BL
                ChannelEq::default(), // BR
            ],
            profile_name: "Custom 7.1 Curve".to_string(),
        }
    }
}

impl SurroundEqConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build an FFmpeg audio filter string for mpv af property with per-channel equalizers.
    pub fn build_audio_filter_string(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let mut filters = Vec::new();

        for (ch_idx, ch) in self.channels.iter().enumerate() {
            if !ch.enabled {
                continue;
            }
            let code = ChannelId::ALL[ch_idx].ffmpeg_code();

            for (band_idx, &gain) in ch.bands.iter().enumerate() {
                let total_gain = (gain + ch.gain_offset_db).clamp(-18.0, 18.0);
                if total_gain.abs() > 0.05 {
                    let freq = SURROUND_EQ_FREQS[band_idx];
                    filters.push(format!(
                        "equalizer=f={:.1}:width_type=o:w=1:g={:.1}:c={}",
                        freq, total_gain, code
                    ));
                }
            }
        }

        filters.join(",")
    }

    /// Reset all 8 channels to 0.0 dB flat response
    pub fn reset_all(&mut self) {
        for ch in &mut self.channels {
            ch.gain_offset_db = 0.0;
            ch.bands = [0.0; 10];
        }
        self.profile_name = "Flat".to_string();
    }

    /// Apply preset tailored specifically for 7.1 surround sound
    pub fn apply_preset(&mut self, preset_name: &str) {
        match preset_name {
            "Cinema Dialogue Focus" => {
                self.reset_all();
                // Boost Center speech frequencies, gently tame bass on Center
                let fc = &mut self.channels[ChannelId::FC as usize];
                fc.bands[0] = -2.0; // 31Hz
                fc.bands[1] = -1.5; // 63Hz
                fc.bands[5] = 2.0;  // 1kHz
                fc.bands[6] = 3.5;  // 2kHz
                fc.bands[7] = 3.0;  // 4kHz
                self.profile_name = "Cinema Dialogue Focus".to_string();
            }
            "Subwoofer Bass Slam" => {
                self.reset_all();
                // Focus low-end energy on LFE Subwoofer channel
                let lfe = &mut self.channels[ChannelId::LFE as usize];
                lfe.bands[0] = 4.0;  // 31Hz
                lfe.bands[1] = 6.0;  // 63Hz
                lfe.bands[2] = 3.0;  // 125Hz
                lfe.bands[3] = -2.0; // 250Hz roll-off
                lfe.bands[4] = -5.0; // 500Hz
                self.profile_name = "Subwoofer Bass Slam".to_string();
            }
            "Action Movie Immersion" => {
                self.reset_all();
                // Enhanced Center speech clarity
                let fc = &mut self.channels[ChannelId::FC as usize];
                fc.bands[5] = 2.0; // 1kHz
                fc.bands[6] = 3.0; // 2kHz
                fc.bands[7] = 2.5; // 4kHz
                // Powerful LFE impact
                let lfe = &mut self.channels[ChannelId::LFE as usize];
                lfe.bands[0] = 3.5;
                lfe.bands[1] = 5.0;
                lfe.bands[2] = 2.0;
                // Atmospheric highs on Surrounds & Rears
                for idx in [ChannelId::SL as usize, ChannelId::SR as usize, ChannelId::BL as usize, ChannelId::BR as usize] {
                    self.channels[idx].bands[8] = 2.0; // 8kHz
                    self.channels[idx].bands[9] = 2.5; // 16kHz
                }
                self.profile_name = "Action Movie Immersion".to_string();
            }
            "Night Mode (Quiet Bass / Clear Speech)" => {
                self.reset_all();
                // Restrict heavy rumble on LFE to prevent waking household
                let lfe = &mut self.channels[ChannelId::LFE as usize];
                lfe.bands[0] = -8.0;
                lfe.bands[1] = -6.0;
                lfe.bands[2] = -3.0;
                // Boost Center dialogue presence
                let fc = &mut self.channels[ChannelId::FC as usize];
                fc.bands[5] = 3.0;
                fc.bands[6] = 4.0;
                fc.bands[7] = 3.0;
                self.profile_name = "Night Mode (Quiet Bass)".to_string();
            }
            "Concert Hall & Live Acoustic" => {
                self.reset_all();
                // Warm, open acoustic curve on Fronts
                for idx in [ChannelId::FL as usize, ChannelId::FR as usize] {
                    self.channels[idx].bands[1] = 1.5; // 63Hz warmth
                    self.channels[idx].bands[8] = 2.0; // 8kHz sparkle
                    self.channels[idx].bands[9] = 2.5; // 16kHz air
                }
                // Ambient spatial fill on Surrounds
                for idx in [ChannelId::SL as usize, ChannelId::SR as usize] {
                    self.channels[idx].bands[4] = 1.0;
                    self.channels[idx].bands[5] = 1.0;
                    self.channels[idx].bands[8] = 1.5;
                }
                self.profile_name = "Concert Hall & Live Acoustic".to_string();
            }
            _ => {
                self.reset_all();
            }
        }
    }
}
