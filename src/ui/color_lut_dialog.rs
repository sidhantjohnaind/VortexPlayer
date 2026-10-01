//! color_lut_dialog.rs — 3D LUT (.cube/.3dl) & Color Gamut Calibration Studio (madVR/MPC-BE style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LutPreset {
    None,
    Technicolor,
    FilmNoir,
    WarmVintage,
    TealOrange,
    BleachBypass,
    HdrToSdrBt709,
    Rec2020ToDciP3,
    Custom,
}

impl LutPreset {
    pub fn display_name(&self) -> &'static str {
        match self {
            LutPreset::None => "None (Pass-through)",
            LutPreset::Technicolor => "🎞️ Technicolor 3-Strip Classic",
            LutPreset::FilmNoir => "🎬 Film Noir High Contrast Monochrome",
            LutPreset::WarmVintage => "🌅 Warm Vintage Kodak Gold",
            LutPreset::TealOrange => "🏖️ Hollywood Teal & Orange Blockbuster",
            LutPreset::BleachBypass => "⚔️ Silver Retention / Bleach Bypass",
            LutPreset::HdrToSdrBt709 => "📺 HDR10 BT.2020 → SDR BT.709 Tone Map",
            LutPreset::Rec2020ToDciP3 => "🎨 Rec.2020 → DCI-P3 Cinema Gamut Clamp",
            LutPreset::Custom => "📁 Custom .cube / .3dl LUT File",
        }
    }
}

pub struct ColorLutDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub preset: LutPreset,
    pub custom_lut_path: Option<PathBuf>,
    pub lut_strength: f32, // 0.0 to 1.0
    pub gamut_mapping: String,
    pub status_message: String,
}

impl Default for ColorLutDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            preset: LutPreset::None,
            custom_lut_path: None,
            lut_strength: 1.0,
            gamut_mapping: "auto".to_string(),
            status_message: "3D LUT studio ready. Load custom .cube files or select cinematographic presets.".to_string(),
        }
    }
}

impl ColorLutDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || self.preset == LutPreset::None {
            player.set_property_string("lut", "");
        } else if let Some(path) = &self.custom_lut_path {
            player.set_property_string("lut", &path.to_string_lossy());
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("🎨 3D LUT Color Studio & Gamut Calibration")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(620.0, 420.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("3D Look-Up Table (LUT) & Color Calibration").strong().size(15.0).color(Color32::from_rgb(255, 180, 100)));
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
                    if ui.checkbox(&mut self.is_active, "🎨 Enable 3D LUT Color Grading").changed() { changed = true; }
                    ui.horizontal(|ui| {
                        ui.label("LUT Preset:");
                        egui::ComboBox::from_id_salt("lut_preset_combo")
                            .selected_text(self.preset.display_name())
                            .show_ui(ui, |ui| {
                                for p in [
                                    LutPreset::None,
                                    LutPreset::Technicolor,
                                    LutPreset::FilmNoir,
                                    LutPreset::WarmVintage,
                                    LutPreset::TealOrange,
                                    LutPreset::BleachBypass,
                                    LutPreset::HdrToSdrBt709,
                                    LutPreset::Rec2020ToDciP3,
                                    LutPreset::Custom,
                                ] {
                                    if ui.selectable_value(&mut self.preset, p, p.display_name()).clicked() {
                                        changed = true;
                                    }
                                }
                            });
                    });
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Custom LUT File (.cube / .3dl)").strong());
                    ui.horizontal(|ui| {
                        let path_str = self.custom_lut_path.as_ref().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|| "(no file selected)".to_string());
                        ui.label(RichText::new(path_str).small().color(Color32::GRAY));
                        if ui.button("Browse...").clicked() {
                            if let Some(file) = rfd::FileDialog::new().add_filter("3D LUT Files", &["cube", "3dl"]).pick_file() {
                                self.custom_lut_path = Some(file);
                                self.preset = LutPreset::Custom;
                                changed = true;
                            }
                        }
                    });

                    ui.add_space(4.0);
                    if ui.add(Slider::new(&mut self.lut_strength, 0.0..=1.0).text("LUT Intensity / Blend Strength")).changed() {
                        changed = true;
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Gamut & Tone Mapping Target").strong());
                    ui.horizontal(|ui| {
                        for target in ["auto", "clip", "desaturate", "warn"] {
                            if ui.selectable_label(self.gamut_mapping == target, target).clicked() {
                                self.gamut_mapping = target.to_string();
                                player.set_property_string("gamut-mapping-mode", target);
                            }
                        }
                    });
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
