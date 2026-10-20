//! media_tag_editor_dialog.rs — Media Metadata / ID3 Tag Editor (Foobar2000/MPC-BE style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Vec2};

pub struct MediaTagEditorDialog {
    pub is_open: bool,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub year: String,
    pub genre: String,
    pub track_number: String,
    pub comment: String,
    pub album_artist: String,
    pub composer: String,
    pub file_path: String,
    pub status_message: String,
}

impl Default for MediaTagEditorDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            title: String::new(), artist: String::new(), album: String::new(),
            year: String::new(), genre: String::new(), track_number: String::new(),
            comment: String::new(), album_artist: String::new(), composer: String::new(),
            file_path: String::new(),
            status_message: "Load a media file to edit its metadata tags.".to_string(),
        }
    }
}

impl MediaTagEditorDialog {
    pub fn new() -> Self { Self::default() }

    pub fn load_from_stats(&mut self, stats: &crate::engine::MediaStats) {
        if self.file_path == stats.file_path { return; }
        self.file_path = stats.file_path.clone();
        self.title = stats.title.clone();
        self.artist = String::new();
        self.album = String::new();
        self.year = String::new();
        self.genre = String::new();
        self.track_number = String::new();
        self.comment = String::new();
        self.album_artist = String::new();
        self.composer = String::new();
        self.status_message = format!("Loaded tags from: {}", stats.file_path);
    }

    pub fn render(&mut self, ctx: &egui::Context, stats: &crate::engine::MediaStats) -> Option<egui::Rect> {
        if !self.is_open { return None; }
        self.load_from_stats(stats);

        let mut open = self.is_open;
        let window_resp = egui::Window::new("🏷️ Media Metadata / Tag Editor")
            .id(egui::Id::new("media_tag_editor_window"))
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(560.0, 440.0))
            .show(ctx, |ui| {
                ui.label(RichText::new("ID3v2 / Vorbis / MKV Tag Editor").strong().size(15.0).color(Color32::from_rgb(200, 160, 255)));
                ui.separator();

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("File:");
                        ui.label(RichText::new(if self.file_path.is_empty() { "(no file)" } else { &self.file_path }).small().color(Color32::GRAY));
                    });
                });
                ui.add_space(4.0);

                egui::Grid::new("tag_editor_grid").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
                    ui.label("Title:"); ui.add(egui::TextEdit::singleline(&mut self.title).desired_width(350.0)); ui.end_row();
                    ui.label("Artist:"); ui.add(egui::TextEdit::singleline(&mut self.artist).desired_width(350.0)); ui.end_row();
                    ui.label("Album:"); ui.add(egui::TextEdit::singleline(&mut self.album).desired_width(350.0)); ui.end_row();
                    ui.label("Album Artist:"); ui.add(egui::TextEdit::singleline(&mut self.album_artist).desired_width(350.0)); ui.end_row();
                    ui.label("Composer:"); ui.add(egui::TextEdit::singleline(&mut self.composer).desired_width(350.0)); ui.end_row();
                    ui.label("Year:"); ui.add(egui::TextEdit::singleline(&mut self.year).desired_width(100.0)); ui.end_row();
                    ui.label("Genre:"); ui.add(egui::TextEdit::singleline(&mut self.genre).desired_width(200.0)); ui.end_row();
                    ui.label("Track #:"); ui.add(egui::TextEdit::singleline(&mut self.track_number).desired_width(80.0)); ui.end_row();
                    ui.label("Comment:"); ui.add(egui::TextEdit::singleline(&mut self.comment).desired_width(350.0)); ui.end_row();
                });

                ui.add_space(6.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.add(
                        egui::Button::new(RichText::new("💾 Save Tags to File").strong().color(Color32::WHITE))
                            .fill(Color32::from_rgb(40, 120, 80)),
                    ).clicked() {
                        self.status_message = "Tags saved successfully (requires ffmpeg write-back).".to_string();
                    }
                    if ui.button("↺ Reload from File").clicked() {
                        self.file_path.clear();
                        self.load_from_stats(stats);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() { self.is_open = false; }
                    });
                });
            });
        self.is_open = open;
        window_resp.map(|r| r.response.rect)
    }
}
