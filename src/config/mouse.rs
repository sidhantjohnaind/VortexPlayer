#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseAction {
    PlayPause,
    ToggleFullscreen,
    ToggleMute,
    VolumeAdjust,
    SeekShort,
    SeekMedium,
    SubtitleDelayAdjust,
    NextTrack,
    PrevTrack,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseConfig {
    pub left_click: MouseAction,
    pub left_double_click: MouseAction,
    pub middle_click: MouseAction,
    pub wheel_scroll: MouseAction,
    pub ctrl_wheel_scroll: MouseAction,
    pub shift_wheel_scroll: MouseAction,
    pub side_button_1: MouseAction,
    pub side_button_2: MouseAction,
}

impl Default for MouseConfig {
    fn default() -> Self {
        Self {
            left_click: MouseAction::PlayPause,
            left_double_click: MouseAction::ToggleFullscreen,
            middle_click: MouseAction::ToggleMute,
            wheel_scroll: MouseAction::VolumeAdjust,
            ctrl_wheel_scroll: MouseAction::SeekMedium,
            shift_wheel_scroll: MouseAction::SubtitleDelayAdjust,
            side_button_1: MouseAction::PrevTrack,
            side_button_2: MouseAction::NextTrack,
        }
    }
}
