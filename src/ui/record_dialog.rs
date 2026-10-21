#![allow(dead_code)]

use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use std::time::Instant;

pub struct StreamRecordDialog {
    pub is_open: bool,
    pub is_recording: bool,
    pub output_path: String,
    pub start_time: Option<Instant>,
}

impl Default for StreamRecordDialog {
    fn default() -> Self {
        let default_path = dirs::video_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("Vortex_Stream_Capture.mkv")
            .to_string_lossy()
            .to_string();

        Self {
            is_open: false,
            is_recording: false,
            output_path: default_path,
            start_time: None,
        }
    }
}

impl StreamRecordDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("📼 Live Stream / Video Recorder")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_size(Vec2::new(380.0, 180.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(4.0);
                    ui.label(RichText::new("Lossless Stream Capture / Video Snipper").strong().color(Color32::from_rgb(180, 140, 255)));
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label("Output File:");
                        ui.text_edit_singleline(&mut self.output_path);
                        if ui.button("📁 Browse...").clicked() {
                            if let Some(file) = rfd::FileDialog::new()
                                .add_filter("Video", &["mkv", "mp4", "ts"])
                                .set_file_name("Vortex_Stream_Capture.mkv")
                                .save_file()
                            {
                                self.output_path = file.to_string_lossy().to_string();
                            }
                        }
                    });

                    ui.add_space(8.0);

                    if self.is_recording {
                        let elapsed = self.start_time.map(|t| t.elapsed().as_secs()).unwrap_or(0);
                        let mins = elapsed / 60;
                        let secs = elapsed % 60;

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("● RECORDING").color(Color32::from_rgb(255, 60, 60)).strong());
                            ui.label(RichText::new(format!("{:02}:{:02}", mins, secs)).monospace().strong());
                        });

                        ui.add_space(8.0);
                        if ui.add(egui::Button::new(RichText::new("⏹ Stop Recording").color(Color32::WHITE)).fill(Color32::from_rgb(180, 40, 40))).clicked() {
                            player.stop_stream_recording();
                            self.is_recording = false;
                            self.start_time = None;
                        }
                    } else {
                        ui.label(RichText::new("Ready to record active video / audio stream").color(Color32::from_rgb(140, 140, 160)));
                        ui.add_space(8.0);

                        if ui.add(egui::Button::new(RichText::new("⏺ Start Lossless Recording").color(Color32::WHITE)).fill(Color32::from_rgb(50, 140, 60))).clicked() {
                            player.start_stream_recording(&self.output_path);
                            self.is_recording = true;
                            self.start_time = Some(Instant::now());
                        }
                    }
                });
            });
        self.is_open = open;
    }
}
