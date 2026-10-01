#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneCut {
    pub timestamp_sec: f64,
    pub score: f64, // 0.0 to 1.0
    pub title: String,
}

pub struct SceneDetector {
    pub sensitivity: f64, // default 0.4
    pub detected_scenes: Vec<SceneCut>,
}

impl Default for SceneDetector {
    fn default() -> Self {
        Self {
            sensitivity: 0.4,
            detected_scenes: Vec::new(),
        }
    }
}

impl SceneDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_filter_string(&self) -> String {
        format!("select='gt(scene,{:.2})',metadata=print:file=scene_log.txt", self.sensitivity)
    }

    pub fn add_scene(&mut self, timestamp: f64, score: f64) {
        let count = self.detected_scenes.len() + 1;
        self.detected_scenes.push(SceneCut {
            timestamp_sec: timestamp,
            score,
            title: format!("Scene {:02}", count),
        });
    }
}
