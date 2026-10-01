#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterType {
    Peak,
    LowShelf,
    HighShelf,
    LowPass,
    HighPass,
    BandPass,
    Notch,
    AllPass,
}

impl FilterType {
    pub const ALL: [FilterType; 8] = [
        FilterType::Peak,
        FilterType::LowShelf,
        FilterType::HighShelf,
        FilterType::LowPass,
        FilterType::HighPass,
        FilterType::BandPass,
        FilterType::Notch,
        FilterType::AllPass,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Peak => "Peak / Bell",
            Self::LowShelf => "Low Shelf",
            Self::HighShelf => "High Shelf",
            Self::LowPass => "Low Pass",
            Self::HighPass => "High Pass",
            Self::BandPass => "Band Pass",
            Self::Notch => "Notch",
            Self::AllPass => "All Pass",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricNode {
    pub enabled: bool,
    pub filter_type: FilterType,
    pub freq: f64,    // 10.0 to 22000.0 Hz
    pub gain: f64,    // -24.0 to +24.0 dB
    pub q: f64,       // 0.1 to 10.0 (Q factor or slope)
}

impl Default for ParametricNode {
    fn default() -> Self {
        Self {
            enabled: true,
            filter_type: FilterType::Peak,
            freq: 1000.0,
            gain: 0.0,
            q: 1.414,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParametricEqConfig {
    pub enabled: bool,
    pub preamp_gain: f64, // -20.0 to +20.0 dB
    pub nodes: Vec<ParametricNode>,
    pub profile_name: String,
}

impl Default for ParametricEqConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            preamp_gain: 0.0,
            nodes: vec![
                ParametricNode { enabled: true, filter_type: FilterType::LowShelf, freq: 105.0, gain: 2.5, q: 0.71 },
                ParametricNode { enabled: true, filter_type: FilterType::Peak, freq: 1200.0, gain: -1.5, q: 1.41 },
                ParametricNode { enabled: true, filter_type: FilterType::Peak, freq: 3000.0, gain: 2.0, q: 2.0 },
                ParametricNode { enabled: true, filter_type: FilterType::HighShelf, freq: 8000.0, gain: -1.0, q: 0.71 },
            ],
            profile_name: "Default Curve".to_string(),
        }
    }
}

impl ParametricEqConfig {
    /// Build an FFmpeg audio filter string for mpv af property.
    pub fn build_audio_filter_string(&self) -> String {
        if !self.enabled || self.nodes.is_empty() {
            return String::new();
        }

        let mut filters = Vec::new();

        if self.preamp_gain.abs() > 0.05 {
            filters.push(format!("volume=volume={:.1}dB:precision=fixed", self.preamp_gain));
        }

        for node in &self.nodes {
            if !node.enabled {
                continue;
            }
            let freq = node.freq.clamp(10.0, 22000.0);
            let gain = node.gain.clamp(-24.0, 24.0);
            let q = node.q.clamp(0.1, 10.0);

            match node.filter_type {
                FilterType::Peak => {
                    if gain.abs() > 0.05 {
                        filters.push(format!("equalizer=f={:.1}:width_type=q:w={:.2}:g={:.1}", freq, q, gain));
                    }
                }
                FilterType::LowShelf => {
                    if gain.abs() > 0.05 {
                        filters.push(format!("lowshelf=f={:.1}:gain={:.1}:poles=2", freq, gain));
                    }
                }
                FilterType::HighShelf => {
                    if gain.abs() > 0.05 {
                        filters.push(format!("highshelf=f={:.1}:gain={:.1}:poles=2", freq, gain));
                    }
                }
                FilterType::LowPass => {
                    filters.push(format!("lowpass=f={:.1}:poles=2", freq));
                }
                FilterType::HighPass => {
                    filters.push(format!("highpass=f={:.1}:poles=2", freq));
                }
                FilterType::BandPass => {
                    filters.push(format!("bandpass=f={:.1}:width_type=q:w={:.2}", freq, q));
                }
                FilterType::Notch => {
                    filters.push(format!("bandreject=f={:.1}:width_type=q:w={:.2}", freq, q));
                }
                FilterType::AllPass => {
                    filters.push(format!("allpass=f={:.1}:width_type=q:w={:.2}", freq, q));
                }
            }
        }

        filters.join(",")
    }

    /// Parse AutoEQ / Equalizer APO text configuration.
    pub fn parse_autoeq(text: &str) -> Option<Self> {
        let mut preamp = 0.0;
        let mut nodes = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Preamp: -6.0 dB
            if line.to_lowercase().starts_with("preamp:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(val) = parts[1].parse::<f64>() {
                        preamp = val;
                    }
                }
            }
            // Filter 1: ON PK Fc 1200 Hz Gain -3.5 dB Q 1.41
            else if line.to_lowercase().starts_with("filter") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let is_on = parts.iter().any(|&p| p.eq_ignore_ascii_case("on"));
                
                let filter_type = if parts.iter().any(|&p| p.eq_ignore_ascii_case("pk") || p.eq_ignore_ascii_case("peak")) {
                    FilterType::Peak
                } else if parts.iter().any(|&p| p.eq_ignore_ascii_case("lsc") || p.eq_ignore_ascii_case("lowshelf")) {
                    FilterType::LowShelf
                } else if parts.iter().any(|&p| p.eq_ignore_ascii_case("hsc") || p.eq_ignore_ascii_case("highshelf")) {
                    FilterType::HighShelf
                } else if parts.iter().any(|&p| p.eq_ignore_ascii_case("lp") || p.eq_ignore_ascii_case("lowpass")) {
                    FilterType::LowPass
                } else if parts.iter().any(|&p| p.eq_ignore_ascii_case("hp") || p.eq_ignore_ascii_case("highpass")) {
                    FilterType::HighPass
                } else {
                    FilterType::Peak
                };

                let mut freq = 1000.0;
                let mut gain = 0.0;
                let mut q = 1.414;

                for (i, &p) in parts.iter().enumerate() {
                    if p.eq_ignore_ascii_case("fc") && i + 1 < parts.len() {
                        if let Ok(v) = parts[i + 1].parse::<f64>() {
                            freq = v;
                        }
                    } else if p.eq_ignore_ascii_case("gain") && i + 1 < parts.len() {
                        if let Ok(v) = parts[i + 1].parse::<f64>() {
                            gain = v;
                        }
                    } else if p.eq_ignore_ascii_case("q") && i + 1 < parts.len() {
                        if let Ok(v) = parts[i + 1].parse::<f64>() {
                            q = v;
                        }
                    }
                }

                nodes.push(ParametricNode {
                    enabled: is_on,
                    filter_type,
                    freq,
                    gain,
                    q,
                });
            }
        }

        if nodes.is_empty() {
            None
        } else {
            Some(Self {
                enabled: true,
                preamp_gain: preamp,
                nodes,
                profile_name: "Imported AutoEQ Profile".to_string(),
            })
        }
    }
}
