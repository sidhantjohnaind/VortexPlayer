//! ab_repeat_dialog.rs — A-B Repeat Interval & Seamless Looper Studio (PotPlayer/KMPlayer/MPC-BE)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

pub struct AbRepeatDialog {
    pub is_open: bool,
    pub point_a: Option<f64>,
    pub point_b: Option<f64>,
    pub is_looping: bool,
    pub loop_count: u32,
    pub target_loops: u32, // 0 = infinite
    pub auto_advance: bool,
    pub lead_in_seconds: f64,
    pub status_message: String,
}

impl Default for AbRepeatDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            point_a: None,
            point_b: None,
            is_looping: false,
            loop_count: 0,
            target_loops: 0,
            auto_advance: false,
            lead_in_seconds: 0.0,
            status_message: "Set Point A and Point B to begin A-B loop repeat.".to_string(),
        }
    }
}

impl AbRepeatDialog {
    pub fn new() -> Self { Self::default() }

    pub fn set_point_a(&mut self, current_time: f64) {
        self.point_a = Some(current_time);
        self.status_message = format!("Point A set to {:.2}s", current_time);
        if let Some(b) = self.point_b {
            if b <= current_time {
                self.point_b = None;
            }
        }
    }

    pub fn set_point_b(&mut self, current_time: f64) {
        if let Some(a) = self.point_a {
            if current_time > a {
                self.point_b = Some(current_time);
                self.is_looping = true;
                self.loop_count = 0;
                self.status_message = format!("Point B set to {:.2}s — Loop active ({:.2}s duration)", current_time, current_time - a);
                return;
            }
        }
        self.status_message = "Point B must be set AFTER Point A.".to_string();
    }

    pub fn clear(&mut self) {
        self.point_a = None;
        self.point_b = None;
        self.is_looping = false;
        self.loop_count = 0;
        self.status_message = "A-B Repeat cleared.".to_string();
    }

    pub fn check_and_seek(&mut self, player: &crate::engine::Player, current_pos: f64) {
        if !self.is_looping { return; }
        if let (Some(a), Some(b)) = (self.point_a, self.point_b) {
            if current_pos >= b || current_pos < a - 0.5 {
                self.loop_count += 1;
                if self.target_loops > 0 && self.loop_count >= self.target_loops {
                    self.is_looping = false;
                    self.status_message = format!("Finished {} target loops. Resuming normal playback.", self.target_loops);
                    return;
                }
                let seek_target = (a - self.lead_in_seconds).max(0.0);
                player.seek_absolute(seek_target);
            }
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player, stats: &crate::engine::MediaStats) {
        if !self.is_open { return; }

        self.check_and_seek(player, stats.time_pos);

        let mut open = self.is_open;
        egui::Window::new("🔁 A-B Repeat & Section Looper")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(560.0, 360.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("A-B Repeat Interval Looper").strong().size(15.0).color(Color32::from_rgb(140, 220, 180)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_looping {
                            ui.label(RichText::new(format!("● LOOPING ({}x)", self.loop_count)).color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● INACTIVE").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("Current Time: {:.2}s", stats.time_pos)).strong().color(Color32::WHITE));
                        if let Some(dur) = if stats.duration > 0.0 { Some(stats.duration) } else { None } {
                            ui.label(RichText::new(format!("/ {:.2}s", dur)).color(Color32::GRAY));
                        }
                    });

                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        let a_text = match self.point_a {
                            Some(t) => format!("Point A: {:.2}s", t),
                            None => "Point A: Not Set".to_string(),
                        };
                        ui.label(RichText::new(a_text).color(Color32::from_rgb(100, 200, 255)));

                        if ui.button("📍 Set [A] Here ([)").clicked() {
                            self.set_point_a(stats.time_pos);
                        }

                        ui.separator();

                        let b_text = match self.point_b {
                            Some(t) => format!("Point B: {:.2}s", t),
                            None => "Point B: Not Set".to_string(),
                        };
                        ui.label(RichText::new(b_text).color(Color32::from_rgb(255, 160, 100)));

                        if ui.button("📍 Set [B] Here (])").clicked() {
                            self.set_point_b(stats.time_pos);
                        }
                    });

                    if let (Some(a), Some(b)) = (self.point_a, self.point_b) {
                        ui.add_space(4.0);
                        ui.label(RichText::new(format!("Loop Interval: {:.2}s duration", b - a)).small().color(Color32::from_rgb(160, 240, 180)));
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Loop Controls & Options").strong());
                    ui.horizontal(|ui| {
                        ui.label("Loop Count Limit:");
                        egui::ComboBox::from_id_salt("ab_loop_limit")
                            .selected_text(if self.target_loops == 0 { "Infinite Loops".to_string() } else { format!("{} Loops", self.target_loops) })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.target_loops, 0, "Infinite Loops");
                                ui.selectable_value(&mut self.target_loops, 1, "1 Loop (Play Once & Stop)");
                                ui.selectable_value(&mut self.target_loops, 3, "3 Loops");
                                ui.selectable_value(&mut self.target_loops, 5, "5 Loops");
                                ui.selectable_value(&mut self.target_loops, 10, "10 Loops");
                            });
                    });

                    ui.add(Slider::new(&mut self.lead_in_seconds, 0.0..=3.0).text("Lead-in Buffer").suffix(" s"));
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if self.is_looping {
                        if ui.button("⏸ Pause Loop").clicked() { self.is_looping = false; }
                    } else if self.point_a.is_some() && self.point_b.is_some() {
                        if ui.button("▶ Resume Loop").clicked() { self.is_looping = true; }
                    }
                    if ui.button("🗑️ Clear A-B (\\)").clicked() { self.clear(); }
                    if let Some(a) = self.point_a {
                        if ui.button("⏮ Jump to A").clicked() { player.seek_absolute(a); }
                    }
                    if let Some(b) = self.point_b {
                        if ui.button("⏭ Jump to B").clicked() { player.seek_absolute(b); }
                    }
                });

                ui.add_space(4.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
            });
        self.is_open = open;
    }
}
