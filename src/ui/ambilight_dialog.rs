//! ambilight_dialog.rs — Ambient Room Lighting & Razer Chroma RGB Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightingDevice {
    RazerChroma,
    PhilipsHue,
    Nanoleaf,
    CorsairIcue,
    ScreenBorderGlow,
}

impl LightingDevice {
    pub fn display_name(&self) -> &'static str {
        match self {
            LightingDevice::RazerChroma => "Razer Chroma RGB (Keyboard, Mouse, Light Strip)",
            LightingDevice::PhilipsHue => "Philips Hue Bridge (Living Room Ambient Bulbs)",
            LightingDevice::Nanoleaf => "Nanoleaf Aurora / Shapes (Wall Panels)",
            LightingDevice::CorsairIcue => "Corsair iCUE SDK Peripherals",
            LightingDevice::ScreenBorderGlow => "On-Screen Dynamic Glow Halo",
        }
    }
}

pub struct AmbilightDialog {
    pub is_open: bool,
    pub is_enabled: bool,
    pub selected_device: LightingDevice,
    pub brightness_boost: f32, // 0.5 to 2.0
    pub saturation_boost: f32, // 0.5 to 2.0
    pub smoothing_ms: u32,     // 20ms to 200ms
    pub edge_sample_depth: u32,// 5% to 25% of screen border
    pub top_color: Color32,
    pub bottom_color: Color32,
    pub left_color: Color32,
    pub right_color: Color32,
    pub status_message: String,
}

impl Default for AmbilightDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_enabled: true,
            selected_device: LightingDevice::RazerChroma,
            brightness_boost: 1.25,
            saturation_boost: 1.40,
            smoothing_ms: 60,
            edge_sample_depth: 10,
            top_color: Color32::from_rgb(180, 70, 20),
            bottom_color: Color32::from_rgb(20, 60, 140),
            left_color: Color32::from_rgb(120, 40, 100),
            right_color: Color32::from_rgb(40, 140, 100),
            status_message: "Razer Chroma SDK & Philips Hue Ambilight connected.".to_string(),
        }
    }
}

impl AmbilightDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("💡 Ambilight & Razer Chroma RGB Lighting Studio")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(700.0, 500.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Smart Ambilight & RGB Peripheral Sync")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(255, 140, 220)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.is_enabled {
                                ui.label(RichText::new("● LIGHTING ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                            } else {
                                ui.label(RichText::new("● DISABLED").color(Color32::GRAY).strong());
                            }
                        });
                    });

                    ui.separator();

                    ui.group(|ui| {
                        ui.checkbox(&mut self.is_enabled, "💡 Enable Real-Time Edge Video Screen Ambilight Sync");
                        ui.horizontal(|ui| {
                            ui.label("Hardware Provider:");
                            egui::ComboBox::from_id_salt("ambilight_provider_combo")
                                .selected_text(self.selected_device.display_name())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.selected_device, LightingDevice::RazerChroma, LightingDevice::RazerChroma.display_name());
                                    ui.selectable_value(&mut self.selected_device, LightingDevice::PhilipsHue, LightingDevice::PhilipsHue.display_name());
                                    ui.selectable_value(&mut self.selected_device, LightingDevice::Nanoleaf, LightingDevice::Nanoleaf.display_name());
                                    ui.selectable_value(&mut self.selected_device, LightingDevice::CorsairIcue, LightingDevice::CorsairIcue.display_name());
                                    ui.selectable_value(&mut self.selected_device, LightingDevice::ScreenBorderGlow, LightingDevice::ScreenBorderGlow.display_name());
                                });
                        });
                    });

                    ui.add_space(8.0);

                    // 4-Quadrant Color Preview Box
                    ui.group(|ui| {
                        ui.label(RichText::new("Live 4-Quadrant Ambient Color Sampling:").strong());
                        ui.vertical_centered(|ui| {
                            // Top Bar
                            egui::Frame::new().fill(self.top_color).show(ui, |ui| {
                                ui.set_min_size(Vec2::new(300.0, 18.0));
                            });

                            ui.horizontal(|ui| {
                                ui.add_space(80.0);
                                // Left Bar
                                egui::Frame::new().fill(self.left_color).show(ui, |ui| {
                                    ui.set_min_size(Vec2::new(18.0, 100.0));
                                });

                                // Center Screen Mockup
                                egui::Frame::new().fill(Color32::from_rgb(10, 12, 16)).show(ui, |ui| {
                                    ui.set_min_size(Vec2::new(264.0, 100.0));
                                    ui.centered_and_justified(|ui| {
                                        ui.label(RichText::new("🎬 VIDEO SURFACE").color(Color32::GRAY));
                                    });
                                });

                                // Right Bar
                                egui::Frame::new().fill(self.right_color).show(ui, |ui| {
                                    ui.set_min_size(Vec2::new(18.0, 100.0));
                                });
                            });

                            // Bottom Bar
                            egui::Frame::new().fill(self.bottom_color).show(ui, |ui| {
                                ui.set_min_size(Vec2::new(300.0, 18.0));
                            });
                        });
                    });

                    ui.add_space(8.0);

                    // Color Calibration Tuning
                    ui.group(|ui| {
                        ui.label(RichText::new("Ambilight Calibration & Responsiveness").strong());
                        ui.horizontal(|ui| {
                            ui.label("Brightness Boost:");
                            ui.add(Slider::new(&mut self.brightness_boost, 0.5..=2.5).suffix("x"));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Vibrancy / Saturation:");
                            ui.add(Slider::new(&mut self.saturation_boost, 0.5..=2.5).suffix("x"));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Color Transition Smoothing:");
                            ui.add(Slider::new(&mut self.smoothing_ms, 10..=200).suffix(" ms"));
                        });
                    });

                    ui.add_space(4.0);
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
