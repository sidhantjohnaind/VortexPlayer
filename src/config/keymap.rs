#![allow(dead_code)]

use super::commands::PlayerCommand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeymapConfig {
    pub bindings: HashMap<PlayerCommand, Vec<KeyBinding>>,
}

impl Default for KeymapConfig {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        
        let mut add = |cmd: PlayerCommand, key: &str, ctrl: bool, shift: bool, alt: bool| {
            bindings.entry(cmd).or_insert_with(Vec::new).push(KeyBinding {
                key: key.to_string(),
                ctrl,
                shift,
                alt,
            });
        };

        add(PlayerCommand::PlayPause, "Space", false, false, false);
        add(PlayerCommand::SeekShortFwd, "Right", false, false, false);
        add(PlayerCommand::SeekShortBack, "Left", false, false, false);
        add(PlayerCommand::SeekMediumFwd, "Right", true, false, false);
        add(PlayerCommand::SeekMediumBack, "Left", true, false, false);
        add(PlayerCommand::SeekLongFwd, "Right", false, true, false);
        add(PlayerCommand::SeekLongBack, "Left", false, true, false);
        add(PlayerCommand::SeekHugeFwd, "Right", false, false, true);
        add(PlayerCommand::SeekHugeBack, "Left", false, false, true);
        add(PlayerCommand::FrameStepFwd, "F", false, false, false);
        add(PlayerCommand::FrameStepBack, "D", false, false, false);
        add(PlayerCommand::VolumeUp, "Up", false, false, false);
        add(PlayerCommand::VolumeDown, "Down", false, false, false);
        add(PlayerCommand::VolumeMute, "M", false, false, false);
        add(PlayerCommand::SubtitleDelayPlus, ">", false, false, false);
        add(PlayerCommand::SubtitleDelayMinus, "<", false, false, false);
        add(PlayerCommand::RepeatSubtitleSentence, "R", false, false, false);
        add(PlayerCommand::SpeedUp, "C", false, false, false);
        add(PlayerCommand::SpeedDown, "X", false, false, false);
        add(PlayerCommand::SpeedReset, "Z", false, false, false);
        add(PlayerCommand::ToggleMediaInfo, "F1", true, false, false);
        add(PlayerCommand::TogglePreferences, "F5", false, false, false);
        add(PlayerCommand::TogglePlaylist, "F6", false, false, false);
        add(PlayerCommand::ToggleControlPanel, "F7", false, false, false);

        Self { bindings }
    }
}
