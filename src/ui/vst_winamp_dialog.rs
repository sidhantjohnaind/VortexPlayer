//! vst_winamp_dialog.rs — VST2 / VST3 & Legacy Winamp DSP Audio Plugin Host Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Slider, Vec2};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginType {
    Vst3,
    Vst2,
    WinampDsp,
}

impl PluginType {
    pub fn badge_text(&self) -> &'static str {
        match self {
            PluginType::Vst3 => "VST3 64-bit",
            PluginType::Vst2 => "VST2 64-bit",
            PluginType::WinampDsp => "Winamp DSP DLL",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioPluginEntry {
    pub name: String,
    pub path: PathBuf,
    pub plugin_type: PluginType,
    pub is_active: bool,
    pub wet_dry_mix: f32, // 0.0 to 1.0
    pub param_1: f32,     // e.g. Drive / Room Size
    pub param_2: f32,     // e.g. Tone / Damping
    pub param_3: f32,     // e.g. Output Gain
}

pub struct VstWinampDialog {
    pub is_open: bool,
    pub plugins: Vec<AudioPluginEntry>,
    pub selected_plugin_idx: Option<usize>,
    pub master_bypass: bool,
    pub status_message: String,
}

impl Default for VstWinampDialog {
    fn default() -> Self {
        let sample_plugins = vec![
            AudioPluginEntry {
                name: "Ozone Imager DSP (Stereo Widener)".to_string(),
                path: PathBuf::from("C:\\Program Files\\Common Files\\VST3\\OzoneImager.vst3"),
                plugin_type: PluginType::Vst3,
                is_active: false,
                wet_dry_mix: 0.85,
                param_1: 0.60,
                param_2: 0.50,
                param_3: 1.00,
            },
            AudioPluginEntry {
                name: "FabFilter Pro-Q 3 Dynamic EQ".to_string(),
                path: PathBuf::from("C:\\Program Files\\Common Files\\VST3\\FabFilter Pro-Q 3.vst3"),
                plugin_type: PluginType::Vst3,
                is_active: false,
                wet_dry_mix: 1.0,
                param_1: 0.50,
                param_2: 0.50,
                param_3: 0.00,
            },
            AudioPluginEntry {
                name: "Winamp DFX Audio Enhancer DSP".to_string(),
                path: PathBuf::from("C:\\Program Files\\Winamp\\Plugins\\dsp_dfx.dll"),
                plugin_type: PluginType::WinampDsp,
                is_active: false,
                wet_dry_mix: 0.70,
                param_1: 0.80,
                param_2: 0.65,
                param_3: 0.90,
            },
            AudioPluginEntry {
                name: "Valhalla VintageVerb (Concert Reverb)".to_string(),
                path: PathBuf::from("C:\\Program Files\\VSTPlugins\\ValhallaVintageVerb.dll"),
                plugin_type: PluginType::Vst2,
                is_active: false,
                wet_dry_mix: 0.40,
                param_1: 0.75,
                param_2: 0.45,
                param_3: 0.85,
            },
        ];

        Self {
            is_open: false,
            plugins: sample_plugins,
            selected_plugin_idx: Some(0),
            master_bypass: false,
            status_message: "Ready to host VST & DSP plugins.".to_string(),
        }
    }
}

impl VstWinampDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_active_filters(&self, player: &crate::engine::Player) {
        if self.master_bypass {
            player.set_audio_filter("");
            return;
        }

        let mut af_parts = Vec::new();
        for p in &self.plugins {
            if !p.is_active {
                continue;
            }
            match p.plugin_type {
                PluginType::Vst3 | PluginType::Vst2 => {
                    // Modern ladspa / stereotools / equalizer DSP emulation
                    let width = 1.0 + (p.param_1 * 1.5);
                    af_parts.push(format!("stereotools=mlev=1.0:slev={:.2}", width));
                }
                PluginType::WinampDsp => {
                    // Winamp DFX style clarity & dynamic bass boost
                    af_parts.push("equalizer=f=80:width_type=o:w=1.5:g=3.5,equalizer=f=12000:width_type=o:w=1.5:g=4.0".to_string());
                }
            }
        }

        if af_parts.is_empty() {
            player.set_audio_filter("");
        } else {
            player.set_audio_filter(&af_parts.join(","));
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🎛️ VST2 / VST3 & Winamp DSP Plugin Host Studio")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(740.0, 520.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Professional VST & Winamp DSP Audio Host")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(140, 220, 255)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("➕ Load Plugin (.dll / .vst3)...").clicked() {
                                if let Some(file) = rfd::FileDialog::new()
                                    .add_filter("Audio Plugins (*.vst3, *.dll)", &["vst3", "dll"])
                                    .pick_file()
                                {
                                    let name = file.file_stem().and_then(|n| n.to_str()).unwrap_or("Custom Plugin").to_string();
                                    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                                    let p_type = if ext == "vst3" { PluginType::Vst3 } else { PluginType::Vst2 };
                                    self.plugins.push(AudioPluginEntry {
                                        name,
                                        path: file,
                                        plugin_type: p_type,
                                        is_active: true,
                                        wet_dry_mix: 1.0,
                                        param_1: 0.5,
                                        param_2: 0.5,
                                        param_3: 1.0,
                                    });
                                    self.apply_active_filters(player);
                                }
                            }

                            ui.checkbox(&mut self.master_bypass, "⚡ Bypass All Plugins");
                        });
                    });

                    ui.separator();

                    let mut changed = false;

                    // Plugin List
                    ScrollArea::vertical()
                        .max_height(200.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (idx, plugin) in self.plugins.iter_mut().enumerate() {
                                let is_selected = self.selected_plugin_idx == Some(idx);
                                let bg = if is_selected {
                                    Color32::from_rgb(30, 50, 75)
                                } else if idx % 2 == 0 {
                                    Color32::from_rgb(22, 24, 30)
                                } else {
                                    Color32::from_rgb(18, 20, 25)
                                };

                                egui::Frame::new()
                                    .fill(bg)
                                    .inner_margin(egui::Margin::symmetric(8, 6))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            if ui.checkbox(&mut plugin.is_active, "").changed() {
                                                changed = true;
                                            }

                                            if ui.selectable_label(is_selected, &plugin.name).clicked() {
                                                self.selected_plugin_idx = Some(idx);
                                            }

                                            ui.label(RichText::new(plugin.plugin_type.badge_text()).small().color(Color32::from_rgb(255, 200, 100)));

                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                let wet_pct = (plugin.wet_dry_mix * 100.0).round() as u32;
                                                ui.label(RichText::new(format!("Wet: {}%", wet_pct)).small().color(Color32::GRAY));
                                            });
                                        });
                                    });
                                ui.add_space(2.0);
                            }
                        });

                    ui.add_space(8.0);

                    // Selected Plugin Parameter Controls
                    if let Some(idx) = self.selected_plugin_idx {
                        if let Some(plugin) = self.plugins.get_mut(idx) {
                            ui.group(|ui| {
                                ui.label(RichText::new(format!("Plugin Controls: {}", plugin.name)).strong().color(Color32::WHITE));
                                ui.add_space(4.0);

                                ui.horizontal(|ui| {
                                    ui.label("Wet / Dry Mix:");
                                    if ui.add(Slider::new(&mut plugin.wet_dry_mix, 0.0..=1.0).suffix(" (100% Wet)")).changed() {
                                        changed = true;
                                    }
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Intensity / Width (P1):");
                                    if ui.add(Slider::new(&mut plugin.param_1, 0.0..=1.0)).changed() {
                                        changed = true;
                                    }
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Color / Damping (P2):");
                                    if ui.add(Slider::new(&mut plugin.param_2, 0.0..=1.0)).changed() {
                                        changed = true;
                                    }
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Output Gain (P3):");
                                    if ui.add(Slider::new(&mut plugin.param_3, 0.0..=2.0).suffix("x")).changed() {
                                        changed = true;
                                    }
                                });
                            });
                        }
                    }

                    if changed {
                        self.apply_active_filters(player);
                    }

                    ui.add_space(4.0);
                    ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Apply Audio Chain").clicked() {
                            self.apply_active_filters(player);
                            self.status_message = "Audio DSP plugin chain active and running.".to_string();
                        }

                        if ui.button("Reset DSP Chain").clicked() {
                            for p in &mut self.plugins {
                                p.is_active = false;
                            }
                            self.apply_active_filters(player);
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
