use super::theme::VortexTheme;
use crate::library::{MediaItem, MediaLibrary};
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Vec2};
use std::path::PathBuf;

pub struct LibraryView {
    pub search_query: String,
    pub selected_tab: String,
}

impl Default for LibraryView {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            selected_tab: "All Media".to_string(),
        }
    }
}

impl LibraryView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        library: &mut MediaLibrary,
    ) -> (Option<PathBuf>, Option<Rect>) {
        if !*is_open {
            return (None, None);
        }

        let mut file_to_play = None;

        let resp = egui::Window::new("Media Library & Poster Wall (F8)")
            .open(is_open)
            .default_width(740.0)
            .default_height(520.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("LIBRARY").strong().color(VortexTheme::current_skin().accent_primary));
                    ui.add(egui::TextEdit::singleline(&mut self.search_query).hint_text("Search library by title, season, or tag...").desired_width(320.0));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("📁 Add Watch Folder").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                library.watch_folders.push(folder.clone());
                                library.scan_folder(&folder);
                            }
                        }
                    });
                });

                ui.separator();

                // ── Poster Grid Wall ─────────────────────────────────────────
                let query = self.search_query.to_lowercase();
                let filtered: Vec<&MediaItem> = library.items.iter().filter(|i| {
                    if query.is_empty() {
                        true
                    } else {
                        i.title.to_lowercase().contains(&query)
                    }
                }).collect();

                if filtered.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.label(RichText::new("No media in library").color(VortexTheme::current_skin().text_muted).size(14.0));
                        ui.label(RichText::new("Click 'Add Watch Folder' above to scan your movies and series.").color(VortexTheme::current_skin().text_secondary));
                    });
                } else {
                    egui::ScrollArea::vertical().id_salt("library_view_scroll").show(ui, |ui| {
                        let col_w = 130.0;
                        let col_h = 190.0;
                        let avail_w = ui.available_width();
                        let cols = ((avail_w / (col_w + 12.0)).floor() as usize).max(1);

                        egui::Grid::new("poster_grid")
                            .spacing(Vec2::new(12.0, 14.0))
                            .show(ui, |ui| {
                                for (idx, item) in filtered.iter().enumerate() {
                                    let (card_rect, resp) = ui.allocate_exact_size(Vec2::new(col_w, col_h), Sense::click());
                                    let painter = ui.painter();

                                    let is_h = resp.hovered();
                                    let bg = if is_h { Color32::from_rgb(32, 35, 46) } else { Color32::from_rgb(18, 20, 26) };
                                    let border = if is_h { VortexTheme::current_skin().accent_primary } else { Color32::from_rgb(30, 34, 44) };

                                    painter.rect_filled(card_rect, CornerRadius::same(4), bg);
                                    painter.rect_stroke(card_rect, CornerRadius::same(4), Stroke::new(1.0, border), StrokeKind::Inside);

                                    // Poster area (top 140px)
                                    let poster_r = Rect::from_min_size(card_rect.min, Vec2::new(col_w, 138.0));
                                    painter.rect_filled(poster_r, CornerRadius::same(4), Color32::from_rgb(12, 13, 16));
                                    
                                    // Movie/TV Emblem
                                    let icon_center = poster_r.center();
                                    painter.circle_filled(icon_center, 18.0, Color32::from_rgb(26, 28, 38));
                                    painter.text(icon_center, Align2::CENTER_CENTER, if item.is_tv_show { "📺" } else { "🎬" }, FontId::proportional(14.0), Color32::WHITE);

                                    // Title & Episode metadata (bottom 50px)
                                    let text_y = card_rect.top() + 144.0;
                                    let title_clipped = if item.title.len() > 18 { format!("{}…", &item.title[..16]) } else { item.title.clone() };
                                    painter.text(Pos2::new(card_rect.left() + 6.0, text_y), Align2::LEFT_TOP, title_clipped, FontId::proportional(10.5), Color32::WHITE);

                                    if let (Some(s), Some(e)) = (item.season, item.episode) {
                                        painter.text(
                                            Pos2::new(card_rect.left() + 6.0, text_y + 16.0),
                                            Align2::LEFT_TOP,
                                            format!("S{:02}E{:02}", s, e),
                                            FontId::monospace(9.5),
                                            VortexTheme::current_skin().accent_primary,
                                        );
                                    }

                                    if resp.clicked() {
                                        file_to_play = Some(item.path.clone());
                                    }

                                    if (idx + 1) % cols == 0 {
                                        ui.end_row();
                                    }
                                }
                            });
                    });
                }
            });

        (file_to_play, resp.map(|r| r.response.rect))
    }
}
