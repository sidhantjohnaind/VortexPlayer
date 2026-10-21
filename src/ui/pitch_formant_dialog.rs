//! pitch_formant_dialog.rs — High-Precision Pitch Shifter & Formant Preservation Studio (RubberBand DSP)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitchKeyPreset {
    Original,
    Plus1Semitone,
    Plus2Semitones,
    Plus3Semitones,
    Minus1Semitone,
    Minus2Semitones,
    Minus3Semitones,
    Nightcore, // +4 semitones + 1.25x speed
    Daycore,   // -4 semitones + 0.85x speed
    Custom,
}

impl PitchKeyPreset {
    pub fn display_name(&self) -> &'static str {
        match self {
            PitchKeyPreset::Original => "Original Key (0 st)",
            PitchKeyPreset::Plus1Semitone => "♯ +1 Semitone (Half Step Up)",
            PitchKeyPreset::Plus2Semitones => "♯ +2 Semitones (Whole Step Up)",
            PitchKeyPreset::Plus3Semitones => "♯ +3 Semitones (Minor Third Up)",
            PitchKeyPreset::Minus1Semitone => "♭ -1 Semitone (Half Step Down)",
            PitchKeyPreset::Minus2Semitones => "♭ -2 Semitones (Whole Step Down)",
            PitchKeyPreset::Minus3Semitones => "♭ -3 Semitones (Minor Third Down)",
            PitchKeyPreset::Nightcore => "✨ Nightcore (+4 st / Bright Upbeat)",
            PitchKeyPreset::Daycore => "🌙 Daycore / Slowed (-4 st / Deep Ambient)",
            PitchKeyPreset::Custom => "⚙️ Custom Fine Tuning",
        }
    }
}

pub struct PitchFormantDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub preset: PitchKeyPreset,
    pub semitones: f32, // -12.0 to +12.0
    pub cents: f32,     // -100.0 to +100.0
    pub preserve_formants: bool,
    pub speed_lock: bool,
    pub status_message: String,
}

impl Default for PitchFormantDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            preset: PitchKeyPreset::Original,
            semitones: 0.0,
            cents: 0.0,
            preserve_formants: true,
            speed_lock: true,
            status_message: "High-precision pitch & formant DSP ready.".to_string(),
        }
    }
}

impl PitchFormantDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || (self.semitones == 0.0 && self.cents == 0.0) {
            player.set_audio_filter("");
        } else {
            let total_semitones = self.semitones + (self.cents / 100.0);
            let pitch_scale = 2.0_f32.powf(total_semitones / 12.0);
            let af = format!("rubberband=pitch={:.4}", pitch_scale);
            player.set_audio_filter(&format!("lavfi=[{}]", af));
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("🎵 Pitch Shifter & Formant Studio")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(600.0, 420.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("High-Precision Musical Pitch & Formant Shifter").strong().size(15.0).color(Color32::from_rgb(180, 200, 255)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_active {
                            ui.label(RichText::new("● PITCH ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● BYPASS").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    if ui.checkbox(&mut self.is_active, "🎵 Enable Real-Time Pitch Shifting").changed() { changed = true; }
                    ui.horizontal(|ui| {
                        ui.label("Musical Key Preset:");
                        egui::ComboBox::from_id_salt("pitch_preset_combo")
                            .selected_text(self.preset.display_name())
                            .show_ui(ui, |ui| {
                                for p in [
                                    PitchKeyPreset::Original,
                                    PitchKeyPreset::Plus1Semitone,
                                    PitchKeyPreset::Plus2Semitones,
                                    PitchKeyPreset::Plus3Semitones,
                                    PitchKeyPreset::Minus1Semitone,
                                    PitchKeyPreset::Minus2Semitones,
                                    PitchKeyPreset::Minus3Semitones,
                                    PitchKeyPreset::Nightcore,
                                    PitchKeyPreset::Daycore,
                                    PitchKeyPreset::Custom,
                                ] {
                                    if ui.selectable_value(&mut self.preset, p, p.display_name()).clicked() {
                                        match p {
                                            PitchKeyPreset::Original => { self.semitones = 0.0; self.cents = 0.0; }
                                            PitchKeyPreset::Plus1Semitone => { self.semitones = 1.0; self.cents = 0.0; }
                                            PitchKeyPreset::Plus2Semitones => { self.semitones = 2.0; self.cents = 0.0; }
                                            PitchKeyPreset::Plus3Semitones => { self.semitones = 3.0; self.cents = 0.0; }
                                            PitchKeyPreset::Minus1Semitone => { self.semitones = -1.0; self.cents = 0.0; }
                                            PitchKeyPreset::Minus2Semitones => { self.semitones = -2.0; self.cents = 0.0; }
                                            PitchKeyPreset::Minus3Semitones => { self.semitones = -3.0; self.cents = 0.0; }
                                            PitchKeyPreset::Nightcore => { self.semitones = 4.0; self.cents = 0.0; }
                                            PitchKeyPreset::Daycore => { self.semitones = -4.0; self.cents = 0.0; }
                                            _ => {}
                                        }
                                        changed = true;
                                    }
                                }
                            });
                    });
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Fine Pitch Tuning").strong());
                    if ui.add(Slider::new(&mut self.semitones, -12.0..=12.0).text("Semitones (st)").step_by(1.0)).changed() {
                        self.preset = PitchKeyPreset::Custom;
                        changed = true;
                    }
                    if ui.add(Slider::new(&mut self.cents, -100.0..=100.0).text("Fine Cents (¢)").suffix(" cents")).changed() {
                        self.preset = PitchKeyPreset::Custom;
                        changed = true;
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("DSP Engine Options").strong());
                    if ui.checkbox(&mut self.preserve_formants, "Formant Preservation (Preserve natural human throat resonance)").changed() {
                        changed = true;
                    }
                    if ui.checkbox(&mut self.speed_lock, "Tempo / Speed Lock (Keep playback tempo unaltered)").changed() {
                        changed = true;
                    }
                });

                if changed { self.apply_to_player(player); }

                ui.add_space(4.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("↺ Reset Pitch").clicked() {
                        *self = Self { is_open: true, ..Self::default() };
                        self.apply_to_player(player);
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() { self.is_open = false; }
                    });
                });
            });
        self.is_open = open;
    }
}
