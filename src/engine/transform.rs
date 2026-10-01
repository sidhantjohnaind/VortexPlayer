#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoTransformState {
    pub zoom: f64,          // 0.25 to 4.0
    pub pan_x: f64,         // -1.0 to +1.0
    pub pan_y: f64,         // -1.0 to +1.0
    pub rotation_deg: i32,  // 0, 90, 180, 270
    pub flip_h: bool,
    pub flip_v: bool,
}

impl Default for VideoTransformState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            rotation_deg: 0,
            flip_h: false,
            flip_v: false,
        }
    }
}

impl VideoTransformState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom + 0.1).min(4.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom - 0.1).max(0.25);
    }

    pub fn rotate_cw(&mut self) {
        self.rotation_deg = (self.rotation_deg + 90) % 360;
    }
}
