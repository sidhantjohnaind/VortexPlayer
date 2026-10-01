#![allow(dead_code)]

use super::bindings::{InputTrigger, MouseTriggerType, TriggerBinding};
use crate::commands::definitions::Command;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputProfileType {
    Default,
    Movie,
    Music,
    Anime,
    LanguageLearning,
    Analysis,
}

impl InputProfileType {
    pub const ALL: [InputProfileType; 6] = [
        InputProfileType::Default,
        InputProfileType::Movie,
        InputProfileType::Music,
        InputProfileType::Anime,
        InputProfileType::LanguageLearning,
        InputProfileType::Analysis,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            InputProfileType::Default => "Default Profile",
            InputProfileType::Movie => "Movie Mode (Clean OSD)",
            InputProfileType::Music => "Music & Audio Mode",
            InputProfileType::Anime => "Anime & RIFE Mode",
            InputProfileType::LanguageLearning => "Language Learning (Repeat & Dual Sub)",
            InputProfileType::Analysis => "Video Analysis (Frame Step & Zoom)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileBindingSet {
    pub bindings: Vec<TriggerBinding>,
}

impl ProfileBindingSet {
    pub fn push_key(&mut self, key: &str, ctrl: bool, shift: bool, alt: bool, cmd: Command, desc: &str) {
        self.bindings.push(TriggerBinding {
            trigger: InputTrigger::Key { key: key.to_string(), ctrl, shift, alt },
            command: cmd,
            description: desc.to_string(),
        });
    }

    pub fn push_mouse(&mut self, action: MouseTriggerType, ctrl: bool, shift: bool, cmd: Command, desc: &str) {
        self.bindings.push(TriggerBinding {
            trigger: InputTrigger::Mouse { action, ctrl, shift },
            command: cmd,
            description: desc.to_string(),
        });
    }
}

impl Default for ProfileBindingSet {
    fn default() -> Self {
        let mut set = ProfileBindingSet { bindings: Vec::new() };

        // Standard Keyboard bindings
        set.push_key("Space", false, false, false, Command::PlayPause, "Play / Pause");
        set.push_key("Right", false, false, false, Command::Seek { seconds: 5.0 }, "Short Seek Forward (+5s)");
        set.push_key("Left", false, false, false, Command::Seek { seconds: -5.0 }, "Short Seek Backward (-5s)");
        set.push_key("Right", true, false, false, Command::Seek { seconds: 30.0 }, "Medium Seek Forward (+30s)");
        set.push_key("Left", true, false, false, Command::Seek { seconds: -30.0 }, "Medium Seek Backward (-30s)");
        set.push_key("Right", false, true, false, Command::Seek { seconds: 60.0 }, "Long Seek Forward (+60s)");
        set.push_key("Left", false, true, false, Command::Seek { seconds: -60.0 }, "Long Seek Backward (-60s)");
        set.push_key("F", false, false, false, Command::FrameStep { forward: true }, "Next Frame Step");
        set.push_key("D", false, false, false, Command::FrameStep { forward: false }, "Previous Frame Step");
        set.push_key("Up", false, false, false, Command::AdjustVolume { delta: 2.0 }, "Volume Up");
        set.push_key("Down", false, false, false, Command::AdjustVolume { delta: -2.0 }, "Volume Down");
        set.push_key("M", false, false, false, Command::ToggleMute, "Toggle Mute");
        set.push_key(">", false, false, false, Command::SubtitleDelay { delta_ms: 100 }, "Subtitle Delay +100ms");
        set.push_key("<", false, false, false, Command::SubtitleDelay { delta_ms: -100 }, "Subtitle Delay -100ms");
        set.push_key("R", false, false, false, Command::PlayPause, "Repeat Current Sentence");
        set.push_key("C", false, false, false, Command::AdjustSpeed { delta: 0.1 }, "Speed Up (+0.1x)");
        set.push_key("X", false, false, false, Command::AdjustSpeed { delta: -0.1 }, "Speed Down (-0.1x)");
        set.push_key("Z", false, false, false, Command::SetSpeed { speed: 1.0 }, "Reset Speed (1.0x)");
        set.push_key("K", true, false, false, Command::TogglePreferences, "Open Input Binding Editor");

        // Standard Mouse bindings
        set.push_mouse(MouseTriggerType::LeftClick, false, false, Command::PlayPause, "Play / Pause");
        set.push_mouse(MouseTriggerType::LeftDoubleClick, false, false, Command::ToggleFullscreen, "Toggle Fullscreen");
        set.push_mouse(MouseTriggerType::MiddleClick, false, false, Command::ToggleMute, "Toggle Mute");
        set.push_mouse(MouseTriggerType::WheelUp, false, false, Command::AdjustVolume { delta: 2.0 }, "Volume Up");
        set.push_mouse(MouseTriggerType::WheelDown, false, false, Command::AdjustVolume { delta: -2.0 }, "Volume Down");
        set.push_mouse(MouseTriggerType::WheelUp, true, false, Command::Seek { seconds: 30.0 }, "Mouse Seek Forward (+30s)");
        set.push_mouse(MouseTriggerType::WheelDown, true, false, Command::Seek { seconds: -30.0 }, "Mouse Seek Backward (-30s)");
        set.push_mouse(MouseTriggerType::WheelUp, false, true, Command::SubtitleDelay { delta_ms: 100 }, "Subtitle Delay +100ms");
        set.push_mouse(MouseTriggerType::WheelDown, false, true, Command::SubtitleDelay { delta_ms: -100 }, "Subtitle Delay -100ms");

        set
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputProfileStore {
    pub active_profile: InputProfileType,
    pub profiles: HashMap<InputProfileType, ProfileBindingSet>,
}

impl Default for InputProfileStore {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        for p in InputProfileType::ALL {
            profiles.insert(p, ProfileBindingSet::default());
        }
        Self {
            active_profile: InputProfileType::Default,
            profiles,
        }
    }
}
