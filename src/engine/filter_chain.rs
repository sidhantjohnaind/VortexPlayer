#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VideoFilterStageKind {
    Deinterlace,
    Denoise,
    Deband,
    SuperResolution,
    Sharpen,
    HdrToneMapping,
    ColorTuning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFilterStage {
    pub name: String,
    pub kind: VideoFilterStageKind,
    pub enabled: bool,
    pub intensity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFilterChain {
    pub stages: Vec<VideoFilterStage>,
}

impl Default for VideoFilterChain {
    fn default() -> Self {
        Self {
            stages: vec![
                VideoFilterStage {
                    name: "Deinterlace (Yadif)".to_string(),
                    kind: VideoFilterStageKind::Deinterlace,
                    enabled: false,
                    intensity: 1.0,
                },
                VideoFilterStage {
                    name: "Denoise (NLMeans)".to_string(),
                    kind: VideoFilterStageKind::Denoise,
                    enabled: false,
                    intensity: 0.2,
                },
                VideoFilterStage {
                    name: "Deband (Banding artifact removal)".to_string(),
                    kind: VideoFilterStageKind::Deband,
                    enabled: false,
                    intensity: 1.0,
                },
                VideoFilterStage {
                    name: "Anime4K / FSRCNNX Scaler".to_string(),
                    kind: VideoFilterStageKind::SuperResolution,
                    enabled: false,
                    intensity: 1.0,
                },
                VideoFilterStage {
                    name: "Adaptive Unsharp Sharpen".to_string(),
                    kind: VideoFilterStageKind::Sharpen,
                    enabled: false,
                    intensity: 0.35,
                },
                VideoFilterStage {
                    name: "HDR10 Tone Mapping (BT.2390)".to_string(),
                    kind: VideoFilterStageKind::HdrToneMapping,
                    enabled: true,
                    intensity: 1.0,
                },
                VideoFilterStage {
                    name: "Color Temperature & Gamma Tuning".to_string(),
                    kind: VideoFilterStageKind::ColorTuning,
                    enabled: false,
                    intensity: 0.0,
                },
            ],
        }
    }
}

impl VideoFilterChain {
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

    pub fn build_mpv_filter_string(&self) -> String {
        let mut filters = Vec::new();
        for stage in &self.stages {
            if !stage.enabled {
                continue;
            }
            match stage.kind {
                VideoFilterStageKind::Deinterlace => {
                    filters.push("yadif=mode=send_frame:parity=auto:deint=all".to_string());
                }
                VideoFilterStageKind::Denoise => {
                    let d = (stage.intensity * 3.0 + 1.0).max(1.0);
                    filters.push(format!("nlmeans=s={:.2}:p=5:r=9", d));
                }
                VideoFilterStageKind::Deband => {
                    filters.push("gradfun=strength=1.2:radius=16".to_string());
                }
                VideoFilterStageKind::Sharpen => {
                    let s = stage.intensity * 1.5;
                    filters.push(format!("unsharp=la={:.2}:ca={:.2}", s, s));
                }
                _ => {}
            }
        }
        filters.join(",")
    }
}
