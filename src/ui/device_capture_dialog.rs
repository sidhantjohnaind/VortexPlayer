#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Vec2};

pub struct DeviceCaptureDialog {
    pub is_open: bool,
    pub video_device: String,
    pub audio_device: String,
}

impl Default for DeviceCaptureDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            video_device: "Integrated Camera".to_string(),
            audio_device: "".to_string(),
        }
    }
}

impl DeviceCaptureDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("📹 Device Capture / Webcam / HDMI (Ctrl+W)")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_size(Vec2::new(380.0, 200.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("DirectShow HDMI Capture & Webcam Stream").strong().color(Color32::from_rgb(180, 140, 255)));
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("Video Device:");
                        ui.text_edit_singleline(&mut self.video_device);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Audio Device (Optional):");
                        ui.text_edit_singleline(&mut self.audio_device);
                    });

                    ui.add_space(10.0);

                    if ui.add(egui::Button::new(RichText::new("▶ Open Live Capture Stream").color(Color32::WHITE)).fill(Color32::from_rgb(60, 140, 70))).clicked() {
                        player.open_directshow_device(&self.video_device, &self.audio_device);
                        self.is_open = false;
                    }
                });
            });
        self.is_open = open;
    }
}
