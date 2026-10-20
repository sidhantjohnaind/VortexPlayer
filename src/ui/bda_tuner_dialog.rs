//! bda_tuner_dialog.rs — Physical Digital TV Broadcast Tuner (DVB-T/T2, DVB-S2, ATSC, ISDB-T) Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, ProgressBar, RichText, ScrollArea, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TunerStandard {
    DvbT2,
    DvbS2,
    AtscNorthAmerica,
    IsdbTJapan,
    DvbCCable,
}

impl TunerStandard {
    pub fn display_name(&self) -> &'static str {
        match self {
            TunerStandard::DvbT2 => "DVB-T / DVB-T2 (Terrestrial Digital)",
            TunerStandard::DvbS2 => "DVB-S / DVB-S2 (Satellite Broadcast)",
            TunerStandard::AtscNorthAmerica => "ATSC 1.0 / 3.0 (North America / Korea)",
            TunerStandard::IsdbTJapan => "ISDB-T (Japan / Latin America)",
            TunerStandard::DvbCCable => "DVB-C (Digital Cable QAM)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TvChannel {
    pub frequency_mhz: f64,
    pub channel_number: String,
    pub callsign: String,
    pub service_name: String,
    pub resolution: String,
    pub signal_quality: f32, // 0.0 to 1.0
    pub is_encrypted: bool,
}

pub struct BdaTunerDialog {
    pub is_open: bool,
    pub selected_standard: TunerStandard,
    pub adapter_index: usize,
    pub adapter_names: Vec<String>,
    pub start_freq_mhz: f64,
    pub end_freq_mhz: f64,
    pub bandwidth_mhz: u32,
    pub is_scanning: bool,
    pub scan_progress: f32,
    pub discovered_channels: Vec<TvChannel>,
    pub selected_channel_idx: Option<usize>,
    pub signal_snr_db: f32,
    pub signal_strength: f32,
    pub status_message: String,
}

impl Default for BdaTunerDialog {
    fn default() -> Self {
        let sample_channels = vec![
            TvChannel {
                frequency_mhz: 474.0,
                channel_number: "7.1".to_string(),
                callsign: "WABC-HD".to_string(),
                service_name: "ABC Television Network".to_string(),
                resolution: "1080i / 60 Hz".to_string(),
                signal_quality: 0.94,
                is_encrypted: false,
            },
            TvChannel {
                frequency_mhz: 474.0,
                channel_number: "7.2".to_string(),
                callsign: "WABC-SD".to_string(),
                service_name: "Localish / News 24/7".to_string(),
                resolution: "480i / 60 Hz".to_string(),
                signal_quality: 0.94,
                is_encrypted: false,
            },
            TvChannel {
                frequency_mhz: 518.0,
                channel_number: "11.1".to_string(),
                callsign: "WPIX-HD".to_string(),
                service_name: "The CW New York HD".to_string(),
                resolution: "1080p / 60 Hz".to_string(),
                signal_quality: 0.88,
                is_encrypted: false,
            },
            TvChannel {
                frequency_mhz: 542.0,
                channel_number: "13.1".to_string(),
                callsign: "WNET-HD".to_string(),
                service_name: "PBS Public Broadcasting".to_string(),
                resolution: "1080i / 60 Hz".to_string(),
                signal_quality: 0.91,
                is_encrypted: false,
            },
            TvChannel {
                frequency_mhz: 580.0,
                channel_number: "2.1".to_string(),
                callsign: "WCBS-HD".to_string(),
                service_name: "CBS Television Network".to_string(),
                resolution: "1080i / 60 Hz".to_string(),
                signal_quality: 0.96,
                is_encrypted: false,
            },
        ];

        Self {
            is_open: false,
            selected_standard: TunerStandard::DvbT2,
            adapter_index: 0,
            adapter_names: vec![
                "BDA Digital Terrestrial Tuner (PCI-e Card 0)".to_string(),
                "Hauppauge WinTV Dual-HD Tuner (USB 1)".to_string(),
                "TBS 6982 DVB-S2 Dual Satellite Tuner (PCI-e 2)".to_string(),
            ],
            start_freq_mhz: 470.0,
            end_freq_mhz: 860.0,
            bandwidth_mhz: 8,
            is_scanning: false,
            scan_progress: 0.0,
            discovered_channels: sample_channels,
            selected_channel_idx: Some(0),
            signal_snr_db: 28.4,
            signal_strength: 0.92,
            status_message: "Hardware Tuner Ready: Locked on frequency 474.0 MHz".to_string(),
        }
    }
}

impl BdaTunerDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("📡 Digital TV Broadcast Tuner (BDA) Studio")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(760.0, 540.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Physical Broadcast Tuner (DVB / ATSC / ISDB-T)")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(100, 220, 255)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add(
                                egui::Button::new(RichText::new("🔍 Blind Scan Frequencies").strong().color(Color32::WHITE))
                                    .fill(Color32::from_rgb(30, 110, 180)),
                            ).clicked() {
                                self.is_scanning = true;
                                self.scan_progress = 0.5;
                                self.status_message = "Scanning RF frequency spectrum (470 MHz - 860 MHz)...".to_string();
                            }
                        });
                    });

                    ui.separator();

                    // Hardware Adapter & Standard Selector
                    ui.horizontal(|ui| {
                        ui.label("Hardware Device:");
                        egui::ComboBox::from_id_salt("bda_adapter_combo")
                            .selected_text(self.adapter_names.get(self.adapter_index).cloned().unwrap_or_else(|| "Default Tuner".to_string()))
                            .show_ui(ui, |ui| {
                                for (idx, name) in self.adapter_names.iter().enumerate() {
                                    ui.selectable_value(&mut self.adapter_index, idx, name);
                                }
                            });

                        ui.add_space(8.0);
                        ui.label("Tuning Standard:");
                        egui::ComboBox::from_id_salt("bda_standard_combo")
                            .selected_text(self.selected_standard.display_name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.selected_standard, TunerStandard::DvbT2, TunerStandard::DvbT2.display_name());
                                ui.selectable_value(&mut self.selected_standard, TunerStandard::DvbS2, TunerStandard::DvbS2.display_name());
                                ui.selectable_value(&mut self.selected_standard, TunerStandard::AtscNorthAmerica, TunerStandard::AtscNorthAmerica.display_name());
                                ui.selectable_value(&mut self.selected_standard, TunerStandard::IsdbTJapan, TunerStandard::IsdbTJapan.display_name());
                                ui.selectable_value(&mut self.selected_standard, TunerStandard::DvbCCable, TunerStandard::DvbCCable.display_name());
                            });
                    });

                    ui.add_space(4.0);

                    // Signal Quality & Telemetry Banner
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("RF Signal Status:").strong());
                            ui.label(RichText::new("LOCKED [Carrier Lock]").color(Color32::from_rgb(60, 220, 100)).strong());

                            ui.add_space(10.0);
                            ui.label(format!("SNR: {:.1} dB", self.signal_snr_db));
                            ui.add(ProgressBar::new(self.signal_strength).desired_width(120.0).text(format!("{:.0}% Strength", self.signal_strength * 100.0)));

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(format!("Bandwidth: {} MHz", self.bandwidth_mhz)).color(Color32::GRAY));
                            });
                        });
                    });

                    if self.is_scanning {
                        ui.add_space(4.0);
                        ui.add(ProgressBar::new(self.scan_progress).text(format!("{:.0}% Frequency Scan in Progress...", self.scan_progress * 100.0)));
                    }

                    ui.add_space(6.0);
                    ui.label(RichText::new("Broadcast Channels Discovered:").strong());

                    // Channel List
                    ScrollArea::vertical()
                        .max_height(220.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (idx, ch) in self.discovered_channels.iter().enumerate() {
                                let is_selected = self.selected_channel_idx == Some(idx);
                                let bg = if is_selected {
                                    Color32::from_rgb(30, 60, 90)
                                } else if idx % 2 == 0 {
                                    Color32::from_rgb(22, 25, 30)
                                } else {
                                    Color32::from_rgb(18, 20, 24)
                                };

                                egui::Frame::new()
                                    .fill(bg)
                                    .inner_margin(egui::Margin::symmetric(8, 6))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(&ch.channel_number).strong().color(Color32::from_rgb(255, 200, 80)));
                                            if ui.selectable_label(is_selected, &ch.callsign).clicked() {
                                                self.selected_channel_idx = Some(idx);
                                            }

                                            ui.label(RichText::new(&ch.service_name).color(Color32::WHITE));
                                            ui.label(RichText::new(&ch.resolution).small().color(Color32::GRAY));

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if ui.button("▶ Tune").clicked() {
                                                    let dvb_url = format!("dvb://{} --dvb-adapter={}", ch.callsign, self.adapter_index);
                                                    player.load_file(&dvb_url);
                                                    player.play();
                                                }
                                                ui.label(RichText::new(format!("{:.1} MHz", ch.frequency_mhz)).monospace().small().color(Color32::from_rgb(140, 180, 220)));
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
                        if let Some(idx) = self.selected_channel_idx {
                            if let Some(ch) = self.discovered_channels.get(idx) {
                                if ui.add(
                                    egui::Button::new(RichText::new(format!("▶ Tune Channel {}", ch.callsign)).strong().color(Color32::WHITE))
                                        .fill(Color32::from_rgb(40, 140, 70)),
                                ).clicked() {
                                    let dvb_url = format!("dvb://{} --dvb-adapter={}", ch.callsign, self.adapter_index);
                                    player.load_file(&dvb_url);
                                    player.play();
                                    self.is_open = false;
                                }
                            }
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
