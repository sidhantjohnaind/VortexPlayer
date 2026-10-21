//! iptv_guide_dialog.rs — IPTV Channel Manager & EPG (Electronic Program Guide) Schedule Browser Dialog

#![allow(dead_code)]

use eframe::egui::{self, Color32, ProgressBar, RichText, ScrollArea, Vec2};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct EpgProgram {
    pub title: String,
    pub description: String,
    pub start_time: String,
    pub end_time: String,
    pub progress: f32,
}

#[derive(Debug, Clone)]
pub struct ChannelItem {
    pub name: String,
    pub group: String,
    pub url: String,
    pub logo: Option<String>,
    pub epg_now: EpgProgram,
    pub epg_next: EpgProgram,
}

pub struct IptvGuideDialog {
    pub is_open: bool,
    pub channels: Vec<ChannelItem>,
    pub selected_channel_idx: Option<usize>,
    pub search_query: String,
    pub selected_group: String,
    pub m3u_url_input: String,
}

impl Default for IptvGuideDialog {
    fn default() -> Self {
        let sample_channels = vec![
            ChannelItem {
                name: "BBC World News HD".to_string(),
                group: "News".to_string(),
                url: "https://stream.example.com/bbc_news/index.m3u8".to_string(),
                logo: None,
                epg_now: EpgProgram {
                    title: "Global News Hour Live".to_string(),
                    description: "Comprehensive live coverage of international headlines and global financial markets.".to_string(),
                    start_time: "06:00".to_string(),
                    end_time: "07:00".to_string(),
                    progress: 0.65,
                },
                epg_next: EpgProgram {
                    title: "World Business Report".to_string(),
                    description: "Global business insights and analysis from trading floors across the world.".to_string(),
                    start_time: "07:00".to_string(),
                    end_time: "07:30".to_string(),
                    progress: 0.0,
                },
            },
            ChannelItem {
                name: "NASA TV 4K Ultra HD".to_string(),
                group: "Documentary".to_string(),
                url: "https://ntv1.akamaized.net/hls/live/2014075/NASA-NTV1-HLS/master.m3u8".to_string(),
                logo: None,
                epg_now: EpgProgram {
                    title: "ISS Live Earth Views (HD)".to_string(),
                    description: "Live views of Earth from the International Space Station External High Definition Cameras.".to_string(),
                    start_time: "05:00".to_string(),
                    end_time: "08:00".to_string(),
                    progress: 0.45,
                },
                epg_next: EpgProgram {
                    title: "Artemis Deep Space Briefing".to_string(),
                    description: "Mission updates on lunar orbiters and deep space exploration vehicles.".to_string(),
                    start_time: "08:00".to_string(),
                    end_time: "09:00".to_string(),
                    progress: 0.0,
                },
            },
            ChannelItem {
                name: "Red Bull TV Action Sports".to_string(),
                group: "Sports".to_string(),
                url: "https://rbmn-live.akamaized.net/hls/live/590964/BoRB-AT/master.m3u8".to_string(),
                logo: None,
                epg_now: EpgProgram {
                    title: "UCI Mountain Bike World Cup Finals".to_string(),
                    description: "Downhill racing championships from Val di Sole, Italy.".to_string(),
                    start_time: "06:30".to_string(),
                    end_time: "08:00".to_string(),
                    progress: 0.20,
                },
                epg_next: EpgProgram {
                    title: "Cliff Diving World Series Highlights".to_string(),
                    description: "Acrobatic high dives from breathtaking coastal cliffs worldwide.".to_string(),
                    start_time: "08:00".to_string(),
                    end_time: "09:00".to_string(),
                    progress: 0.0,
                },
            },
            ChannelItem {
                name: "Bloomberg Financial TV".to_string(),
                group: "News".to_string(),
                url: "https://liveproduseast.global.ssl.fastly.net/us/Channel-USTV-AWS-virginia-1/live.m3u8".to_string(),
                logo: None,
                epg_now: EpgProgram {
                    title: "Surveillance: Markets & Tech Trends".to_string(),
                    description: "In-depth discussions on monetary policy, tech earnings and commodities.".to_string(),
                    start_time: "06:00".to_string(),
                    end_time: "07:30".to_string(),
                    progress: 0.50,
                },
                epg_next: EpgProgram {
                    title: "Bloomberg Technology with Ed Ludlow".to_string(),
                    description: "Daily focus on AI innovations, enterprise hardware, and venture capital.".to_string(),
                    start_time: "07:30".to_string(),
                    end_time: "08:30".to_string(),
                    progress: 0.0,
                },
            },
        ];

        Self {
            is_open: false,
            channels: sample_channels,
            selected_channel_idx: Some(0),
            search_query: String::new(),
            selected_group: "All".to_string(),
            m3u_url_input: String::new(),
        }
    }
}

impl IptvGuideDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("📺 IPTV Channel Guide & EPG Browser (Ctrl+I)")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(820.0, 540.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("IPTV Digital Broadcast & EPG Program Guide")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(120, 220, 180)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("📂 Load M3U Playlist").clicked() {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("M3U Playlist", &["m3u", "m3u8"])
                                    .pick_file()
                                {
                                    self.load_m3u_file(&path);
                                }
                            }
                        });
                    });

                    ui.separator();

                    // Search & Category Bar
                    ui.horizontal(|ui| {
                        ui.label("🔍 Search:");
                        ui.add(egui::TextEdit::singleline(&mut self.search_query).desired_width(180.0));

                        ui.add_space(8.0);
                        ui.label("Category:");
                        egui::ComboBox::from_id_salt("iptv_category_combo")
                            .selected_text(&self.selected_group)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.selected_group, "All".to_string(), "All Channels");
                                ui.selectable_value(&mut self.selected_group, "News".to_string(), "News");
                                ui.selectable_value(&mut self.selected_group, "Sports".to_string(), "Sports");
                                ui.selectable_value(&mut self.selected_group, "Documentary".to_string(), "Documentary");
                                ui.selectable_value(&mut self.selected_group, "Movies".to_string(), "Movies");
                                ui.selectable_value(&mut self.selected_group, "Entertainment".to_string(), "Entertainment");
                            });
                    });

                    ui.add_space(6.0);

                    // Main split area: Channel list (left) & EPG details (right)
                    ui.columns(2, |cols| {
                        // Left Column: Channel List
                        cols[0].vertical(|ui| {
                            ui.label(RichText::new("Channels").strong());
                            ui.add_space(2.0);

                            ScrollArea::vertical()
                                .max_height(380.0)
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    let query = self.search_query.to_lowercase();
                                    for (idx, ch) in self.channels.iter().enumerate() {
                                        if self.selected_group != "All" && ch.group != self.selected_group {
                                            continue;
                                        }
                                        if !query.is_empty() && !ch.name.to_lowercase().contains(&query) {
                                            continue;
                                        }

                                        let is_selected = self.selected_channel_idx == Some(idx);
                                        let bg = if is_selected {
                                            Color32::from_rgb(40, 70, 60)
                                        } else if idx % 2 == 0 {
                                            Color32::from_rgb(25, 27, 30)
                                        } else {
                                            Color32::from_rgb(20, 22, 25)
                                        };

                                        egui::Frame::new()
                                            .fill(bg)
                                            .inner_margin(egui::Margin::symmetric(8, 6))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(RichText::new(format!("{:02}.", idx + 1)).color(Color32::GRAY).small());
                                                    if ui.selectable_label(is_selected, &ch.name).clicked() {
                                                        self.selected_channel_idx = Some(idx);
                                                    }
                                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                        ui.label(RichText::new(&ch.group).small().color(Color32::from_rgb(140, 180, 160)));
                                                    });
                                                });
                                            });
                                        ui.add_space(2.0);
                                    }
                                });
                        });

                        // Right Column: EPG Schedule Viewer
                        cols[1].vertical(|ui| {
                            ui.label(RichText::new("EPG Electronic Program Schedule").strong());
                            ui.add_space(4.0);

                            if let Some(idx) = self.selected_channel_idx {
                                if let Some(ch) = self.channels.get(idx) {
                                    // Channel Banner
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(&ch.name).strong().size(16.0).color(Color32::WHITE));
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if ui.add(egui::Button::new(RichText::new("▶ Tune Channel").strong().color(Color32::WHITE)).fill(Color32::from_rgb(30, 130, 80))).clicked() {
                                                    player.load_file(&ch.url);
                                                    player.play();
                                                }
                                            });
                                        });
                                        ui.label(RichText::new(&ch.url).small().color(Color32::GRAY));
                                    });


                                    ui.add_space(8.0);

                                    // NOW Playing
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("🔴 LIVE NOW").strong().color(Color32::from_rgb(255, 90, 90)));
                                            ui.label(format!("{} - {}", ch.epg_now.start_time, ch.epg_now.end_time));
                                        });
                                        ui.label(RichText::new(&ch.epg_now.title).strong().size(14.0).color(Color32::from_rgb(255, 230, 120)));
                                        ui.label(RichText::new(&ch.epg_now.description).small().color(Color32::from_rgb(180, 180, 190)));
                                        ui.add_space(4.0);
                                        ui.add(ProgressBar::new(ch.epg_now.progress).text("Elapsed"));
                                    });

                                    ui.add_space(6.0);

                                    // NEXT Playing
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("⏳ UP NEXT").strong().color(Color32::from_rgb(120, 180, 255)));
                                            ui.label(format!("{} - {}", ch.epg_next.start_time, ch.epg_next.end_time));
                                        });
                                        ui.label(RichText::new(&ch.epg_next.title).strong().size(13.0).color(Color32::WHITE));
                                        ui.label(RichText::new(&ch.epg_next.description).small().color(Color32::from_rgb(160, 160, 170)));
                                    });
                                }
                            } else {
                                ui.label("Select a channel to view its program guide.");
                            }
                        });
                    });

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!("Total IPTV Channels: {}", self.channels.len()));
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

    fn load_m3u_file(&mut self, path: &PathBuf) {
        if let Ok(content) = std::fs::read_to_string(path) {
            let mut new_channels = Vec::new();
            let mut current_name = String::new();
            let mut current_group = "General".to_string();

            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("#EXTINF:") {
                    if let Some(comma) = trimmed.rfind(',') {
                        current_name = trimmed[comma + 1..].trim().to_string();
                    }
                    if let Some(grp) = trimmed.find("group-title=\"") {
                        let rem = &trimmed[grp + 13..];
                        if let Some(end) = rem.find('\"') {
                            current_group = rem[..end].to_string();
                        }
                    }
                } else if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    new_channels.push(ChannelItem {
                        name: if current_name.is_empty() { "IPTV Stream".to_string() } else { current_name.clone() },
                        group: current_group.clone(),
                        url: trimmed.to_string(),
                        logo: None,
                        epg_now: EpgProgram {
                            title: "Live Broadcast".to_string(),
                            description: "Live IPTV stream transmission.".to_string(),
                            start_time: "00:00".to_string(),
                            end_time: "23:59".to_string(),
                            progress: 0.5,
                        },
                        epg_next: EpgProgram {
                            title: "Scheduled Program".to_string(),
                            description: "Upcoming broadcast feed.".to_string(),
                            start_time: "00:00".to_string(),
                            end_time: "00:00".to_string(),
                            progress: 0.0,
                        },
                    });
                    current_name.clear();
                    current_group = "General".to_string();
                }
            }

            if !new_channels.is_empty() {
                self.channels = new_channels;
                self.selected_channel_idx = Some(0);
            }
        }
    }
}
