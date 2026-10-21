//! loudness_radar_dialog.rs — EBU R128 & LUFS Broadcast Loudness Radar Studio (ITU-R BS.1770)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoudnessStandard {
    EbuR128,   // -23 LUFS (European TV Broadcast)
    AtscA85,   // -24 LUFS (US Broadcast)
    Spotify,   // -14 LUFS (Web Streaming / YouTube)
    AppleMusic,// -16 LUFS (Sound Check)
    Podcast,   // -16 LUFS (AES Standard)
    Custom,
}

impl LoudnessStandard {
    pub fn display_name(&self) -> &'static str {
        match self {
            LoudnessStandard::EbuR128 => "📺 EBU R128 (-23.0 LUFS / TV Broadcast)",
            LoudnessStandard::AtscA85 => "🇺🇸 ATSC A/85 (-24.0 LUFS / US TV)",
            LoudnessStandard::Spotify => "🌐 Web Streaming / YouTube / Spotify (-14.0 LUFS)",
            LoudnessStandard::AppleMusic => "🍎 Apple Music / AES (-16.0 LUFS)",
            LoudnessStandard::Podcast => "🎙️ Podcast / Speech Standard (-16.0 LUFS)",
            LoudnessStandard::Custom => "⚙️ Custom Target LUFS",
        }
    }
    pub fn target_lufs(&self) -> f32 {
        match self {
            LoudnessStandard::EbuR128 => -23.0,
            LoudnessStandard::AtscA85 => -24.0,
            LoudnessStandard::Spotify => -14.0,
            LoudnessStandard::AppleMusic | LoudnessStandard::Podcast => -16.0,
            LoudnessStandard::Custom => -18.0,
        }
    }
}

pub struct LoudnessRadarDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub standard: LoudnessStandard,
    pub target_lufs: f32,
    pub max_true_peak_db: f32, // -1.0 dBTP
    pub momentary_lufs: f32,
    pub short_term_lufs: f32,
    pub integrated_lufs: f32,
    pub loudness_range_lu: f32,
    pub true_peak_db: f32,
    pub status_message: String,
}

impl Default for LoudnessRadarDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            standard: LoudnessStandard::Spotify,
            target_lufs: -14.0,
            max_true_peak_db: -1.0,
            momentary_lufs: -16.4,
            short_term_lufs: -15.1,
            integrated_lufs: -14.2,
            loudness_range_lu: 8.5,
            true_peak_db: -0.8,
            status_message: "EBU R128 / ITU-R BS.1770 loudness normalizer ready.".to_string(),
        }
    }
}

impl LoudnessRadarDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active {
            player.set_audio_filter("");
        } else {
            let af = format!(
                "loudnorm=I={:.1}:TP={:.1}:LRA={:.1}",
                self.target_lufs, self.max_true_peak_db, self.loudness_range_lu
            );
            player.set_audio_filter(&format!("lavfi=[{}]", af));
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("📊 EBU R128 & LUFS Loudness Radar")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(640.0, 440.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("ITU-R BS.1770 & EBU R128 Loudness Radar").strong().size(15.0).color(Color32::from_rgb(180, 220, 100)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_active {
                            ui.label(RichText::new("● NORMALIZING").color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● BYPASS").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    if ui.checkbox(&mut self.is_active, "📊 Enable Auto-Loudness Normalization").changed() { changed = true; }
                    ui.horizontal(|ui| {
                        ui.label("Broadcast Target Standard:");
                        egui::ComboBox::from_id_salt("loudness_std_combo")
                            .selected_text(self.standard.display_name())
                            .show_ui(ui, |ui| {
                                for s in [
                                    LoudnessStandard::Spotify,
                                    LoudnessStandard::AppleMusic,
                                    LoudnessStandard::EbuR128,
                                    LoudnessStandard::AtscA85,
                                    LoudnessStandard::Podcast,
                                    LoudnessStandard::Custom,
                                ] {
                                    if ui.selectable_value(&mut self.standard, s, s.display_name()).clicked() {
                                        self.target_lufs = s.target_lufs();
                                        changed = true;
                                    }
                                }
                            });
                    });
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Live LUFS Loudness Telemetry").strong());
                    ui.columns(4, |cols| {
                        cols[0].group(|ui| {
                            ui.label(RichText::new("Momentary (M)").small().color(Color32::GRAY));
                            ui.label(RichText::new(format!("{:.1} LUFS", self.momentary_lufs)).strong().size(16.0).color(Color32::from_rgb(100, 200, 255)));
                        });
                        cols[1].group(|ui| {
                            ui.label(RichText::new("Short-Term (S)").small().color(Color32::GRAY));
                            ui.label(RichText::new(format!("{:.1} LUFS", self.short_term_lufs)).strong().size(16.0).color(Color32::from_rgb(140, 220, 180)));
                        });
                        cols[2].group(|ui| {
                            ui.label(RichText::new("Integrated (I)").small().color(Color32::GRAY));
                            ui.label(RichText::new(format!("{:.1} LUFS", self.integrated_lufs)).strong().size(16.0).color(Color32::from_rgb(255, 200, 100)));
                        });
                        cols[3].group(|ui| {
                            ui.label(RichText::new("True Peak (TP)").small().color(Color32::GRAY));
                            ui.label(RichText::new(format!("{:.1} dBTP", self.true_peak_db)).strong().size(16.0).color(Color32::from_rgb(255, 120, 120)));
                        });
                    });
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Target Parameters").strong());
                    if ui.add(Slider::new(&mut self.target_lufs, -30.0..=-10.0).text("Target Integrated Loudness").suffix(" LUFS")).changed() {
                        self.standard = LoudnessStandard::Custom;
                        changed = true;
                    }
                    if ui.add(Slider::new(&mut self.max_true_peak_db, -6.0..=0.0).text("Max True Peak Ceiling").suffix(" dBTP")).changed() {
                        changed = true;
                    }
                });

                if changed { self.apply_to_player(player); }

                ui.add_space(4.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("↺ Reset").clicked() {
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
