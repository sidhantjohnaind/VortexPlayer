#![allow(dead_code)]

use super::theme::VortexTheme;
use crate::bookmark::{format_time, BookmarkManager};
use crate::config::AppConfig;
use crate::engine::chapters::ChapterItem;
use crate::engine::Player;
use eframe::egui::{self, Align2, Color32, CornerRadius, RichText, Stroke, Vec2};
use std::sync::Arc;

pub struct AutoSkipDialog {
    pub is_open: bool,
    pub new_start_sec: f64,
    pub new_end_sec: f64,
    pub new_label: String,
}

impl Default for AutoSkipDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            new_start_sec: 0.0,
            new_end_sec: 90.0,
            new_label: "Opening Theme".to_string(),
        }
    }
}

impl AutoSkipDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        bookmark_mgr: &mut BookmarkManager,
        config: &mut AppConfig,
        chapters: &[ChapterItem],
        player: Option<&Arc<Player>>,
        current_time: f64,
        duration: f64,
    ) -> Option<egui::Rect> {
        if !self.is_open {
            return None;
        }

        let mut open = self.is_open;
        let mut should_close = false;

        let window_resp = egui::Window::new("Auto-Skip Range Manager & Table (PotPlayer Style)")
            .id(egui::Id::new("auto_skip_range_manager_window"))
            .open(&mut open)
            .resizable(true)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_width(700.0)
            .default_height(500.0)
            .show(ctx, |ui| {
                // 1. Master Engine Switch & Current Playhead
                ui.horizontal(|ui| {
                    let mut engine_active = bookmark_mgr.auto_skip_bookmarks || config.skip_intro_enabled;
                    let engine_color = if engine_active { Color32::from_rgb(255, 90, 70) } else { Color32::GRAY };
                    if ui.checkbox(
                        &mut engine_active,
                        RichText::new("Enable Auto-Skip Engine (Ctrl+Alt+S)")
                            .strong()
                            .color(engine_color),
                    ).changed() {
                        bookmark_mgr.auto_skip_bookmarks = engine_active;
                        config.skip_intro_enabled = engine_active;
                        let _ = config.save();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("⏱ Playhead: {}", format_time(current_time)))
                                .monospace()
                                .color(Color32::from_rgb(180, 200, 240)),
                        );
                    });
                });

                ui.add_space(4.0);
                ui.separator();

                // 2. Global Preset Skip Durations (Opening / Ending)
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Fixed Preset Skips:").strong().color(VortexTheme::POT_YELLOW));
                        ui.add_space(10.0);

                        let mut intro_active = config.skip_intro_sec > 0.0;
                        if ui.checkbox(&mut intro_active, "Skip Intro:").changed() {
                            config.skip_intro_sec = if intro_active { 90.0 } else { 0.0 };
                            let _ = config.save();
                        }
                        if intro_active {
                            if ui.add(egui::DragValue::new(&mut config.skip_intro_sec).range(0.0..=300.0).speed(1.0).suffix("s")).changed() {
                                let _ = config.save();
                            }
                        }

                        ui.add_space(12.0);

                        let mut outro_active = config.skip_outro_sec > 0.0;
                        if ui.checkbox(&mut outro_active, "Skip Outro:").changed() {
                            config.skip_outro_sec = if outro_active { 90.0 } else { 0.0 };
                            let _ = config.save();
                        }
                        if outro_active {
                            if ui.add(egui::DragValue::new(&mut config.skip_outro_sec).range(0.0..=300.0).speed(1.0).suffix("s")).changed() {
                                let _ = config.save();
                            }
                        }

                        // 1-Click Import Chapters Button
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let has_chapters = !chapters.is_empty();
                            let import_btn = ui.add_enabled(
                                has_chapters,
                                egui::Button::new(RichText::new("📥 Import OP/ED from Chapters").strong().color(Color32::from_rgb(70, 180, 255))),
                            );
                            if import_btn.clicked() {
                                let mut imported_count = 0;
                                for (i, ch) in chapters.iter().enumerate() {
                                    let u = ch.title.to_uppercase();
                                    if u.contains("OP") || u.contains("OPENING") || u.contains("INTRO")
                                        || u.contains("PROLOGUE") || u.contains("RECAP")
                                        || u.contains("ED") || u.contains("ENDING") || u.contains("CREDITS")
                                        || u.contains("PREVIEW") || u.contains("THEME")
                                    {
                                        let start = ch.time_pos;
                                        let end = if let Some(next_c) = chapters.get(i + 1) {
                                            next_c.time_pos
                                        } else if duration > start {
                                            duration
                                        } else {
                                            start + 90.0
                                        };
                                        if end > start {
                                            let label = if ch.title.trim().is_empty() {
                                                format!("Chapter {}", i + 1)
                                            } else {
                                                ch.title.trim().to_string()
                                            };
                                            bookmark_mgr.add_skip_interval(start, end, label);
                                            imported_count += 1;
                                        }
                                    }
                                }
                                if imported_count > 0 {
                                    bookmark_mgr.sync_skip_intervals_to_bookmarks();
                                }
                            }
                        });
                    });
                });

                ui.add_space(4.0);

                // 3. Add Custom Skip Range Box with Chapter Picker Dropdown
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("➕ Add Interval:").strong().color(Color32::from_rgb(130, 220, 140)));
                        ui.label("Title:");
                        ui.add(egui::TextEdit::singleline(&mut self.new_label).desired_width(120.0));

                        // Chapter selection menu from media file
                        let ch_count = chapters.len();
                        let ch_btn_text = if ch_count > 0 {
                            format!("📖 Chapters ({}) ▾", ch_count)
                        } else {
                            "📖 Chapters (0) ▾".to_string()
                        };
                        let ch_btn_color = if ch_count > 0 {
                            Color32::from_rgb(70, 190, 255)
                        } else {
                            Color32::GRAY
                        };

                        ui.menu_button(RichText::new(ch_btn_text).color(ch_btn_color).strong(), |ui| {
                            if chapters.is_empty() {
                                ui.label(RichText::new("No chapters detected in current media file.").italics().color(Color32::GRAY));
                            } else {
                                ui.label(RichText::new(format!("Chapters from File ({} found):", ch_count)).strong().color(VortexTheme::POT_YELLOW));
                                ui.label(RichText::new("Click a chapter to load title and timestamps:").size(10.5).color(Color32::from_rgb(150, 160, 180)));
                                ui.separator();
                                egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                                    for (i, ch) in chapters.iter().enumerate() {
                                        let ch_title = if ch.title.trim().is_empty() {
                                            format!("Chapter {}", i + 1)
                                        } else {
                                            ch.title.trim().to_string()
                                        };
                                        let start = ch.time_pos;
                                        let end = if let Some(next_c) = chapters.get(i + 1) {
                                            next_c.time_pos
                                        } else if duration > start {
                                            duration
                                        } else {
                                            start + 90.0
                                        };
                                        let dur = (end - start).max(0.0);

                                        ui.horizontal(|ui| {
                                            let row_label = format!("{}. {} ({} - {}, dur: {})", i + 1, ch_title, format_time(start), format_time(end), format_time(dur));
                                            if ui.button(RichText::new(row_label).strong()).on_hover_text("Load chapter title, start, and end time into input fields").clicked() {
                                                self.new_label = ch_title.clone();
                                                self.new_start_sec = start;
                                                self.new_end_sec = end;
                                                ui.close();
                                            }
                                            if ui.small_button("➕ Add").on_hover_text("Directly add this chapter to Auto-Skip Table").clicked() {
                                                bookmark_mgr.add_skip_interval(start, end, ch_title.clone());
                                                ui.close();
                                            }
                                        });
                                    }
                                });
                            }
                            ui.separator();
                            ui.label(RichText::new("Quick Title Presets:").size(10.5).color(Color32::from_rgb(160, 165, 185)));
                            ui.horizontal_wrapped(|ui| {
                                for preset in &["OP", "ED", "Opening", "Ending", "Intro", "Outro", "Recap", "Preview"] {
                                    if ui.small_button(*preset).on_hover_text(format!("Set title to '{}'", preset)).clicked() {
                                        self.new_label = preset.to_string();
                                        ui.close();
                                    }
                                }
                            });
                        });

                        ui.add_space(4.0);
                        ui.label("Start:");
                        ui.add(egui::DragValue::new(&mut self.new_start_sec).range(0.0..=duration.max(10.0)).speed(1.0).suffix("s"));
                        if ui.small_button("⏱ Curr").on_hover_text("Use current playhead position as start time").clicked() {
                            self.new_start_sec = current_time;
                        }

                        ui.add_space(4.0);
                        ui.label("End:");
                        ui.add(egui::DragValue::new(&mut self.new_end_sec).range(0.0..=duration.max(10.0)).speed(1.0).suffix("s"));
                        if ui.small_button("⏱ Curr").on_hover_text("Use current playhead position as end time").clicked() {
                            self.new_end_sec = current_time;
                        }
                        if ui.small_button("+85s").on_hover_text("Set End = Start + 85s (Standard Anime OP)").clicked() {
                            self.new_end_sec = self.new_start_sec + 85.0;
                        }
                        if ui.small_button("+90s").on_hover_text("Set End = Start + 90s (TV Intro)").clicked() {
                            self.new_end_sec = self.new_start_sec + 90.0;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(RichText::new("➕ Add").strong().color(Color32::WHITE)).clicked() {
                                if self.new_end_sec > self.new_start_sec {
                                    bookmark_mgr.add_skip_interval(self.new_start_sec, self.new_end_sec, self.new_label.trim().to_string());
                                }
                            }
                        });
                    });
                });

                ui.add_space(4.0);

                // 4. PotPlayer Skip Range Table
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Auto-Skip Table:").strong().size(12.5));
                    let active_count = bookmark_mgr.skip_intervals.iter().filter(|i| i.enabled).count();
                    ui.label(
                        RichText::new(format!("({} active / {} total)", active_count, bookmark_mgr.skip_intervals.len()))
                            .size(11.0)
                            .color(Color32::from_rgb(160, 165, 180)),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("Clear All").clicked() {
                            bookmark_mgr.clear_skip_intervals();
                        }
                        if ui.small_button("Deselect All").clicked() {
                            for item in &mut bookmark_mgr.skip_intervals {
                                item.enabled = false;
                            }
                            bookmark_mgr.sync_skip_intervals_to_bookmarks();
                        }
                        if ui.small_button("Select All").clicked() {
                            for item in &mut bookmark_mgr.skip_intervals {
                                item.enabled = true;
                            }
                            bookmark_mgr.sync_skip_intervals_to_bookmarks();
                        }
                    });
                });

                // Table Header
                let table_header_rect = ui.available_rect_before_wrap();
                let header_h = 24.0;
                let header_r = egui::Rect::from_min_size(table_header_rect.min, Vec2::new(table_header_rect.width(), header_h));
                ui.painter().rect_filled(header_r, CornerRadius::same(3), Color32::from_rgb(26, 29, 38));
                ui.painter().rect_stroke(header_r, CornerRadius::same(3), Stroke::new(1.0, Color32::from_rgb(45, 50, 65)), egui::StrokeKind::Inside);

                ui.horizontal(|ui| {
                    ui.set_height(header_h);
                    ui.add_space(6.0);
                    ui.label(RichText::new("Active").strong().size(11.0).color(Color32::from_rgb(180, 190, 210)));
                    ui.add_space(20.0);
                    ui.label(RichText::new("Label / Description").strong().size(11.0).color(Color32::from_rgb(180, 190, 210)));
                    ui.add_space(110.0);
                    ui.label(RichText::new("Start").strong().size(11.0).color(Color32::from_rgb(180, 190, 210)));
                    ui.add_space(32.0);
                    ui.label(RichText::new("End").strong().size(11.0).color(Color32::from_rgb(180, 190, 210)));
                    ui.add_space(36.0);
                    ui.label(RichText::new("Duration").strong().size(11.0).color(Color32::from_rgb(180, 190, 210)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        ui.label(RichText::new("Actions").strong().size(11.0).color(Color32::from_rgb(180, 190, 210)));
                    });
                });

                // Scrollable Table Body
                egui::ScrollArea::vertical()
                    .id_salt("auto_skip_table_scroll")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        let mut delete_index = None;
                        let mut sync_needed = false;

                        if bookmark_mgr.skip_intervals.is_empty() {
                            ui.add_space(20.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new("No skip intervals configured for this video.")
                                        .color(Color32::from_rgb(140, 145, 160)),
                                );
                                ui.label(
                                    RichText::new("Add custom intervals above or click 'Import OP/ED from Chapters'.")
                                        .size(10.5)
                                        .color(Color32::from_rgb(110, 115, 130)),
                                );
                            });
                            ui.add_space(20.0);
                        } else {
                            for (idx, interval) in bookmark_mgr.skip_intervals.iter_mut().enumerate() {
                                let row_bg = if idx % 2 == 0 {
                                    Color32::from_rgba_unmultiplied(255, 255, 255, 4)
                                } else {
                                    Color32::TRANSPARENT
                                };

                                let row_response = ui.horizontal(|ui| {
                                    if row_bg != Color32::TRANSPARENT {
                                        let rect = ui.available_rect_before_wrap();
                                        ui.painter().rect_filled(rect, CornerRadius::ZERO, row_bg);
                                    }

                                    // 1. Active Checkbox
                                    if ui.checkbox(&mut interval.enabled, "").changed() {
                                        sync_needed = true;
                                    }

                                    // 2. Label
                                    let label_col = if interval.enabled {
                                        Color32::from_rgb(230, 235, 245)
                                    } else {
                                        Color32::from_rgb(120, 125, 135)
                                    };
                                    ui.add(egui::Label::new(
                                        RichText::new(&interval.label).strong().color(label_col)
                                    ).truncate()).on_hover_text(&interval.label);

                                    // 3. Timestamps & Duration
                                    let dur_sec = (interval.end - interval.start).max(0.0);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        // Actions
                                        if ui.small_button("✕").on_hover_text("Delete this skip interval").clicked() {
                                            delete_index = Some(idx);
                                        }
                                        if let Some(p) = player {
                                            if ui.small_button("▶ Jump").on_hover_text("Seek to start of this interval").clicked() {
                                                p.seek_absolute(interval.start);
                                            }
                                        }

                                        ui.add_space(16.0);
                                        // Duration
                                        ui.label(
                                            RichText::new(format_time(dur_sec))
                                                .monospace()
                                                .color(Color32::from_rgb(100, 180, 240)),
                                        );

                                        ui.add_space(20.0);
                                        // End Time
                                        ui.label(
                                            RichText::new(format_time(interval.end))
                                                .monospace()
                                                .color(Color32::from_rgb(200, 205, 220)),
                                        );

                                        ui.add_space(20.0);
                                        // Start Time
                                        ui.label(
                                            RichText::new(format_time(interval.start))
                                                .monospace()
                                                .color(Color32::from_rgb(200, 205, 220)),
                                        );
                                    });
                                });
                                let _ = row_response;
                            }
                        }

                        if let Some(del_idx) = delete_index {
                            bookmark_mgr.remove_skip_interval(del_idx);
                        } else if sync_needed {
                            bookmark_mgr.sync_skip_intervals_to_bookmarks();
                        }
                    });

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("💡 Configured skip ranges are automatically saved in the video's .pbf bookmark file.")
                            .size(10.5)
                            .color(Color32::from_rgb(140, 145, 160)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("  Close  ").clicked() {
                            should_close = true;
                        }
                    });
                });
            });

        if should_close {
            open = false;
        }
        self.is_open = open;
        window_resp.map(|r| r.response.rect)
    }
}
