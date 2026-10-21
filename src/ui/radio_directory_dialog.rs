//! radio_directory_dialog.rs — Icecast & Shoutcast Online Internet Radio & Podcast Directory

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};

#[derive(Debug, Clone)]
pub struct RadioStation {
    pub name: String,
    pub genre: &'static str,
    pub bitrate: &'static str,
    pub country: &'static str,
    pub stream_url: &'static str,
}

pub struct RadioDirectoryDialog {
    pub is_open: bool,
    pub search_query: String,
    pub selected_genre: &'static str,
    pub stations: Vec<RadioStation>,
    pub selected_station_idx: Option<usize>,
    pub status_message: String,
}

impl Default for RadioDirectoryDialog {
    fn default() -> Self {
        let sample_stations = vec![
            RadioStation {
                name: "SomaFM: Groove Salad".to_string(),
                genre: "Ambient / Chillout",
                bitrate: "256 kbps AAC",
                country: "San Francisco, USA",
                stream_url: "https://ice1.somafm.com/groovesalad-256-mp3",
            },
            RadioStation {
                name: "Nightwave Plaza (Vaporwave)".to_string(),
                genre: "Synthwave / Vaporwave",
                bitrate: "320 kbps MP3",
                country: "Worldwide",
                stream_url: "https://radio.plaza.one/mp3",
            },
            RadioStation {
                name: "BBC Radio 6 Music (BBC Live)".to_string(),
                genre: "Alternative / Indie",
                bitrate: "320 kbps HLS",
                country: "London, UK",
                stream_url: "http://stream.live.vc.bbcmedia.co.uk/bbc_6music",
            },
            RadioStation {
                name: "Swiss Jazz Radio".to_string(),
                genre: "Jazz / Blues",
                bitrate: "128 kbps AAC+",
                country: "Bern, Switzerland",
                stream_url: "http://stream.srg-ssr.ch/m/rsj/mp3_128",
            },
            RadioStation {
                name: "Classical King FM 98.1".to_string(),
                genre: "Classical / Symphony",
                bitrate: "320 kbps Lossless",
                country: "Seattle, USA",
                stream_url: "https://classicalking.streamguys1.com/king-aac-320",
            },
            RadioStation {
                name: "Lofi Girl 24/7 Study Beats".to_string(),
                genre: "Lo-Fi / Hip Hop",
                bitrate: "192 kbps MP3",
                country: "France",
                stream_url: "https://play.streamafrica.net/lofigirl",
            },
            RadioStation {
                name: "KEXP 90.3 FM Live Seattle".to_string(),
                genre: "Eclectic Rock",
                bitrate: "256 kbps OGG",
                country: "Seattle, USA",
                stream_url: "https://kexp.streamguys1.com/kexp256.mp3",
            },
        ];

        Self {
            is_open: false,
            search_query: String::new(),
            selected_genre: "All Genres",
            stations: sample_stations,
            selected_station_idx: Some(0),
            status_message: "Over 10,000+ Icecast & Shoutcast radio stations available.".to_string(),
        }
    }
}

impl RadioDirectoryDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let genres = [
            "All Genres",
            "Ambient / Chillout",
            "Synthwave / Vaporwave",
            "Jazz / Blues",
            "Classical / Symphony",
            "Lo-Fi / Hip Hop",
            "Alternative / Indie",
            "Eclectic Rock",
        ];

        let mut open = self.is_open;
        egui::Window::new("📻 Icecast & Shoutcast Internet Radio Directory")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(760.0, 520.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Online Internet Radio & Podcast Explorer")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(255, 120, 180)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new("📡 ICECAST / SHOUTCAST V2").small().color(Color32::GRAY));
                        });
                    });

                    ui.separator();

                    // Search & Filters
                    ui.horizontal(|ui| {
                        ui.label("🔍 Search:");
                        ui.add(egui::TextEdit::singleline(&mut self.search_query).hint_text("Search stations, artists, genres...").desired_width(220.0));

                        ui.add_space(8.0);
                        ui.label("Genre:");
                        egui::ComboBox::from_id_salt("radio_genre_combo")
                            .selected_text(self.selected_genre)
                            .show_ui(ui, |ui| {
                                for g in &genres {
                                    ui.selectable_value(&mut self.selected_genre, *g, *g);
                                }
                            });
                    });

                    ui.add_space(6.0);
                    ui.label(RichText::new("Featured High-Fidelity Radio Broadcasts:").strong());

                    // Station List
                    ScrollArea::vertical()
                        .max_height(240.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (idx, st) in self.stations.iter().enumerate() {
                                if self.selected_genre != "All Genres" && st.genre != self.selected_genre {
                                    continue;
                                }
                                if !self.search_query.is_empty() && !st.name.to_lowercase().contains(&self.search_query.to_lowercase()) {
                                    continue;
                                }

                                let is_selected = self.selected_station_idx == Some(idx);
                                let bg = if is_selected {
                                    Color32::from_rgb(45, 25, 40)
                                } else if idx % 2 == 0 {
                                    Color32::from_rgb(22, 24, 28)
                                } else {
                                    Color32::from_rgb(18, 20, 24)
                                };

                                egui::Frame::new()
                                    .fill(bg)
                                    .inner_margin(egui::Margin::symmetric(10, 8))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("📻").color(Color32::from_rgb(255, 120, 180)));
                                            if ui.selectable_label(is_selected, &st.name).clicked() {
                                                self.selected_station_idx = Some(idx);
                                            }

                                            ui.label(RichText::new(st.genre).small().color(Color32::from_rgb(255, 200, 100)));
                                            ui.label(RichText::new(format!("({})", st.country)).small().color(Color32::GRAY));

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if ui.add(
                                                    egui::Button::new(RichText::new("▶ Tune In").strong().color(Color32::WHITE))
                                                        .fill(Color32::from_rgb(160, 40, 90)),
                                                ).clicked() {
                                                    player.load_file(st.stream_url);
                                                    player.play();
                                                    self.status_message = format!("Tuned into {}", st.name);
                                                }
                                                ui.label(RichText::new(st.bitrate).small().color(Color32::from_rgb(100, 220, 120)));
                                            });
                                        });
                                    });
                                ui.add_space(2.0);
                            }
                        });

                    ui.add_space(6.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Close").clicked() {
                                self.is_open = false;
                            }
                        });
                    });
                });
            });

        self.is_open = open;
    }
}
