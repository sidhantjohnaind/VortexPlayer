#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodecOverrideRule {
    pub codec_match: String, // e.g. "hevc", "av1", "h264"
    pub preferred_hwdec: String, // e.g. "nvdec", "d3d11va", "qsv"
    pub min_resolution_height: u32, // e.g. 2160 for 4k
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodecRulesEngine {
    pub rules: Vec<CodecOverrideRule>,
}

impl Default for CodecRulesEngine {
    fn default() -> Self {
        Self {
            rules: vec![
                CodecOverrideRule {
                    codec_match: "hevc".to_string(),
                    preferred_hwdec: "nvdec".to_string(),
                    min_resolution_height: 0,
                },
                CodecOverrideRule {
                    codec_match: "av1".to_string(),
                    preferred_hwdec: "d3d11va".to_string(),
                    min_resolution_height: 0,
                },
            ],
        }
    }
}

impl CodecRulesEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve_hwdec(&self, codec: &str, height: u32) -> Option<&str> {
        let codec_low = codec.to_lowercase();
        for rule in &self.rules {
            if codec_low.contains(&rule.codec_match.to_lowercase()) && height >= rule.min_resolution_height {
                return Some(&rule.preferred_hwdec);
            }
        }
        None
    }
}
