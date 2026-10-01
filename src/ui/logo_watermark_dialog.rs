//! logo_watermark_dialog.rs — Custom Idle Logo & Video Watermark Overlay Studio Dialog

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatermarkPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

pub struct LogoWatermarkDialog {
    pub is_open: bool,
    pub watermark_enabled: bool,
    pub watermark_text: String,
    pub watermark_position: WatermarkPosition,
    pub watermark_opacity: f32,
    pub watermark_font_size: u32,
    pub watermark_color: Color32,

    pub custom_idle_logo_path: Option<PathBuf>,
}

impl Default for LogoWatermarkDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            watermark_enabled: false,
            watermark_text: "VORTEX PLAYER PRO".to_string(),
            watermark_position: WatermarkPosition::TopRight,
            watermark_opacity: 0.75,
            watermark_font_size: 24,
            watermark_color: Color32::from_rgb(255, 255, 255),
            custom_idle_logo_path: None,
        }
    }
}

impl LogoWatermarkDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🖼️ Custom Logo & Watermark Overlay Studio")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(460.0, 420.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Video Watermark & Idle Screen Customizer")
                            .strong()
                            .color(Color32::from_rgb(240, 200, 100)),
                    );
                    ui.label(
                        RichText::new("Overlay dynamic text/station logos onto video playback and configure custom idle backgrounds.")
                            .small()
                            .color(Color32::from_rgb(160, 160, 170)),
                    );
                    ui.add_space(8.0);

                    // ── Live Video Watermark Section ──
                    ui.group(|ui| {
                        ui.label(RichText::new("Live Video Watermark Overlay").strong());
                        ui.add_space(4.0);

                        if ui.checkbox(&mut self.watermark_enabled, "Overlay Video Watermark").changed() {
                            self.apply_watermark_filter(player);
                        }

                        ui.horizontal(|ui| {
                            ui.label("Watermark Text:");
                            if ui.add(egui::TextEdit::singleline(&mut self.watermark_text).desired_width(200.0)).changed() {
                                if self.watermark_enabled {
                                    self.apply_watermark_filter(player);
                                }
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Position:");
                            let prev_pos = self.watermark_position;
                            egui::ComboBox::from_id_salt("wm_pos_combo")
                                .selected_text(match self.watermark_position {
                                    WatermarkPosition::TopLeft => "Top-Left",
                                    WatermarkPosition::TopRight => "Top-Right",
                                    WatermarkPosition::BottomLeft => "Bottom-Left",
                                    WatermarkPosition::BottomRight => "Bottom-Right",
                                    WatermarkPosition::Center => "Center",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.watermark_position, WatermarkPosition::TopLeft, "Top-Left");
                                    ui.selectable_value(&mut self.watermark_position, WatermarkPosition::TopRight, "Top-Right");
                                    ui.selectable_value(&mut self.watermark_position, WatermarkPosition::BottomLeft, "Bottom-Left");
                                    ui.selectable_value(&mut self.watermark_position, WatermarkPosition::BottomRight, "Bottom-Right");
                                    ui.selectable_value(&mut self.watermark_position, WatermarkPosition::Center, "Center");
                                });

                            if prev_pos != self.watermark_position && self.watermark_enabled {
                                self.apply_watermark_filter(player);
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Opacity:");
                            if ui.add(Slider::new(&mut self.watermark_opacity, 0.1..=1.0).text("alpha")).changed() {
                                if self.watermark_enabled {
                                    self.apply_watermark_filter(player);
                                }
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Font Size:");
                            if ui.add(Slider::new(&mut self.watermark_font_size, 12..=72).text("pt")).changed() {
                                if self.watermark_enabled {
                                    self.apply_watermark_filter(player);
                                }
                            }
                        });
                    });

                    ui.add_space(8.0);

                    // ── Custom Idle Background Logo ──
                    ui.group(|ui| {
                        ui.label(RichText::new("Custom Idle Screen Logo").strong());
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            let label_str = match &self.custom_idle_logo_path {
                                Some(p) => p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "Selected".to_string()),
                                None => "Default Vortex Logo".to_string(),
                            };
                            ui.label(format!("Idle Logo: {}", label_str));

                            if ui.button("📁 Browse Image").clicked() {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("Image", &["png", "jpg", "jpeg", "webp", "bmp", "svg"])
                                    .pick_file()
                                {
                                    self.custom_idle_logo_path = Some(path);
                                }
                            }

                            if self.custom_idle_logo_path.is_some() {
                                if ui.button("Reset").clicked() {
                                    self.custom_idle_logo_path = None;
                                }
                            }
                        });
                    });

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("↺ Clear Overlays").clicked() {
                            self.watermark_enabled = false;
                            player.set_video_filter("");
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

    pub fn apply_watermark_filter(&self, player: &crate::engine::Player) {
        if !self.watermark_enabled || self.watermark_text.is_empty() {
            player.set_video_filter("");
            return;
        }

        let (x, y) = match self.watermark_position {
            WatermarkPosition::TopLeft => ("30", "30"),
            WatermarkPosition::TopRight => ("w-tw-30", "30"),
            WatermarkPosition::BottomLeft => ("30", "h-th-30"),
            WatermarkPosition::BottomRight => ("w-tw-30", "h-th-30"),
            WatermarkPosition::Center => ("(w-tw)/2", "(h-th)/2"),
        };

        let filter_str = format!(
            "drawtext=text='{}':x={}:y={}:fontsize={}:fontcolor=white@{:.2}:shadowcolor=black@0.6:shadowx=2:shadowy=2",
            self.watermark_text.replace('\'', "\\'"),
            x,
            y,
            self.watermark_font_size,
            self.watermark_opacity
        );

        player.set_video_filter(&filter_str);
    }
}
