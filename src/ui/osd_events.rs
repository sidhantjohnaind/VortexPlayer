#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OsdEvent {
    VolumeChanged { volume: f64, is_muted: bool },
    TrackChanged { track_type: String, title: String },
    SpeedChanged { speed: f64 },
    ZoomChanged { zoom: f64 },
    ProfileActivated { name: String },
    ChapterChanged { title: String, number: usize },
    BookmarkCreated { time_sec: f64, note: String },
    CustomMessage { text: String },
}

pub struct ActiveOsdNotification {
    pub event: OsdEvent,
    pub created_at: Instant,
    pub duration_ms: u64,
}

impl ActiveOsdNotification {
    pub fn new(event: OsdEvent, duration_ms: u64) -> Self {
        Self {
            event,
            created_at: Instant::now(),
            duration_ms,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().as_millis() as u64 > self.duration_ms
    }

    pub fn alpha(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let total = self.duration_ms as f32 / 1000.0;
        if elapsed < 0.15 {
            (elapsed / 0.15).min(1.0)
        } else if elapsed > total - 0.3 {
            ((total - elapsed) / 0.3).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }
}
