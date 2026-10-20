//! binaural_crossfeed_dialog.rs — Bauer BS2B Headphone Crossfeed & 3D Spatial HRTF Virtualizer (MPC-BE style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossfeedPreset {
    BauerDefault,
    ChuMoy,
    JanMeier,
    KemarHrtf,
    Custom,
}

impl CrossfeedPreset {
    pub fn display_name(&self) -> &'static str {
        match self {
            CrossfeedPreset::BauerDefault => "Bauer BS2B Default (700 Hz / 4.5 dB)",
            CrossfeedPreset::ChuMoy => "Chu Moy Pocket Crossfeed (700 Hz / 6.0 dB)",
            CrossfeedPreset::JanMeier => "Jan Meier Natural Stereo (650 Hz / 9.5 dB)",
            CrossfeedPreset::KemarHrtf => "KEMAR Binaural 3D Spatial HRTF",
            CrossfeedPreset::Custom => "Custom Audiophile Tuned",
        }
    }
}

pub struct BinauralCrossfeedDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub preset: CrossfeedPreset,
    pub cutoff_freq_hz: u32,  // 300 to 2000 Hz
    pub feed_level_db: f32,   // 1.0 to 15.0 dB
    pub spatial_width: f32,   // 0.5 to 2.0x
    pub status_message: String,
}

impl Default for BinauralCrossfeedDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            preset: CrossfeedPreset::BauerDefault,
            cutoff_freq_hz: 700,
            feed_level_db: 4.5,
            spatial_width: 1.0,
            status_message: "Bauer stereophonic-to-binaural crossfeed ready.".to_string(),
        }
    }
}

impl BinauralCrossfeedDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active {
            player.set_audio_filter("");
        } else {
            match self.preset {
                CrossfeedPreset::BauerDefault => {
                    player.set_audio_filter("bs2b=profile=default");
                }
                CrossfeedPreset::ChuMoy => {
                    player.set_audio_filter("bs2b=profile=cmoy");
                }
                CrossfeedPreset::JanMeier => {
                    player.set_audio_filter("bs2b=profile=jmeier");
                }
                CrossfeedPreset::KemarHrtf => {
                    player.set_audio_filter("lavfi=[sofalizer=sofa=default.sofa:type=freq:radius=1.0]");
                }
                CrossfeedPreset::Custom => {
                    player.set_audio_filter(&format!("bs2b=fcut={}:feed={:.1}", self.cutoff_freq_hz, self.feed_level_db));
                }
            }
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🎧 Audiophile Headphone Crossfeed & HRTF (MPC-BE Style)")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(680.0, 480.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Bauer BS2B Crossfeed & Binaural HRTF")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(180, 220, 100)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.is_active {
                                ui.label(RichText::new("● HEADPHONE DSP ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                            } else {
                                ui.label(RichText::new("● BYPASSED").color(Color32::GRAY).strong());
                            }
                        });
                    });

                    ui.separator();

                    let mut changed = false;

                    ui.group(|ui| {
                        if ui.checkbox(&mut self.is_active, "🎧 Enable Bauer BS2B Binaural Headphone Crossfeed").changed() {
                            changed = true;
                        }
                        ui.label(
                            RichText::new("Eliminates headphone listening fatigue and unnatural stereo separation by simulating natural acoustic speaker crosstalk in the room.")
                                .small()
                                .color(Color32::GRAY),
                        );

                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            ui.label("Crossfeed Tuning Preset:");
                            egui::ComboBox::from_id_salt("crossfeed_preset_combo")
                                .selected_text(self.preset.display_name())
                                .show_ui(ui, |ui| {
                                    if ui.selectable_value(&mut self.preset, CrossfeedPreset::BauerDefault, CrossfeedPreset::BauerDefault.display_name()).clicked() {
                                        self.cutoff_freq_hz = 700;
                                        self.feed_level_db = 4.5;
                                        changed = true;
                                    }
                                    if ui.selectable_value(&mut self.preset, CrossfeedPreset::ChuMoy, CrossfeedPreset::ChuMoy.display_name()).clicked() {
                                        self.cutoff_freq_hz = 700;
                                        self.feed_level_db = 6.0;
                                        changed = true;
                                    }
                                    if ui.selectable_value(&mut self.preset, CrossfeedPreset::JanMeier, CrossfeedPreset::JanMeier.display_name()).clicked() {
                                        self.cutoff_freq_hz = 650;
                                        self.feed_level_db = 9.5;
                                        changed = true;
                                    }
                                    if ui.selectable_value(&mut self.preset, CrossfeedPreset::KemarHrtf, CrossfeedPreset::KemarHrtf.display_name()).clicked() {
                                        changed = true;
                                    }
                                    if ui.selectable_value(&mut self.preset, CrossfeedPreset::Custom, CrossfeedPreset::Custom.display_name()).clicked() {
                                        changed = true;
                                    }
                                });
                        });
                    });

                    ui.add_space(6.0);

                    // Custom Sliders
                    ui.group(|ui| {
                        ui.label(RichText::new("Acoustic Head Shadow & Crosstalk Parameters").strong());

                        ui.horizontal(|ui| {
                            ui.label("Cutoff Frequency (F_cut):");
                            if ui.add(Slider::new(&mut self.cutoff_freq_hz, 300..=2000).suffix(" Hz")).changed() {
                                self.preset = CrossfeedPreset::Custom;
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Crossfeed Attenuation Level:");
                            if ui.add(Slider::new(&mut self.feed_level_db, 1.0..=15.0).suffix(" dB")).changed() {
                                self.preset = CrossfeedPreset::Custom;
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("3D Soundstage Width:");
                            if ui.add(Slider::new(&mut self.spatial_width, 0.5..=2.0).suffix("x")).changed() {
                                changed = true;
                            }
                        });
                    });

                    if changed {
                        self.apply_to_player(player);
                    }

                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("↺ Reset to Standard").clicked() {
                            self.preset = CrossfeedPreset::BauerDefault;
                            self.cutoff_freq_hz = 700;
                            self.feed_level_db = 4.5;
                            self.spatial_width = 1.0;
                            self.apply_to_player(player);
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
}
