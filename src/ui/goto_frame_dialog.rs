//! goto_frame_dialog.rs — Go-To Exact Frame Number Dialog (MPC-BE style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Vec2};

pub struct GotoFrameDialog {
    pub is_open: bool,
    pub frame_input: String,
    pub current_frame: u64,
    pub total_frames: u64,
    pub fps: f64,
    pub status_message: String,
}

impl Default for GotoFrameDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            frame_input: String::new(),
            current_frame: 0,
            total_frames: 0,
            fps: 23.976,
            status_message: "Enter a frame number to jump to.".to_string(),
        }
    }
}

impl GotoFrameDialog {
    pub fn new() -> Self { Self::default() }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player, stats: &crate::engine::MediaStats) {
        if !self.is_open { return; }

        self.fps = if stats.video_fps > 0.0 { stats.video_fps } else { 23.976 };
        self.current_frame = (stats.time_pos * self.fps) as u64;
        self.total_frames = (stats.duration * self.fps) as u64;

        let mut open = self.is_open;
        egui::Window::new("🎞️ Go To Frame")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_size(Vec2::new(400.0, 200.0))
            .show(ctx, |ui| {
                ui.label(RichText::new("Jump to Exact Video Frame Number").strong().size(15.0).color(Color32::from_rgb(200, 220, 140)));
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("Current: Frame {} / {}", self.current_frame, self.total_frames)).color(Color32::GRAY));
                    ui.label(RichText::new(format!("({:.3} fps)", self.fps)).small().color(Color32::from_rgb(100, 150, 180)));
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Frame #:");
                    let response = ui.add(egui::TextEdit::singleline(&mut self.frame_input).desired_width(140.0).hint_text("e.g. 12450"));
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if let Ok(frame) = self.frame_input.trim().parse::<u64>() {
                            let seek_time = frame as f64 / self.fps;
                            player.seek_absolute(seek_time);
                            self.status_message = format!("Jumped to frame {} (at {:.3}s)", frame, seek_time);
                        }
                    }
                });

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if ui.button("▶ Jump").clicked() {
                        if let Ok(frame) = self.frame_input.trim().parse::<u64>() {
                            let seek_time = frame as f64 / self.fps;
                            player.seek_absolute(seek_time);
                            self.status_message = format!("Jumped to frame {} (at {:.3}s)", frame, seek_time);
                        } else {
                            self.status_message = "Invalid frame number.".to_string();
                        }
                    }
                    if ui.button("⏮ First Frame").clicked() {
                        player.seek_absolute(0.0);
                        self.status_message = "Jumped to frame 0.".to_string();
                    }
                    if ui.button("⏭ Last Frame").clicked() {
                        let t = (self.total_frames.saturating_sub(1)) as f64 / self.fps;
                        player.seek_absolute(t);
                        self.status_message = format!("Jumped to last frame {}", self.total_frames.saturating_sub(1));
                    }
                });

                ui.add_space(4.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
            });
        self.is_open = open;
    }
}
