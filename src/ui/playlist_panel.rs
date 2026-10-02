#![allow(dead_code)]

//! playlist_panel.rs — PotPlayer Multi-Tab Right Sidebar Drawer:
//! 1. Playlist Studio (Default, Albums, Favorites, History, Search, Bottom Action Bar)
//! 2. File Explorer (Drives, Quick Folders, Directory Tree, Double-Click Play, Enqueue)
//! 3. Chapters & Bookmarks (Embedded Chapters, PBF Bookmarks, Instant Jump, Add/Remove)
//! 4. Subtitle Transcript (Dialogue Search, Timestamps, Jump-to-Speech)

use super::theme::VortexTheme;
use crate::bookmark::{format_time, BookmarkManager};
use crate::engine::MediaStats;
use crate::playlist::scanner::is_media_file;
use crate::playlist::{Playlist, PlaylistTab};
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerTab {
    #[default]
    Playlist,
    FileBrowser,
    ChaptersBookmarks,
    Subtitles,
}

pub struct PlaylistPanel;

pub struct PlaylistActions {
    pub play_path: Option<PathBuf>,
    pub enqueue_path: Option<PathBuf>,
    pub seek_to: Option<f64>,
    pub add_files: bool,
    pub add_folder: bool,
    pub add_url: bool,
    pub clear_playlist: bool,
    pub cycle_repeat: bool,
    pub toggle_shuffle: bool,
    pub toggle_detach: bool,
    pub toggle_pin: bool,
    pub close_playlist: bool,
    pub add_bookmark: bool,
    pub delete_bookmark_idx: Option<usize>,
    pub toggle_restore_prev: bool,
}

fn safe_truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() > max_chars {
        let truncated: String = s.chars().take(max_chars.saturating_sub(3)).collect();
        format!("{}...", truncated)
    } else {
        s.to_string()
    }
}

fn format_duration_potplayer(secs: f64) -> String {
    if secs <= 0.0 {
        return "--:--".to_string();
    }
    let total_s = secs.round() as u64;
    let h = total_s / 3600;
    let m = (total_s % 3600) / 60;
    let s = total_s % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("00:{:02}:{:02}", m, s)
    }
}

fn get_format_badge_style(ext: &str) -> (Color32, Color32, Color32, &'static str) {
    match ext.to_lowercase().as_str() {
        "flac" => (Color32::from_rgb(18, 48, 28), Color32::from_rgb(38, 105, 58), Color32::from_rgb(90, 220, 130), "FLAC"),
        "mp3" => (Color32::from_rgb(52, 34, 14), Color32::from_rgb(110, 72, 28), Color32::from_rgb(255, 180, 80), "MP3"),
        "wav" => (Color32::from_rgb(18, 38, 56), Color32::from_rgb(35, 78, 115), Color32::from_rgb(100, 180, 255), "WAV"),
        "m4a" | "aac" => (Color32::from_rgb(16, 44, 44), Color32::from_rgb(32, 92, 90), Color32::from_rgb(70, 225, 215), "AAC"),
        "ogg" | "opus" => (Color32::from_rgb(40, 20, 48), Color32::from_rgb(85, 40, 100), Color32::from_rgb(210, 120, 245), "OGG"),
        "mp4" | "mkv" | "avi" | "mov" | "webm" => (Color32::from_rgb(20, 28, 48), Color32::from_rgb(45, 65, 110), Color32::from_rgb(130, 170, 255), "VID"),
        _ => (Color32::from_rgb(26, 28, 36), Color32::from_rgb(48, 52, 64), Color32::from_rgb(160, 165, 180), "AUD"),
    }
}

impl PlaylistPanel {
    pub const WIDTH: f32 = 330.0;

    pub fn render(
        ui: &mut egui::Ui,
        playlist: &mut Playlist,
        active_video_path: Option<&str>,
        is_detached: bool,
        is_pinned: bool,
        active_drawer_tab: &mut DrawerTab,
        stats: &MediaStats,
        bookmark_mgr: &BookmarkManager,
        sub_lines: &[(f64, f64, String)],
        browser_dir: &mut PathBuf,
        browser_search: &mut String,
        sub_search: &mut String,
        restore_prev: bool,
    ) -> PlaylistActions {
        let mut actions = PlaylistActions {
            play_path: None,
            enqueue_path: None,
            seek_to: None,
            add_files: false,
            add_folder: false,
            add_url: false,
            clear_playlist: false,
            cycle_repeat: false,
            toggle_shuffle: false,
            toggle_detach: false,
            toggle_pin: false,
            close_playlist: false,
            add_bookmark: false,
            delete_bookmark_idx: None,
            toggle_restore_prev: false,
        };

        if browser_dir.as_os_str().is_empty() {
            *browser_dir = dirs::video_dir()
                .or_else(dirs::download_dir)
                .or_else(dirs::home_dir)
                .unwrap_or_else(|| PathBuf::from("C:\\"));
        }

        ui.vertical(|ui| {
            ui.set_min_height(ui.available_height());
            ui.spacing_mut().item_spacing = Vec2::new(0.0, 4.0);

            if is_detached {
                // ── PotPlayer Standalone Playlist Window Header Tabs ──────────────
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(3.0, 0.0);

                    // Default Tab (Pill Style)
                    let is_default = playlist.active_tab == PlaylistTab::DefaultPlaylist;
                    let tab_txt = format!("Default ({})", playlist.items.len());
                    let (r, resp) = ui.allocate_exact_size(Vec2::new(95.0, 22.0), Sense::click());
                    let bg = if is_default { Color32::from_rgb(26, 28, 35) } else { Color32::from_rgb(8, 9, 12) };
                    ui.painter().rect_filled(r, CornerRadius::same(3), bg);
                    ui.painter().rect_stroke(r, CornerRadius::same(3), Stroke::new(1.0, if is_default { Color32::from_rgb(55, 60, 75) } else { Color32::from_rgb(28, 30, 38) }), StrokeKind::Inside);
                    ui.painter().text(r.center(), Align2::CENTER_CENTER, tab_txt, FontId::proportional(11.0), if is_default { Color32::WHITE } else { Color32::from_rgb(160, 165, 180) });
                    if resp.clicked() {
                        playlist.active_tab = PlaylistTab::DefaultPlaylist;
                    }

                    // "+" Add Tab / Add Files Button
                    let (plus_r, plus_resp) = ui.allocate_exact_size(Vec2::new(24.0, 22.0), Sense::click());
                    let plus_bg = if plus_resp.hovered() { Color32::from_rgb(30, 32, 40) } else { Color32::from_rgb(8, 9, 12) };
                    ui.painter().rect_filled(plus_r, CornerRadius::same(3), plus_bg);
                    ui.painter().rect_stroke(plus_r, CornerRadius::same(3), Stroke::new(1.0, Color32::from_rgb(28, 30, 38)), StrokeKind::Inside);
                    ui.painter().text(plus_r.center(), Align2::CENTER_CENTER, "+", FontId::proportional(13.0), Color32::from_rgb(180, 185, 200));
                    if plus_resp.clicked() {
                        actions.add_files = true;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Close button (✕)
                        let close_btn = egui::Button::new(egui::RichText::new("✕").size(11.0).color(Color32::from_rgb(170, 175, 190)))
                            .frame(false);
                        if ui.add(close_btn).on_hover_text("Close Playlist").clicked() {
                            actions.close_playlist = true;
                        }

                        // Dock button (📎)
                        let detach_btn = egui::Button::new(egui::RichText::new("📎 Dock").size(10.0));
                        if ui.add(detach_btn).on_hover_text("Dock back into player sidebar").clicked() {
                            actions.toggle_detach = true;
                        }

                        // Pin on Top Button (📌)
                        let pin_btn = egui::Button::new(
                            egui::RichText::new(if is_pinned { "📌" } else { "📍" })
                                .size(11.0)
                                .color(if is_pinned { VortexTheme::POT_YELLOW } else { Color32::from_rgb(140, 145, 160) })
                        );
                        if ui.add(pin_btn).on_hover_text(if is_pinned { "Pinned Always on Top" } else { "Pin window always on top" }).clicked() {
                            actions.toggle_pin = true;
                        }
                    });
                });
            } else {
                // ── Docked Sidebar Top Header Bar: Title + Detach / Close Controls ──────────────
                ui.horizontal(|ui| {
                    ui.add_space(2.0);
                    let title_label = match *active_drawer_tab {
                        DrawerTab::Playlist => "PLAYLIST STUDIO",
                        DrawerTab::FileBrowser => "FILE EXPLORER",
                        DrawerTab::ChaptersBookmarks => "CHAPTERS & MARKS",
                        DrawerTab::Subtitles => "SUBTITLE TRANSCRIPT",
                    };
                    ui.label(
                        egui::RichText::new(title_label)
                            .color(VortexTheme::POT_YELLOW)
                            .font(FontId::proportional(11.5))
                            .strong(),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let close_btn = egui::Button::new(egui::RichText::new("✕").size(11.0).color(Color32::from_rgb(170, 175, 190)))
                            .frame(false);
                        if ui.add(close_btn).on_hover_text("Close Drawer (F8)").clicked() {
                            actions.close_playlist = true;
                        }

                        let detach_btn = egui::Button::new(
                            egui::RichText::new("⤢ Pop Out")
                                .size(10.0)
                                .color(Color32::from_rgb(180, 185, 200))
                        );
                        if ui.add(detach_btn)
                            .on_hover_text("Detach into floating window")
                            .clicked()
                        {
                            actions.toggle_detach = true;
                        }
                    });
                });

                // Primary 4-Tab Drawer Switcher
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);
                    let main_tabs = [
                        (DrawerTab::Playlist, "📋 List"),
                        (DrawerTab::FileBrowser, "📁 Files"),
                        (DrawerTab::ChaptersBookmarks, "📑 Mark"),
                        (DrawerTab::Subtitles, "💬 Subs"),
                    ];

                    let tab_w = ((ui.available_width() - 6.0) / 4.0).max(48.0);
                    for (tab, label) in main_tabs {
                        let is_active = *active_drawer_tab == tab;
                        let (r, resp) = ui.allocate_exact_size(Vec2::new(tab_w, 24.0), Sense::click());
                        let painter = ui.painter();

                        let bg = if is_active {
                            Color32::from_rgb(26, 28, 36)
                        } else if resp.hovered() {
                            Color32::from_rgb(18, 20, 26)
                        } else {
                            Color32::from_rgb(0, 0, 0)
                        };

                        painter.rect_filled(r, CornerRadius::same(3), bg);
                        painter.rect_stroke(
                            r,
                            CornerRadius::same(3),
                            Stroke::new(1.0, if is_active { VortexTheme::POT_YELLOW } else { Color32::from_rgb(30, 32, 40) }),
                            StrokeKind::Inside,
                        );

                        painter.text(
                            r.center(),
                            Align2::CENTER_CENTER,
                            label,
                            FontId::proportional(11.0),
                            if is_active { Color32::WHITE } else { Color32::from_rgb(155, 160, 175) },
                        );

                        if resp.clicked() {
                            *active_drawer_tab = tab;
                        }
                    }
                });
            }

            ui.add_space(2.0);

            match *active_drawer_tab {
                // =========================================================================
                // TAB 1: PLAYLIST STUDIO
                // =========================================================================
                DrawerTab::Playlist => {
                    // Segmented Sub-Tabs
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(3.0, 0.0);
                        let sub_tabs = [
                            (PlaylistTab::DefaultPlaylist, format!("Default ({})", playlist.items.len())),
                            (PlaylistTab::Favorites, format!("★ Fav ({})", playlist.favorites.len())),
                            (PlaylistTab::History, format!("🕒 Hist ({})", playlist.history.len())),
                        ];

                        let tab_w = ((ui.available_width() - 6.0) / 3.0).max(50.0);
                        for (tab, label) in sub_tabs {
                            let is_active = playlist.active_tab == tab;
                            let (r, resp) = ui.allocate_exact_size(Vec2::new(tab_w, 20.0), Sense::click());
                            let painter = ui.painter();

                            let bg = if is_active {
                                Color32::from_rgb(22, 24, 30)
                            } else if resp.hovered() {
                                Color32::from_rgb(16, 17, 22)
                            } else {
                                Color32::from_rgb(0, 0, 0)
                            };

                            painter.rect_filled(r, CornerRadius::same(2), bg);
                            painter.text(
                                r.center(),
                                Align2::CENTER_CENTER,
                                label,
                                FontId::proportional(10.0),
                                if is_active { VortexTheme::POT_YELLOW } else { Color32::from_rgb(140, 145, 160) },
                            );

                            if resp.clicked() {
                                playlist.active_tab = tab;
                            }
                        }
                    });

                    // Search & Filter Bar
                    ui.horizontal(|ui| {
                        ui.add_space(2.0);
                        ui.add_sized(
                            Vec2::new(ui.available_width() - 28.0, 22.0),
                            egui::TextEdit::singleline(&mut playlist.search_query)
                                .hint_text("🔍 Filter playlist...")
                                .font(FontId::proportional(11.0)),
                        );
                        if !playlist.search_query.is_empty() {
                            let clear_btn = egui::Button::new(egui::RichText::new("✕").size(10.0).color(Color32::from_rgb(170, 175, 190)));
                            if ui.add(clear_btn).on_hover_text("Clear filter").clicked() {
                                playlist.search_query.clear();
                            }
                        }
                    });

                    ui.separator();

                    // Virtualized Track List
                    let available_h = (ui.available_height() - 42.0).max(100.0);
                    let mut to_remove_idx: Option<usize> = None;
                    let mut to_move_up: Option<usize> = None;
                    let mut to_move_down: Option<usize> = None;
                    let mut toggle_fav_path: Option<PathBuf> = None;
                    let item_h = 22.0;

                    let query = playlist.search_query.trim().to_lowercase();
                    let filtered_indices: Vec<usize> = match playlist.active_tab {
                        PlaylistTab::Favorites => {
                            playlist.favorites.iter().enumerate()
                                .filter(|(_, p)| query.is_empty() || p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase().contains(&query))
                                .map(|(i, _)| i)
                                .collect()
                        }
                        PlaylistTab::History => {
                            playlist.history.iter().enumerate()
                                .filter(|(_, p)| query.is_empty() || p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase().contains(&query))
                                .map(|(i, _)| i)
                                .collect()
                        }
                        _ => {
                            playlist.items.iter().enumerate()
                                .filter(|(_, it)| query.is_empty() || it.title.to_lowercase().contains(&query))
                                .map(|(i, _)| i)
                                .collect()
                        }
                    };

                    let total_rows = filtered_indices.len();

                    egui::ScrollArea::vertical()
                        .max_height(available_h)
                        .min_scrolled_height(available_h)
                        .auto_shrink([false, false])
                        .show_rows(ui, item_h, total_rows, |ui, row_range| {
                            if total_rows == 0 {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(30.0);
                                    ui.label(egui::RichText::new("Playlist is empty").color(Color32::from_rgb(130, 135, 150)));
                                    ui.label(egui::RichText::new("Drag & drop files or click + ADD").size(10.5).color(Color32::from_rgb(90, 95, 110)));
                                });
                                return;
                            }

                            for row_idx in row_range {
                                let orig_idx = filtered_indices[row_idx];
                                let (item_path, item_title, item_duration_str, is_current) = match playlist.active_tab {
                                    PlaylistTab::Favorites => {
                                        let p = &playlist.favorites[orig_idx];
                                        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown").to_string();
                                        let is_cur = active_video_path.map(|av| av == p.to_string_lossy()).unwrap_or(false);
                                        let dur_str = if is_cur && stats.duration > 0.0 {
                                            format_duration_potplayer(stats.duration)
                                        } else {
                                            "--:--".to_string()
                                        };
                                        (p.clone(), name, dur_str, is_cur)
                                    }
                                    PlaylistTab::History => {
                                        let p = &playlist.history[orig_idx];
                                        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown").to_string();
                                        let is_cur = active_video_path.map(|av| av == p.to_string_lossy()).unwrap_or(false);
                                        let dur_str = if is_cur && stats.duration > 0.0 {
                                            format_duration_potplayer(stats.duration)
                                        } else {
                                            "--:--".to_string()
                                        };
                                        (p.clone(), name, dur_str, is_cur)
                                    }
                                    _ => {
                                        let it = &playlist.items[orig_idx];
                                        let is_cur = playlist.current_index == Some(orig_idx);
                                        let dur_val = if is_cur && stats.duration > 0.0 {
                                            Some(stats.duration)
                                        } else {
                                            it.duration_secs.filter(|s| *s > 0.0)
                                        };
                                        let dur_str = dur_val.map(format_duration_potplayer).unwrap_or_else(|| "--:--".to_string());
                                        (it.path.clone(), it.title.clone(), dur_str, is_cur)
                                    }
                                };

                                let (rect, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), item_h), Sense::click());
                                let painter = ui.painter();

                                let bg = if is_current {
                                    Color32::from_rgb(24, 27, 35)
                                } else if resp.hovered() {
                                    Color32::from_rgb(18, 20, 26)
                                } else if row_idx % 2 == 1 {
                                    Color32::from_rgb(6, 7, 10)
                                } else {
                                    Color32::from_rgb(0, 0, 0)
                                };

                                painter.rect_filled(rect, CornerRadius::same(2), bg);

                                if is_current {
                                    painter.rect_stroke(rect, CornerRadius::same(2), Stroke::new(1.0, Color32::from_rgb(50, 58, 72)), StrokeKind::Inside);
                                }

                                // Format badge icon on the left (e.g. green FLAC badge)
                                let ext = item_path.extension().and_then(|e| e.to_str()).unwrap_or("");
                                let (badge_bg, badge_stroke, badge_text_col, badge_label) = get_format_badge_style(ext);
                                let badge_r = Rect::from_min_size(Pos2::new(rect.left() + 4.0, rect.center().y - 6.0), Vec2::new(22.0, 12.0));
                                painter.rect_filled(badge_r, CornerRadius::same(2), badge_bg);
                                painter.rect_stroke(badge_r, CornerRadius::same(2), Stroke::new(1.0, badge_stroke), StrokeKind::Inside);
                                painter.text(badge_r.center(), Align2::CENTER_CENTER, badge_label, FontId::monospace(7.5), badge_text_col);

                                // Track label with track number prefix matching PotPlayer: "01. 01 - Title.flac"
                                let display_title = format!("{:02}. {}", orig_idx + 1, item_title);
                                let text_col = if is_current {
                                    Color32::WHITE
                                } else if resp.hovered() {
                                    Color32::from_rgb(240, 242, 248)
                                } else {
                                    Color32::from_rgb(175, 180, 192)
                                };

                                let title_w = rect.width() - 95.0;
                                let truncated = safe_truncate(&display_title, (title_w / 6.5).max(12.0) as usize);
                                painter.text(
                                    Pos2::new(rect.left() + 30.0, rect.center().y),
                                    Align2::LEFT_CENTER,
                                    truncated,
                                    FontId::proportional(11.0),
                                    text_col,
                                );

                                painter.text(
                                    Pos2::new(rect.right() - 8.0, rect.center().y),
                                    Align2::RIGHT_CENTER,
                                    item_duration_str,
                                    FontId::monospace(10.0),
                                    if is_current { Color32::WHITE } else { Color32::from_rgb(120, 125, 140) },
                                );

                                if resp.double_clicked() || resp.clicked() {
                                    if playlist.active_tab == PlaylistTab::DefaultPlaylist {
                                        playlist.current_index = Some(orig_idx);
                                    }
                                    actions.play_path = Some(item_path.clone());
                                }

                                resp.context_menu(|ui| {
                                    if ui.button("▶  Play Now").clicked() {
                                        actions.play_path = Some(item_path.clone());
                                        ui.close();
                                    }
                                    let is_fav = playlist.favorites.contains(&item_path);
                                    let fav_text = if is_fav { "★  Remove from Favorites" } else { "☆  Add to Favorites" };
                                    if ui.button(fav_text).clicked() {
                                        toggle_fav_path = Some(item_path.clone());
                                        ui.close();
                                    }
                                    ui.separator();
                                    if ui.button("▲  Move Up").clicked() {
                                        to_move_up = Some(orig_idx);
                                        ui.close();
                                    }
                                    if ui.button("▼  Move Down").clicked() {
                                        to_move_down = Some(orig_idx);
                                        ui.close();
                                    }
                                    if ui.button("🗑  Remove from List").clicked() {
                                        to_remove_idx = Some(orig_idx);
                                        ui.close();
                                    }
                                });
                            }
                        });

                    if let Some(idx) = to_remove_idx {
                        if idx < playlist.items.len() {
                            playlist.items.remove(idx);
                        }
                    }
                    if let Some(idx) = to_move_up {
                        if idx > 0 && idx < playlist.items.len() {
                            playlist.items.swap(idx, idx - 1);
                        }
                    }
                    if let Some(idx) = to_move_down {
                        if idx + 1 < playlist.items.len() {
                            playlist.items.swap(idx, idx + 1);
                        }
                    }
                    if let Some(path) = toggle_fav_path {
                        if let Some(pos) = playlist.favorites.iter().position(|p| p == &path) {
                            playlist.favorites.remove(pos);
                        } else {
                            playlist.favorites.push(path);
                        }
                    }

                    // Bottom Toolbar Group
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(3.0, 0.0);

                        if ui.button(egui::RichText::new("+ ADD").size(10.0)).on_hover_text("Add files to playlist").clicked() {
                            actions.add_files = true;
                        }
                        if ui.button(egui::RichText::new("📁 DIR").size(10.0)).on_hover_text("Add whole folder").clicked() {
                            actions.add_folder = true;
                        }
                        if ui.button(egui::RichText::new("🌐 URL").size(10.0)).on_hover_text("Open network stream URL").clicked() {
                            actions.add_url = true;
                        }
                        if ui.button(egui::RichText::new("- DEL").size(10.0)).on_hover_text("Clear all tracks").clicked() {
                            actions.clear_playlist = true;
                        }
                        if ui.button(egui::RichText::new("⇅ SORT").size(10.0)).on_hover_text("Sort tracks naturally").clicked() {
                            playlist.sort_natural();
                        }

                        let rep_label = match playlist.repeat_mode {
                            crate::playlist::RepeatMode::RepeatAll => "🔁 All",
                            crate::playlist::RepeatMode::RepeatTrack => "🔂 One",
                            crate::playlist::RepeatMode::Off => "🔁 Off",
                        };
                        let is_rep_active = playlist.repeat_mode != crate::playlist::RepeatMode::Off;
                        let rep_btn = egui::Button::new(
                            egui::RichText::new(rep_label)
                                .size(10.0)
                                .color(if is_rep_active { VortexTheme::POT_YELLOW } else { Color32::from_rgb(160, 165, 175) })
                        );
                        if ui.add(rep_btn).on_hover_text("Cycle Repeat Mode").clicked() {
                            playlist.cycle_repeat_mode();
                            actions.cycle_repeat = true;
                        }

                        let is_shuf_active = playlist.is_shuffle();
                        let shuf_btn = egui::Button::new(
                            egui::RichText::new("🔀")
                                .size(10.0)
                                .color(if is_shuf_active { VortexTheme::POT_YELLOW } else { Color32::from_rgb(160, 165, 175) })
                        );
                        if ui.add(shuf_btn).on_hover_text("Toggle Shuffle Playback").clicked() {
                            playlist.toggle_shuffle();
                            actions.toggle_shuffle = true;
                        }

                        let restore_btn = egui::Button::new(
                            egui::RichText::new(if restore_prev { "💾 Prev: ON" } else { "💾 Prev: OFF" })
                                .size(10.0)
                                .color(if restore_prev { VortexTheme::POT_YELLOW } else { Color32::from_rgb(140, 145, 155) })
                        );
                        if ui.add(restore_btn).on_hover_text("Toggle restoring previous playlist on direct launch (switched OFF by default)").clicked() {
                            actions.toggle_restore_prev = true;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(egui::RichText::new("🧹").size(10.0)).on_hover_text("Remove missing files").clicked() {
                                playlist.remove_missing_files();
                            }
                        });
                    });
                }

                // =========================================================================
                // TAB 2: BUILT-IN FILE EXPLORER
                // =========================================================================
                DrawerTab::FileBrowser => {
                    // Quick Drive Selector Bar
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(3.0, 0.0);
                        let quick_locations = [
                            ("C:", PathBuf::from("C:\\")),
                            ("D:", PathBuf::from("D:\\")),
                            ("Videos", dirs::video_dir().unwrap_or_else(|| PathBuf::from("C:\\"))),
                            ("Downloads", dirs::download_dir().unwrap_or_else(|| PathBuf::from("C:\\"))),
                        ];

                        for (name, path) in quick_locations {
                            if path.exists() {
                                if ui.button(egui::RichText::new(name).size(10.0)).clicked() {
                                    *browser_dir = path;
                                }
                            }
                        }
                    });

                    // Current Path & Up Navigation
                    ui.horizontal(|ui| {
                        if ui.button("⬆ Up").on_hover_text("Go to parent directory").clicked() {
                            if let Some(parent) = browser_dir.parent() {
                                *browser_dir = parent.to_path_buf();
                            }
                        }

                        let folder_name = browser_dir.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or_else(|| browser_dir.to_str().unwrap_or("Root"));
                        ui.label(egui::RichText::new(folder_name).strong().color(VortexTheme::POT_YELLOW));
                    });

                    // Search box in current folder
                    ui.horizontal(|ui| {
                        ui.add_space(2.0);
                        ui.add_sized(
                            Vec2::new(ui.available_width() - 28.0, 22.0),
                            egui::TextEdit::singleline(browser_search)
                                .hint_text("🔍 Filter files in folder...")
                                .font(FontId::proportional(11.0)),
                        );
                        if !browser_search.is_empty() {
                            if ui.button("✕").clicked() {
                                browser_search.clear();
                            }
                        }
                    });

                    ui.separator();

                    // Read Directory Entries (Cached to avoid per-frame disk I/O)
                    let cache_id = ui.id().with("cached_browser_dir_items");
                    let (cached_dir, mut dir_items): (PathBuf, Vec<(PathBuf, bool, String)>) = ui.ctx().data(|d| {
                        d.get_temp(cache_id).unwrap_or_else(|| (PathBuf::new(), Vec::new()))
                    });

                    if cached_dir != *browser_dir {
                        dir_items.clear();
                        if let Ok(entries) = std::fs::read_dir(&*browser_dir) {
                            for entry in entries.flatten() {
                                let p = entry.path();
                                let is_dir = p.is_dir();
                                if is_dir || is_media_file(&p) {
                                    let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                                    dir_items.push((p, is_dir, name));
                                }
                            }
                        }

                        // Sort folders first, then natural sort for files
                        dir_items.sort_by(|a, b| {
                            if a.1 && !b.1 {
                                std::cmp::Ordering::Less
                            } else if !a.1 && b.1 {
                                std::cmp::Ordering::Greater
                            } else {
                                natord::compare(&a.2, &b.2)
                            }
                        });

                        ui.ctx().data_mut(|d| d.insert_temp(cache_id, (browser_dir.clone(), dir_items.clone())));
                    }

                    let filtered_items: Vec<&(PathBuf, bool, String)> = dir_items.iter().filter(|(_, _, name)| {
                        browser_search.is_empty() || name.to_lowercase().contains(&browser_search.to_lowercase())
                    }).collect();

                    let scroll_h = (ui.available_height() - 35.0).max(100.0);
                    let mut next_nav_dir: Option<PathBuf> = None;

                    egui::ScrollArea::vertical()
                        .max_height(scroll_h)
                        .min_scrolled_height(scroll_h)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if filtered_items.is_empty() {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(30.0);
                                    ui.label(egui::RichText::new("No media files or folders found").color(Color32::from_rgb(130, 135, 150)));
                                });
                                return;
                            }

                            for (item_path, is_dir, name) in filtered_items {
                                ui.horizontal(|ui| {
                                    let icon = if *is_dir { "📁" } else { "🎬" };
                                    let text = format!("{} {}", icon, name);
                                    let resp = ui.selectable_label(false, text);

                                    if *is_dir {
                                        if resp.double_clicked() || resp.clicked() {
                                            next_nav_dir = Some(item_path.clone());
                                        }
                                    } else {
                                        if resp.double_clicked() || resp.clicked() {
                                            actions.play_path = Some(item_path.clone());
                                        }
                                        if ui.small_button("+ List").on_hover_text("Enqueue to playlist").clicked() {
                                            actions.enqueue_path = Some(item_path.clone());
                                        }
                                    }
                                });
                            }
                        });

                    if let Some(new_dir) = next_nav_dir {
                        *browser_dir = new_dir;
                    }

                    // Bottom Directory Actions
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("▶ Play All in Folder").clicked() {
                            actions.play_path = Some(browser_dir.clone());
                        }
                        if ui.button("+ Add Folder to List").clicked() {
                            actions.enqueue_path = Some(browser_dir.clone());
                        }
                    });
                }

                // =========================================================================
                // TAB 3: CHAPTERS & BOOKMARKS
                // =========================================================================
                DrawerTab::ChaptersBookmarks => {
                    ui.horizontal(|ui| {
                        if ui.button(egui::RichText::new("➕ Add Bookmark (P)").color(VortexTheme::POT_YELLOW).strong()).clicked() {
                            actions.add_bookmark = true;
                        }
                    });

                    ui.separator();

                    let scroll_h = ui.available_height().max(100.0);
                    egui::ScrollArea::vertical()
                        .max_height(scroll_h)
                        .min_scrolled_height(scroll_h)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            // 1. Embedded Video Chapters
                            if !stats.chapters.is_empty() {
                                ui.label(egui::RichText::new("EMBEDDED CHAPTERS").strong().color(Color32::from_rgb(140, 190, 255)));
                                for (idx, ch) in stats.chapters.iter().enumerate() {
                                    let time_str = format_time(ch.time_pos);
                                    let title = if ch.title.is_empty() { format!("Chapter {}", idx + 1) } else { ch.title.clone() };
                                    ui.horizontal(|ui| {
                                        if ui.button(egui::RichText::new(&time_str).monospace().color(VortexTheme::POT_YELLOW)).clicked() {
                                            actions.seek_to = Some(ch.time_pos);
                                        }
                                        if ui.selectable_label(false, &title).clicked() {
                                            actions.seek_to = Some(ch.time_pos);
                                        }
                                    });
                                }
                                ui.add_space(8.0);
                                ui.separator();
                            }

                            // 2. User PBF Bookmarks
                            ui.label(egui::RichText::new(format!("USER BOOKMARKS ({})", bookmark_mgr.bookmarks.len())).strong().color(VortexTheme::POT_YELLOW));
                            if bookmark_mgr.bookmarks.is_empty() {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(20.0);
                                    ui.label(egui::RichText::new("No bookmarks saved for this media.").color(Color32::from_rgb(130, 135, 150)));
                                    ui.label(egui::RichText::new("Press P or click '+ Add Bookmark'").size(10.5).color(Color32::from_rgb(100, 105, 120)));
                                });
                            } else {
                                for (b_idx, bm) in bookmark_mgr.bookmarks.iter().enumerate() {
                                    let time_str = format_time(bm.time_pos);
                                    ui.horizontal(|ui| {
                                        if ui.button(egui::RichText::new(&time_str).monospace().color(VortexTheme::POT_YELLOW)).clicked() {
                                            actions.seek_to = Some(bm.time_pos);
                                        }
                                        if ui.selectable_label(false, &bm.title).clicked() {
                                            actions.seek_to = Some(bm.time_pos);
                                        }
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.small_button("✕").on_hover_text("Delete bookmark").clicked() {
                                                actions.delete_bookmark_idx = Some(b_idx);
                                            }
                                        });
                                    });
                                }
                            }
                        });
                }

                // =========================================================================
                // TAB 4: SUBTITLE TRANSCRIPT EXPLORER
                // =========================================================================
                DrawerTab::Subtitles => {
                    // Search bar across subtitle dialogue
                    ui.horizontal(|ui| {
                        ui.add_space(2.0);
                        ui.add_sized(
                            Vec2::new(ui.available_width() - 28.0, 22.0),
                            egui::TextEdit::singleline(sub_search)
                                .hint_text("🔍 Search dialogue text...")
                                .font(FontId::proportional(11.0)),
                        );
                        if !sub_search.is_empty() {
                            if ui.button("✕").clicked() {
                                sub_search.clear();
                            }
                        }
                    });

                    ui.separator();

                    let scroll_h = ui.available_height().max(100.0);
                    egui::ScrollArea::vertical()
                        .max_height(scroll_h)
                        .min_scrolled_height(scroll_h)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if sub_lines.is_empty() {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(30.0);
                                    ui.label(egui::RichText::new("No subtitle lines loaded").color(Color32::from_rgb(130, 135, 150)));
                                    ui.label(egui::RichText::new("Load an external .srt/.vtt or select a text subtitle track").size(10.5).color(Color32::from_rgb(100, 105, 120)));
                                });
                            } else {
                                let filter_query = sub_search.trim().to_lowercase();
                                let mut count = 0;
                                for (start, _end, text) in sub_lines {
                                    if filter_query.is_empty() || text.to_lowercase().contains(&filter_query) {
                                        count += 1;
                                        let time_str = format_time(*start);
                                        ui.horizontal(|ui| {
                                            if ui.button(egui::RichText::new(&time_str).monospace().size(10.0).color(VortexTheme::POT_YELLOW)).clicked() {
                                                actions.seek_to = Some(*start);
                                            }
                                            if ui.selectable_label(false, text).clicked() {
                                                actions.seek_to = Some(*start);
                                            }
                                        });
                                    }
                                }
                                if count == 0 {
                                    ui.label(egui::RichText::new("No dialogue matches your query").color(Color32::from_rgb(130, 135, 150)));
                                }
                            }
                        });
                }
            }
        });

        actions
    }
}
