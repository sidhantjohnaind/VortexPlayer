//! video_crop_dialog.rs — Custom Video Crop & Pan-Scan Region Tool (Vortex/VLC style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CropPreset {
    None,
    Crop16x9From4x3,
    Crop4x3From16x9,
    Crop21x9From16x9,
    Crop1x1Square,
    CropLetterbox,
    Custom,
}

impl CropPreset {
    pub fn display_name(&self) -> &'static str {
        match self {
            CropPreset::None => "No Crop (Original)",
            CropPreset::Crop16x9From4x3 => "16:9 from 4:3 (Pillarbox Crop)",
            CropPreset::Crop4x3From16x9 => "4:3 from 16:9 (Letterbox Crop)",
            CropPreset::Crop21x9From16x9 => "21:9 UltraWide from 16:9",
            CropPreset::Crop1x1Square => "1:1 Square (Instagram)",
            CropPreset::CropLetterbox => "Remove Black Bars (Auto Detect)",
            CropPreset::Custom => "Custom Region",
        }
    }
}

pub struct VideoCropDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub preset: CropPreset,
    pub crop_left: u32,
    pub crop_right: u32,
    pub crop_top: u32,
    pub crop_bottom: u32,
    pub status_message: String,
}

impl Default for VideoCropDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            preset: CropPreset::None,
            crop_left: 0, crop_right: 0, crop_top: 0, crop_bottom: 0,
            status_message: "Video crop & pan-scan tool ready.".to_string(),
        }
    }
}

impl VideoCropDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || self.preset == CropPreset::None {
            player.set_property_string("video-crop", "");
        } else {
            let crop = format!("{}x{}+{}+{}", self.crop_right, self.crop_bottom, self.crop_left, self.crop_top);
            player.set_property_string("video-crop", &crop);
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("✂️ Video Crop & Pan-Scan Tool")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(560.0, 380.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Custom Video Region Crop & Pan-Scan").strong().size(15.0).color(Color32::from_rgb(140, 200, 255)));
                });
                ui.separator();
                ui.group(|ui| {
                    if ui.checkbox(&mut self.is_active, "✂️ Enable Video Crop").changed() { changed = true; }
                    ui.horizontal(|ui| {
                        ui.label("Crop Preset:");
                        egui::ComboBox::from_id_salt("crop_preset")
                            .selected_text(self.preset.display_name())
                            .show_ui(ui, |ui| {
                                for p in [CropPreset::None, CropPreset::Crop16x9From4x3, CropPreset::Crop4x3From16x9, CropPreset::Crop21x9From16x9, CropPreset::Crop1x1Square, CropPreset::CropLetterbox, CropPreset::Custom] {
                                    if ui.selectable_value(&mut self.preset, p, p.display_name()).clicked() {
                                        match p {
                                            CropPreset::Crop4x3From16x9 => { self.crop_left = 240; self.crop_right = 240; self.crop_top = 0; self.crop_bottom = 0; }
                                            CropPreset::Crop16x9From4x3 => { self.crop_left = 0; self.crop_right = 0; self.crop_top = 60; self.crop_bottom = 60; }
                                            CropPreset::Crop21x9From16x9 => { self.crop_left = 0; self.crop_right = 0; self.crop_top = 120; self.crop_bottom = 120; }
                                            CropPreset::Crop1x1Square => { self.crop_left = 240; self.crop_right = 240; self.crop_top = 0; self.crop_bottom = 0; }
                                            CropPreset::CropLetterbox => { self.crop_left = 0; self.crop_right = 0; self.crop_top = 140; self.crop_bottom = 140; }
                                            _ => { self.crop_left = 0; self.crop_right = 0; self.crop_top = 0; self.crop_bottom = 0; }
                                        }
                                        changed = true;
                                    }
                                }
                            });
                    });
                });
                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Crop Region (pixels)").strong());
                    if ui.add(Slider::new(&mut self.crop_left, 0..=500).text("Left")).changed() { self.preset = CropPreset::Custom; changed = true; }
                    if ui.add(Slider::new(&mut self.crop_right, 0..=500).text("Right")).changed() { self.preset = CropPreset::Custom; changed = true; }
                    if ui.add(Slider::new(&mut self.crop_top, 0..=500).text("Top")).changed() { self.preset = CropPreset::Custom; changed = true; }
                    if ui.add(Slider::new(&mut self.crop_bottom, 0..=500).text("Bottom")).changed() { self.preset = CropPreset::Custom; changed = true; }
                });
                if changed { self.apply_to_player(player); }
                ui.add_space(2.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("↺ Reset Crop").clicked() { *self = Self { is_open: true, ..Self::default() }; self.apply_to_player(player); }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { if ui.button("Close").clicked() { self.is_open = false; } });
                });
            });
        self.is_open = open;
    }
}
