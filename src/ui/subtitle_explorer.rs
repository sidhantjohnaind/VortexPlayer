use super::theme::VortexTheme;
use eframe::egui::{self, Color32, CornerRadius, FontId, Margin, Rect, RichText, Stroke, Vec2};
use std::path::PathBuf;

pub struct SubtitleExplorerDialog {
    pub is_open: bool,
    pub is_pinned: bool,
    pub search_query: String,
    pub lines: Vec<(f64, f64, String)>, // start, end, text
    pub loaded_source: String,
    pub current_time: f64,
    pub pending_load_path: Option<PathBuf>,
}

impl Default for SubtitleExplorerDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_pinned: false,
            search_query: String::new(),
            lines: Vec::new(),
            loaded_source: String::new(),
            current_time: 0.0,
            pending_load_path: None,
        }
    }
}

impl SubtitleExplorerDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    /// Load parsed subtitle cues from an on-disk subtitle file (.srt, .vtt, .ass, .ssa, .smi)
    pub fn load_file(&mut self, path: &std::path::Path) -> bool {
        if let Ok(parsed) = crate::subtitles::load_subtitles_from_file(path) {
            if !parsed.is_empty() {
                self.lines = parsed;
                self.loaded_source = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                return true;
            }
        }
        false
    }

    /// Record or synchronize a real-time subtitle cue from the playback engine
    pub fn record_live_cue(&mut self, time_pos: f64, text: &str) {
        let clean = text.trim();
        if clean.is_empty() {
            return;
        }
        // Check if cue with same text already exists around this timestamp
        let already_exists = self.lines.iter().any(|(s, e, t)| {
            t == clean && (time_pos >= *s - 1.5 && time_pos <= *e + 1.5)
        });
        if !already_exists {
            self.lines.push((time_pos, time_pos + 3.0, clean.to_string()));
            self.lines.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            if self.loaded_source.is_empty() {
                self.loaded_source = "Live Playback Stream".to_string();
            }
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, seek_to_time: &mut Option<f64>) {
        if !self.is_open {
            return;
        }

        let skin = VortexTheme::current_skin();
        let mut open_flag = self.is_open;
        let mut close_requested = false;
        let mut browse_sub_clicked = false;

        // Custom window without OS title bar: pass "" so title_bar(false) is applied
        crate::ui::native_window::show_native_window(
            ctx,
            "vortex_subtitle_explorer_window",
            "",
            Vec2::new(620.0, 480.0),
            true,
            &mut open_flag,
            |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 6.0);

                // ── Custom Unified Header Bar ─────────────────────────────────
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);
                    ui.label(RichText::new("📝 Subtitle Explorer").size(13.5).strong().color(skin.accent_primary));
                    ui.label(RichText::new("• Timeline Dialogue Search").size(11.5).color(skin.text_muted));

                    if !self.loaded_source.is_empty() {
                        let short_src = if self.loaded_source.chars().count() > 28 {
                            format!("{}...", self.loaded_source.chars().take(26).collect::<String>())
                        } else {
                            self.loaded_source.clone()
                        };
                        let badge_frame = egui::Frame::new()
                            .fill(Color32::from_rgb(20, 26, 38))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(40, 55, 80)))
                            .corner_radius(CornerRadius::same(3))
                            .inner_margin(Margin::symmetric(5, 1));
                        badge_frame.show(ui, |ui| {
                            ui.label(RichText::new(format!("📂 {}", short_src)).size(10.5).color(Color32::from_rgb(140, 180, 240)));
                        });
                    }

                    // Right side controls: [📂 Load Subtitle] [📌 Pin] [✕]
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 1. Close button (Rightmost)
                        let close_btn = ui.add(
                            egui::Button::new(RichText::new("✕").size(12.0).color(skin.text_secondary))
                                .fill(Color32::TRANSPARENT)
                                .stroke(Stroke::NONE)
                        ).on_hover_text("Close Subtitle Explorer (Esc)");
                        if close_btn.clicked() {
                            close_requested = true;
                        }

                        ui.add_space(4.0);

                        // 2. Pin button (Immediately adjacent to Close!)
                        let pin_icon = if self.is_pinned { "📌 Unpin" } else { "📍 Pin" };
                        let pin_btn = ui.add(
                            egui::Button::new(
                                RichText::new(pin_icon)
                                    .size(11.0)
                                    .color(if self.is_pinned { skin.accent_bright } else { Color32::from_rgb(175, 180, 195) })
                            )
                            .fill(if self.is_pinned { Color32::from_rgb(32, 40, 56) } else { Color32::from_rgb(22, 24, 30) })
                            .stroke(Stroke::new(1.0, if self.is_pinned { Color32::from_rgb(50, 70, 110) } else { Color32::from_rgb(40, 44, 56) }))
                            .corner_radius(CornerRadius::same(4))
                        ).on_hover_text(if self.is_pinned { "Unpin window" } else { "Pin window on top of playback" });
                        if pin_btn.clicked() {
                            self.is_pinned = !self.is_pinned;
                        }

                        ui.add_space(4.0);

                        // 3. Load External Subtitle File Button
                        let load_btn = ui.add(
                            egui::Button::new(
                                RichText::new("📂 Open Subtitle File...")
                                    .size(11.0)
                                    .color(skin.text_primary)
                            )
                            .fill(Color32::from_rgb(22, 26, 36))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(45, 55, 75)))
                            .corner_radius(CornerRadius::same(4))
                        ).on_hover_text("Load an external .srt, .ass, .ssa, .vtt, or .smi subtitle file");
                        if load_btn.clicked() {
                            browse_sub_clicked = true;
                        }
                    });
                });

                ui.separator();

                // ── Integrated Search Box ─────────────────────────────────────
                let has_query = !self.search_query.is_empty();
                let search_frame = egui::Frame::new()
                    .fill(Color32::from_rgb(14, 16, 22))
                    .stroke(Stroke::new(1.0, if has_query { skin.accent_primary } else { Color32::from_rgb(35, 40, 52) }))
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::symmetric(8, 5));

                search_frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);
                        ui.label(RichText::new("🔍").size(12.0).color(skin.accent_primary));

                        let avail_w = ui.available_width() - if has_query { 130.0 } else { 105.0 };
                        let text_edit = egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text("Search dialogue text or timestamps (e.g. 'hello', '01:30')...")
                            .font(FontId::proportional(12.0))
                            .text_color(Color32::WHITE)
                            .desired_width(avail_w.max(80.0))
                            .frame(egui::Frame::NONE);
                        ui.add(text_edit);

                        if has_query {
                            let clear_btn = egui::Button::new(RichText::new("✕").size(10.0).color(skin.text_muted))
                                .fill(Color32::from_rgb(26, 30, 40))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(45, 50, 65)))
                                .corner_radius(CornerRadius::same(3));
                            if ui.add_sized(Vec2::new(18.0, 18.0), clear_btn).on_hover_text("Clear search query").clicked() {
                                self.search_query.clear();
                            }
                        }

                        // Match count badge
                        let query_lower = self.search_query.trim().to_lowercase();
                        let match_count = if query_lower.is_empty() {
                            self.lines.len()
                        } else {
                            self.lines.iter().filter(|(_, _, text)| text.to_lowercase().contains(&query_lower)).count()
                        };

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let badge_text = if query_lower.is_empty() {
                                format!("{} lines", self.lines.len())
                            } else {
                                format!("{}/{} matches", match_count, self.lines.len())
                            };
                            let badge_col = if match_count > 0 { skin.accent_primary } else { Color32::from_rgb(240, 100, 100) };
                            ui.label(RichText::new(badge_text).size(10.5).color(badge_col).strong());
                        });
                    });
                });

                ui.add_space(2.0);

                // ── Dialogue Lines List ───────────────────────────────────────
                if self.lines.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(35.0);
                        ui.label(RichText::new("📖 No Subtitle Dialogue Loaded").size(14.0).strong().color(skin.text_primary));
                        ui.add_space(4.0);
                        ui.label(RichText::new("Subtitles will appear here when playing media with embedded or external subtitles.").color(skin.text_muted).size(11.5));
                        ui.add_space(10.0);
                        let open_file_btn = egui::Button::new(
                            RichText::new("📂 Browse & Load Subtitle File (.srt / .vtt / .ass)...")
                                .size(12.0)
                                .color(Color32::WHITE)
                        )
                        .fill(Color32::from_rgb(28, 40, 60))
                        .stroke(Stroke::new(1.0, skin.accent_primary))
                        .corner_radius(CornerRadius::same(6));
                        if ui.add(open_file_btn).clicked() {
                            browse_sub_clicked = true;
                        }
                    });
                } else {
                    let query_lower = self.search_query.trim().to_lowercase();
                    egui::ScrollArea::vertical()
                        .id_salt("vortex_subtitle_explorer_scroll_area")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(0.0, 3.0);
                            let mut shown_count = 0;

                            for (start, end, text) in self.lines.iter() {
                                let matches = query_lower.is_empty() || text.to_lowercase().contains(&query_lower);
                                if !matches {
                                    continue;
                                }
                                shown_count += 1;

                                let is_active = self.current_time >= *start && self.current_time <= *end;
                                let mins = (*start / 60.0).floor() as u32;
                                let secs = (*start % 60.0).floor() as u32;
                                let ms = ((*start - (*start).floor()) * 1000.0).round() as u32;
                                let time_str = format!("{:02}:{:02}.{:01}", mins, secs, ms / 100);

                                let row_frame = egui::Frame::new()
                                    .fill(if is_active {
                                        Color32::from_rgb(26, 36, 52)
                                    } else if shown_count % 2 == 0 {
                                        Color32::from_rgb(16, 18, 24)
                                    } else {
                                        Color32::from_rgb(12, 14, 18)
                                    })
                                    .stroke(Stroke::new(1.0, if is_active { skin.accent_primary } else { Color32::from_rgb(24, 28, 36) }))
                                    .corner_radius(CornerRadius::same(4))
                                    .inner_margin(Margin::symmetric(7, 4));

                                row_frame.show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing = Vec2::new(8.0, 0.0);

                                        // Jump Timestamp Button
                                        let jump_btn = egui::Button::new(
                                            RichText::new(format!("▶ {}", time_str))
                                                .monospace()
                                                .size(11.0)
                                                .color(if is_active { skin.accent_bright } else { skin.accent_primary })
                                        )
                                        .fill(if is_active { Color32::from_rgb(35, 50, 75) } else { Color32::from_rgb(20, 24, 34) })
                                        .stroke(Stroke::new(1.0, if is_active { skin.accent_primary } else { Color32::from_rgb(35, 42, 58) }))
                                        .corner_radius(CornerRadius::same(3));

                                        if ui.add(jump_btn).on_hover_text(format!("Jump video to {}", time_str)).clicked() {
                                            *seek_to_time = Some(*start);
                                        }

                                        // Dialogue Text
                                        let text_label = egui::Label::new(
                                            RichText::new(text)
                                                .size(11.5)
                                                .color(if is_active { Color32::WHITE } else { skin.text_primary })
                                        );
                                        let text_resp = ui.add(text_label);
                                        if text_resp.clicked() {
                                            *seek_to_time = Some(*start);
                                        }

                                        // Copy button on right side
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            let copy_btn = egui::Button::new(RichText::new("📋").size(9.5).color(skin.text_muted))
                                                .fill(Color32::TRANSPARENT)
                                                .stroke(Stroke::NONE);
                                            if ui.add(copy_btn).on_hover_text("Copy subtitle dialogue to clipboard").clicked() {
                                                ui.ctx().copy_text(text.clone());
                                            }
                                        });
                                    });
                                });
                            }

                            if shown_count == 0 {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(20.0);
                                    ui.label(RichText::new(format!("No dialogue matched \"{}\"", query_lower)).color(skin.text_muted).size(11.5));
                                });
                            }
                        });
                }
            },
        );

        if browse_sub_clicked {
            if let Some(sub) = rfd::FileDialog::new()
                .add_filter("Subtitles (*.srt, *.ass, *.ssa, *.vtt, *.smi)", &["srt", "ass", "ssa", "vtt", "smi"])
                .pick_file()
            {
                if self.load_file(&sub) {
                    self.pending_load_path = Some(sub);
                }
            }
        }

        if close_requested {
            self.is_open = false;
        } else {
            self.is_open = open_flag;
        }
    }
}
