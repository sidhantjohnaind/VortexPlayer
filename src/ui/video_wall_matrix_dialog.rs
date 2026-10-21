//! video_wall_matrix_dialog.rs — Multi-Projector Video Wall Grid Matrix & Bezel Compensation Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

pub struct VideoWallMatrixDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub rows: usize, // 1 to 4
    pub cols: usize, // 1 to 4
    pub active_row: usize,
    pub active_col: usize,
    pub bezel_horizontal_px: u32,
    pub bezel_vertical_px: u32,
    pub edge_blend_overlap_pct: f32,
    pub status_message: String,
}

impl Default for VideoWallMatrixDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            rows: 2,
            cols: 2,
            active_row: 0,
            active_col: 0,
            bezel_horizontal_px: 12,
            bezel_vertical_px: 12,
            edge_blend_overlap_pct: 0.0,
            status_message: "Multi-screen video wall tile matrix ready.".to_string(),
        }
    }
}

impl VideoWallMatrixDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || (self.rows == 1 && self.cols == 1) {
            player.set_property_string("vf", "");
        } else {
            // lavfi crop filter: crop=iw/cols:ih/rows:(active_col*iw/cols):(active_row*ih/rows)
            let vf = format!(
                "lavfi=[crop=iw/{}:ih/{}:(iw/{})*{}:(ih/{})*{}]",
                self.cols, self.rows, self.cols, self.active_col, self.rows, self.active_row
            );
            player.set_property_string("vf", &vf);
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🔲 Multi-Screen Video Wall Matrix & Bezel Compensation")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(680.0, 520.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Video Wall Grid Matrix & Projector Blending")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(100, 220, 200)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.is_active {
                                ui.label(RichText::new("● WALL ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                            } else {
                                ui.label(RichText::new("● BYPASSED").color(Color32::GRAY).strong());
                            }
                        });
                    });

                    ui.separator();

                    let mut changed = false;

                    ui.group(|ui| {
                        if ui.checkbox(&mut self.is_active, "🔲 Enable Video Wall Tile Matrix Splitter").changed() {
                            changed = true;
                        }

                        ui.horizontal(|ui| {
                            ui.label("Display Matrix Layout:");
                            if ui.selectable_label(self.rows == 2 && self.cols == 2, "2 × 2 (4 Displays)").clicked() {
                                self.rows = 2;
                                self.cols = 2;
                                changed = true;
                            }
                            if ui.selectable_label(self.rows == 1 && self.cols == 2, "1 × 2 Dual Wide").clicked() {
                                self.rows = 1;
                                self.cols = 2;
                                changed = true;
                            }
                            if ui.selectable_label(self.rows == 3 && self.cols == 3, "3 × 3 (9 Displays)").clicked() {
                                self.rows = 3;
                                self.cols = 3;
                                changed = true;
                            }
                            if ui.selectable_label(self.rows == 4 && self.cols == 4, "4 × 4 (16 Wall)").clicked() {
                                self.rows = 4;
                                self.cols = 4;
                                changed = true;
                            }
                        });
                    });

                    ui.add_space(6.0);

                    // Interactive Matrix Grid Tile Selector
                    ui.group(|ui| {
                        ui.label(RichText::new("Select Current Screen Tile Placement:").strong());
                        ui.vertical_centered(|ui| {
                            for r in 0..self.rows {
                                ui.horizontal(|ui| {
                                    for c in 0..self.cols {
                                        let is_current = self.active_row == r && self.active_col == c;
                                        let bg = if is_current {
                                            Color32::from_rgb(40, 140, 100)
                                        } else {
                                            Color32::from_rgb(25, 30, 40)
                                        };

                                        let label = format!("Screen [{}, {}]", r + 1, c + 1);
                                        if ui.add(
                                            egui::Button::new(RichText::new(label).strong().color(Color32::WHITE))
                                                .min_size(Vec2::new(90.0, 50.0))
                                                .fill(bg),
                                        ).clicked() {
                                            self.active_row = r;
                                            self.active_col = c;
                                            changed = true;
                                        }
                                    }
                                });
                            }
                        });
                    });

                    ui.add_space(6.0);

                    // Bezel Compensation & Edge Blending
                    ui.group(|ui| {
                        ui.label(RichText::new("Bezel Frame Compensation & Edge Blend").strong());

                        ui.horizontal(|ui| {
                            ui.label("Horizontal Bezel Gap:");
                            if ui.add(Slider::new(&mut self.bezel_horizontal_px, 0..=80).suffix(" px")).changed() {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Vertical Bezel Gap:");
                            if ui.add(Slider::new(&mut self.bezel_vertical_px, 0..=80).suffix(" px")).changed() {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Projector Soft Edge Blend Overlap:");
                            if ui.add(Slider::new(&mut self.edge_blend_overlap_pct, 0.0..=25.0).suffix(" %")).changed() {
                                changed = true;
                            }
                        });
                    });

                    if changed {
                        self.apply_to_player(player);
                    }

                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Reset Video Wall").clicked() {
                            self.rows = 1;
                            self.cols = 1;
                            self.active_row = 0;
                            self.active_col = 0;
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
