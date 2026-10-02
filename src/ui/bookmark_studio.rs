use super::theme::VortexTheme;
use crate::bookmark::BookmarkManager;
use eframe::egui::{self, Align2, Color32, RichText, Vec2};

pub struct BookmarkStudioDialog {
    pub is_open: bool,
    pub search_query: String,
}

impl Default for BookmarkStudioDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            search_query: String::new(),
        }
    }
}

impl BookmarkStudioDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        bookmark_mgr: &mut BookmarkManager,
        _current_file: &str,
        seek_to_time: &mut Option<f64>,
    ) {
        if !self.is_open {
            return;
        }

        let mut open_flag = self.is_open;
        egui::Window::new("🔖 Bookmark Studio & Timeline Annotator")
            .open(&mut open_flag)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_width(600.0)
            .default_height(450.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("BOOKMARKS & SCENE NOTES").strong().color(VortexTheme::VORTEX_YELLOW));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.text_edit_singleline(&mut self.search_query);
                        ui.label("🔍 Search:");
                    });
                });
                ui.separator();

                if bookmark_mgr.bookmarks.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.label(RichText::new("(No bookmarks for current file)").color(Color32::from_rgb(140, 140, 160)));
                        ui.label("Press 'P' or use the context menu to add bookmarks.");
                    });
                } else {
                    egui::ScrollArea::vertical().id_salt("bookmark_studio_scroll").show(ui, |ui| {
                        for bm in bookmark_mgr.bookmarks.iter() {
                            let matches = self.search_query.is_empty() || bm.title.to_lowercase().contains(&self.search_query.to_lowercase());
                            if matches {
                                ui.horizontal(|ui| {
                                    if ui.button(RichText::new(format!("▶ Jump to {:.1}s", bm.time_pos)).strong().color(VortexTheme::VORTEX_YELLOW)).clicked() {
                                        *seek_to_time = Some(bm.time_pos);
                                    }
                                    ui.label(RichText::new(&bm.title).monospace());
                                });
                                ui.separator();
                            }
                        }
                    });
                }
            });
        self.is_open = open_flag;
    }
}
