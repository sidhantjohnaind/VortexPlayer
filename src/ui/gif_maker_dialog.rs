#![allow(dead_code)]

use eframe::egui::{self, Color32, CornerRadius, RichText, Vec2};
use std::process::Command;

pub struct GifMakerDialog {
    pub is_open: bool,
    pub start_time: f64,
    pub end_time: f64,
    pub fps: u32,
    pub scale_width: u32,
    pub output_format: String, // "GIF" or "WebP" or "Lossless MP4"
    pub burn_subtitles: bool,
    pub status: String,
}

impl Default for GifMakerDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            start_time: 0.0,
            end_time: 5.0,
            fps: 15,
            scale_width: 480,
            output_format: "GIF".to_string(),
            burn_subtitles: true,
            status: String::new(),
        }
    }
}

impl GifMakerDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, stats: &crate::engine::MediaStats) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🎞️ Animated GIF & Video Clip Maker (Ctrl+G)")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_size(Vec2::new(420.0, 320.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Export Animated GIF, WebP, or Lossless Video Cut").strong().color(Color32::from_rgb(180, 140, 255)));
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("Start Time (s):");
                        ui.add(egui::DragValue::new(&mut self.start_time).speed(0.5).range(0.0..=stats.duration));
                        if ui.button("📍 Set Current").clicked() {
                            self.start_time = stats.time_pos;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("End Time (s):");
                        ui.add(egui::DragValue::new(&mut self.end_time).speed(0.5).range(0.0..=stats.duration));
                        if ui.button("📍 Set Current").clicked() {
                            self.end_time = stats.time_pos;
                        }
                    });

                    let duration = (self.end_time - self.start_time).max(0.0);
                    ui.label(format!("Clip Duration: {:.2} seconds", duration));
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label("Export Format:");
                        egui::ComboBox::from_id_salt("gif_fmt")
                            .selected_text(&self.output_format)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.output_format, "GIF".to_string(), "Animated GIF");
                                ui.selectable_value(&mut self.output_format, "WebP".to_string(), "Animated WebP");
                                ui.selectable_value(&mut self.output_format, "Lossless MP4".to_string(), "Lossless Cut (MP4)");
                            });
                    });

                    if self.output_format != "Lossless MP4" {
                        ui.horizontal(|ui| {
                            ui.label("Framerate (FPS):");
                            ui.add(egui::Slider::new(&mut self.fps, 5..=30));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Width (px):");
                            ui.add(egui::Slider::new(&mut self.scale_width, 240..=1280));
                        });

                        ui.checkbox(&mut self.burn_subtitles, "Burn-in Active Subtitles");
                    }

                    ui.add_space(10.0);

                    if ui.add(egui::Button::new(RichText::new("🚀 Export Snippet / Clip").color(Color32::WHITE)).fill(Color32::from_rgb(90, 50, 180))).clicked() {
                        let out_dir = dirs::video_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
                        let ext = match self.output_format.as_str() {
                            "WebP" => "webp",
                            "Lossless MP4" => "mp4",
                            _ => "gif",
                        };
                        let out_file = out_dir.join(format!("Vortex_Clip_{}.{}", (self.start_time) as u64, ext));
                        let out_str = out_file.to_string_lossy().to_string();

                        if self.output_format == "Lossless MP4" {
                            let _ = crate::platform::silent_command("ffmpeg")
                                .args(&[
                                    "-ss", &format!("{:.2}", self.start_time),
                                    "-to", &format!("{:.2}", self.end_time),
                                    "-i", &stats.file_path,
                                    "-c", "copy",
                                    "-y", &out_str
                                ])
                                .spawn();
                        } else {
                            let vf = format!("fps={},scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse", self.fps, self.scale_width);
                            let _ = crate::platform::silent_command("ffmpeg")
                                .args(&[
                                    "-ss", &format!("{:.2}", self.start_time),
                                    "-to", &format!("{:.2}", self.end_time),
                                    "-i", &stats.file_path,
                                    "-vf", &vf,
                                    "-y", &out_str
                                ])
                                .spawn();
                        }
                        self.status = format!("Exporting to {}!", out_file.file_name().and_then(|n| n.to_str()).unwrap_or("clip"));
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
