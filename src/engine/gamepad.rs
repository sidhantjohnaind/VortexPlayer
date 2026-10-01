#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadAction {
    PlayPause,
    SeekFwd,
    SeekBack,
    VolumeUp,
    VolumeDown,
    NextTrack,
    PrevTrack,
    ToggleSubtitles,
    CycleAudio,
}

pub struct GamepadEngine {
    pub enabled: bool,
    pub last_button_state: u16,
}

impl Default for GamepadEngine {
    fn default() -> Self {
        Self {
            enabled: true,
            last_button_state: 0,
        }
    }
}

impl GamepadEngine {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(windows)]
    pub fn poll(&mut self) -> Option<GamepadAction> {
        if !self.enabled {
            return None;
        }

        // Safe stub / placeholder polling for XInput gamepad state
        None
    }

    #[cfg(not(windows))]
    pub fn poll(&mut self) -> Option<GamepadAction> {
        None
    }
}
