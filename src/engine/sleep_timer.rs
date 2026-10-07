use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybackCompletionAction {
    DoNothing,
    PausePlayback,
    ExitPlayer,
    SleepPc,
    HibernatePc,
    ShutdownPc,
}

impl PlaybackCompletionAction {
    pub fn label(&self) -> &'static str {
        match self {
            Self::DoNothing => "Do Nothing",
            Self::PausePlayback => "Pause Playback",
            Self::ExitPlayer => "Close VortexPlayer",
            Self::SleepPc => "Put PC to Sleep",
            Self::HibernatePc => "Hibernate PC",
            Self::ShutdownPc => "Shut Down PC",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::DoNothing => "⏸",
            Self::PausePlayback => "⏸",
            Self::ExitPlayer => "⏻",
            Self::SleepPc => "🌙",
            Self::HibernatePc => "💤",
            Self::ShutdownPc => "🛑",
        }
    }
}

pub struct SleepTimerEngine {
    pub active: bool,
    pub target_duration: Duration,
    pub started_at: Instant,
    pub completion_action: PlaybackCompletionAction,
    pub trigger_on_playlist_end: bool,
}

impl Default for SleepTimerEngine {
    fn default() -> Self {
        Self {
            active: false,
            target_duration: Duration::from_secs(1800), // 30 mins
            started_at: Instant::now(),
            completion_action: PlaybackCompletionAction::PausePlayback,
            trigger_on_playlist_end: false,
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
        self.trigger_on_playlist_end = false;
    }

    pub fn start_at_playlist_end(&mut self, action: PlaybackCompletionAction) {
        self.active = true;
        self.completion_action = action;
        self.trigger_on_playlist_end = true;
        self.started_at = Instant::now();
    }

    pub fn cancel(&mut self) {
        self.active = false;
        self.trigger_on_playlist_end = false;
    }

    pub fn is_expired(&self) -> bool {
        if !self.active {
            return false;
        }
        if self.trigger_on_playlist_end {
            false // handled by playlist completion check
        } else {
            self.started_at.elapsed() >= self.target_duration
        }
    }

    pub fn remaining(&self) -> Option<Duration> {
        if !self.active || self.trigger_on_playlist_end {
            None
        } else {
            let elapsed = self.started_at.elapsed();
            if elapsed >= self.target_duration {
                Some(Duration::ZERO)
            } else {
                Some(self.target_duration - elapsed)
            }
        }
    }

    pub fn progress(&self) -> f32 {
        if !self.active || self.trigger_on_playlist_end {
            0.0
        } else {
            let elapsed = self.started_at.elapsed().as_secs_f32();
            let total = self.target_duration.as_secs_f32();
            if total > 0.0 {
                (elapsed / total).clamp(0.0, 1.0)
            } else {
                1.0
            }
        }
    }

    pub fn execute_system_action(&self) {
        match self.completion_action {
            PlaybackCompletionAction::DoNothing | PlaybackCompletionAction::PausePlayback => {}
            PlaybackCompletionAction::ExitPlayer => {
                std::process::exit(0);
            }
            PlaybackCompletionAction::SleepPc => {
                #[cfg(windows)]
                {
                    let _ = std::process::Command::new("rundll32.exe")
                        .args(["powrprof.dll,SetSuspendState", "0,1,0"])
                        .spawn();
                }
                #[cfg(not(windows))]
                {
                    let _ = std::process::Command::new("systemctl").arg("suspend").spawn();
                }
            }
            PlaybackCompletionAction::HibernatePc => {
                #[cfg(windows)]
                {
                    let _ = std::process::Command::new("shutdown.exe").args(["/h"]).spawn();
                }
                #[cfg(not(windows))]
                {
                    let _ = std::process::Command::new("systemctl").arg("hibernate").spawn();
                }
            }
            PlaybackCompletionAction::ShutdownPc => {
                #[cfg(windows)]
                {
                    let _ = std::process::Command::new("shutdown.exe").args(["/s", "/t", "30"]).spawn();
                }
                #[cfg(not(windows))]
                {
                    let _ = std::process::Command::new("shutdown").args(["-h", "+1"]).spawn();
                }
            }
        }
    }
}
