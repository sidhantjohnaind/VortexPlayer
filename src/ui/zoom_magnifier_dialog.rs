//! zoom_magnifier_dialog.rs — Interactive Video Magnifying Glass & Detail Zoom Lens

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

pub struct ZoomMagnifierDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub zoom_level: f32, // 1.0 to 8.0x
    pub pan_x: f32,      // -1.0 to +1.0
    pub pan_y: f32,      // -1.0 to +1.0
    pub show_crosshair: bool,
    pub show_mini_radar: bool,
    pub status_message: String,
}

impl Default for ZoomMagnifierDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            zoom_level: 2.0,
            pan_x: 0.0,
            pan_y: 0.0,
            show_crosshair: true,
            show_mini_radar: true,
            status_message: "Interactive magnifying glass ready.".to_string(),
        }
    }
}

impl ZoomMagnifierDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || self.zoom_level <= 1.01 {
            player.set_property_string("video-zoom", "0");
            player.set_property_string("video-pan-x", "0");
            player.set_property_string("video-pan-y", "0");
        } else {
            let log_zoom = (self.zoom_level - 1.0).max(0.0);
            player.set_property_string("video-zoom", &format!("{:.2}", log_zoom));
            player.set_property_string("video-pan-x", &format!("{:.2}", self.pan_x));
            player.set_property_string("video-pan-y", &format!("{:.2}", self.pan_y));
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🔍 Interactive Live Video Zoom Magnifier Lens")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(640.0, 440.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Real-Time Detail Zoom Lens & Locator")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(120, 220, 255)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.is_active {
                                ui.label(RichText::new("● LENS ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                            } else {
                                ui.label(RichText::new("● BYPASSED").color(Color32::GRAY).strong());
                            }
                        });
                    });

                    ui.separator();

                    let mut changed = false;

                    ui.group(|ui| {
                        if ui.checkbox(&mut self.is_active, "🔍 Enable Live Video Magnification Lens").changed() {
                            changed = true;
                        }

                        ui.horizontal(|ui| {
                            ui.label("Zoom Factor:");
                            if ui.add(Slider::new(&mut self.zoom_level, 1.0..=8.0).suffix("x")).changed() {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Horizontal Pan (X):");
                            if ui.add(Slider::new(&mut self.pan_x, -1.0..=1.0)).changed() {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Vertical Pan (Y):");
                            if ui.add(Slider::new(&mut self.pan_y, -1.0..=1.0)).changed() {
                                changed = true;
                            }
                        });
                    });

                    ui.add_space(8.0);

                    // Mini-Map Radar Box
                    ui.group(|ui| {
                        ui.label(RichText::new("Frame Location Mini-Map:").strong());
                        ui.vertical_centered(|ui| {
                            let (rect, _resp) = ui.allocate_exact_size(Vec2::new(200.0, 120.0), egui::Sense::click_and_drag());
                            let painter = ui.painter();

                            // Full frame background
                            painter.rect_filled(rect, 4.0, Color32::from_rgb(20, 24, 30));
                            painter.rect_stroke(rect, 4.0, egui::Stroke::new(1.0, Color32::from_rgb(60, 70, 90)), egui::StrokeKind::Outside);

                            // Zoomed viewport window indicator
                            let w_pct = (1.0 / self.zoom_level).min(1.0);
                            let h_pct = (1.0 / self.zoom_level).min(1.0);

                            let cx = rect.center().x + (self.pan_x * rect.width() * 0.4);
                            let cy = rect.center().y + (self.pan_y * rect.height() * 0.4);

                            let box_w = rect.width() * w_pct;
                            let box_h = rect.height() * h_pct;

                            let sub_rect = egui::Rect::from_center_size(egui::pos2(cx, cy), Vec2::new(box_w, box_h));
                            painter.rect_filled(sub_rect, 2.0, Color32::from_rgba_unmultiplied(100, 200, 255, 60));
                            painter.rect_stroke(sub_rect, 2.0, egui::Stroke::new(1.5, Color32::from_rgb(100, 200, 255)), egui::StrokeKind::Outside);
                        });
                    });

                    if changed {
                        self.apply_to_player(player);
                    }

                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("↺ Reset Zoom & Center").clicked() {
                            self.zoom_level = 1.0;
                            self.pan_x = 0.0;
                            self.pan_y = 0.0;
                            self.is_active = false;
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
