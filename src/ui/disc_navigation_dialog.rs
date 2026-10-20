//! disc_navigation_dialog.rs — DVD / Blu-ray Title Browser, Angle Switcher & Disc Navigation Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};
#[cfg(windows)]
use std::os::windows::process::CommandExt;


#[derive(Debug, Clone)]
pub struct DiscTitleInfo {
    pub title_number: u32,
    pub duration_seconds: f64,
    pub chapters_count: usize,
    pub audio_languages: Vec<String>,
    pub subtitle_languages: Vec<String>,
}

pub struct DiscNavigationDialog {
    pub is_open: bool,
    pub disc_path: String,
    pub titles: Vec<DiscTitleInfo>,
    pub selected_title: u32,
    pub current_angle: u32,
    pub total_angles: u32,
}

impl Default for DiscNavigationDialog {
    fn default() -> Self {
        let sample_titles = vec![
            DiscTitleInfo {
                title_number: 1,
                duration_seconds: 7320.0, // 2h 02m
                chapters_count: 18,
                audio_languages: vec!["English Dolby Atmos (7.1)".to_string(), "French DTS-HD (5.1)".to_string(), "Director's Commentary".to_string()],
                subtitle_languages: vec!["English (Full)".to_string(), "English (SDH)".to_string(), "French".to_string(), "Spanish".to_string()],
            },
            DiscTitleInfo {
                title_number: 2,
                duration_seconds: 1420.0, // 23m Making of
                chapters_count: 4,
                audio_languages: vec!["English Stereo (2.0)".to_string()],
                subtitle_languages: vec!["English".to_string()],
            },
            DiscTitleInfo {
                title_number: 3,
                duration_seconds: 640.0, // Deleted Scenes
                chapters_count: 6,
                audio_languages: vec!["English Stereo (2.0)".to_string()],
                subtitle_languages: vec!["English".to_string()],
            },
        ];

        Self {
            is_open: false,
            disc_path: "D:\\".to_string(),
            titles: sample_titles,
            selected_title: 1,
            current_angle: 1,
            total_angles: 3,
        }
    }
}

impl DiscNavigationDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format_duration(seconds: f64) -> String {
        let h = (seconds / 3600.0).floor() as u32;
        let m = ((seconds % 3600.0) / 60.0).floor() as u32;
        let s = (seconds % 60.0).round() as u32;
        if h > 0 {
            format!("{:02}:{:02}:{:02}", h, m, s)
        } else {
            format!("{:02}:{:02}", m, s)
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("📀 DVD & Blu-ray Title Browser / Navigation")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(700.0, 480.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Disc Title Navigation & Angle Control")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(100, 200, 255)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("📂 Open VIDEO_TS / BDMV / ISO...").clicked() {
                                if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                                    self.disc_path = dir.to_string_lossy().to_string();
                                }
                            }
                        });
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label(format!("Disc Location: {}", self.disc_path));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("⏏ Eject Drive Tray").clicked() {
                                Self::eject_drive(&self.disc_path);
                            }
                        });
                    });

                    ui.add_space(8.0);

                    // Multi-Angle Selector
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("📹 Camera Angle Switcher:").strong());
                            for a in 1..=self.total_angles {
                                if ui.selectable_label(self.current_angle == a, format!("Angle {}", a)).clicked() {
                                    self.current_angle = a;
                                    player.set_property_string("angle", &a.to_string());
                                }
                            }
                        });
                    });

                    ui.add_space(6.0);
                    ui.label(RichText::new("Available Disc Titles / Features:").strong());

                    // Titles List
                    ScrollArea::vertical()
                        .max_height(240.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for title in &self.titles {
                                let is_selected = self.selected_title == title.title_number;
                                let bg = if is_selected {
                                    Color32::from_rgb(35, 55, 75)
                                } else if title.title_number % 2 == 0 {
                                    Color32::from_rgb(24, 26, 32)
                                } else {
                                    Color32::from_rgb(20, 21, 26)
                                };

                                egui::Frame::new()
                                    .fill(bg)
                                    .inner_margin(egui::Margin::symmetric(8, 6))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            if ui.button(format!("▶ Title {}", title.title_number)).clicked() {
                                                self.selected_title = title.title_number;
                                                let disc_url = format!("dvd://{} --dvd-device=\"{}\"", title.title_number, self.disc_path);
                                                player.load_file(&disc_url);
                                                player.play();
                                            }

                                            ui.label(RichText::new(Self::format_duration(title.duration_seconds)).strong().color(Color32::from_rgb(255, 220, 120)));
                                            ui.label(format!("• {} Chapters", title.chapters_count));

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(title.audio_languages.join(" | ")).small().color(Color32::GRAY));
                                            });
                                        });
                                    });
                                ui.add_space(2.0);
                            }
                        });

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("▶ Play Main Feature (Title 1)").clicked() {
                            let disc_url = format!("dvd://1 --dvd-device=\"{}\"", self.disc_path);
                            player.load_file(&disc_url);
                            player.play();
                            self.is_open = false;
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

    pub fn eject_drive(_drive_str: &str) {
        #[cfg(windows)]
        {
            let letter = _drive_str.chars().next().unwrap_or('D');
            let script = format!(
                "(New-Object -com 'Shell.Application').Namespace(17).ParseName('{}:').InvokeVerb('Eject')",
                letter
            );
            std::thread::spawn(move || {
                let _ = std::process::Command::new("powershell")
                    .args(["-NoProfile", "-NonInteractive", "-Command", &script])
                    .creation_flags(0x08000000)
                    .output();
            });
        }
    }
}
