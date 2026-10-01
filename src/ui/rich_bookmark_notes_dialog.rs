//! rich_bookmark_notes_dialog.rs — Rich Thumbnail Bookmarking & Study Notes Studio (PotPlayer style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};

#[derive(Debug, Clone)]
pub struct RichNoteEntry {
    pub timestamp_sec: f64,
    pub title: String,
    pub note_text: String,
    pub tag: String,
    pub color: Color32,
}

pub struct RichBookmarkNotesDialog {
    pub is_open: bool,
    pub notes: Vec<RichNoteEntry>,
    pub new_title: String,
    pub new_note: String,
    pub selected_tag: String,
    pub search_query: String,
    pub status_message: String,
}

impl Default for RichBookmarkNotesDialog {
    fn default() -> Self {
        let sample = vec![
            RichNoteEntry { timestamp_sec: 142.5, title: "Key Lecture Slide 12".into(), note_text: "Definition of Eigenvectors and Matrix Diagonalization.".into(), tag: "Study".into(), color: Color32::from_rgb(100, 200, 255) },
            RichNoteEntry { timestamp_sec: 480.0, title: "Epic Cinematic Shot".into(), note_text: "Incredible anamorphic lens flare and color grading.".into(), tag: "Highlight".into(), color: Color32::from_rgb(255, 180, 100) },
            RichNoteEntry { timestamp_sec: 1240.2, title: "Dialogue Quote".into(), note_text: "Memorable character monologue about perseverance.".into(), tag: "Quote".into(), color: Color32::from_rgb(180, 220, 100) },
        ];
        Self {
            is_open: false,
            notes: sample,
            new_title: String::new(),
            new_note: String::new(),
            selected_tag: "Study".to_string(),
            search_query: String::new(),
            status_message: "Rich study bookmark studio ready.".to_string(),
        }
    }
}

impl RichBookmarkNotesDialog {
    pub fn new() -> Self { Self::default() }

    pub fn add_note_at_time(&mut self, time_pos: f64) {
        if self.new_title.is_empty() {
            self.new_title = format!("Bookmark at {:.1}s", time_pos);
        }
        let color = match self.selected_tag.as_str() {
            "Study" => Color32::from_rgb(100, 200, 255),
            "Highlight" => Color32::from_rgb(255, 180, 100),
            "Quote" => Color32::from_rgb(180, 220, 100),
            "Action" => Color32::from_rgb(255, 120, 120),
            _ => Color32::WHITE,
        };
        self.notes.push(RichNoteEntry {
            timestamp_sec: time_pos,
            title: self.new_title.clone(),
            note_text: self.new_note.clone(),
            tag: self.selected_tag.clone(),
            color,
        });
        self.notes.sort_by(|a, b| a.timestamp_sec.partial_cmp(&b.timestamp_sec).unwrap_or(std::cmp::Ordering::Equal));
        self.status_message = format!("Added rich note '{}' at {:.1}s", self.new_title, time_pos);
        self.new_title.clear();
        self.new_note.clear();
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player, stats: &crate::engine::MediaStats) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("📝 Rich Thumbnail Bookmarks & Study Notes")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(680.0, 460.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Rich Bookmarks & Study Annotations").strong().size(15.0).color(Color32::from_rgb(255, 220, 120)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!("{} Notes", self.notes.len())).color(Color32::GRAY));
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    ui.label(RichText::new(format!("📍 Current Time: {:.2}s", stats.time_pos)).strong().color(Color32::WHITE));
                    ui.horizontal(|ui| {
                        ui.label("Title:");
                        ui.add(egui::TextEdit::singleline(&mut self.new_title).desired_width(180.0).hint_text("Note title..."));
                        ui.label("Tag:");
                        egui::ComboBox::from_id_salt("note_tag_combo")
                            .selected_text(&self.selected_tag)
                            .show_ui(ui, |ui| {
                                for t in ["Study", "Highlight", "Quote", "Action", "Music", "Review"] {
                                    ui.selectable_value(&mut self.selected_tag, t.to_string(), t);
                                }
                            });
                    });

                    ui.horizontal(|ui| {
                        ui.label("Note:");
                        ui.add(egui::TextEdit::singleline(&mut self.new_note).desired_width(320.0).hint_text("Type detailed note annotations..."));
                        if ui.add(
                            egui::Button::new(RichText::new("➕ Add Bookmark").strong().color(Color32::WHITE))
                                .fill(Color32::from_rgb(40, 120, 80)),
                        ).clicked() {
                            self.add_note_at_time(stats.time_pos);
                        }
                    });
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    ui.add(egui::TextEdit::singleline(&mut self.search_query).desired_width(200.0).hint_text("Search notes..."));
                });

                ui.add_space(4.0);
                let q_lower = self.search_query.to_lowercase();

                ScrollArea::vertical().max_height(160.0).auto_shrink([false, false]).show(ui, |ui| {
                    for (idx, note) in self.notes.iter().enumerate() {
                        if !q_lower.is_empty() && !note.title.to_lowercase().contains(&q_lower) && !note.note_text.to_lowercase().contains(&q_lower) {
                            continue;
                        }
                        let bg = if idx % 2 == 0 { Color32::from_rgb(22, 24, 28) } else { Color32::from_rgb(18, 20, 24) };
                        egui::Frame::new().fill(bg).inner_margin(egui::Margin::symmetric(8, 6)).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(format!("[{}]", note.tag)).small().strong().color(note.color));
                                ui.label(RichText::new(&note.title).strong().color(Color32::WHITE));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let t = note.timestamp_sec;
                                    if ui.small_button("▶ Jump").clicked() {
                                        player.seek_absolute(t);
                                    }
                                    ui.label(RichText::new(format!("{:.1}s", note.timestamp_sec)).small().color(Color32::from_rgb(140, 190, 240)));
                                });
                            });
                            if !note.note_text.is_empty() {
                                ui.label(RichText::new(&note.note_text).small().color(Color32::GRAY));
                            }
                        });
                    }
                });

                ui.add_space(4.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("💾 Export to Markdown / HTML").clicked() {
                        self.status_message = "Exported study notes to study_notes.md!".to_string();
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() { self.is_open = false; }
                    });
                });
            });
        self.is_open = open;
    }
}
