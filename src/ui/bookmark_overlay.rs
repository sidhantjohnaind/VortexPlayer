use super::theme::VortexTheme;
use crate::bookmark::{format_time, BookmarkManager, PbfFile};
use crate::engine::{MediaStats, Player};
use eframe::egui::{self, Align, Align2, Layout, Rect, RichText, Vec2};

pub struct BookmarkOverlay;

impl BookmarkOverlay {
    pub fn render(
        ctx: &egui::Context,
        is_open: &mut bool,
        player: &Player,
        stats: &MediaStats,
        bookmark_mgr: &mut BookmarkManager,
    ) -> Option<Rect> {
        if !*is_open {
            return None;
        }

        let resp = egui::Window::new("Bookmarks & PBF Manager (Ctrl+Shift+B)")
            .open(is_open)
            .resizable(true)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_width(450.0)
            .default_height(350.0)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 6.0);

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Saved Bookmarks").strong().color(VortexTheme::VORTEX_YELLOW));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("+ Add at Current Pos").clicked() {
                            bookmark_mgr.add_bookmark(stats.time_pos, None);
                        }
                    });
                });

                ui.separator();

                let list_height = ui.available_height() - 40.0;
                egui::ScrollArea::vertical()
                    .id_salt("bookmark_overlay_scroll")
                    .max_height(list_height)
                    .show(ui, |ui| {
                        if bookmark_mgr.bookmarks.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.add_space(30.0);
                                ui.label(RichText::new("No bookmarks saved for this file").color(VortexTheme::TEXT_MUTED).size(12.0));
                                ui.label(RichText::new("Press [P] or [Ctrl+B] to bookmark timestamps").color(VortexTheme::TEXT_MUTED).size(11.0));
                            });
                        } else {
                            let mut delete_idx = None;

                            for (idx, bm) in bookmark_mgr.bookmarks.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);

                                    // Timestamp Button to Jump
                                    let time_label = ui.button(
                                        RichText::new(format_time(bm.time_pos))
                                            .monospace()
                                            .color(VortexTheme::VORTEX_YELLOW)
                                            .size(11.5),
                                    );
                                    if time_label.clicked() {
                                        player.seek_absolute(bm.time_pos);
                                    }

                                    // Title Text Edit
                                    ui.add(egui::TextEdit::singleline(&mut bm.title).desired_width(ui.available_width() - 35.0));

                                    // Delete Button
                                    if ui.button(RichText::new("✕").size(10.0).color(VortexTheme::TEXT_MUTED)).clicked() {
                                        delete_idx = Some(idx);
                                    }
                                });
                            }

                            if let Some(idx) = delete_idx {
                                bookmark_mgr.remove_bookmark(idx);
                            }
                        }
                    });

                ui.separator();

                // Bottom Export/Import & Clear Toolbar
                ui.horizontal(|ui| {
                    if ui.button("Export .PBF").clicked() {
                        if let Some(ref path) = bookmark_mgr.current_video_path {
                            let _ = PbfFile::save_for_video(path, &bookmark_mgr.bookmarks);
                        }
                    }
                    if ui.button("Import .PBF...").clicked() {
                        if let Some(file) = rfd::FileDialog::new().add_filter("VertexPlayer Bookmark (*.pbf)", &["pbf"]).pick_file() {
                            if let Ok(content) = std::fs::read_to_string(&file) {
                                let imported = PbfFile::parse(&content);
                                bookmark_mgr.bookmarks = imported;
                            }
                        }
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("Clear All").clicked() {
                            bookmark_mgr.clear_bookmarks();
                        }
                    });
                });
            });
        resp.map(|r| r.response.rect)
    }
}
