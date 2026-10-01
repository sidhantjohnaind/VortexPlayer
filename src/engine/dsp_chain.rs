#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioDspStageKind {
    ReplayGain,
    Equalizer18Band,
    ParametricEq,
    DynamicNormalizer,
    StereoWidener,
    Limiter,
    ChannelMatrix71,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDspStage {
    pub name: String,
    pub kind: AudioDspStageKind,
    pub enabled: bool,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDspChain {
    pub stages: Vec<AudioDspStage>,
}

impl Default for AudioDspChain {
    fn default() -> Self {
        Self {
            stages: vec![
                AudioDspStage {
                    name: "ReplayGain (Loudness Matching)".to_string(),
                    kind: AudioDspStageKind::ReplayGain,
                    enabled: false,
                    value: 0.0,
                },
                AudioDspStage {
                    name: "18-Band Graphic Equalizer".to_string(),
                    kind: AudioDspStageKind::Equalizer18Band,
                    enabled: false,
                    value: 0.0,
                },
                AudioDspStage {
                    name: "Parametric EQ".to_string(),
                    kind: AudioDspStageKind::ParametricEq,
                    enabled: false,
                    value: 0.0,
                },
                AudioDspStage {
                    name: "Dynamic Normalizer (dynaudnorm)".to_string(),
                    kind: AudioDspStageKind::DynamicNormalizer,
                    enabled: false,
                    value: 0.9,
                },
                AudioDspStage {
                    name: "Stereo Spatial Widener (extrastereo)".to_string(),
                    kind: AudioDspStageKind::StereoWidener,
                    enabled: false,
                    value: 0.0,
                },
                AudioDspStage {
                    name: "Peak Soft Limiter".to_string(),
                    kind: AudioDspStageKind::Limiter,
                    enabled: true,
                    value: 0.0,
                },
                AudioDspStage {
                    name: "7.1 Surround Channel Matrix".to_string(),
                    kind: AudioDspStageKind::ChannelMatrix71,
                    enabled: false,
                    value: 0.0,
                },
            ],
        }
    }
}

impl AudioDspChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn move_up(&mut self, idx: usize) {
        if idx > 0 && idx < self.stages.len() {
            self.stages.swap(idx, idx - 1);
        }
    }

    pub fn move_down(&mut self, idx: usize) {
        if idx + 1 < self.stages.len() {
            self.stages.swap(idx, idx + 1);
        }
    }

    pub fn build_mpv_audio_filter_string(&self) -> String {
        let mut filters = Vec::new();
        for stage in &self.stages {
            if !stage.enabled {
                continue;
            }
            match stage.kind {
                AudioDspStageKind::DynamicNormalizer => {
                    filters.push(format!("dynaudnorm=p={:.2}:s=5", stage.value.clamp(0.1, 1.0)));
                }
                AudioDspStageKind::StereoWidener => {
                    if stage.value > 0.01 {
                        filters.push(format!("extrastereo=m={:.2}", stage.value));
                    }
                }
                _ => {}
            }
        }
        filters.join(",")
    }
}
