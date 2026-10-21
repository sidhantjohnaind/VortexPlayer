//! playback_history_dialog.rs — Playback History & Watch Statistics Log (Vortex style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub file_path: String,
    pub file_name: String,
    pub last_played: String,
    pub watch_count: u32,
    pub last_position: f64,
    pub duration: f64,
    pub completed: bool,
}

pub struct PlaybackHistoryDialog {
    pub is_open: bool,
    pub entries: Vec<HistoryEntry>,
    pub search_query: String,
    pub sort_by_recent: bool,
    pub status_message: String,
}

impl Default for PlaybackHistoryDialog {
    fn default() -> Self {
        Self::load()
    }
}

impl PlaybackHistoryDialog {
    pub fn new() -> Self {
        Self::load()
    }

    pub fn history_path() -> PathBuf {
        let base = dirs::config_dir()
            .or_else(dirs::data_dir)
            .unwrap_or_else(|| PathBuf::from("."));
        let dir = base.join("vortex-player");
        let _ = std::fs::create_dir_all(&dir);
        dir.join("history.json")
    }

    pub fn load() -> Self {
        let path = Self::history_path();
        let entries: Vec<HistoryEntry> = if path.exists() {
            if let Ok(file) = std::fs::File::open(&path) {
                serde_json::from_reader(file).unwrap_or_default()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let count = entries.len();
        Self {
            is_open: false,
            entries,
            search_query: String::new(),
            sort_by_recent: true,
            status_message: if count > 0 {
                format!("{} entries in playback history.", count)
            } else {
                "No playback history yet.".to_string()
            },
        }
    }

    pub fn save(&self) {
        let path = Self::history_path();
        if let Ok(file) = std::fs::File::create(&path) {
            let _ = serde_json::to_writer_pretty(file, &self.entries);
        }
    }

    pub fn add_entry(&mut self, path: &str, position: f64, duration: f64) {
        if path.is_empty() {
            return;
        }
        let now_str = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        if let Some(existing) = self.entries.iter_mut().find(|e| e.file_path == path) {
            existing.watch_count += 1;
            if position > 0.0 {
                existing.last_position = position;
            }
            if duration > 0.0 {
                existing.duration = duration;
            }
            if existing.duration > 0.0 && existing.last_position >= existing.duration * 0.9 {
                existing.completed = true;
            }
            existing.last_played = now_str;
        } else {
            let name = PathBuf::from(path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string());
            self.entries.insert(
                0,
                HistoryEntry {
                    file_path: path.to_string(),
                    file_name: name,
                    last_played: now_str,
                    watch_count: 1,
                    last_position: position,
                    duration,
                    completed: duration > 0.0 && position >= duration * 0.9,
                },
            );
        }

        if self.entries.len() > 100 {
            self.entries.truncate(100);
        }
        self.status_message = format!("{} entries in playback history.", self.entries.len());
        self.save();
    }

    pub fn update_position(&mut self, path: &str, position: f64, duration: f64) {
        if path.is_empty() {
            return;
        }
        if let Some(existing) = self.entries.iter_mut().find(|e| e.file_path == path) {
            existing.last_position = position;
            if duration > 0.0 {
                existing.duration = duration;
            }
            if existing.duration > 0.0 && existing.last_position >= existing.duration * 0.9 {
                existing.completed = true;
            }
            self.save();
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, play_file: &mut Option<PathBuf>) -> Option<egui::Rect> {
        if !self.is_open {
            return None;
        }
        let mut open = self.is_open;
        let window_resp = egui::Window::new("📜 Playback History & Watch Statistics")
            .id(egui::Id::new("playback_history_window"))
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(750.0, 460.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Playback History Log")
                            .strong()
                            .size(15.0)
                            .color(Color32::from_rgb(200, 180, 255)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{} entries", self.entries.len()))
                                .color(Color32::GRAY),
                        );
                    });
                });
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("🔍");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search_query)
                            .desired_width(250.0)
                            .hint_text("Search history..."),
                    );
                    if ui.button("Clear All History").clicked() {
                        self.entries.clear();
                        self.save();
                    }
                });

                ui.add_space(4.0);
                let query_lower = self.search_query.to_lowercase();

                ScrollArea::vertical()
                    .max_height(320.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.entries.is_empty() {
                            ui.add_space(20.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new("No files played yet.")
                                        .color(Color32::from_rgb(140, 140, 160))
                                        .size(13.0),
                                );
                            });
                            return;
                        }

                        for (idx, entry) in self.entries.iter().enumerate() {
                            if !query_lower.is_empty()
                                && !entry.file_name.to_lowercase().contains(&query_lower)
                            {
                                continue;
                            }
                            let bg = if idx % 2 == 0 {
                                Color32::from_rgb(22, 24, 28)
                            } else {
                                Color32::from_rgb(18, 20, 24)
                            };
                            let pct = if entry.duration > 0.0 {
                                (entry.last_position / entry.duration * 100.0).clamp(0.0, 100.0)
                                    as u32
                            } else {
                                0
                            };

                            egui::Frame::new()
                                .fill(bg)
                                .inner_margin(egui::Margin::symmetric(8, 6))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        if entry.completed {
                                            ui.label(
                                                RichText::new("✓")
                                                    .color(Color32::from_rgb(60, 200, 100)),
                                            );
                                        } else if pct > 0 {
                                            ui.label(
                                                RichText::new(format!("{}%", pct))
                                                    .color(Color32::from_rgb(255, 200, 80)),
                                            );
                                        } else {
                                            ui.label(
                                                RichText::new("•")
                                                    .color(Color32::from_rgb(140, 140, 160)),
                                            );
                                        }
                                        ui.label(
                                            RichText::new(&entry.file_name)
                                                .strong()
                                                .color(Color32::WHITE),
                                        );
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                let path = entry.file_path.clone();
                                                if ui.small_button("▶ Play").clicked() {
                                                    *play_file = Some(PathBuf::from(&path));
                                                }
                                                ui.label(
                                                    RichText::new(format!(
                                                        "×{}",
                                                        entry.watch_count
                                                    ))
                                                    .small()
                                                    .color(Color32::from_rgb(180, 140, 255)),
                                                );
                                                ui.label(
                                                    RichText::new(&entry.last_played)
                                                        .small()
                                                        .color(Color32::GRAY),
                                                );
                                            },
                                        );
                                    });
                                });
                            ui.add_space(1.0);
                        }
                    });

                ui.add_space(2.0);
                ui.label(
                    RichText::new(&self.status_message)
                        .small()
                        .color(Color32::from_rgb(140, 190, 220)),
                );
            });
        self.is_open = open;
        window_resp.map(|r| r.response.rect)
    }
}
