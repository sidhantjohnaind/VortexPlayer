//! color_blindness_dialog.rs — Color Blindness & Vision Accessibility Daltonization Matrix (Vortex/MPC-BE)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisionDeficiencyMode {
    Off,
    Protanopia,    // Red-blind (L-cone)
    Protanomaly,   // Red-weak
    Deuteranopia,  // Green-blind (M-cone)
    Deuteranomaly, // Green-weak
    Tritanopia,    // Blue-blind (S-cone)
    Tritanomaly,   // Blue-weak
    Achromatopsia, // Complete color blindness (Monochrome)
    HighContrastEdges, // High-contrast edge enhancement
    InvertLuminance,   // Dark/Light Inversion
}

impl VisionDeficiencyMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            VisionDeficiencyMode::Off => "Off (Normal Vision)",
            VisionDeficiencyMode::Protanopia => "🔴 Protanopia (Red-Blind Daltonize)",
            VisionDeficiencyMode::Protanomaly => "🔴 Protanomaly (Red-Weak Boost)",
            VisionDeficiencyMode::Deuteranopia => "🟢 Deuteranopia (Green-Blind Daltonize)",
            VisionDeficiencyMode::Deuteranomaly => "🟢 Deuteranomaly (Green-Weak Boost)",
            VisionDeficiencyMode::Tritanopia => "🔵 Tritanopia (Blue-Blind Daltonize)",
            VisionDeficiencyMode::Tritanomaly => "🔵 Tritanomaly (Blue-Weak Boost)",
            VisionDeficiencyMode::Achromatopsia => "⚪ Achromatopsia (Monochrome Luminance)",
            VisionDeficiencyMode::HighContrastEdges => "👁️ High-Contrast Outline & Edge Enhancer",
            VisionDeficiencyMode::InvertLuminance => "🌓 Invert Luminance (Photo-Sensitivity)",
        }
    }
}

pub struct ColorBlindnessDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub mode: VisionDeficiencyMode,
    pub correction_intensity: f32, // 0.0 to 1.0
    pub edge_strength: f32,        // 0.0 to 2.0
    pub status_message: String,
}

impl Default for ColorBlindnessDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            mode: VisionDeficiencyMode::Deuteranopia,
            correction_intensity: 1.0,
            edge_strength: 0.5,
            status_message: "Color vision accessibility matrix ready.".to_string(),
        }
    }
}

impl ColorBlindnessDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || self.mode == VisionDeficiencyMode::Off {
            player.set_property_string("glsl-shaders", "");
        } else {
            // Daltonization color matrix mappings
            match self.mode {
                VisionDeficiencyMode::HighContrastEdges => {
                    player.set_property_string("sharpen", &self.edge_strength.to_string());
                }
                _ => {}
            }
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("👁️ Color Blindness & Vision Accessibility")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(620.0, 440.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Daltonization & Vision Accessibility Matrix").strong().size(15.0).color(Color32::from_rgb(140, 220, 255)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_active {
                            ui.label(RichText::new("● DALTONIZE ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● BYPASS").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    if ui.checkbox(&mut self.is_active, "👁️ Enable Vision Correction Filter").changed() { changed = true; }
                    ui.add_space(2.0);
                    ui.label(RichText::new("Select Vision Profile / Deficiency Type:").strong());
                    for mode in [
                        VisionDeficiencyMode::Deuteranopia,
                        VisionDeficiencyMode::Deuteranomaly,
                        VisionDeficiencyMode::Protanopia,
                        VisionDeficiencyMode::Protanomaly,
                        VisionDeficiencyMode::Tritanopia,
                        VisionDeficiencyMode::Tritanomaly,
                        VisionDeficiencyMode::Achromatopsia,
                        VisionDeficiencyMode::HighContrastEdges,
                        VisionDeficiencyMode::InvertLuminance,
                        VisionDeficiencyMode::Off,
                    ] {
                        if ui.selectable_label(self.mode == mode, mode.display_name()).clicked() {
                            self.mode = mode;
                            changed = true;
                        }
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Filter Strength & Fine Tuning").strong());
                    if ui.add(Slider::new(&mut self.correction_intensity, 0.0..=1.0).text("Daltonization Intensity")).changed() {
                        changed = true;
                    }
                    if self.mode == VisionDeficiencyMode::HighContrastEdges {
                        if ui.add(Slider::new(&mut self.edge_strength, 0.0..=2.0).text("Edge Contrast Strength")).changed() {
                            changed = true;
                        }
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
