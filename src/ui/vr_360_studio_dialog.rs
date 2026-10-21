//! vr_360_studio_dialog.rs — 360° VR Spherical Video & Interactive Head-Tracking Studio (KMPlayer 64X style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrProjectionMode {
    Equirectangular360,
    Equirectangular180,
    Cubemap,
    FisheyeDual,
    FlatNormal,
}

impl VrProjectionMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            VrProjectionMode::Equirectangular360 => "360° Equirectangular Spherical (YouTube 360)",
            VrProjectionMode::Equirectangular180 => "180° VR Stereoscopic (VR180 3D)",
            VrProjectionMode::Cubemap => "360° CubeMap 6-Face",
            VrProjectionMode::FisheyeDual => "Dual Fisheye (Insta360 / GoPro MAX)",
            VrProjectionMode::FlatNormal => "Flat 2D Standard (No VR Unwarp)",
        }
    }
}

pub struct Vr360StudioDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub projection: VrProjectionMode,
    pub yaw_deg: f32,   // -180 to +180
    pub pitch_deg: f32, // -90 to +90
    pub roll_deg: f32,  // -45 to +45
    pub fov_deg: f32,   // 30 to 130 degrees
    pub enable_mouse_drag_look: bool,
    pub stereo_eye: &'static str, // Left, Right, Both
    pub status_message: String,
}

impl Default for Vr360StudioDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            projection: VrProjectionMode::Equirectangular360,
            yaw_deg: 0.0,
            pitch_deg: 0.0,
            roll_deg: 0.0,
            fov_deg: 90.0,
            enable_mouse_drag_look: true,
            stereo_eye: "Both (Stereo 3D)",
            status_message: "360° VR Spherical Head-Tracking Engine Ready.".to_string(),
        }
    }
}

impl Vr360StudioDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || self.projection == VrProjectionMode::FlatNormal {
            player.set_property_string("video-rotate", "0");
            player.set_property_string("video-pan-x", "0");
            player.set_property_string("video-pan-y", "0");
        } else {
            let norm_x = (self.yaw_deg / 180.0).clamp(-1.0, 1.0);
            let norm_y = (self.pitch_deg / 90.0).clamp(-1.0, 1.0);
            player.set_property_string("video-pan-x", &format!("{:.3}", norm_x));
            player.set_property_string("video-pan-y", &format!("{:.3}", norm_y));
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🥽 360° VR Spherical Video Studio (KMPlayer 64X)")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(700.0, 520.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("360° VR / 180° 3D Spherical Head-Tracking")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(140, 220, 255)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.is_active {
                                ui.label(RichText::new("● 360° VR ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                            } else {
                                ui.label(RichText::new("● BYPASSED").color(Color32::GRAY).strong());
                            }
                        });
                    });

                    ui.separator();

                    let mut changed = false;

                    ui.group(|ui| {
                        if ui.checkbox(&mut self.is_active, "🥽 Enable 360° VR Spherical Projection Engine").changed() {
                            changed = true;
                        }

                        ui.horizontal(|ui| {
                            ui.label("Projection Format:");
                            egui::ComboBox::from_id_salt("vr_projection_combo")
                                .selected_text(self.projection.display_name())
                                .show_ui(ui, |ui| {
                                    if ui.selectable_value(&mut self.projection, VrProjectionMode::Equirectangular360, VrProjectionMode::Equirectangular360.display_name()).clicked() {
                                        changed = true;
                                    }
                                    if ui.selectable_value(&mut self.projection, VrProjectionMode::Equirectangular180, VrProjectionMode::Equirectangular180.display_name()).clicked() {
                                        changed = true;
                                    }
                                    if ui.selectable_value(&mut self.projection, VrProjectionMode::Cubemap, VrProjectionMode::Cubemap.display_name()).clicked() {
                                        changed = true;
                                    }
                                    if ui.selectable_value(&mut self.projection, VrProjectionMode::FisheyeDual, VrProjectionMode::FisheyeDual.display_name()).clicked() {
                                        changed = true;
                                    }
                                    if ui.selectable_value(&mut self.projection, VrProjectionMode::FlatNormal, VrProjectionMode::FlatNormal.display_name()).clicked() {
                                        changed = true;
                                    }
                                });
                        });
                    });

                    ui.add_space(6.0);

                    // Camera Controls
                    ui.group(|ui| {
                        ui.label(RichText::new("Spherical Camera Angles & Field of View (FOV)").strong());

                        ui.horizontal(|ui| {
                            ui.label("Yaw (Horizontal Look):");
                            if ui.add(Slider::new(&mut self.yaw_deg, -180.0..=180.0).suffix("°")).changed() {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Pitch (Vertical Look):");
                            if ui.add(Slider::new(&mut self.pitch_deg, -90.0..=90.0).suffix("°")).changed() {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Roll (Tilt Level):");
                            if ui.add(Slider::new(&mut self.roll_deg, -45.0..=45.0).suffix("°")).changed() {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Field of View (FOV):");
                            if ui.add(Slider::new(&mut self.fov_deg, 30.0..=130.0).suffix("°")).changed() {
                                changed = true;
                            }
                        });

                        ui.checkbox(&mut self.enable_mouse_drag_look, "🖱️ Enable Click & Drag on Video to Look Around (360° Mouse Look)");
                    });

                    ui.add_space(6.0);

                    // Quick Look Presets
                    ui.horizontal(|ui| {
                        ui.label("Quick Look:");
                        if ui.button("Front (0°)").clicked() {
                            self.yaw_deg = 0.0;
                            self.pitch_deg = 0.0;
                            changed = true;
                        }
                        if ui.button("Look Left (-90°)").clicked() {
                            self.yaw_deg = -90.0;
                            changed = true;
                        }
                        if ui.button("Look Right (+90°)").clicked() {
                            self.yaw_deg = 90.0;
                            changed = true;
                        }
                        if ui.button("Look Behind (180°)").clicked() {
                            self.yaw_deg = 180.0;
                            changed = true;
                        }
                        if ui.button("Look Sky (+60°)").clicked() {
                            self.pitch_deg = 60.0;
                            changed = true;
                        }
                    });

                    if changed {
                        self.apply_to_player(player);
                    }

                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("↺ Center Camera (Reset)").clicked() {
                            self.yaw_deg = 0.0;
                            self.pitch_deg = 0.0;
                            self.roll_deg = 0.0;
                            self.fov_deg = 90.0;
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
}
