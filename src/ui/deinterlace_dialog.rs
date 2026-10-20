//! deinterlace_dialog.rs — Deinterlace Mode Selector & Post-Processing Controls (All Players)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeinterlaceMode {
    Auto,
    Off,
    Yadif,
    YadifDouble,
    Bwdif,
    BwdifDouble,
    Linear,
    Blend,
}

impl DeinterlaceMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            DeinterlaceMode::Auto => "Auto (Detect & Deinterlace)",
            DeinterlaceMode::Off => "Off (No Deinterlacing)",
            DeinterlaceMode::Yadif => "YADIF (Yet Another Deinterlacing Filter)",
            DeinterlaceMode::YadifDouble => "YADIF 2× (Double Rate 50→100fps)",
            DeinterlaceMode::Bwdif => "BWDIF (Bob Weaver / High Quality)",
            DeinterlaceMode::BwdifDouble => "BWDIF 2× (Double Rate Bob Weaver)",
            DeinterlaceMode::Linear => "Linear Interpolation (Fast)",
            DeinterlaceMode::Blend => "Blend (Average Fields)",
        }
    }
    pub fn mpv_name(&self) -> &'static str {
        match self {
            DeinterlaceMode::Auto => "auto",
            DeinterlaceMode::Off => "no",
            DeinterlaceMode::Yadif | DeinterlaceMode::YadifDouble => "yes",
            DeinterlaceMode::Bwdif | DeinterlaceMode::BwdifDouble => "yes",
            DeinterlaceMode::Linear | DeinterlaceMode::Blend => "yes",
        }
    }
}

pub struct DeinterlaceDialog {
    pub is_open: bool,
    pub mode: DeinterlaceMode,
    pub enable_deband: bool,
    pub deband_iterations: u32,
    pub deband_threshold: u32,
    pub enable_denoise: bool,
    pub denoise_strength: f32,
    pub status_message: String,
}

impl Default for DeinterlaceDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            mode: DeinterlaceMode::Auto,
            enable_deband: false,
            deband_iterations: 1,
            deband_threshold: 64,
            enable_denoise: false,
            denoise_strength: 0.5,
            status_message: "Video post-processing controls ready.".to_string(),
        }
    }
}

impl DeinterlaceDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        player.set_property_string("deinterlace", self.mode.mpv_name());
        player.set_property_string("deband", if self.enable_deband { "yes" } else { "no" });
        if self.enable_deband {
            player.set_property_string("deband-iterations", &self.deband_iterations.to_string());
            player.set_property_string("deband-threshold", &self.deband_threshold.to_string());
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("📺 Deinterlace & Video Post-Processing")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(560.0, 400.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.label(RichText::new("Deinterlace, Deband & Denoise Controls").strong().size(15.0).color(Color32::from_rgb(180, 200, 140)));
                ui.separator();

                ui.group(|ui| {
                    ui.label(RichText::new("Deinterlace Mode:").strong());
                    for mode in [DeinterlaceMode::Auto, DeinterlaceMode::Off, DeinterlaceMode::Yadif, DeinterlaceMode::YadifDouble, DeinterlaceMode::Bwdif, DeinterlaceMode::BwdifDouble, DeinterlaceMode::Linear, DeinterlaceMode::Blend] {
                        if ui.selectable_label(self.mode == mode, mode.display_name()).clicked() {
                            self.mode = mode;
                            changed = true;
                        }
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Video Quality Post-Processing").strong());
                    if ui.checkbox(&mut self.enable_deband, "Enable Debanding (Reduce Color Banding Artifacts)").changed() { changed = true; }
                    if self.enable_deband {
                        ui.horizontal(|ui| {
                            ui.label("Iterations:");
                            if ui.add(egui::Slider::new(&mut self.deband_iterations, 1..=4)).changed() { changed = true; }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Threshold:");
                            if ui.add(egui::Slider::new(&mut self.deband_threshold, 16..=128)).changed() { changed = true; }
                        });
                    }
                    if ui.checkbox(&mut self.enable_denoise, "Enable Temporal Denoise (NLMeans)").changed() { changed = true; }
                    if self.enable_denoise {
                        ui.horizontal(|ui| {
                            ui.label("Strength:");
                            if ui.add(egui::Slider::new(&mut self.denoise_strength, 0.1..=1.0)).changed() { changed = true; }
                        });
                    }
                });

                if changed { self.apply_to_player(player); }
                ui.add_space(2.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("↺ Reset All").clicked() {
                        *self = Self { is_open: true, ..Self::default() };
                        self.apply_to_player(player);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { if ui.button("Close").clicked() { self.is_open = false; } });
                });
            });
        self.is_open = open;
    }
}
