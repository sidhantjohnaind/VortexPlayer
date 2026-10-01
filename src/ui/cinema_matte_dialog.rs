//! cinema_matte_dialog.rs — Cinema Screen Matte & Ambient Curtain Studio (madVR/Theater Scope)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinemaMattePreset {
    Off,
    CinemaScope235,  // 2.35:1
    CinemaScope239,  // 2.39:1
    UltraPanavision, // 2.76:1
    ImaxExpanded,    // 1.43:1 / 1.90:1
    Pillarbox43,     // 4:3
    Custom,
}

impl CinemaMattePreset {
    pub fn display_name(&self) -> &'static str {
        match self {
            CinemaMattePreset::Off => "Off (Full Screen Passthrough)",
            CinemaMattePreset::CinemaScope235 => "🎬 2.35:1 CinemaScope Anamorphic",
            CinemaMattePreset::CinemaScope239 => "🎬 2.39:1 DCI Theatrical Widescreen",
            CinemaMattePreset::UltraPanavision => "🎞️ 2.76:1 Ultra Panavision 70 (Ben-Hur / Hateful 8)",
            CinemaMattePreset::ImaxExpanded => "🌌 1.43:1 / 1.90:1 IMAX Aspect Ratio",
            CinemaMattePreset::Pillarbox43 => "📺 4:3 Vintage Academy Ratio Mask",
            CinemaMattePreset::Custom => "⚙️ Custom Matte Curtains",
        }
    }
}

pub struct CinemaMatteDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub preset: CinemaMattePreset,
    pub top_curtain_pct: f32,    // 0.0 to 30.0%
    pub bottom_curtain_pct: f32, // 0.0 to 30.0%
    pub left_curtain_pct: f32,   // 0.0 to 30.0%
    pub right_curtain_pct: f32,  // 0.0 to 30.0%
    pub curtain_opacity: f32,    // 0.5 to 1.0
    pub edge_feather_px: f32,    // 0 to 20 px
    pub ambient_bias_glow: bool,
    pub status_message: String,
}

impl Default for CinemaMatteDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            preset: CinemaMattePreset::CinemaScope239,
            top_curtain_pct: 12.5,
            bottom_curtain_pct: 12.5,
            left_curtain_pct: 0.0,
            right_curtain_pct: 0.0,
            curtain_opacity: 1.0,
            edge_feather_px: 2.0,
            ambient_bias_glow: false,
            status_message: "Cinema screen matte curtains ready.".to_string(),
        }
    }
}

impl CinemaMatteDialog {
    pub fn new() -> Self { Self::default() }

    fn apply_preset(&mut self) {
        match self.preset {
            CinemaMattePreset::CinemaScope235 => {
                self.top_curtain_pct = 12.0; self.bottom_curtain_pct = 12.0;
                self.left_curtain_pct = 0.0; self.right_curtain_pct = 0.0;
            }
            CinemaMattePreset::CinemaScope239 => {
                self.top_curtain_pct = 13.0; self.bottom_curtain_pct = 13.0;
                self.left_curtain_pct = 0.0; self.right_curtain_pct = 0.0;
            }
            CinemaMattePreset::UltraPanavision => {
                self.top_curtain_pct = 18.0; self.bottom_curtain_pct = 18.0;
                self.left_curtain_pct = 0.0; self.right_curtain_pct = 0.0;
            }
            CinemaMattePreset::Pillarbox43 => {
                self.top_curtain_pct = 0.0; self.bottom_curtain_pct = 0.0;
                self.left_curtain_pct = 12.5; self.right_curtain_pct = 12.5;
            }
            _ => {}
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("🎭 Cinema Screen Matte & Ambient Curtains")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(600.0, 420.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Cinema Scope Matte Masking & Curtains").strong().size(15.0).color(Color32::from_rgb(255, 200, 120)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_active {
                            ui.label(RichText::new("● MATTE ACTIVE").color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● OFF").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    ui.checkbox(&mut self.is_active, "🎭 Enable Screen Masking Curtains");
                    ui.horizontal(|ui| {
                        ui.label("Aspect Ratio Preset:");
                        egui::ComboBox::from_id_salt("cinema_matte_preset")
                            .selected_text(self.preset.display_name())
                            .show_ui(ui, |ui| {
                                for p in [
                                    CinemaMattePreset::CinemaScope239,
                                    CinemaMattePreset::CinemaScope235,
                                    CinemaMattePreset::UltraPanavision,
                                    CinemaMattePreset::ImaxExpanded,
                                    CinemaMattePreset::Pillarbox43,
                                    CinemaMattePreset::Custom,
                                    CinemaMattePreset::Off,
                                ] {
                                    if ui.selectable_value(&mut self.preset, p, p.display_name()).clicked() {
                                        self.apply_preset();
                                    }
                                }
                            });
                    });
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Curtain Dimension Margins (%)").strong());
                    if ui.add(Slider::new(&mut self.top_curtain_pct, 0.0..=30.0).text("Top Mask").suffix("%")).changed() { self.preset = CinemaMattePreset::Custom; }
                    if ui.add(Slider::new(&mut self.bottom_curtain_pct, 0.0..=30.0).text("Bottom Mask").suffix("%")).changed() { self.preset = CinemaMattePreset::Custom; }
                    if ui.add(Slider::new(&mut self.left_curtain_pct, 0.0..=30.0).text("Left Mask").suffix("%")).changed() { self.preset = CinemaMattePreset::Custom; }
                    if ui.add(Slider::new(&mut self.right_curtain_pct, 0.0..=30.0).text("Right Mask").suffix("%")).changed() { self.preset = CinemaMattePreset::Custom; }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Curtain Aesthetics & Edge Blending").strong());
                    ui.add(Slider::new(&mut self.curtain_opacity, 0.2..=1.0).text("Blackout Opacity"));
                    ui.add(Slider::new(&mut self.edge_feather_px, 0.0..=20.0).text("Edge Softness / Feather").suffix(" px"));
                    ui.checkbox(&mut self.ambient_bias_glow, "Enable Ambient Edge Bias Lighting (Subtle Screen Glow)");
                });

                ui.add_space(4.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("↺ Reset").clicked() { *self = Self { is_open: true, ..Self::default() }; }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() { self.is_open = false; }
                    });
                });
            });
        self.is_open = open;
    }
}
