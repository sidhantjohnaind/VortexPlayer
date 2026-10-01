//! motion_interpolation_dialog.rs — Motion Interpolation, Frame Blending & SmoothMotion Studio (SVP/MPV/PotPlayer)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationAlgorithm {
    Off,
    SmoothMotion,
    Oversample,
    Linear,
    Bicubic,
    Spline36,
    Gaussian,
    Box,
}

impl InterpolationAlgorithm {
    pub fn display_name(&self) -> &'static str {
        match self {
            InterpolationAlgorithm::Off => "Off (Native Video Cadence)",
            InterpolationAlgorithm::SmoothMotion => "SmoothMotion (No Judder Frame Blending)",
            InterpolationAlgorithm::Oversample => "Oversample (Zero-Ghosting High Cadence)",
            InterpolationAlgorithm::Linear => "Linear Interpolation (Fast Motion Blur)",
            InterpolationAlgorithm::Bicubic => "Bicubic (Sharp High-FPS Curve)",
            InterpolationAlgorithm::Spline36 => "Spline36 (Artifact-Free Cinematic Smoothing)",
            InterpolationAlgorithm::Gaussian => "Gaussian (Dreamlike Soft Motion)",
            InterpolationAlgorithm::Box => "Box Filter (Sharp Step)",
        }
    }

    pub fn tscale_str(&self) -> &'static str {
        match self {
            InterpolationAlgorithm::Off => "oversample",
            InterpolationAlgorithm::SmoothMotion => "oversample",
            InterpolationAlgorithm::Oversample => "oversample",
            InterpolationAlgorithm::Linear => "linear",
            InterpolationAlgorithm::Bicubic => "bicubic",
            InterpolationAlgorithm::Spline36 => "spline36",
            InterpolationAlgorithm::Gaussian => "gaussian",
            InterpolationAlgorithm::Box => "box",
        }
    }
}

pub struct MotionInterpolationDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub algorithm: InterpolationAlgorithm,
    pub target_fps: u32,
    pub motion_blur_strength: f32,
    pub status_message: String,
}

impl Default for MotionInterpolationDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            algorithm: InterpolationAlgorithm::SmoothMotion,
            target_fps: 60,
            motion_blur_strength: 0.0,
            status_message: "Motion interpolation ready. Smooth out 24fps film judder.".to_string(),
        }
    }
}

impl MotionInterpolationDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || self.algorithm == InterpolationAlgorithm::Off {
            player.set_property_string("interpolation", "no");
        } else {
            player.set_property_string("interpolation", "yes");
            player.set_property_string("tscale", self.algorithm.tscale_str());
            player.set_property_string("video-sync", "display-resample");
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player, stats: &crate::engine::MediaStats) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("🏎️ Motion Interpolation & SmoothMotion")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(600.0, 420.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("SmoothMotion & Motion Interpolation Engine").strong().size(15.0).color(Color32::from_rgb(100, 220, 255)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_active {
                            ui.label(RichText::new("● ACTIVE (display-resample)").color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● DISABLED").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    if ui.checkbox(&mut self.is_active, "🏎️ Enable Motion Interpolation & Cadence Matching").changed() { changed = true; }
                    ui.horizontal(|ui| {
                        let fps_display = if stats.video_fps > 0.0 { format!("{:.3} fps", stats.video_fps) } else { "Unknown fps".to_string() };
                        ui.label(RichText::new(format!("Source Video: {}", fps_display)).color(Color32::GRAY));
                        if stats.display_fps > 0.0 {
                            ui.label(RichText::new(format!("Display: {:.2} Hz", stats.display_fps)).color(Color32::from_rgb(180, 220, 140)));
                        }
                    });
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Interpolation Filter / Algorithm:").strong());
                    for algo in [
                        InterpolationAlgorithm::SmoothMotion,
                        InterpolationAlgorithm::Oversample,
                        InterpolationAlgorithm::Spline36,
                        InterpolationAlgorithm::Bicubic,
                        InterpolationAlgorithm::Linear,
                        InterpolationAlgorithm::Gaussian,
                        InterpolationAlgorithm::Off,
                    ] {
                        if ui.selectable_label(self.algorithm == algo, algo.display_name()).clicked() {
                            self.algorithm = algo;
                            changed = true;
                        }
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Target Display Refresh Rate").strong());
                    ui.horizontal(|ui| {
                        for hz in [60, 120, 144, 165, 240] {
                            if ui.selectable_label(self.target_fps == hz, format!("{} Hz", hz)).clicked() {
                                self.target_fps = hz;
                                changed = true;
                            }
                        }
                    });

                    if ui.add(Slider::new(&mut self.motion_blur_strength, 0.0..=1.0).text("Motion Blur Blend Amount")).changed() {
                        changed = true;
                    }
                });

                if changed { self.apply_to_player(player); }

                ui.add_space(4.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("↺ Reset").clicked() {
                        *self = Self { is_open: true, ..Self::default() };
                        self.apply_to_player(player);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() { self.is_open = false; }
                    });
                });
            });
        self.is_open = open;
    }
}
