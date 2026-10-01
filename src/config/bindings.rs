#![allow(dead_code)]

use crate::commands::definitions::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseTriggerType {
    LeftClick,
    LeftDoubleClick,
    MiddleClick,
    RightClick,
    WheelUp,
    WheelDown,
    SideButton1,
    SideButton2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GamepadTriggerButton {
    A,
    B,
    X,
    Y,
    LeftBumper,
    RightBumper,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Start,
    Back,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputTrigger {
    Key { key: String, ctrl: bool, shift: bool, alt: bool },
    Mouse { action: MouseTriggerType, ctrl: bool, shift: bool },
    Gamepad { button: GamepadTriggerButton },
}

impl InputTrigger {
    pub fn display_str(&self) -> String {
        match self {
            InputTrigger::Key { key, ctrl, shift, alt } => {
                let mut parts = Vec::new();
                if *ctrl { parts.push("Ctrl"); }
                if *alt { parts.push("Alt"); }
                if *shift { parts.push("Shift"); }
                parts.push(key.as_str());
                parts.join("+")
            }
            InputTrigger::Mouse { action, ctrl, shift } => {
                let mut prefix = String::new();
                if *ctrl { prefix.push_str("Ctrl+"); }
                if *shift { prefix.push_str("Shift+"); }
                format!("{}{:?}", prefix, action)
            }
            InputTrigger::Gamepad { button } => format!("Gamepad {:?}", button),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerBinding {
    pub trigger: InputTrigger,
    pub command: Command,
    pub description: String,
}
