use super::theme::VortexTheme;
use crate::engine::lyrics::{load_lyrics_for_media, LyricTrack};
use crate::engine::Player;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2,
};
use std::path::Path;

pub struct LyricsOverlay {
    pub is_enabled: bool,
    pub is_expanded: bool,
    pub track: Option<LyricTrack>,
    pub last_file_path: String,
    pub user_offset_sec: f64,
    pub copied_feedback: f64,
    pub auto_scroll: bool,
    pub last_active_idx: Option<usize>,
}

impl Default for LyricsOverlay {
    fn default() -> Self {
        Self {
            is_enabled: true,
            is_expanded: false,
            track: None,
            last_file_path: String::new(),
            user_offset_sec: 0.0,
            copied_feedback: 0.0,
            auto_scroll: true,
            last_active_idx: None,
        }
    }
}

impl LyricsOverlay {
    pub fn new() -> Self {
        Self::default()
    }

    /// Automatically scans and loads lyrics for the currently active media file
    pub fn update_track(
        &mut self,
        file_path: &str,
        player: Option<&Player>,
        mediainfo_text: Option<&str>,
    ) {
        if file_path.is_empty() {
            if !self.last_file_path.is_empty() {
                self.last_file_path.clear();
                self.track = None;
                self.user_offset_sec = 0.0;
                self.last_active_idx = None;
            }
            return;
        }

        let path_changed = self.last_file_path != file_path;
        if path_changed {
            self.last_file_path = file_path.to_string();
            self.track = None;
            self.user_offset_sec = 0.0;
            self.last_active_idx = None;
        }

        // If lyrics are not yet loaded (or if MediaInfo just became available), attempt load
        if self.track.is_none() {
            let mpv_lyrics = player.and_then(|p| {
                let keys = [
                    "metadata/by-key/Lyrics",
                    "metadata/by-key/lyrics",
                    "metadata/by-key/LYRICS",
                    "metadata/by-key/UNSYNCEDLYRICS",
                    "metadata/by-key/unsyncedlyrics",
                    "metadata/by-key/USLT",
                    "metadata/by-key/lyrics-eng",
                    "metadata/by-key/©lyr",
                    "metadata/by-key/SYNCEDLYRICS",
                ];
                for k in &keys {
                    if let Some(val) = p.get_property_string(k) {
                        let trimmed = val.trim();
                        if !trimmed.is_empty() {
                            return Some(trimmed.to_string());
                        }
                    }
                }
                None
            });

            self.track = load_lyrics_for_media(
                file_path,
                mpv_lyrics.as_deref(),
                mediainfo_text,
            );
        }
    }

    /// Manually loads an external `.lrc` file selected by the user
    pub fn load_from_file(&mut self, lrc_path: &Path) -> bool {
        if let Some(track) = LyricTrack::from_lrc_file(lrc_path) {
            self.track = Some(track);
            self.is_enabled = true;
            true
        } else {
            false
        }
    }

    pub fn toggle_enabled(&mut self) {
        self.is_enabled = !self.is_enabled;
    }

    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    pub fn adjust_offset(&mut self, delta: f64) {
        self.user_offset_sec += delta;
    }

    pub fn copy_to_clipboard(&mut self, ctx: &egui::Context) {
        if let Some(ref lrc) = self.track {
            let text = lrc
                .lines
                .iter()
                .map(|l| l.text.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            ctx.copy_text(text);
            self.copied_feedback = ctx.input(|i| i.time);
        }
    }

    /// Renders the Synchronized Lyrics Overlay
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        player_opt: Option<&Player>,
        video_rect: Rect,
        current_time: f64,
        is_song_mode: bool,
    ) {
        if !self.is_enabled {
            return;
        }

        let lrc = match self.track {
            Some(ref t) => t,
            None => return,
        };

        if lrc.lines.is_empty() {
            return;
        }

        let effective_time = (current_time + self.user_offset_sec).max(0.0);
        let active_idx = lrc.get_current_line(effective_time).map(|(i, _)| i);
        let skin = VortexTheme::current_skin();
        let cur_time = ui.ctx().input(|i| i.time);
        let is_copied = cur_time - self.copied_feedback < 2.0;

        // ── MODE A: FULL / EXPANDED SCROLLING LYRICS STUDIO ────────────────────────
        if self.is_expanded {
            let panel_w = (video_rect.width() - 48.0).clamp(360.0, 680.0);
            let panel_h = (video_rect.height() - 140.0).clamp(240.0, 620.0);
            let panel_rect = Rect::from_center_size(video_rect.center(), Vec2::new(panel_w, panel_h));

            let painter = ui.painter();
            // Drop shadow
            painter.rect_filled(
                panel_rect.expand(6.0).translate(Vec2::new(0.0, 6.0)),
                CornerRadius::same(16),
                Color32::from_rgba_unmultiplied(0, 0, 0, 160),
            );
            // Frosted Obsidian Background
            painter.rect_filled(
                panel_rect,
                CornerRadius::same(14),
                Color32::from_rgba_unmultiplied(12, 14, 20, 245),
            );
            painter.rect_stroke(
                panel_rect,
                CornerRadius::same(14),
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 32)),
                StrokeKind::Inside,
            );

            let mut close_expanded = false;
            let mut copy_clicked = false;
            let mut offset_delta = 0.0f64;

            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(panel_rect.shrink(12.0)));
            let ui = &mut child_ui;

            // Header
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("🎙️ Synchronized Lyrics")
                        .strong()
                        .size(14.0)
                        .color(skin.accent_primary),
                );

                if let Some(ref title) = lrc.title {
                    ui.label(
                        egui::RichText::new(format!("• {}", title))
                            .size(12.0)
                            .color(Color32::from_rgb(200, 205, 220)),
                    );
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add(egui::Button::new("❌").small())
                        .on_hover_text("Close Lyrics Studio")
                        .clicked()
                    {
                        close_expanded = true;
                    }

                    if ui
                        .add(egui::Button::new(if is_copied { "✓ Copied" } else { "📋 Copy" }).small())
                        .on_hover_text("Copy Full Lyrics to Clipboard")
                        .clicked()
                    {
                        copy_clicked = true;
                    }

                    ui.add_space(8.0);
                    if ui.add(egui::Button::new("+0.5s").small()).on_hover_text("Delay lyrics by 0.5s").clicked() {
                        offset_delta += 0.5;
                    }
                    if ui.add(egui::Button::new("-0.5s").small()).on_hover_text("Advance lyrics by 0.5s").clicked() {
                        offset_delta -= 0.5;
                    }
                        let offset_text = if self.user_offset_sec.abs() > 0.01 {
                            format!("Sync: {:+.1}s", self.user_offset_sec)
                        } else {
                            "Sync: 0.0s".to_string()
                        };
                        ui.label(
                            egui::RichText::new(offset_text)
                                .size(11.0)
                                .color(Color32::from_rgb(140, 145, 160)),
                        );
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                // Scrolling Lines
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (idx, line) in lrc.lines.iter().enumerate() {
                            let is_current = active_idx == Some(idx);
                            let min = (line.timestamp_sec / 60.0).floor() as u32;
                            let sec = (line.timestamp_sec % 60.0).floor() as u32;
                            let time_badge = format!("{:02}:{:02}", min, sec);

                            ui.horizontal(|ui| {
                                let badge_col = if is_current {
                                    skin.accent_primary
                                } else {
                                    Color32::from_rgb(100, 105, 120)
                                };
                                ui.label(
                                    egui::RichText::new(time_badge)
                                        .size(11.0)
                                        .monospace()
                                        .color(badge_col),
                                );

                                let text_style = if is_current {
                                    egui::RichText::new(&line.text)
                                        .size(16.0)
                                        .strong()
                                        .color(skin.accent_primary)
                                } else {
                                    egui::RichText::new(&line.text)
                                        .size(13.5)
                                        .color(Color32::from_rgb(180, 185, 200))
                                };

                                let resp = ui.selectable_label(is_current, text_style);
                                if resp.clicked() {
                                    if let Some(player) = player_opt {
                                        player.seek_absolute(line.timestamp_sec);
                                    }
                                }
                                if is_current && self.last_active_idx != Some(idx) {
                                    resp.scroll_to_me(Some(egui::Align::Center));
                                }
                            });
                            ui.add_space(2.0);
                        }
                    });

            if close_expanded {
                self.is_expanded = false;
            }
            if copy_clicked {
                let text = lrc.lines.iter().map(|l| l.text.as_str()).collect::<Vec<_>>().join("\n");
                ui.ctx().copy_text(text);
                self.copied_feedback = cur_time;
            }
            if offset_delta.abs() > 0.001 {
                self.user_offset_sec += offset_delta;
            }
            self.last_active_idx = active_idx;
            return;
        }

        // ── MODE B: COMPACT FLOATING KARAOKE CARD ────────────────────────────────
        let card_w = (video_rect.width() - 32.0).clamp(320.0, 720.0);
        let card_h = 74.0;
        let card_y = if is_song_mode {
            // In song mode, float cleanly above the bottom music control bar
            (video_rect.bottom() - 130.0).clamp(video_rect.top() + 80.0, video_rect.bottom() - 65.0)
        } else {
            // In video mode, float cleanly in bottom subtitle area
            (video_rect.bottom() - 85.0).clamp(video_rect.top() + 60.0, video_rect.bottom() - 50.0)
        };
        let card_rect = Rect::from_center_size(Pos2::new(video_rect.center().x, card_y), Vec2::new(card_w, card_h));

        let card_resp = ui.interact(card_rect, ui.id().with("lyrics_floating_card"), Sense::hover());
        let is_hovered = card_resp.hovered();

        let painter = ui.painter();

        // 1. Multi-pass ambient soft drop shadow
        let shadow_passes = [(6.0, 4.0, 100), (3.0, 2.0, 140)];
        for (expand, y_off, alpha) in shadow_passes {
            painter.rect_filled(
                card_rect.expand(expand).translate(Vec2::new(0.0, y_off)),
                CornerRadius::same(14),
                Color32::from_rgba_unmultiplied(0, 0, 0, alpha),
            );
        }

        // 2. Frosted Obsidian Glass Background
        painter.rect_filled(
            card_rect,
            CornerRadius::same(12),
            Color32::from_rgba_unmultiplied(12, 14, 20, 225),
        );
        painter.rect_stroke(
            card_rect,
            CornerRadius::same(12),
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 28)),
            StrokeKind::Inside,
        );

        let center_x = card_rect.center().x;

        // 3. Render Lyric Lines
        if let Some(idx) = active_idx {
            if let Some(active_line) = lrc.lines.get(idx) {
                // Active Current Line (Highlighted Bold Accent Color with Subtle Drop Shadow)
                let line_y = card_rect.top() + 24.0;
                // Text shadow for maximum legibility
                painter.text(
                    Pos2::new(center_x + 1.0, line_y + 1.0),
                    Align2::CENTER_CENTER,
                    &active_line.text,
                    FontId::proportional(18.5),
                    Color32::from_rgba_unmultiplied(0, 0, 0, 180),
                );
                painter.text(
                    Pos2::new(center_x, line_y),
                    Align2::CENTER_CENTER,
                    &active_line.text,
                    FontId::proportional(18.5),
                    skin.accent_primary,
                );
            }

            // Next Upcoming Line (Muted Grey / Silver)
            if let Some(next_line) = lrc.lines.get(idx + 1) {
                painter.text(
                    Pos2::new(center_x, card_rect.top() + 52.0),
                    Align2::CENTER_CENTER,
                    &next_line.text,
                    FontId::proportional(13.0),
                    Color32::from_rgb(165, 172, 190),
                );
            }
        } else {
            // Before first line (Intro / Instrumental)
            painter.text(
                Pos2::new(center_x, card_rect.top() + 24.0),
                Align2::CENTER_CENTER,
                "🎵  Instrumental",
                FontId::proportional(15.0),
                Color32::from_rgb(140, 145, 160),
            );
            if let Some(first_line) = lrc.lines.first() {
                painter.text(
                    Pos2::new(center_x, card_rect.top() + 52.0),
                    Align2::CENTER_CENTER,
                    &first_line.text,
                    FontId::proportional(13.0),
                    Color32::from_rgb(165, 172, 190),
                );
            }
        }

        // 4. Hover Action Bar (Top Right of Card)
        if is_hovered {
            let mut action_x = card_rect.right() - 8.0;
            let action_y = card_rect.top() + 8.0;

            // Close button (❌)
            let close_rect = Rect::from_min_size(Pos2::new(action_x - 18.0, action_y), Vec2::new(18.0, 18.0));
            action_x -= 22.0;
            let close_resp = ui.interact(close_rect, ui.id().with("lyr_close"), Sense::click())
                .on_hover_text("Hide Lyrics Overlay (Ctrl+L)");
            painter.text(
                close_rect.center(),
                Align2::CENTER_CENTER,
                "❌",
                FontId::proportional(9.0),
                if close_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(130, 135, 150) },
            );
            if close_resp.clicked() {
                self.is_enabled = false;
            }

            // Expand button (↕)
            let exp_rect = Rect::from_min_size(Pos2::new(action_x - 18.0, action_y), Vec2::new(18.0, 18.0));
            action_x -= 22.0;
            let exp_resp = ui.interact(exp_rect, ui.id().with("lyr_expand"), Sense::click())
                .on_hover_text("Expand Full Scrolling Lyrics Studio");
            painter.text(
                exp_rect.center(),
                Align2::CENTER_CENTER,
                "↕",
                FontId::proportional(11.0),
                if exp_resp.hovered() { skin.accent_primary } else { Color32::from_rgb(130, 135, 150) },
            );
            if exp_resp.clicked() {
                self.is_expanded = true;
            }

            // Copy button (📋)
            let copy_rect = Rect::from_min_size(Pos2::new(action_x - 18.0, action_y), Vec2::new(18.0, 18.0));
            let copy_resp = ui.interact(copy_rect, ui.id().with("lyr_copy"), Sense::click())
                .on_hover_text(if is_copied { "Lyrics Copied!" } else { "Copy Full Lyrics" });
            painter.text(
                copy_rect.center(),
                Align2::CENTER_CENTER,
                if is_copied { "✓" } else { "📋" },
                FontId::proportional(9.5),
                if copy_resp.hovered() || is_copied { skin.accent_primary } else { Color32::from_rgb(130, 135, 150) },
            );
            if copy_resp.clicked() {
                let text = lrc.lines.iter().map(|l| l.text.as_str()).collect::<Vec<_>>().join("\n");
                ui.ctx().copy_text(text);
                self.copied_feedback = cur_time;
            }
        }
        self.last_active_idx = active_idx;
    }
}
