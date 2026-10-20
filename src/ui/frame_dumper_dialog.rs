//! frame_dumper_dialog.rs — Continuous Burst Frame Dumper & Sequence Extractor Studio (Vortex style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameDumpInterval {
    EveryFrame,
    EveryKeyframe,
    EveryNSeconds,
    EveryNFrames,
}

impl FrameDumpInterval {
    pub fn display_name(&self) -> &'static str {
        match self {
            FrameDumpInterval::EveryFrame => "Every Video Frame (All Frames)",
            FrameDumpInterval::EveryKeyframe => "Keyframes / I-Frames Only",
            FrameDumpInterval::EveryNSeconds => "Interval in Seconds (Time Step)",
            FrameDumpInterval::EveryNFrames => "Interval in Frames (e.g. Every 10 Frames)",
        }
    }
}

pub struct FrameDumperDialog {
    pub is_open: bool,
    pub is_dumping: bool,
    pub output_directory: PathBuf,
    pub interval_mode: FrameDumpInterval,
    pub interval_seconds: f64,
    pub interval_frames: u32,
    pub file_format: String, // "png", "jpg", "webp"
    pub naming_pattern: String,
    pub max_frames_limit: u32, // 0 = unlimited
    pub frames_captured: u32,
    pub status_message: String,
}

impl Default for FrameDumperDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_dumping: false,
            output_directory: PathBuf::from("C:\\Captures"),
            interval_mode: FrameDumpInterval::EveryNSeconds,
            interval_seconds: 1.0,
            interval_frames: 30,
            file_format: "png".to_string(),
            naming_pattern: "Frame_%06d.png".to_string(),
            max_frames_limit: 500,
            frames_captured: 0,
            status_message: "Continuous frame dumper ready. Configure output directory and start capture.".to_string(),
        }
    }
}

impl FrameDumperDialog {
    pub fn new() -> Self { Self::default() }

    pub fn render(&mut self, ctx: &egui::Context, _player: &crate::engine::Player, stats: &crate::engine::MediaStats) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("📸 Continuous Burst Frame Dumper")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(620.0, 440.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Sequential Frame Dumper & Burst Image Extractor").strong().size(15.0).color(Color32::from_rgb(255, 200, 100)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_dumping {
                            ui.label(RichText::new(format!("● DUMPING ({} frames)", self.frames_captured)).color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● IDLE").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    ui.label(RichText::new("Output Folder:").strong());
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(self.output_directory.to_string_lossy()).small().color(Color32::GRAY));
                        if ui.button("Browse...").clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                self.output_directory = folder;
                            }
                        }
                    });
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Capture Interval & Cadence:").strong());
                    for mode in [
                        FrameDumpInterval::EveryNSeconds,
                        FrameDumpInterval::EveryNFrames,
                        FrameDumpInterval::EveryKeyframe,
                        FrameDumpInterval::EveryFrame,
                    ] {
                        if ui.selectable_label(self.interval_mode == mode, mode.display_name()).clicked() {
                            self.interval_mode = mode;
                        }
                    }

                    if self.interval_mode == FrameDumpInterval::EveryNSeconds {
                        ui.add(Slider::new(&mut self.interval_seconds, 0.1..=10.0).text("Interval").suffix(" s"));
                    } else if self.interval_mode == FrameDumpInterval::EveryNFrames {
                        ui.add(Slider::new(&mut self.interval_frames, 1..=300).text("Interval").suffix(" frames"));
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Image Format & Naming:").strong());
                    ui.horizontal(|ui| {
                        ui.label("Format:");
                        for fmt in ["png", "jpg", "webp", "bmp"] {
                            if ui.selectable_label(self.file_format == fmt, fmt.to_uppercase()).clicked() {
                                self.file_format = fmt.to_string();
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Naming Pattern:");
                        ui.add(egui::TextEdit::singleline(&mut self.naming_pattern).desired_width(180.0));
                    });

                    ui.add(Slider::new(&mut self.max_frames_limit, 0..=2000).text("Max Frame Limit (0 = unlimited)").suffix(" frames"));
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if self.is_dumping {
                        if ui.button("⏹ Stop Frame Dump").clicked() {
                            self.is_dumping = false;
                            self.status_message = format!("Frame capture finished. Total {} frames saved.", self.frames_captured);
                        }
                    } else {
                        if ui.add(
                            egui::Button::new(RichText::new("▶ Start Burst Frame Capture").strong().color(Color32::WHITE))
                                .fill(Color32::from_rgb(40, 120, 70)),
                        ).clicked() {
                            self.is_dumping = true;
                            self.frames_captured = 0;
                            self.status_message = format!("Extracting frames to {}...", self.output_directory.display());
                        }
                    }

                    if ui.button("📂 Open Output Folder").clicked() {
                        let _ = crate::platform::silent_command("explorer").arg(&self.output_directory).spawn();
                    }
                });

                ui.add_space(2.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("Source: {:.2}s / {:.2}s", stats.time_pos, stats.duration)).small().color(Color32::GRAY));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() { self.is_open = false; }
                    });
                });
            });
        self.is_open = open;
    }
}
