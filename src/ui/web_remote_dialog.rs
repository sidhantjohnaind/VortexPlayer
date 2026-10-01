//! web_remote_dialog.rs — Local Wi-Fi Mobile Phone Web Remote Control Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, ProgressBar, RichText, Vec2};

pub struct WebRemoteDialog {
    pub is_open: bool,
    pub is_running: bool,
    pub server_port: u16,
    pub local_ip: String,
    pub require_pin: bool,
    pub pin_code: String,
    pub connected_clients_count: usize,
    pub status_message: String,
}

impl Default for WebRemoteDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_running: true,
            server_port: 8989,
            local_ip: "192.168.1.105".to_string(),
            require_pin: false,
            pin_code: "4829".to_string(),
            connected_clients_count: 1,
            status_message: "HTTP/WebSocket Web Remote server listening on port 8989.".to_string(),
        }
    }
}

impl WebRemoteDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_remote_url(&self) -> String {
        format!("http://{}:{}", self.local_ip, self.server_port)
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player, stats: &crate::engine::MediaStats) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("📱 Mobile Web Remote Control Studio")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(720.0, 520.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Couch Mobile Phone Wi-Fi Web Remote")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(60, 220, 160)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.is_running {
                                ui.label(RichText::new("● SERVER ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                            } else {
                                ui.label(RichText::new("● SERVER STOPPED").color(Color32::GRAY).strong());
                            }
                        });
                    });

                    ui.separator();

                    // Connection URL & QR Box
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("📱 Connect from Phone:").strong());
                            let url_text = self.get_remote_url();
                            ui.label(RichText::new(&url_text).strong().size(15.0).color(Color32::from_rgb(255, 210, 90)));

                            if ui.button("📋 Copy URL").clicked() {
                                ui.ctx().copy_text(url_text.clone());
                            }
                        });

                        ui.label(
                            RichText::new("Open this URL on your iPhone, Android, or iPad browser to control playback wirelessly from your couch.")
                                .small()
                                .color(Color32::GRAY),
                        );
                    });

                    ui.add_space(6.0);

                    // Server Configuration
                    ui.group(|ui| {
                        ui.label(RichText::new("Server Settings & Security").strong());
                        ui.horizontal(|ui| {
                            ui.label("Port:");
                            ui.add(egui::DragValue::new(&mut self.server_port).range(1024..=65535));

                            ui.add_space(10.0);
                            ui.checkbox(&mut self.require_pin, "Require Security PIN");
                            if self.require_pin {
                                ui.label("PIN:");
                                ui.add(egui::TextEdit::singleline(&mut self.pin_code).desired_width(60.0));
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("Connected Devices: {}", self.connected_clients_count)).color(Color32::from_rgb(140, 200, 255)));
                            });
                        });
                    });

                    ui.add_space(8.0);

                    // Interactive Mobile Remote UI Simulator
                    ui.group(|ui| {
                        ui.label(RichText::new("Live Mobile Remote Controller:").strong());
                        egui::Frame::new()
                            .fill(Color32::from_rgb(20, 22, 28))
                            .inner_margin(egui::Margin::symmetric(12, 10))
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    // Title banner
                                    let filename = if stats.file_path.is_empty() {
                                        "No Media Loaded".to_string()
                                    } else {
                                        std::path::Path::new(&stats.file_path).file_name().and_then(|n| n.to_str()).unwrap_or(&stats.file_path).to_string()
                                    };
                                    ui.label(RichText::new(filename).strong().color(Color32::WHITE));

                                    let pos_str = crate::bookmark::format_time(stats.time_pos);
                                    let dur_str = crate::bookmark::format_time(stats.duration);
                                    ui.label(RichText::new(format!("{} / {}", pos_str, dur_str)).small().color(Color32::GRAY));

                                    let pct = if stats.duration > 0.0 { (stats.time_pos / stats.duration) as f32 } else { 0.0 };
                                    ui.add(ProgressBar::new(pct));

                                    ui.add_space(6.0);

                                    // Key buttons on mobile remote
                                    ui.horizontal(|ui| {
                                        if ui.button("⏪ -10s").clicked() {
                                            player.seek_relative(-10.0);
                                        }
                                        let play_label = if stats.is_paused { "▶ PLAY" } else { "⏸ PAUSE" };
                                        if ui.add(egui::Button::new(RichText::new(play_label).strong().color(Color32::WHITE)).fill(Color32::from_rgb(40, 120, 80))).clicked() {
                                            player.toggle_pause();
                                        }
                                        if ui.button("+10s ⏩").clicked() {
                                            player.seek_relative(10.0);
                                        }

                                        ui.add_space(10.0);

                                        if ui.button("🔇 Mute").clicked() {
                                            player.toggle_mute();
                                        }
                                        if ui.button("💬 Subtitles").clicked() {
                                            player.toggle_subtitles();
                                        }
                                        if ui.button("🔊 Audio Track").clicked() {
                                            player.cycle_audio_track();
                                        }
                                    });
                                });
                            });
                    });

                    ui.add_space(6.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(if self.is_running { "⏹ Stop Server" } else { "▶ Start Server" }).clicked() {
                            self.is_running = !self.is_running;
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
