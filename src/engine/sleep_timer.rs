#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybackCompletionAction {
    DoNothing,
    ExitPlayer,
    SleepPc,
    HibernatePc,
    ShutdownPc,
}

pub struct SleepTimerEngine {
    pub active: bool,
    pub target_duration: Duration,
    pub started_at: Instant,
    pub completion_action: PlaybackCompletionAction,
}

impl Default for SleepTimerEngine {
    fn default() -> Self {
        Self {
            active: false,
            target_duration: Duration::from_secs(1800), // 30 mins
            started_at: Instant::now(),
            completion_action: PlaybackCompletionAction::DoNothing,
        }
    }
}

impl SleepTimerEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, minutes: u64, action: PlaybackCompletionAction) {
        self.active = true;
        self.target_duration = Duration::from_secs(minutes * 60);
        self.started_at = Instant::now();
        self.completion_action = action;
    }

    pub fn cancel(&mut self) {
        self.active = false;
    }

    pub fn is_expired(&self) -> bool {
        self.active && self.started_at.elapsed() >= self.target_duration
    }
}
