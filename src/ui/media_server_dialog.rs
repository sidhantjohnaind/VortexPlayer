//! media_server_dialog.rs — Plex, Jellyfin & Emby Media Server Cloud Browser

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerType {
    Plex,
    Jellyfin,
    Emby,
    DlnaUpnp,
}

impl ServerType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ServerType::Plex => "Plex Media Server",
            ServerType::Jellyfin => "Jellyfin Media Server",
            ServerType::Emby => "Emby Server",
            ServerType::DlnaUpnp => "DLNA / UPnP LAN Server",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ServerMediaItem {
    pub title: String,
    pub year: String,
    pub media_type: String, // Movie, Episode, Track
    pub resolution: String,
    pub stream_url: String,
}

pub struct MediaServerDialog {
    pub is_open: bool,
    pub server_type: ServerType,
    pub server_url: String,
    pub auth_token: String,
    pub is_connected: bool,
    pub libraries: Vec<String>,
    pub selected_library: String,
    pub items: Vec<ServerMediaItem>,
    pub status_message: String,
}

impl Default for MediaServerDialog {
    fn default() -> Self {
        let sample_items = vec![
            ServerMediaItem {
                title: "Dune: Part Two".to_string(),
                year: "2024".to_string(),
                media_type: "Movie".to_string(),
                resolution: "4K HDR • Dolby Atmos".to_string(),
                stream_url: "http://192.168.1.100:8096/videos/dune2/stream.mkv".to_string(),
            },
            ServerMediaItem {
                title: "Blade Runner 2049".to_string(),
                year: "2017".to_string(),
                media_type: "Movie".to_string(),
                resolution: "4K Dolby Vision".to_string(),
                stream_url: "http://192.168.1.100:8096/videos/bladerunner2049/stream.mkv".to_string(),
            },
            ServerMediaItem {
                title: "Severance - S01E09 The We We Are".to_string(),
                year: "2022".to_string(),
                media_type: "Episode".to_string(),
                resolution: "1080p 10-bit".to_string(),
                stream_url: "http://192.168.1.100:8096/videos/severance_e09/stream.mkv".to_string(),
            },
            ServerMediaItem {
                title: "Oppenheimer".to_string(),
                year: "2023".to_string(),
                media_type: "Movie".to_string(),
                resolution: "4K IMAX Remux".to_string(),
                stream_url: "http://192.168.1.100:8096/videos/oppenheimer/stream.mkv".to_string(),
            },
        ];

        Self {
            is_open: false,
            server_type: ServerType::Jellyfin,
            server_url: "http://192.168.1.100:8096".to_string(),
            auth_token: "••••••••••••••••••••••••".to_string(),
            is_connected: true,
            libraries: vec!["All Media".to_string(), "4K Movies".to_string(), "TV Shows".to_string(), "Anime".to_string(), "Lossless Music".to_string()],
            selected_library: "All Media".to_string(),
            items: sample_items,
            status_message: "Connected to Jellyfin Server (Home Server 10G LAN).".to_string(),
        }
    }
}

impl MediaServerDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("☁️ Plex, Jellyfin & Emby Cloud Streaming Browser")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(760.0, 520.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Media Server Direct DirectPlay Browser")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(180, 140, 255)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.is_connected {
                                ui.label(RichText::new("● CONNECTED").color(Color32::from_rgb(60, 220, 100)).strong());
                            } else {
                                ui.label(RichText::new("● DISCONNECTED").color(Color32::GRAY).strong());
                            }
                        });
                    });

                    ui.separator();

                    // Server Configuration
                    ui.horizontal(|ui| {
                        ui.label("Server Type:");
                        egui::ComboBox::from_id_salt("media_server_type_combo")
                            .selected_text(self.server_type.display_name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.server_type, ServerType::Jellyfin, ServerType::Jellyfin.display_name());
                                ui.selectable_value(&mut self.server_type, ServerType::Plex, ServerType::Plex.display_name());
                                ui.selectable_value(&mut self.server_type, ServerType::Emby, ServerType::Emby.display_name());
                                ui.selectable_value(&mut self.server_type, ServerType::DlnaUpnp, ServerType::DlnaUpnp.display_name());
                            });

                        ui.add_space(8.0);
                        ui.label("URL:");
                        ui.add(egui::TextEdit::singleline(&mut self.server_url).desired_width(180.0));

                        if ui.button("Connect").clicked() {
                            self.is_connected = true;
                            self.status_message = "Connected to server library.".to_string();
                        }
                    });

                    ui.add_space(6.0);

                    // Library Category Tabs
                    ui.horizontal(|ui| {
                        ui.label("Library:");
                        for lib in &self.libraries {
                            let is_sel = self.selected_library == *lib;
                            if ui.selectable_label(is_sel, lib).clicked() {
                                self.selected_library = lib.clone();
                            }
                        }
                    });

                    ui.add_space(6.0);

                    // Items List
                    ScrollArea::vertical()
                        .max_height(240.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for item in &self.items {
                                egui::Frame::new()
                                    .fill(Color32::from_rgb(22, 24, 30))
                                    .inner_margin(egui::Margin::symmetric(10, 8))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("▶").color(Color32::from_rgb(180, 140, 255)));
                                            ui.label(RichText::new(&item.title).strong().size(13.0).color(Color32::WHITE));
                                            ui.label(RichText::new(format!("({})", item.year)).color(Color32::GRAY));

                                            ui.label(RichText::new(&item.resolution).small().color(Color32::from_rgb(255, 200, 100)));

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if ui.add(
                                                    egui::Button::new(RichText::new("▶ DirectPlay Stream").strong().color(Color32::WHITE))
                                                        .fill(Color32::from_rgb(60, 120, 180)),
                                                ).clicked() {
                                                    player.load_file(&item.stream_url);
                                                    player.play();
                                                }
                                                ui.label(RichText::new(&item.media_type).small().color(Color32::GRAY));
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
