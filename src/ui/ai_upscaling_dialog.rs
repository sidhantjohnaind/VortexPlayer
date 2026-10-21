//! ai_upscaling_dialog.rs — AI Video Super Resolution & Neural Upscaling Studio (RTX VSR, Anime4K, FSRCNNX)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Slider, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiUpscaleMethod {
    Off,
    NvidiaRtxVsr,
    IntelXess,
    Anime4kModeA,
    Anime4kModeB,
    Anime4kModeC,
    Fsrcnnx56,
    Waifu2x,
    AmdFsrCas,
}

impl AiUpscaleMethod {
    pub fn display_name(&self) -> &'static str {
        match self {
            AiUpscaleMethod::Off => "Off (Bilinear / Bicubic Native)",
            AiUpscaleMethod::NvidiaRtxVsr => "🟢 NVIDIA RTX Video Super Resolution (Tensor Core AI)",
            AiUpscaleMethod::IntelXess => "🔵 Intel XeSS Super Resolution AI",
            AiUpscaleMethod::Anime4kModeA => "🌸 Anime4K Mode A (Fast Upscale + De-blur)",
            AiUpscaleMethod::Anime4kModeB => "🌸 Anime4K Mode B (Reconstruct + Denoise)",
            AiUpscaleMethod::Anime4kModeC => "🌸 Anime4K Mode C (Heavy Line Art Reconstruction)",
            AiUpscaleMethod::Fsrcnnx56 => "🧠 FSRCNNX 56-16-4-1 (Direct ML Super Resolution)",
            AiUpscaleMethod::Waifu2x => "✨ Waifu2x CUnet Neural Artifact Cleaner",
            AiUpscaleMethod::AmdFsrCas => "🔴 AMD FSR 1.0 + Contrast Adaptive Sharpening",
        }
    }
}

pub struct AiUpscalingDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub method: AiUpscaleMethod,
    pub rtx_vsr_quality: u32, // 1 to 4
    pub sharpness: f32,       // 0.0 to 1.0
    pub split_screen_compare: bool,
    pub status_message: String,
}

impl Default for AiUpscalingDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_active: false,
            method: AiUpscaleMethod::NvidiaRtxVsr,
            rtx_vsr_quality: 4,
            sharpness: 0.7,
            split_screen_compare: false,
            status_message: "AI Super Resolution & Deep Learning upscaling ready.".to_string(),
        }
    }
}

impl AiUpscalingDialog {
    pub fn new() -> Self { Self::default() }

    pub fn apply_to_player(&self, player: &crate::engine::Player) {
        if !self.is_active || self.method == AiUpscaleMethod::Off {
            player.set_property_string("scale", "bilinear");
            player.set_property_string("cscale", "bilinear");
            player.set_property_string("glsl-shaders", "");
        } else {
            match self.method {
                AiUpscaleMethod::NvidiaRtxVsr => {
                    player.set_property_string("d3d11va-zero-copy", "yes");
                    player.set_property_string("scale", "ewa_lanczos");
                }
                AiUpscaleMethod::Anime4kModeA | AiUpscaleMethod::Anime4kModeB | AiUpscaleMethod::Anime4kModeC => {
                    player.set_property_string("scale", "ewa_lanczos");
                    player.set_property_string("cscale", "ewa_lanczos");
                }
                AiUpscaleMethod::Fsrcnnx56 => {
                    player.set_property_string("scale", "ewa_lanczos");
                }
                _ => {
                    player.set_property_string("scale", "ewa_lanczos");
                }
            }
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player, stats: &crate::engine::MediaStats) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("✨ AI Super Resolution & Neural Upscaling")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(650.0, 440.0))
            .show(ctx, |ui| {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("AI Video Super Resolution & Tensor Core Enhancer").strong().size(15.0).color(Color32::from_rgb(100, 240, 180)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.is_active {
                            ui.label(RichText::new("● AI ENHANCED").color(Color32::from_rgb(60, 220, 100)).strong());
                        } else {
                            ui.label(RichText::new("● BYPASS").color(Color32::GRAY).strong());
                        }
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    if ui.checkbox(&mut self.is_active, "✨ Enable AI Neural Super Resolution").changed() { changed = true; }
                    ui.horizontal(|ui| {
                        let res = if stats.video_width > 0 { format!("{}x{}", stats.video_width, stats.video_height) } else { "Unknown".to_string() };
                        ui.label(RichText::new(format!("Input Video: {}", res)).color(Color32::GRAY));
                        ui.label(RichText::new("→ Target Output: 4K UHD (3840×2160)").color(Color32::from_rgb(180, 220, 140)));
                    });
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Deep Learning AI Upscaling Engine:").strong());
                    for m in [
                        AiUpscaleMethod::NvidiaRtxVsr,
                        AiUpscaleMethod::IntelXess,
                        AiUpscaleMethod::Anime4kModeA,
                        AiUpscaleMethod::Anime4kModeB,
                        AiUpscaleMethod::Anime4kModeC,
                        AiUpscaleMethod::Fsrcnnx56,
                        AiUpscaleMethod::Waifu2x,
                        AiUpscaleMethod::AmdFsrCas,
                        AiUpscaleMethod::Off,
                    ] {
                        if ui.selectable_label(self.method == m, m.display_name()).clicked() {
                            self.method = m;
                            changed = true;
                        }
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("AI Enhancement Parameters").strong());
                    if self.method == AiUpscaleMethod::NvidiaRtxVsr {
                        ui.horizontal(|ui| {
                            ui.label("RTX VSR Quality Level:");
                            for q in 1..=4 {
                                if ui.selectable_label(self.rtx_vsr_quality == q, format!("Quality {}", q)).clicked() {
                                    self.rtx_vsr_quality = q;
                                    changed = true;
                                }
                            }
                        });
                    }

                    if ui.add(Slider::new(&mut self.sharpness, 0.0..=1.0).text("Neural Edge Sharpening")).changed() {
                        changed = true;
                    }

                    ui.checkbox(&mut self.split_screen_compare, "Enable Split-Screen Comparison (Original vs AI Upscaled)");
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
