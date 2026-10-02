use super::theme::VortexTheme;
use eframe::egui::{self, Color32, FontId, RichText};

pub struct SubtitleExplorerDialog {
    pub is_open: bool,
    pub search_query: String,
    pub lines: Vec<(f64, f64, String)>, // start, end, text
}

impl Default for SubtitleExplorerDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            search_query: String::new(),
            lines: Vec::new(),
        }
    }
}

impl SubtitleExplorerDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, seek_to_time: &mut Option<f64>) {
        if !self.is_open {
            return;
        }

        let mut open_flag = self.is_open;
        egui::Window::new("📝 Subtitle Explorer & Timeline Search")
            .open(&mut open_flag)
            .default_width(550.0)
            .default_height(400.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("SUBTITLE LINES & DIALOGUE").strong().color(VortexTheme::VORTEX_YELLOW));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.text_edit_singleline(&mut self.search_query);
                        ui.label("🔍 Search Text:");
                    });
                });
                ui.separator();

                if self.lines.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.label(RichText::new("(No parsed subtitle lines loaded)").color(Color32::from_rgb(140, 140, 160)));
                        ui.label("Subtitles will appear here when an external or embedded text subtitle is active.");
                    });
                } else {
                    egui::ScrollArea::vertical().id_salt("sub_explorer_scroll").show(ui, |ui| {
                        for (start, _end, text) in self.lines.iter() {
                            let matches = self.search_query.is_empty() || text.to_lowercase().contains(&self.search_query.to_lowercase());
                            if matches {
                                ui.horizontal(|ui| {
                                    let mins = (start / 60.0).floor() as u32;
                                    let secs = (start % 60.0).floor() as u32;
                                    let time_str = format!("{:02}:{:02}", mins, secs);
                                    if ui.button(RichText::new(time_str).monospace().color(VortexTheme::VORTEX_YELLOW)).clicked() {
                                        *seek_to_time = Some(*start);
                                    }
                                    ui.label(text);
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
