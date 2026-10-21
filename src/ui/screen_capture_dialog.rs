//! screen_capture_dialog.rs — Live Desktop & Screen Region Capture Input Source Dialog

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureSourceType {
    PrimaryMonitor,
    SecondaryMonitor,
    CustomRegion,
    ActiveWindow,
}

pub struct ScreenCaptureDialog {
    pub is_open: bool,
    pub source_type: CaptureSourceType,
    pub framerate: u32,
    pub capture_mouse: bool,
    pub region_x: u32,
    pub region_y: u32,
    pub region_w: u32,
    pub region_h: u32,
    pub window_title: String,
}

impl Default for ScreenCaptureDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            source_type: CaptureSourceType::PrimaryMonitor,
            framerate: 60,
            capture_mouse: true,
            region_x: 0,
            region_y: 0,
            region_w: 1920,
            region_h: 1080,
            window_title: String::new(),
        }
    }
}

impl ScreenCaptureDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🖥️ Screen & Desktop Live Capture (Ctrl+Shift+S)")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(450.0, 380.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Live Desktop & Window Screen Grabber")
                            .strong()
                            .color(Color32::from_rgb(100, 220, 255)),
                    );
                    ui.label(
                        RichText::new("Stream your Windows desktop, active application window, or custom coordinate region directly as a video source.")
                            .small()
                            .color(Color32::from_rgb(160, 160, 170)),
                    );
                    ui.add_space(8.0);

                    ui.group(|ui| {
                        ui.label(RichText::new("Capture Source").strong());
                        ui.add_space(4.0);

                        ui.radio_value(&mut self.source_type, CaptureSourceType::PrimaryMonitor, "🖥️ Entire Primary Desktop Screen");
                        ui.radio_value(&mut self.source_type, CaptureSourceType::SecondaryMonitor, "🖥️ Secondary Monitor (Display 2)");
                        ui.radio_value(&mut self.source_type, CaptureSourceType::CustomRegion, "📐 Custom Coordinate Crop Region (X, Y, W, H)");
                        ui.radio_value(&mut self.source_type, CaptureSourceType::ActiveWindow, "🪟 Specific Application Window by Title");
                    });

                    ui.add_space(6.0);

                    if self.source_type == CaptureSourceType::CustomRegion {
                        ui.group(|ui| {
                            ui.label(RichText::new("Region Coordinates").strong());
                            ui.horizontal(|ui| {
                                ui.label("X:");
                                ui.add(egui::DragValue::new(&mut self.region_x).range(0..=7680));
                                ui.label("Y:");
                                ui.add(egui::DragValue::new(&mut self.region_y).range(0..=4320));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Width:");
                                ui.add(egui::DragValue::new(&mut self.region_w).range(100..=7680));
                                ui.label("Height:");
                                ui.add(egui::DragValue::new(&mut self.region_h).range(100..=4320));
                            });
                        });
                        ui.add_space(6.0);
                    } else if self.source_type == CaptureSourceType::ActiveWindow {
                        ui.group(|ui| {
                            ui.label(RichText::new("Window Title").strong());
                            ui.horizontal(|ui| {
                                ui.label("Exact Title:");
                                ui.add(egui::TextEdit::singleline(&mut self.window_title).desired_width(220.0));
                            });
                        });
                        ui.add_space(6.0);
                    }

                    ui.group(|ui| {
                        ui.label(RichText::new("Capture Performance & Settings").strong());
                        ui.horizontal(|ui| {
                            ui.label("Target Frame Rate:");
                            egui::ComboBox::from_id_salt("screen_fps_combo")
                                .selected_text(format!("{} FPS", self.framerate))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.framerate, 30, "30 FPS (Standard)");
                                    ui.selectable_value(&mut self.framerate, 60, "60 FPS (Smooth)");
                                    ui.selectable_value(&mut self.framerate, 120, "120 FPS (High-Refresh)");
                                });
                        });

                        ui.checkbox(&mut self.capture_mouse, "🖱️ Draw Mouse Pointer in Live Stream");
                    });

                    ui.add_space(10.0);

                    if ui.add(
                        egui::Button::new(RichText::new("▶ Launch Live Desktop Stream").strong().color(Color32::WHITE))
                            .fill(Color32::from_rgb(40, 140, 80))
                            .min_size(Vec2::new(ui.available_width(), 32.0)),
                    ).clicked() {
                        let stream_uri = self.build_capture_url();
                        player.load_file(&stream_uri);
                        player.play();
                        self.is_open = false;
                    }


                    ui.add_space(4.0);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            self.is_open = false;
                        }
                    });
                });
            });

        self.is_open = open;
    }

    pub fn build_capture_url(&self) -> String {
        let draw_mouse_val = if self.capture_mouse { 1 } else { 0 };
        match self.source_type {
            CaptureSourceType::PrimaryMonitor => {
                format!("avdevice://gdigrab:desktop?framerate={}&draw_mouse={}", self.framerate, draw_mouse_val)
            }
            CaptureSourceType::SecondaryMonitor => {
                format!("avdevice://gdigrab:desktop?framerate={}&draw_mouse={}&offset_x=1920", self.framerate, draw_mouse_val)
            }
            CaptureSourceType::CustomRegion => {
                format!(
                    "avdevice://gdigrab:desktop?framerate={}&draw_mouse={}&offset_x={}&offset_y={}&video_size={}x{}",
                    self.framerate, draw_mouse_val, self.region_x, self.region_y, self.region_w, self.region_h
                )
            }
            CaptureSourceType::ActiveWindow => {
                let win = if self.window_title.is_empty() { "VortexPlayer" } else { &self.window_title };
                format!("avdevice://gdigrab:title={}&framerate={}&draw_mouse={}", win, self.framerate, draw_mouse_val)
            }
        }
    }
}
