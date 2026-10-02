//! chapter_marker_dialog.rs — Interactive Chapter Timeline Manager & Scene Title Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};

#[derive(Debug, Clone)]
pub struct ChapterEntry {
    pub index: usize,
    pub title: String,
    pub start_time: f64,
    pub end_time: f64,
}

pub struct ChapterMarkerDialog {
    pub is_open: bool,
    pub last_file_path: String,
    pub chapters: Vec<ChapterEntry>,
    pub selected_idx: Option<usize>,
    pub new_chapter_title: String,
    pub auto_skip_intros: bool,
    pub status_message: String,
}

impl Default for ChapterMarkerDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            last_file_path: String::new(),
            chapters: Vec::new(),
            selected_idx: Some(0),
            new_chapter_title: "Custom Chapter Marker".to_string(),
            auto_skip_intros: true,
            status_message: "No chapter scene markers loaded.".to_string(),
        }
    }
}

impl ChapterMarkerDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        player: &crate::engine::Player,
        stats: &crate::engine::MediaStats,
        config: &mut crate::config::AppConfig,
    ) {
        if !self.is_open {
            return;
        }

        // Dynamically load real container chapters when media changes or dialog opens
        if (!stats.file_path.is_empty() && self.last_file_path != stats.file_path) || (self.chapters.is_empty() && !stats.chapters.is_empty()) {
            self.last_file_path = stats.file_path.clone();
            if !stats.chapters.is_empty() {
                self.chapters = stats.chapters.iter().enumerate().map(|(i, c)| {
                    let end_time = if let Some(next_c) = stats.chapters.get(i + 1) {
                        next_c.time_pos
                    } else if stats.duration > c.time_pos {
                        stats.duration
                    } else {
                        c.time_pos + 180.0
                    };
                    ChapterEntry {
                        index: (c.index + 1) as usize,
                        title: if c.title.trim().is_empty() { format!("Chapter {}", i + 1) } else { c.title.clone() },
                        start_time: c.time_pos,
                        end_time,
                    }
                }).collect();
                self.status_message = format!("{} container chapters loaded from metadata.", self.chapters.len());
            }
        }

        let mut open = self.is_open;
        egui::Window::new("📑 Chapter Timeline Manager & Scene Title Studio")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(740.0, 500.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Interactive Chapter Browser & Scene Indexer")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(255, 200, 100)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let curr_time = crate::bookmark::format_time(stats.time_pos);
                            ui.label(RichText::new(format!("Current: {}", curr_time)).strong().color(Color32::from_rgb(100, 220, 120)));
                        });
                    });

                    ui.separator();

                    // Quick Add Marker at current time
                    ui.horizontal(|ui| {
                        ui.label("Add Marker:");
                        ui.add(egui::TextEdit::singleline(&mut self.new_chapter_title).desired_width(220.0));
                        if ui.button("➕ Add at Current Time").clicked() {
                            let next_idx = self.chapters.len() + 1;
                            self.chapters.push(ChapterEntry {
                                index: next_idx,
                                title: self.new_chapter_title.clone(),
                                start_time: stats.time_pos,
                                end_time: stats.time_pos + 300.0,
                            });
                            self.chapters.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap_or(std::cmp::Ordering::Equal));
                            self.status_message = format!("Added chapter at {}", crate::bookmark::format_time(stats.time_pos));
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.checkbox(&mut config.skip_chapters_enabled, "⚡ Auto-Skip Theme Chapters (OP/ED)").changed() {
                                let _ = config.save();
                            }
                        });
                    });

                    ui.add_space(6.0);
                    ui.label(RichText::new("Container Chapter List:").strong());

                    // Chapter List Table
                    ScrollArea::vertical()
                        .max_height(240.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if self.chapters.is_empty() {
                                ui.label(RichText::new("(No chapters available for current media)").color(Color32::GRAY));
                            }
                            for (idx, ch) in self.chapters.iter().enumerate() {
                                let is_active = stats.time_pos >= ch.start_time && stats.time_pos < ch.end_time;
                                let is_selected = self.selected_idx == Some(idx);
                                let is_skip = config.matches_skip_chapter(&ch.title);

                                let bg = if is_active {
                                    Color32::from_rgb(45, 50, 25)
                                } else if is_selected {
                                    Color32::from_rgb(30, 45, 65)
                                } else if idx % 2 == 0 {
                                    Color32::from_rgb(22, 24, 28)
                                } else {
                                    Color32::from_rgb(18, 20, 24)
                                };

                                egui::Frame::new()
                                    .fill(bg)
                                    .inner_margin(egui::Margin::symmetric(10, 8))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(format!("#{}:", ch.index)).strong().color(Color32::from_rgb(255, 180, 80)));
                                            if ui.selectable_label(is_selected, &ch.title).clicked() {
                                                self.selected_idx = Some(idx);
                                            }

                                            if is_skip {
                                                ui.label(RichText::new("⏭ AUTO-SKIP").size(10.0).color(Color32::from_rgb(255, 120, 80)).strong());
                                            }

                                            let start_str = crate::bookmark::format_time(ch.start_time);
                                            let end_str = crate::bookmark::format_time(ch.end_time);
                                            ui.label(RichText::new(format!("{} → {}", start_str, end_str)).small().color(Color32::GRAY));

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if ui.add(
                                                    egui::Button::new(RichText::new("▶ Jump").strong().color(Color32::WHITE))
                                                        .fill(Color32::from_rgb(40, 120, 80)),
                                                ).clicked() {
                                                    player.seek_absolute(ch.start_time);
                                                }
                                                if is_skip {
                                                    if ui.add(
                                                        egui::Button::new(RichText::new("⏭ Skip").size(11.0).color(Color32::WHITE))
                                                            .fill(Color32::from_rgb(160, 60, 50)),
                                                    ).clicked() {
                                                        player.seek_absolute(ch.end_time);
                                                    }
                                                }
                                                if is_active {
                                                    ui.label(RichText::new("● PLAYING").color(Color32::from_rgb(255, 200, 50)).strong());
                                                }
                                            });
                                        });
                                    });
                                ui.add_space(2.0);
                            }
                        });

                    ui.add_space(6.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Next Chapter (PageDown)").clicked() {
                            player.next_chapter();
                        }
                        if ui.button("Prev Chapter (PageUp)").clicked() {
                            player.prev_chapter();
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
