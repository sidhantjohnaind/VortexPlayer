#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerEvent {
    FileLoaded { path: String, duration: f64 },
    PlaybackStarted,
    PlaybackPaused,
    PlaybackStopped,
    Seeked { target_time: f64 },
    ChapterChanged { chapter_index: u32, title: String },
    SubtitleChanged { track_id: i64 },
    AudioChanged { track_id: i64 },
    VolumeChanged { volume: f64 },
    FileEnded,
}

pub struct ScriptEventBus {
    pub enabled: bool,
    pub event_history: Vec<PlayerEvent>,
}

impl Default for ScriptEventBus {
    fn default() -> Self {
        Self {
            enabled: true,
            event_history: Vec::new(),
        }
    }
}

impl ScriptEventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(&mut self, event: PlayerEvent) {
        if self.enabled {
            if self.event_history.len() > 100 {
                self.event_history.remove(0);
            }
            self.event_history.push(event);
        }
    }
}
