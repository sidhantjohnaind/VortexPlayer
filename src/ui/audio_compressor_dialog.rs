//! audio_compressor_dialog.rs — Dynamic Range Compressor / Limiter / Night Mode (VLC/MPC-BE style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressorPreset {
    Off,
    NightMode,
    DialogueBoost,
    MovieTheater,
    HeavyCompression,
    Custom,
}

impl CompressorPreset {
    pub fn display_name(&self) -> &'static str {
        match self {
            CompressorPreset::Off => "Off (Bypass)",
            CompressorPreset::NightMode => "🌙 Night Mode (Quiet Listening)",
            CompressorPreset::DialogueBoost => "🗣️ Dialogue Boost (Voice Clarity)",
            CompressorPreset::MovieTheater => "🎬 Movie Theater (Wide Dynamic Range)",
            CompressorPreset::HeavyCompression => "🔊 Heavy Compression (Loudness War)",
            CompressorPreset::Custom => "⚙️ Custom Audiophile Tuned",
        }
    }
}

pub struct AudioCompressorDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub preset: CompressorPreset,
    pub threshold_db: f32,   // -50 to 0 dB
    pub ratio: f32,          // 1.0 to 20.0
    pub attack_ms: f32,      // 0.01 to 200 ms
    pub release_ms: f32,     // 10 to 2000 ms
    pub makeup_gain_db: f32, // 0 to 30 dB
    pub knee_db: f32,        // 1 to 10 dB
    pub status_message: String,
}

impl Default for AudioCompressorDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            preset: CompressorPreset::Off,
            threshold_db: -20.0,
            ratio: 4.0,
            attack_ms: 20.0,
            release_ms: 250.0,
            makeup_gain_db: 6.0,
            knee_db: 2.5,
            status_message: "Dynamic range compressor ready.".to_string(),
        }
    }
}

impl AudioCompressorDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || self.preset == CompressorPreset::Off {
            player.set_audio_filter("");
        } else {
            let af = format!(
                "acompressor=threshold={:.1}dB:ratio={:.1}:attack={:.1}:release={:.1}:makeup={:.1}dB:knee={:.1}dB",
                self.threshold_db, self.ratio, self.attack_ms, self.release_ms, self.makeup_gain_db, self.knee_db
            );
            player.set_audio_filter(&format!("lavfi=[{}]", af));
        }
    }

    fn apply_preset(&mut self) {
        match self.preset {
            CompressorPreset::NightMode => {
                self.threshold_db = -25.0; self.ratio = 6.0; self.attack_ms = 5.0;
                self.release_ms = 200.0; self.makeup_gain_db = 12.0; self.knee_db = 3.0;
            }
            CompressorPreset::DialogueBoost => {
                self.threshold_db = -18.0; self.ratio = 3.0; self.attack_ms = 10.0;
                self.release_ms = 300.0; self.makeup_gain_db = 8.0; self.knee_db = 4.0;
            }
            CompressorPreset::MovieTheater => {
                self.threshold_db = -30.0; self.ratio = 2.0; self.attack_ms = 30.0;
                self.release_ms = 500.0; self.makeup_gain_db = 4.0; self.knee_db = 6.0;
            }
            CompressorPreset::HeavyCompression => {
                self.threshold_db = -15.0; self.ratio = 12.0; self.attack_ms = 1.0;
                self.release_ms = 100.0; self.makeup_gain_db = 15.0; self.knee_db = 1.0;
            }
            _ => {}
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("🔊 Dynamic Range Compressor & Night Mode")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(640.0, 420.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Audio Dynamic Range Compressor / Limiter").strong().size(15.0).color(Color32::from_rgb(220, 180, 100)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_active {
                            ui.label(RichText::new("● ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● BYPASS").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();
                ui.group(|ui| {
                    if ui.checkbox(&mut self.is_active, "Enable Dynamic Range Compressor").changed() { changed = true; }
                    ui.horizontal(|ui| {
                        ui.label("Preset:");
                        egui::ComboBox::from_id_salt("comp_preset")
                            .selected_text(self.preset.display_name())
                            .show_ui(ui, |ui| {
                                for p in [CompressorPreset::Off, CompressorPreset::NightMode, CompressorPreset::DialogueBoost, CompressorPreset::MovieTheater, CompressorPreset::HeavyCompression, CompressorPreset::Custom] {
                                    if ui.selectable_value(&mut self.preset, p, p.display_name()).clicked() {
                                        self.apply_preset();
                                        changed = true;
                                    }
                                }
                            });
                    });
                });
                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Compressor Parameters").strong());
                    if ui.add(Slider::new(&mut self.threshold_db, -50.0..=0.0).text("Threshold").suffix(" dB")).changed() { self.preset = CompressorPreset::Custom; changed = true; }
                    if ui.add(Slider::new(&mut self.ratio, 1.0..=20.0).text("Ratio")).changed() { self.preset = CompressorPreset::Custom; changed = true; }
                    if ui.add(Slider::new(&mut self.attack_ms, 0.01..=200.0).text("Attack").suffix(" ms")).changed() { self.preset = CompressorPreset::Custom; changed = true; }
                    if ui.add(Slider::new(&mut self.release_ms, 10.0..=2000.0).text("Release").suffix(" ms")).changed() { self.preset = CompressorPreset::Custom; changed = true; }
                    if ui.add(Slider::new(&mut self.makeup_gain_db, 0.0..=30.0).text("Makeup Gain").suffix(" dB")).changed() { self.preset = CompressorPreset::Custom; changed = true; }
                    if ui.add(Slider::new(&mut self.knee_db, 1.0..=10.0).text("Knee Width").suffix(" dB")).changed() { self.preset = CompressorPreset::Custom; changed = true; }
                });
                if changed { self.apply_to_player(player); }
                ui.add_space(2.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("↺ Reset").clicked() { *self = Self { is_open: true, ..Self::default() }; self.apply_to_player(player); }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() { self.is_open = false; }
                    });
                });
            });
        self.is_open = open;
    }
}
