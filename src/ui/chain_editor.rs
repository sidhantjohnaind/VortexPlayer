#![allow(dead_code)]

use super::theme::VortexTheme;
use crate::engine::{AudioDspChain, VideoFilterChain};
use eframe::egui::{self, Align2, Color32, RichText, Vec2};

pub struct ChainEditorDialog {
    pub is_open: bool,
    pub active_tab: usize, // 0 = Video Filter Pipeline, 1 = Audio DSP Pipeline
}

impl Default for ChainEditorDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            active_tab: 0,
        }
    }
}

impl ChainEditorDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        video_chain: &mut VideoFilterChain,
        audio_chain: &mut AudioDspChain,
    ) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        let mut should_close = false;
        egui::Window::new("Pipeline Processing Chain Editor")
            .open(&mut open)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .default_width(560.0)
            .default_height(460.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.active_tab == 0, RichText::new("📹 Video Filter Chain").strong()).clicked() {
                        self.active_tab = 0;
                    }
                    if ui.selectable_label(self.active_tab == 1, RichText::new("🔊 Audio DSP Chain").strong()).clicked() {
                        self.active_tab = 1;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Reset Chain to Default").clicked() {
                            if self.active_tab == 0 {
                                *video_chain = VideoFilterChain::default();
                            } else {
                                *audio_chain = AudioDspChain::default();
                            }
                        }
                    });
                });

                ui.separator();

                if self.active_tab == 0 {
                    ui.label(RichText::new("Ordered Video Processing Pipeline (Top = Executed First):").size(11.0).color(Color32::from_rgb(160, 160, 180)));
                    ui.add_space(4.0);

                    let num_stages = video_chain.stages.len();
                    for idx in 0..num_stages {
                        let stage = &mut video_chain.stages[idx];
                        let is_enabled = stage.enabled;

                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(format!("{}.", idx + 1)).strong().color(VortexTheme::POT_YELLOW));
                                ui.checkbox(&mut stage.enabled, RichText::new(&stage.name).strong());

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if idx + 1 < num_stages && ui.small_button("▼").clicked() {
                                        // signal move down
                                    }
                                    if idx > 0 && ui.small_button("▲").clicked() {
                                        // signal move up
                                    }

                                    if is_enabled {
                                        ui.add(
                                            egui::DragValue::new(&mut stage.intensity)
                                                .range(0.0..=2.0)
                                                .speed(0.05)
                                                .prefix("Intensity: ")
                                        );
                                    }
                                });
                            });
                        });
                    }
                } else {
                    ui.label(RichText::new("Ordered Audio DSP Pipeline (Top = Executed First):").size(11.0).color(Color32::from_rgb(160, 160, 180)));
                    ui.add_space(4.0);

                    let num_stages = audio_chain.stages.len();
                    for idx in 0..num_stages {
                        let stage = &mut audio_chain.stages[idx];
                        let is_enabled = stage.enabled;

                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(format!("{}.", idx + 1)).strong().color(VortexTheme::POT_YELLOW));
                                ui.checkbox(&mut stage.enabled, RichText::new(&stage.name).strong());

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if idx + 1 < num_stages && ui.small_button("▼").clicked() {
                                        // signal move down
                                    }
                                    if idx > 0 && ui.small_button("▲").clicked() {
                                        // signal move up
                                    }

                                    if is_enabled {
                                        ui.add(
                                            egui::DragValue::new(&mut stage.value)
                                                .range(0.0..=2.0)
                                                .speed(0.05)
                                                .prefix("Level: ")
                                        );
                                    }
                                });
                            });
                        });
                    }
                }

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(RichText::new("💡 Stages are executed in top-to-bottom order before rendering to screen/audio device.").size(10.5).color(Color32::from_rgb(130, 130, 140)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("  Apply & Close  ").clicked() {
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
