#![allow(dead_code)]

use crate::commands::definitions::Command;
use eframe::egui::Pos2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GestureDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureBinding {
    pub path: Vec<GestureDirection>,
    pub command: Command,
    pub label: String,
}

pub struct MouseGestureTracker {
    pub is_tracking: bool,
    pub path_points: Vec<Pos2>,
    pub directions: Vec<GestureDirection>,
    pub bindings: Vec<GestureBinding>,
}

impl Default for MouseGestureTracker {
    fn default() -> Self {
        Self {
            is_tracking: false,
            path_points: Vec::new(),
            directions: Vec::new(),
            bindings: vec![
                GestureBinding {
                    path: vec![GestureDirection::Right],
                    command: Command::Seek { seconds: 30.0 },
                    label: "Seek Forward (+30s)".to_string(),
                },
                GestureBinding {
                    path: vec![GestureDirection::Left],
                    command: Command::Seek { seconds: -30.0 },
                    label: "Seek Backward (-30s)".to_string(),
                },
                GestureBinding {
                    path: vec![GestureDirection::Up],
                    command: Command::AdjustVolume { delta: 5.0 },
                    label: "Volume Up (+5%)".to_string(),
                },
                GestureBinding {
                    path: vec![GestureDirection::Down],
                    command: Command::AdjustVolume { delta: -5.0 },
                    label: "Volume Down (-5%)".to_string(),
                },
            ],
        }
    }
}

impl MouseGestureTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, start_pos: Pos2) {
        self.is_tracking = true;
        self.path_points.clear();
        self.directions.clear();
        self.path_points.push(start_pos);
    }

    pub fn update(&mut self, current_pos: Pos2) {
        if !self.is_tracking {
            return;
        }
        if let Some(last) = self.path_points.last() {
            let dx = current_pos.x - last.x;
            let dy = current_pos.y - last.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > 35.0 {
                let dir = if dx.abs() > dy.abs() {
                    if dx > 0.0 { GestureDirection::Right } else { GestureDirection::Left }
                } else {
                    if dy > 0.0 { GestureDirection::Down } else { GestureDirection::Up }
                };

                if self.directions.last() != Some(&dir) {
                    self.directions.push(dir);
                }
                self.path_points.push(current_pos);
            }
        }
    }

    pub fn finish(&mut self) -> Option<Command> {
        self.is_tracking = false;
        if self.directions.is_empty() {
            return None;
        }

        for binding in &self.bindings {
            if binding.path == self.directions {
                return Some(binding.command.clone());
            }
        }
        None
    }
}
