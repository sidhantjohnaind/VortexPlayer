#![allow(dead_code)]

use eframe::egui::{self, Color32, CornerRadius, RichText, Vec2};
use std::process::Command;
use std::thread;

pub struct ContactSheetDialog {
    pub is_open: bool,
    pub columns: u32,
    pub rows: u32,
    pub include_header: bool,
    pub is_generating: bool,
    pub status_msg: String,
}

impl Default for ContactSheetDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            columns: 4,
            rows: 4,
            include_header: true,
            is_generating: false,
            status_msg: String::new(),
        }
    }
}

impl ContactSheetDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, stats: &crate::engine::MediaStats) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🖼️ Storyboard Contact Sheet Generator")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_size(Vec2::new(380.0, 260.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Export Video Storyboard Grid").strong().color(Color32::from_rgb(180, 140, 255)));
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label("Grid Columns:");
                        ui.add(egui::Slider::new(&mut self.columns, 2..=8));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Grid Rows:");
                        ui.add(egui::Slider::new(&mut self.rows, 2..=8));
                    });

                    ui.checkbox(&mut self.include_header, "Include Video Metadata Header");

                    ui.add_space(8.0);
                    let total_tiles = self.columns * self.rows;
                    ui.label(format!("Total Snapshots: {} tiles", total_tiles));

                    if !stats.file_path.is_empty() {
                        ui.label(format!("Duration: {:.1}s  |  Resolution: {}x{}", stats.duration, stats.video_width, stats.video_height));
                    }

                    ui.add_space(10.0);

                    if self.is_generating {
                        ui.label(RichText::new("⏳ Generating Contact Sheet in background...").color(Color32::from_rgb(255, 190, 80)));
                    } else if ui.add(egui::Button::new(RichText::new("📸 Generate & Save Contact Sheet").color(Color32::WHITE)).fill(Color32::from_rgb(80, 40, 160))).clicked() {
                        if !stats.file_path.is_empty() && stats.duration > 0.0 {
                            let out_dir = dirs::picture_dir()
                                .or_else(dirs::video_dir)
                                .unwrap_or_else(|| std::path::PathBuf::from("."));
                            let clean_name = std::path::Path::new(&stats.file_path)
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("Storyboard");
                            let out_file = out_dir.join(format!("{}_Storyboard_{}x{}.jpg", clean_name, self.columns, self.rows));
                            let out_path = out_file.to_string_lossy().to_string();
                            let in_path = stats.file_path.clone();
                            let cols = self.columns;
                            let rows = self.rows;
                            let total = total_tiles as f64;
                            let dur = stats.duration;
                            let interval = (dur / total).max(0.5);

                            self.is_generating = true;
                            self.status_msg = format!("Generating {} tiles to {}", total_tiles, out_file.file_name().unwrap_or_default().to_string_lossy());

                            thread::spawn(move || {
                                let vf = format!("fps=1/{:.2},scale=400:-1,tile={}x{}:padding=4:color=black", interval, cols, rows);
                                let _ = crate::platform::silent_command("ffmpeg")
                                    .args(&[
                                        "-i", &in_path,
                                        "-vf", &vf,
                                        "-frames:v", "1",
                                        "-q:v", "2",
                                        "-y", &out_path,
                                    ])
                                    .output();
                            });
                        } else {
                            self.status_msg = "Please open a video file first!".to_string();
                        }
                    }

                    if !self.status_msg.is_empty() {
                        ui.add_space(6.0);
                        ui.label(RichText::new(&self.status_msg).color(Color32::from_rgb(100, 220, 140)));
                    }
                });
            });
        self.is_open = open;
    }
}
