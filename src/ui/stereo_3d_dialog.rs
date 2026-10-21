//! stereo_3d_dialog.rs — Stereoscopic 3D Studio & 2D-to-3D Synthesis Dialog

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stereo3dPreset {
    Off,
    SbsToAnaglyphRedCyan,
    TabToAnaglyphRedCyan,
    SbsToAnaglyphGreenMagenta,
    SbsToAnaglyphDubois,
    Synthetic2dTo3d,
    SbsTo2DLeft,
    TabTo2DTop,
    SwapEyes,
}

pub struct Stereo3dDialog {
    pub is_open: bool,
    pub active_preset: Stereo3dPreset,
    pub parallax_depth: f32,
    pub swap_eyes: bool,
    pub synthetic_depth_strength: f32,
}

impl Default for Stereo3dDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            active_preset: Stereo3dPreset::Off,
            parallax_depth: 0.0,
            swap_eyes: false,
            synthetic_depth_strength: 3.0,
        }
    }
}

impl Stereo3dDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🕶️ Stereoscopic 3D & 2D-to-3D Studio (Ctrl+Alt+3)")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(420.0, 360.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Stereoscopic 3D Video Decoder & Anaglyph Studio")
                            .strong()
                            .color(Color32::from_rgb(100, 200, 255)),
                    );
                    ui.label(
                        RichText::new("Convert Side-by-Side (SBS), Top-Bottom, or 2D footage into Anaglyph 3D Glasses output.")
                            .small()
                            .color(Color32::from_rgb(160, 160, 170)),
                    );
                    ui.add_space(8.0);

                    ui.group(|ui| {
                        ui.label(RichText::new("3D Output Mode").strong());
                        ui.add_space(4.0);

                        let mut mode_changed = false;

                        if ui.radio_value(&mut self.active_preset, Stereo3dPreset::Off, "❌ Disabled (Standard 2D)").changed() {
                            mode_changed = true;
                        }
                        if ui.radio_value(&mut self.active_preset, Stereo3dPreset::SbsToAnaglyphRedCyan, "🔴🔵 Side-by-Side (SBS) → Red/Cyan Anaglyph (Optimized)").changed() {
                            mode_changed = true;
                        }
                        if ui.radio_value(&mut self.active_preset, Stereo3dPreset::TabToAnaglyphRedCyan, "🔴🔵 Top-and-Bottom (TaB) → Red/Cyan Anaglyph").changed() {
                            mode_changed = true;
                        }
                        if ui.radio_value(&mut self.active_preset, Stereo3dPreset::SbsToAnaglyphGreenMagenta, "🟢🟣 Side-by-Side (SBS) → Green/Magenta Anaglyph").changed() {
                            mode_changed = true;
                        }
                        if ui.radio_value(&mut self.active_preset, Stereo3dPreset::SbsToAnaglyphDubois, "🕶️ Side-by-Side (SBS) → Amber/Blue Dubois High-Quality").changed() {
                            mode_changed = true;
                        }
                        if ui.radio_value(&mut self.active_preset, Stereo3dPreset::Synthetic2dTo3d, "✨ Real-Time 2D → 3D Synthetic Depth Synthesis").changed() {
                            mode_changed = true;
                        }
                        if ui.radio_value(&mut self.active_preset, Stereo3dPreset::SbsTo2DLeft, "👁️ Side-by-Side → 2D Monoscopic (Left Eye Only)").changed() {
                            mode_changed = true;
                        }
                        if ui.radio_value(&mut self.active_preset, Stereo3dPreset::TabTo2DTop, "👁️ Top-Bottom → 2D Monoscopic (Top Eye Only)").changed() {
                            mode_changed = true;
                        }

                        if mode_changed {
                            self.apply_to_player(player);
                        }
                    });

                    ui.add_space(8.0);

                    ui.group(|ui| {
                        ui.label(RichText::new("3D Adjustments & Calibration").strong());
                        ui.add_space(4.0);

                        if ui.checkbox(&mut self.swap_eyes, "Invert Left / Right Eye (Swap)").changed() {
                            self.apply_to_player(player);
                        }

                        if self.active_preset == Stereo3dPreset::Synthetic2dTo3d {
                            ui.add_space(4.0);
                            ui.label("2D-to-3D Parallax Separation Intensity:");
                            if ui.add(Slider::new(&mut self.synthetic_depth_strength, 0.5..=10.0).text("px")).changed() {
                                self.apply_to_player(player);
                            }
                        }
                    });

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("↺ Reset to 2D").clicked() {
                            self.active_preset = Stereo3dPreset::Off;
                            self.swap_eyes = false;
                            self.apply_to_player(player);
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Close").clicked() {
                                self.is_open = false;
                            }
                        });
                    });
                });
            });

        self.is_open = open;
    }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        let filter_cmd = match self.active_preset {
            Stereo3dPreset::Off => "".to_string(),
            Stereo3dPreset::SbsToAnaglyphRedCyan => {
                if self.swap_eyes {
                    "stereo3d=sbsr:arcd".to_string()
                } else {
                    "stereo3d=sbsl:arcd".to_string()
                }
            }
            Stereo3dPreset::TabToAnaglyphRedCyan => {
                if self.swap_eyes {
                    "stereo3d=abr:arcd".to_string()
                } else {
                    "stereo3d=abl:arcd".to_string()
                }
            }
            Stereo3dPreset::SbsToAnaglyphGreenMagenta => {
                if self.swap_eyes {
                    "stereo3d=sbsr:al".to_string()
                } else {
                    "stereo3d=sbsl:al".to_string()
                }
            }
            Stereo3dPreset::SbsToAnaglyphDubois => {
                if self.swap_eyes {
                    "stereo3d=sbsr:dubois".to_string()
                } else {
                    "stereo3d=sbsl:dubois".to_string()
                }
            }
            Stereo3dPreset::Synthetic2dTo3d => {
                let px = self.synthetic_depth_strength.round() as i32;
                format!("split[left][right];[right]colorchannelmixer=rr=0:gg=1:bb=1,boxblur=1[r_anag];[left]colorchannelmixer=rr=1:gg=0:bb=0,crop=in_w-{px}:in_h:0:0,pad=in_w+{px}:in_h:{px}:0[l_anag];[l_anag][r_anag]blend=all_mode=addition", px = px)
            }
            Stereo3dPreset::SbsTo2DLeft => "stereo3d=sbsl:mono".to_string(),
            Stereo3dPreset::TabTo2DTop => "stereo3d=abl:mono".to_string(),
            Stereo3dPreset::SwapEyes => "stereo3d=sbsl:sbsr".to_string(),
        };

        if filter_cmd.is_empty() {
            player.set_video_filter("");
        } else {
            player.set_video_filter(&filter_cmd);
        }
    }
}
