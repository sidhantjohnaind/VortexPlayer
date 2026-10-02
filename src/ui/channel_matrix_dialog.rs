#![allow(dead_code)]

use super::theme::VortexTheme;
use eframe::egui::{self, Align2, Color32, RichText, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMatrixConfig {
    pub enabled: bool,
    /// 8 rows (Inputs) x 8 columns (Outputs)
    pub matrix: [[f32; 8]; 8],
    pub solo_channels: [bool; 8],
    pub mute_channels: [bool; 8],
    pub phase_invert: [bool; 8],
    pub delay_ms: [f32; 8],
}

impl Default for ChannelMatrixConfig {
    fn default() -> Self {
        let mut matrix = [[0.0f32; 8]; 8];
        for i in 0..8 {
            matrix[i][i] = 1.0;
        }
        Self {
            enabled: false,
            matrix,
            solo_channels: [false; 8],
            mute_channels: [false; 8],
            phase_invert: [false; 8],
            delay_ms: [0.0; 8],
        }
    }
}

pub struct ChannelMatrixDialog {
    pub is_open: bool,
}

impl Default for ChannelMatrixDialog {
    fn default() -> Self {
        Self { is_open: false }
    }
}

impl ChannelMatrixDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub const CHANNELS: [&'static str; 8] = ["L", "R", "C", "LFE", "SL", "SR", "BL", "BR"];

    pub fn render(&mut self, ctx: &egui::Context, config: &mut ChannelMatrixConfig) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        let mut should_close = false;
        egui::Window::new("7.1 Surround Channel Matrix Router")
            .open(&mut open)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_width(640.0)
            .default_height(480.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut config.enabled, RichText::new("Enable Custom Channel Matrix").strong().color(VortexTheme::VORTEX_YELLOW));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Reset Identity").clicked() {
                            *config = ChannelMatrixConfig::default();
                        }
                    });
                });

                ui.separator();

                // Presets row
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Routing Presets:").size(11.0).strong());
                    if ui.small_button("Stereo → 7.1").clicked() {
                        config.enabled = true;
                        config.matrix = [[0.0; 8]; 8];
                        config.matrix[0][0] = 1.0; // L -> L
                        config.matrix[1][1] = 1.0; // R -> R
                        config.matrix[0][2] = 0.7; // L -> C
                        config.matrix[1][2] = 0.7; // R -> C
                        config.matrix[0][3] = 0.5; // L -> LFE
                        config.matrix[1][3] = 0.5; // R -> LFE
                        config.matrix[0][4] = 0.8; // L -> SL
                        config.matrix[1][5] = 0.8; // R -> SR
                        config.matrix[0][6] = 0.6; // L -> BL
                        config.matrix[1][7] = 0.6; // R -> BR
                    }
                    if ui.small_button("5.1 → 7.1").clicked() {
                        config.enabled = true;
                        config.matrix = [[0.0; 8]; 8];
                        for i in 0..6 {
                            config.matrix[i][i] = 1.0;
                        }
                        // Copy SL/SR to BL/BR
                        config.matrix[4][6] = 0.7;
                        config.matrix[5][7] = 0.7;
                    }
                    if ui.small_button("7.1 → Stereo").clicked() {
                        config.enabled = true;
                        config.matrix = [[0.0; 8]; 8];
                        config.matrix[0][0] = 1.0;
                        config.matrix[1][1] = 1.0;
                        config.matrix[2][0] = 0.707; // C -> L
                        config.matrix[2][1] = 0.707; // C -> R
                        config.matrix[3][0] = 0.5;   // LFE -> L
                        config.matrix[3][1] = 0.5;   // LFE -> R
                        config.matrix[4][0] = 0.707; // SL -> L
                        config.matrix[5][1] = 0.707; // SR -> R
                        config.matrix[6][0] = 0.5;   // BL -> L
                        config.matrix[7][1] = 0.5;   // BR -> R
                    }
                    if ui.small_button("7.1 → Headphones (Binaural)").clicked() {
                        config.enabled = true;
                        config.matrix = [[0.0; 8]; 8];
                        config.matrix[0][0] = 1.0;
                        config.matrix[1][1] = 1.0;
                        config.matrix[2][0] = 0.707;
                        config.matrix[2][1] = 0.707;
                        config.matrix[3][0] = 0.6;
                        config.matrix[3][1] = 0.6;
                        config.matrix[4][0] = 0.8;
                        config.matrix[4][1] = 0.2;
                        config.matrix[5][0] = 0.2;
                        config.matrix[5][1] = 0.8;
                        config.matrix[6][0] = 0.7;
                        config.matrix[6][1] = 0.3;
                        config.matrix[7][0] = 0.3;
                        config.matrix[7][1] = 0.7;
                    }
                });

                ui.add_space(8.0);

                // 8x8 Routing Matrix Table
                egui::Grid::new("7_1_channel_grid")
                    .striped(true)
                    .spacing(Vec2::new(6.0, 4.0))
                    .show(ui, |ui| {
                        // Header: Outputs
                        ui.label(RichText::new("IN \\ OUT").strong().color(Color32::from_rgb(180, 180, 200)));
                        for col in Self::CHANNELS {
                            ui.label(RichText::new(col).strong().color(VortexTheme::VORTEX_YELLOW));
                        }
                        ui.label(RichText::new("Mute").size(10.5));
                        ui.label(RichText::new("Solo").size(10.5));
                        ui.label(RichText::new("Phase").size(10.5));
                        ui.label(RichText::new("Delay").size(10.5));
                        ui.end_row();

                        // Rows: Inputs
                        for row in 0..8 {
                            ui.label(RichText::new(Self::CHANNELS[row]).strong().color(Color32::WHITE));

                            for col in 0..8 {
                                let val = &mut config.matrix[row][col];
                                ui.add(
                                    egui::DragValue::new(val)
                                        .range(0.0..=2.0)
                                        .speed(0.05)
                                        .custom_formatter(|v, _| if v > 0.001 { format!("{:.2}", v) } else { "-".to_string() })
                                );
                            }

                            // Mute checkbox
                            ui.checkbox(&mut config.mute_channels[row], "");

                            // Solo checkbox
                            ui.checkbox(&mut config.solo_channels[row], "");

                            // Phase inversion
                            let phase = config.phase_invert[row];
                            if ui.selectable_label(phase, if phase { "Ø 180°" } else { "0°" }).clicked() {
                                config.phase_invert[row] = !phase;
                            }

                            // Delay (ms)
                            ui.add(egui::DragValue::new(&mut config.delay_ms[row]).range(-50.0..=50.0).suffix("ms"));

                            ui.end_row();
                        }
                    });

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(RichText::new("💡 Values indicate input multiplier into output destination channels (1.00 = 100% unity gain).").size(10.5).color(Color32::from_rgb(140, 140, 150)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("  Close  ").clicked() {
                            should_close = true;
                        }
                    });
                });
            });

        if should_close {
            open = false;
        }
        self.is_open = open;
    }
}
