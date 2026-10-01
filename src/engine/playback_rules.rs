#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackRule {
    pub enabled: bool,
    pub name: String,
    pub match_keyword: Option<String>,
    pub match_min_height: Option<u32>,
    pub match_is_hdr: Option<bool>,
    pub match_max_fps: Option<f64>,
    pub target_hwdec: Option<String>,
    pub target_profile: Option<String>,
    pub enable_rife: Option<bool>,
    pub enable_deinterlace: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackRulesEngine {
    pub rules: Vec<PlaybackRule>,
}

impl Default for PlaybackRulesEngine {
    fn default() -> Self {
        Self {
            rules: vec![
                PlaybackRule {
                    enabled: false, // user-configurable, disabled by default
                    name: "Anime Profile Auto-Match".to_string(),
                    match_keyword: Some("anime".to_string()),
                    match_min_height: None,
                    match_is_hdr: None,
                    match_max_fps: None,
                    target_hwdec: Some("d3d11va".to_string()),
                    target_profile: Some("Anime".to_string()),
                    enable_rife: Some(true),
                    enable_deinterlace: None,
                },
                PlaybackRule {
                    enabled: true,
                    name: "4K HDR Master Routing".to_string(),
                    match_keyword: None,
                    match_min_height: Some(2160),
                    match_is_hdr: Some(true),
                    match_max_fps: None,
                    target_hwdec: Some("nvdec".to_string()),
                    target_profile: Some("HDR10".to_string()),
                    enable_rife: Some(false),
                    enable_deinterlace: None,
                },
                PlaybackRule {
                    enabled: false, // user-configurable, disabled by default
                    name: "Low Framerate AI Motion Interpolation".to_string(),
                    match_keyword: None,
                    match_min_height: None,
                    match_is_hdr: None,
                    match_max_fps: Some(30.0),
                    target_hwdec: None,
                    target_profile: None,
                    enable_rife: Some(true),
                    enable_deinterlace: None,
                },
            ],
        }
    }
}

impl PlaybackRulesEngine {
    pub fn new() -> Self {
        Self::default()
    }
}
