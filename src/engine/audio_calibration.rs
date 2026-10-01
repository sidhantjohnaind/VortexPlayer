#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelCalibration {
    pub gain_db: f64,     // -24.0 to +12.0 dB
    pub delay_ms: f64,    // 0.0 to 50.0 ms
    pub phase_invert: bool,
}

impl Default for ChannelCalibration {
    fn default() -> Self {
        Self {
            gain_db: 0.0,
            delay_ms: 0.0,
            phase_invert: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerCalibrationMatrix {
    pub enabled: bool,
    pub front_left: ChannelCalibration,
    pub front_right: ChannelCalibration,
    pub center: ChannelCalibration,
    pub lfe: ChannelCalibration,
    pub surround_left: ChannelCalibration,
    pub surround_right: ChannelCalibration,
    pub rear_left: ChannelCalibration,
    pub rear_right: ChannelCalibration,
    
    // Dynamic Range & DSP
    pub compressor_enabled: bool,
    pub limiter_enabled: bool,
    pub noise_gate_enabled: bool,
    pub loudnorm_ebu_r128: bool,
    pub bauer_crossfeed: bool,
    
    // Bitstreaming
    pub passthrough_ac3: bool,
    pub passthrough_eac3: bool,
    pub passthrough_dts: bool,
    pub passthrough_truehd: bool,
    pub passthrough_dtshd: bool,
}

impl Default for SpeakerCalibrationMatrix {
    fn default() -> Self {
        Self {
            enabled: false,
            front_left: ChannelCalibration::default(),
            front_right: ChannelCalibration::default(),
            center: ChannelCalibration::default(),
            lfe: ChannelCalibration::default(),
            surround_left: ChannelCalibration::default(),
            surround_right: ChannelCalibration::default(),
            rear_left: ChannelCalibration::default(),
            rear_right: ChannelCalibration::default(),
            compressor_enabled: false,
            limiter_enabled: false,
            noise_gate_enabled: false,
            loudnorm_ebu_r128: false,
            bauer_crossfeed: false,
            passthrough_ac3: false,
            passthrough_eac3: false,
            passthrough_dts: false,
            passthrough_truehd: false,
            passthrough_dtshd: false,
        }
    }
}

impl SpeakerCalibrationMatrix {
    pub fn build_bitstream_string(&self) -> String {
        let mut list = Vec::new();
        if self.passthrough_ac3 { list.push("ac3"); }
        if self.passthrough_eac3 { list.push("eac3"); }
        if self.passthrough_dts { list.push("dts"); }
        if self.passthrough_truehd { list.push("truehd"); }
        if self.passthrough_dtshd { list.push("dts-hd"); }
        list.join(",")
    }

    pub fn build_dsp_filter_string(&self) -> String {
        let mut filters = Vec::new();
        if self.loudnorm_ebu_r128 {
            filters.push("loudnorm=I=-16:TP=-1.5:LRA=11".to_string());
        }
        if self.compressor_enabled {
            filters.push("acompressor=threshold=-20dB:ratio=4:attack=5:release=50".to_string());
        }
        if self.limiter_enabled {
            filters.push("alimiter=limit=0.95:attack=5:release=50".to_string());
        }
        if self.noise_gate_enabled {
            filters.push("agate=threshold=-40dB:ratio=2:range=0.1".to_string());
        }
        if self.bauer_crossfeed {
            filters.push("bs2b=profile=johansen".to_string());
        }
        filters.join(",")
    }
}
