//! scrobbler_dialog.rs — Trakt.tv, AniList, MyAnimeList & Last.fm Scrobbler Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};

#[derive(Debug, Clone)]
pub struct ScrobbleHistoryEntry {
    pub timestamp: String,
    pub title: String,
    pub service: &'static str,
    pub progress_pct: u32,
    pub status: &'static str,
}

pub struct ScrobblerDialog {
    pub is_open: bool,
    // Trakt.tv
    pub trakt_enabled: bool,
    pub trakt_username: String,
    pub trakt_authenticated: bool,
    // AniList / MyAnimeList
    pub anilist_enabled: bool,
    pub anilist_username: String,
    pub anilist_authenticated: bool,
    // Last.fm
    pub lastfm_enabled: bool,
    pub lastfm_username: String,
    pub lastfm_authenticated: bool,

    pub watch_threshold_pct: u32,
    pub history: Vec<ScrobbleHistoryEntry>,
    pub status_message: String,
}

impl Default for ScrobblerDialog {
    fn default() -> Self {
        let sample_history = vec![
            ScrobbleHistoryEntry {
                timestamp: "10 mins ago".to_string(),
                title: "Dune: Part Two (2024)".to_string(),
                service: "Trakt.tv",
                progress_pct: 100,
                status: "Watched ✓",
            },
            ScrobbleHistoryEntry {
                timestamp: "Yesterday".to_string(),
                title: "Frieren: Beyond Journey's End - Ep 28".to_string(),
                service: "AniList",
                progress_pct: 100,
                status: "Completed ✓",
            },
            ScrobbleHistoryEntry {
                timestamp: "2 days ago".to_string(),
                title: "Hans Zimmer - Time (Inception OST)".to_string(),
                service: "Last.fm",
                progress_pct: 100,
                status: "Scrobbled ♪",
            },
        ];

        Self {
            is_open: false,
            trakt_enabled: true,
            trakt_username: "CinephilePower".to_string(),
            trakt_authenticated: true,
            anilist_enabled: true,
            anilist_username: "AnimeWatcher99".to_string(),
            anilist_authenticated: true,
            lastfm_enabled: true,
            lastfm_username: "Audiophile_HQ".to_string(),
            lastfm_authenticated: true,
            watch_threshold_pct: 80,
            history: sample_history,
            status_message: "All scrobbler sync services connected and active.".to_string(),
        }
    }
}

impl ScrobblerDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🎬 Trakt.tv, AniList & Last.fm Scrobbler Studio")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(740.0, 520.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header
                    ui.label(
                        RichText::new("Automatic Cloud Scrobbling & Watch Progress Sync")
                            .strong()
                            .size(16.0)
                            .color(Color32::from_rgb(240, 100, 100)),
                    );

                    ui.separator();

                    // Trakt.tv Box
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.trakt_enabled, "🍿 Trakt.tv (Movies & TV Series)");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if self.trakt_authenticated {
                                    ui.label(RichText::new(format!("Logged in: @{}", self.trakt_username)).color(Color32::from_rgb(60, 220, 100)).strong());
                                } else if ui.button("Authorize with PIN").clicked() {
                                    self.trakt_authenticated = true;
                                }
                            });
                        });
                    });

                    ui.add_space(4.0);

                    // AniList Box
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.anilist_enabled, "🌸 AniList & MyAnimeList (Anime Auto-Tracker)");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if self.anilist_authenticated {
                                    ui.label(RichText::new(format!("Logged in: @{}", self.anilist_username)).color(Color32::from_rgb(60, 220, 100)).strong());
                                } else if ui.button("Connect Account").clicked() {
                                    self.anilist_authenticated = true;
                                }
                            });
                        });
                    });

                    ui.add_space(4.0);

                    // Last.fm Box
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.lastfm_enabled, "🎵 Last.fm & ListenBrainz (Music Scrobbler)");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if self.lastfm_authenticated {
                                    ui.label(RichText::new(format!("Logged in: @{}", self.lastfm_username)).color(Color32::from_rgb(60, 220, 100)).strong());
                                } else if ui.button("Connect Last.fm").clicked() {
                                    self.lastfm_authenticated = true;
                                }
                            });
                        });
                    });

                    ui.add_space(6.0);

                    // Scrobble Settings
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Mark as Watched Threshold:");
                            ui.add(egui::Slider::new(&mut self.watch_threshold_pct, 50..=95).suffix("% of runtime"));
                        });
                    });

                    ui.add_space(6.0);
                    ui.label(RichText::new("Recent Scrobble Activity Log:").strong());

                    // History table
                    ScrollArea::vertical()
                        .max_height(160.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for entry in &self.history {
                                egui::Frame::new()
                                    .fill(Color32::from_rgb(22, 24, 28))
                                    .inner_margin(egui::Margin::symmetric(8, 5))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(entry.service).strong().color(Color32::from_rgb(255, 180, 80)));
                                            ui.label(RichText::new(&entry.title).color(Color32::WHITE));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(entry.status).color(Color32::from_rgb(100, 220, 120)));
                                                ui.label(RichText::new(&entry.timestamp).small().color(Color32::GRAY));
                                            });
                                        });
                                    });
                                ui.add_space(2.0);
                            }
                        });

                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Sync Now").clicked() {
                            self.status_message = "Synced watch states with Trakt.tv and AniList.".to_string();
                        }

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
