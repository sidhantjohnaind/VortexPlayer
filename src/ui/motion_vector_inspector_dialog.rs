//! motion_vector_inspector_dialog.rs — Motion Vector & Codec Bitstream Macroblock Inspector (VLC/FFmpeg debug)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorVisualizationMode {
    Off,
    ForwardPVector,
    BackwardBVector,
    AllMotionVectors,
    MacroblockQuantizationHeatmap,
    FrameTypeOverlay,
}

impl VectorVisualizationMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            VectorVisualizationMode::Off => "Off (Normal Video Frame)",
            VectorVisualizationMode::ForwardPVector => "➡️ Forward P-Frame Motion Vectors",
            VectorVisualizationMode::BackwardBVector => "⬅️ Backward B-Frame Motion Vectors",
            VectorVisualizationMode::AllMotionVectors => "↔️ All Bi-directional Motion Vectors (P + B)",
            VectorVisualizationMode::MacroblockQuantizationHeatmap => "🔥 Macroblock Quantization (QP) Heatmap",
            VectorVisualizationMode::FrameTypeOverlay => "🏷️ Real-Time I/P/B Frame Type Identifier",
        }
    }
}

pub struct MotionVectorInspectorDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub mode: VectorVisualizationMode,
    pub show_gop_structure: bool,
    pub current_frame_type: &'static str,
    pub current_qp_average: f32,
    pub motion_energy: f32,
    pub status_message: String,
}

impl Default for MotionVectorInspectorDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            mode: VectorVisualizationMode::AllMotionVectors,
            show_gop_structure: true,
            current_frame_type: "B-Frame",
            current_qp_average: 22.4,
            motion_energy: 14.8,
            status_message: "Motion vector & bitstream diagnostics ready.".to_string(),
        }
    }
}

impl MotionVectorInspectorDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || self.mode == VectorVisualizationMode::Off {
            player.set_property_string("vf", "");
        } else {
            match self.mode {
                VectorVisualizationMode::ForwardPVector => {
                    player.set_property_string("vf", "lavfi=[codecview=mv=pf]");
                }
                VectorVisualizationMode::BackwardBVector => {
                    player.set_property_string("vf", "lavfi=[codecview=mv=bf]");
                }
                VectorVisualizationMode::AllMotionVectors => {
                    player.set_property_string("vf", "lavfi=[codecview=mv=pf+bf+bb]");
                }
                VectorVisualizationMode::MacroblockQuantizationHeatmap => {
                    player.set_property_string("vf", "lavfi=[codecview=qp=true]");
                }
                _ => {}
            }
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player, stats: &crate::engine::MediaStats) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("🔬 Motion Vector & Codec Inspector")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(620.0, 440.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Codec Bitstream & Motion Vector Inspector").strong().size(15.0).color(Color32::from_rgb(100, 240, 220)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_active {
                            ui.label(RichText::new("● INSPECTING").color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● BYPASS").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    if ui.checkbox(&mut self.is_active, "🔬 Enable Motion Vector Overlay").changed() { changed = true; }
                    ui.add_space(2.0);
                    ui.label(RichText::new("Visualization Mode:").strong());
                    for m in [
                        VectorVisualizationMode::AllMotionVectors,
                        VectorVisualizationMode::ForwardPVector,
                        VectorVisualizationMode::BackwardBVector,
                        VectorVisualizationMode::MacroblockQuantizationHeatmap,
                        VectorVisualizationMode::FrameTypeOverlay,
                        VectorVisualizationMode::Off,
                    ] {
                        if ui.selectable_label(self.mode == m, m.display_name()).clicked() {
                            self.mode = m;
                            changed = true;
                        }
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Real-Time Bitstream Telemetry").strong());
                    ui.columns(4, |cols| {
                        cols[0].group(|ui| {
                            ui.label(RichText::new("Frame Type").small().color(Color32::GRAY));
                            ui.label(RichText::new(self.current_frame_type).strong().size(16.0).color(Color32::from_rgb(255, 200, 100)));
                        });
                        cols[1].group(|ui| {
                            ui.label(RichText::new("Avg Quant (QP)").small().color(Color32::GRAY));
                            ui.label(RichText::new(format!("{:.1}", self.current_qp_average)).strong().size(16.0).color(Color32::from_rgb(140, 220, 180)));
                        });
                        cols[2].group(|ui| {
                            ui.label(RichText::new("Motion Energy").small().color(Color32::GRAY));
                            ui.label(RichText::new(format!("{:.1}", self.motion_energy)).strong().size(16.0).color(Color32::from_rgb(100, 200, 255)));
                        });
                        cols[3].group(|ui| {
                            ui.label(RichText::new("Codec").small().color(Color32::GRAY));
                            let codec = if stats.video_codec.is_empty() { "H.264 / HEVC" } else { &stats.video_codec };
                            ui.label(RichText::new(codec).strong().size(14.0).color(Color32::WHITE));
                        });
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
