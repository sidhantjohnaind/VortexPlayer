//! karaoke_studio_dialog.rs — Karaoke Center Vocal Remover, Voice Isolation & Pitch/Tempo Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

pub struct KaraokeStudioDialog {
    pub is_open: bool,
    pub vocal_remover: bool,
    pub voice_clarity: bool,
    pub pitch_semitones: f64,
    pub playback_speed: f64,
    pub audio_delay_ms: f64,
    pub spatial_reverb: usize,
}

impl Default for KaraokeStudioDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            vocal_remover: false,
            voice_clarity: false,
            pitch_semitones: 0.0,
            playback_speed: 1.0,
            audio_delay_ms: 0.0,
            spatial_reverb: 0,
        }
    }
}

impl KaraokeStudioDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🎤 Karaoke & Pitch/Tempo Studio (Ctrl+K)")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(440.0, 420.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("Center Vocal Removal, Voice Isolation & Real-Time Pitch Studio")
                            .strong()
                            .color(Color32::from_rgb(255, 140, 200)),
                    );
                    ui.label(
                        RichText::new("Suppress center vocal tracks for karaoke, enhance dialogue clarity, or adjust musical key.")
                            .small()
                            .color(Color32::from_rgb(160, 160, 170)),
                    );
                    ui.add_space(8.0);

                    // ── Vocal & Speech Filters ──
                    ui.group(|ui| {
                        ui.label(RichText::new("Vocal Processing").strong());
                        ui.add_space(4.0);

                        if ui.checkbox(&mut self.vocal_remover, "🎤 Karaoke Mode (Center-Channel Vocal Remover)").changed() {
                            self.apply_audio_dsp(player);
                        }
                        ui.label(
                            RichText::new("  Attenuates centered vocals via phase cancellation while preserving stereo accompaniment.")
                                .small()
                                .color(Color32::from_rgb(140, 140, 150)),
                        );

                        ui.add_space(6.0);

                        if ui.checkbox(&mut self.voice_clarity, "🗣️ Dialogue & Voice Clarity Enhancer").changed() {
                            self.apply_audio_dsp(player);
                        }
                        ui.label(
                            RichText::new("  Boosts speech clarity frequencies (300Hz - 3.5kHz) and applies dynamic volume leveling.")
                                .small()
                                .color(Color32::from_rgb(140, 140, 150)),
                        );
                    });

                    ui.add_space(8.0);

                    // ── Pitch & Key Transposition ──
                    ui.group(|ui| {
                        ui.label(RichText::new("Pitch & Musical Key Transposition").strong());
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            ui.label("Pitch Shift:");
                            if ui.add(Slider::new(&mut self.pitch_semitones, -12.0..=12.0).step_by(1.0).text("semitones")).changed() {
                                self.apply_audio_dsp(player);
                            }
                            if ui.button("0 (Reset)").clicked() {
                                self.pitch_semitones = 0.0;
                                self.apply_audio_dsp(player);
                            }
                        });

                        ui.label(
                            RichText::new("  High-fidelity pitch transposition without altering playback tempo (Rubberband engine).")
                                .small()
                                .color(Color32::from_rgb(140, 140, 150)),
                        );
                    });

                    ui.add_space(8.0);

                    // ── Playback Speed / Tempo ──
                    ui.group(|ui| {
                        ui.label(RichText::new("Tempo & Synchronization").strong());
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            ui.label("Playback Speed:");
                            if ui.add(Slider::new(&mut self.playback_speed, 0.25..=4.0).step_by(0.05).text("×")).changed() {
                                player.set_speed(self.playback_speed);
                            }
                            if ui.button("1.0×").clicked() {
                                self.playback_speed = 1.0;
                                player.set_speed(1.0);
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Audio Delay:");
                            if ui.add(Slider::new(&mut self.audio_delay_ms, -5000.0..=5000.0).step_by(50.0).text("ms")).changed() {
                                player.set_audio_delay(self.audio_delay_ms / 1000.0);
                            }
                            if ui.button("0 ms").clicked() {
                                self.audio_delay_ms = 0.0;
                                player.set_audio_delay(0.0);
                            }
                        });
                    });

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("↺ Reset All Audio DSP").clicked() {
                            self.vocal_remover = false;
                            self.voice_clarity = false;
                            self.pitch_semitones = 0.0;
                            self.playback_speed = 1.0;
                            self.audio_delay_ms = 0.0;
                            player.set_speed(1.0);
                            player.set_audio_delay(0.0);
                            self.apply_audio_dsp(player);
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

    fn apply_audio_dsp(&self, player: &crate::engine::Player) {
        let mut filters = Vec::new();

        if self.pitch_semitones.abs() > 0.05 {
            let scale = (2.0f64).powf(self.pitch_semitones / 12.0);
            filters.push(format!("rubberband=pitch-scale={:.4}", scale));
        }

        if self.vocal_remover {
            filters.push("stereotools=mlev=0:slev=1.5".to_string());
        }

        if self.voice_clarity {
            filters.push("highpass=f=120,equalizer=f=3000:width_type=o:w=1.5:g=4.0,dynaudnorm=p=0.9:s=3".to_string());
        }

        player.set_audio_filter(&filters.join(","));
    }
}
