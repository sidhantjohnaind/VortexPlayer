//! video_puzzle_dialog.rs — Classic VLC Video Interactive Sliding Puzzle Minigame

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Vec2};

pub struct VideoPuzzleDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub grid_dimension: usize, // 3 for 3x3, 4 for 4x4
    pub tiles: Vec<usize>,
    pub moves_count: u32,
    pub is_solved: bool,
    pub status_message: String,
}

impl Default for VideoPuzzleDialog {
    fn default() -> Self {
        let dim = 3;
        let mut tiles: Vec<usize> = (0..(dim * dim)).collect();
        // Shuffle initial state
        tiles.swap(1, 2);
        tiles.swap(4, 7);
        tiles.swap(0, 5);

        Self {
            is_open: false,
            is_active: false,
            grid_dimension: dim,
            tiles,
            moves_count: 0,
            is_solved: false,
            status_message: "Click adjacent tiles to slide and solve the video puzzle!".to_string(),
        }
    }
}

impl VideoPuzzleDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shuffle(&mut self) {
        let n = self.grid_dimension * self.grid_dimension;
        self.tiles = (0..n).collect();
        // Deterministic pseudo-random shuffle
        for i in 0..n {
            let target = (i * 7 + 3) % n;
            self.tiles.swap(i, target);
        }
        self.moves_count = 0;
        self.is_solved = false;
        self.status_message = "Puzzle shuffled! Good luck solving it!".to_string();
    }

    pub fn check_solved(&mut self) {
        let n = self.grid_dimension * self.grid_dimension;
        let mut solved = true;
        for i in 0..n {
            if self.tiles.get(i) != Some(&i) {
                solved = false;
                break;
            }
        }
        self.is_solved = solved;
        if solved {
            self.status_message = format!("🎉 CONGRATULATIONS! Solved in {} moves!", self.moves_count);
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, _player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🧩 Interactive Video Puzzle Game (VLC Classic)")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(580.0, 520.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Interactive Real-Time Video Puzzle")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(255, 210, 80)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.is_solved {
                                ui.label(RichText::new("★ SOLVED!").color(Color32::from_rgb(60, 220, 100)).strong());
                            } else {
                                ui.label(RichText::new(format!("Moves: {}", self.moves_count)).color(Color32::WHITE).strong());
                            }
                        });
                    });

                    ui.separator();

                    // Grid dimension selector & buttons
                    ui.horizontal(|ui| {
                        ui.label("Grid Size:");
                        if ui.selectable_label(self.grid_dimension == 3, "3 × 3 (9 Pieces)").clicked() {
                            self.grid_dimension = 3;
                            self.shuffle();
                        }
                        if ui.selectable_label(self.grid_dimension == 4, "4 × 4 (16 Pieces)").clicked() {
                            self.grid_dimension = 4;
                            self.shuffle();
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🔀 Shuffle").clicked() {
                                self.shuffle();
                            }
                        });
                    });

                    ui.add_space(8.0);

                    // Interactive Puzzle Game Board
                    let dim = self.grid_dimension;
                    let tile_size = if dim == 3 { 80.0 } else { 60.0 };

                    ui.vertical_centered(|ui| {
                        let mut clicked_swap: Option<(usize, usize)> = None;

                        egui::Frame::new()
                            .fill(Color32::from_rgb(16, 18, 22))
                            .inner_margin(egui::Margin::same(12))
                            .show(ui, |ui| {
                                for row in 0..dim {
                                    ui.horizontal(|ui| {
                                        for col in 0..dim {
                                            let idx = row * dim + col;
                                            if let Some(&tile_val) = self.tiles.get(idx) {
                                                let is_correct = tile_val == idx;
                                                let bg = if is_correct {
                                                    Color32::from_rgb(30, 80, 50)
                                                } else {
                                                    Color32::from_rgb(40, 50, 70)
                                                };

                                                let btn_text = format!("[ {} ]\nTile #{}", tile_val + 1, idx + 1);
                                                let btn = egui::Button::new(RichText::new(btn_text).strong().size(13.0).color(Color32::WHITE))
                                                    .min_size(Vec2::new(tile_size, tile_size))
                                                    .fill(bg);

                                                if ui.add(btn).clicked() {
                                                    // Swap with next neighbor
                                                    let next_idx = (idx + 1) % (dim * dim);
                                                    clicked_swap = Some((idx, next_idx));
                                                }
                                            }
                                        }
                                    });
                                }
                            });

                        if let Some((a, b)) = clicked_swap {
                            self.tiles.swap(a, b);
                            self.moves_count += 1;
                            self.check_solved();
                        }
                    });

                    ui.add_space(8.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Solve Automatically").clicked() {
                            let n = self.grid_dimension * self.grid_dimension;
                            self.tiles = (0..n).collect();
                            self.is_solved = true;
                            self.status_message = "Solved automatically!".to_string();
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
