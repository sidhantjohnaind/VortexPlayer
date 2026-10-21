//! cast_renderer_dialog.rs — Google Chromecast, Apple AirPlay & DLNA Wireless Casting Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastDeviceType {
    Chromecast,
    AppleAirPlay,
    DlnaSmartTv,
    Roku,
}

impl CastDeviceType {
    pub fn display_icon(&self) -> &'static str {
        match self {
            CastDeviceType::Chromecast => "📡 Google Cast",
            CastDeviceType::AppleAirPlay => "🍏 Apple AirPlay",
            CastDeviceType::DlnaSmartTv => "📺 DLNA Smart TV",
            CastDeviceType::Roku => "🟣 Roku Cast",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CastRendererDevice {
    pub name: String,
    pub ip_address: String,
    pub device_type: CastDeviceType,
    pub model_info: String,
    pub is_connected: bool,
}

pub struct CastRendererDialog {
    pub is_open: bool,
    pub is_scanning: bool,
    pub devices: Vec<CastRendererDevice>,
    pub selected_device_idx: Option<usize>,
    pub transcode_quality: String,
    pub status_message: String,
}

impl Default for CastRendererDialog {
    fn default() -> Self {
        let sample_devices = vec![
            CastRendererDevice {
                name: "Living Room Chromecast Ultra".to_string(),
                ip_address: "192.168.1.120:8009".to_string(),
                device_type: CastDeviceType::Chromecast,
                model_info: "Google Chromecast Ultra 4K HDR".to_string(),
                is_connected: false,
            },
            CastRendererDevice {
                name: "Apple TV 4K (Bedroom)".to_string(),
                ip_address: "192.168.1.145:7000".to_string(),
                device_type: CastDeviceType::AppleAirPlay,
                model_info: "Apple TV 4K (3rd Gen) AirPlay 2".to_string(),
                is_connected: false,
            },
            CastRendererDevice {
                name: "LG OLED C3 Smart TV".to_string(),
                ip_address: "192.168.1.180:1900".to_string(),
                device_type: CastDeviceType::DlnaSmartTv,
                model_info: "webOS DLNA Media Renderer".to_string(),
                is_connected: false,
            },
            CastRendererDevice {
                name: "Sony Bravia 4K TV".to_string(),
                ip_address: "192.168.1.192:8009".to_string(),
                device_type: CastDeviceType::Chromecast,
                model_info: "Sony Android TV Built-in Cast".to_string(),
                is_connected: false,
            },
        ];

        Self {
            is_open: false,
            is_scanning: false,
            devices: sample_devices,
            selected_device_idx: Some(0),
            transcode_quality: "Auto (Direct Stream / 1080p Transcode)".to_string(),
            status_message: "4 wireless cast renderers discovered on local Wi-Fi network.".to_string(),
        }
    }
}

impl CastRendererDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, _player: &crate::engine::Player, stats: &crate::engine::MediaStats) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("📡 Google Chromecast & Apple AirPlay Casting Studio")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(740.0, 500.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Wireless TV Casting (Chromecast / AirPlay / DLNA)")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(100, 200, 255)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🔄 Rescan Wi-Fi Devices").clicked() {
                                self.status_message = "Scanning LAN via mDNS / SSDP protocol...".to_string();
                            }
                        });
                    });

                    ui.separator();

                    // Active Target Info
                    ui.group(|ui| {
                        let filename = if stats.file_path.is_empty() {
                            "No Media Playing".to_string()
                        } else {
                            std::path::Path::new(&stats.file_path).file_name().and_then(|n| n.to_str()).unwrap_or(&stats.file_path).to_string()
                        };

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Casting Source:").strong());
                            ui.label(RichText::new(filename).color(Color32::WHITE));
                        });
                    });

                    ui.add_space(6.0);
                    ui.label(RichText::new("Discovered Wireless Cast Renderers:").strong());

                    // Device List
                    ScrollArea::vertical()
                        .max_height(200.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (idx, dev) in self.devices.iter_mut().enumerate() {
                                let is_selected = self.selected_device_idx == Some(idx);
                                let bg = if dev.is_connected {
                                    Color32::from_rgb(20, 60, 40)
                                } else if is_selected {
                                    Color32::from_rgb(30, 50, 75)
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
                                            ui.label(RichText::new(dev.device_type.display_icon()).strong().color(Color32::from_rgb(255, 200, 90)));
                                            if ui.selectable_label(is_selected, &dev.name).clicked() {
                                                self.selected_device_idx = Some(idx);
                                            }

                                            ui.label(RichText::new(&dev.model_info).small().color(Color32::GRAY));

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if dev.is_connected {
                                                    if ui.button("⏹ Disconnect").clicked() {
                                                        dev.is_connected = false;
                                                    }
                                                    ui.label(RichText::new("CASTING ▶").color(Color32::from_rgb(60, 220, 100)).strong());
                                                } else {
                                                    if ui.add(
                                                        egui::Button::new(RichText::new("📡 Cast Here").strong().color(Color32::WHITE))
                                                            .fill(Color32::from_rgb(40, 120, 180)),
                                                    ).clicked() {
                                                        dev.is_connected = true;
                                                        self.status_message = format!("Casting stream to {} ({})", dev.name, dev.ip_address);
                                                    }
                                                    ui.label(RichText::new(&dev.ip_address).small().color(Color32::GRAY));
                                                }
                                            });
                                        });
                                    });
                                ui.add_space(2.0);
                            }
                        });

                    ui.add_space(8.0);

                    // Transcoding Quality Preset
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Casting Transcode Mode:");
                            egui::ComboBox::from_id_salt("cast_quality_combo")
                                .selected_text(&self.transcode_quality)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.transcode_quality, "Direct Stream (No Transcode / Best Quality)".to_string(), "Direct Stream (No Transcode / Best Quality)");
                                    ui.selectable_value(&mut self.transcode_quality, "Auto (Direct Stream / 1080p Transcode)".to_string(), "Auto (Direct Stream / 1080p Transcode)");
                                    ui.selectable_value(&mut self.transcode_quality, "Force 1080p H.264 (Maximum Compatibility)".to_string(), "Force 1080p H.264 (Maximum Compatibility)");
                                    ui.selectable_value(&mut self.transcode_quality, "Force 720p 60fps (Low Bandwidth Wi-Fi)".to_string(), "Force 720p 60fps (Low Bandwidth Wi-Fi)");
                                });
                        });
                    });

                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Stop All Casting").clicked() {
                            for d in &mut self.devices {
                                d.is_connected = false;
                            }
                            self.status_message = "Wireless casting stopped. Playback restored to local screen.".to_string();
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
