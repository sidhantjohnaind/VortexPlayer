#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Vec2};
use std::process::{Child, Command};

pub struct BroadcastDialog {
    pub is_open: bool,
    pub rtmp_url: String,
    pub stream_key: String,
    pub video_bitrate_kbps: u32,
    pub is_broadcasting: bool,
    pub process: Option<Child>,
    pub status: String,
}

impl Default for BroadcastDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            rtmp_url: "rtmp://live.twitch.tv/app/".to_string(),
            stream_key: String::new(),
            video_bitrate_kbps: 6000,
            is_broadcasting: false,
            process: None,
            status: String::new(),
        }
    }
}

impl BroadcastDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, stats: &crate::engine::MediaStats) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("📡 Live RTMP Stream Broadcasting (Ctrl+B)")
            .open(&mut open)
            .collapsible(false)
            .default_size(Vec2::new(420.0, 260.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Broadcast Video to Twitch / YouTube Live / RTMP").strong().color(Color32::from_rgb(180, 140, 255)));
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("RTMP Server:");
                        ui.text_edit_singleline(&mut self.rtmp_url);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Stream Key:");
                        ui.add(egui::TextEdit::singleline(&mut self.stream_key).password(true));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Video Bitrate:");
                        ui.add(egui::Slider::new(&mut self.video_bitrate_kbps, 1500..=12000).suffix(" kbps"));
                    });

                    ui.add_space(10.0);

                    if self.is_broadcasting {
                        ui.label(RichText::new("● LIVE BROADCASTING").color(Color32::from_rgb(255, 60, 60)).strong());
                        ui.add_space(6.0);
                        if ui.add(egui::Button::new(RichText::new("⏹ Stop Live Stream").color(Color32::WHITE)).fill(Color32::from_rgb(180, 40, 40))).clicked() {
                            if let Some(mut child) = self.process.take() {
                                let _ = child.kill();
                            }
                            self.is_broadcasting = false;
                            self.status = "Broadcast stopped.".to_string();
                        }
                    } else {
                        if ui.add(egui::Button::new(RichText::new("🔴 Start Live Broadcast").color(Color32::WHITE)).fill(Color32::from_rgb(180, 40, 40))).clicked() {
                            if !self.stream_key.is_empty() && !stats.file_path.is_empty() {
                                let target_url = format!("{}{}", self.rtmp_url, self.stream_key);
                                let child = crate::platform::silent_command("ffmpeg")
                                    .args(&[
                                        "-re",
                                        "-ss", &format!("{:.1}", stats.time_pos),
                                        "-i", &stats.file_path,
                                        "-c:v", "libx264",
                                        "-preset", "veryfast",
                                        "-b:v", &format!("{}k", self.video_bitrate_kbps),
                                        "-c:a", "aac",
                                        "-b:a", "160k",
                                        "-f", "flv",
                                        &target_url
                                    ])
                                    .spawn();

                                match child {
                                    Ok(c) => {
                                        self.process = Some(c);
                                        self.is_broadcasting = true;
                                        self.status = "Live stream started!".to_string();
                                    }
                                    Err(e) => {
                                        self.status = format!("Failed to launch ffmpeg: {}", e);
                                    }
                                }
                            } else {
                                self.status = "Please provide a valid stream key and open media.".to_string();
                            }
                        }
                    }

                    if !self.status.is_empty() {
                        ui.add_space(6.0);
                        ui.label(RichText::new(&self.status).color(Color32::from_rgb(100, 220, 140)));
                    }
                });
            });
        self.is_open = open;
    }
}
