#![allow(dead_code)]

use eframe::egui::{self, Color32, FontId, RichText, Vec2};
use std::fs;

pub struct ShaderStudioDialog {
    pub is_open: bool,
    pub shader_code: String,
    pub selected_preset: String,
    pub status: String,
}

impl Default for ShaderStudioDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            shader_code: r#"//!HOOK MAIN
//!BIND HOOKED
//!DESC Custom Vortex Invert Shader

vec4 hook() {
    vec4 color = HOOKED_tex(HOOKED_pos);
    return vec4(1.0 - color.rgb, color.a);
}"#.to_string(),
            selected_preset: "Invert Colors".to_string(),
            status: String::new(),
        }
    }
}

impl ShaderStudioDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("🎨 Pixel Shader Studio & Code Editor")
            .open(&mut open)
            .collapsible(false)
            .default_size(Vec2::new(560.0, 420.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("GLSL / HLSL Video Shader Editor").strong().color(Color32::from_rgb(180, 140, 255)));
                        ui.add_space(10.0);

                        ui.label("Preset:");
                        egui::ComboBox::from_id_salt("shader_preset_combo")
                            .selected_text(&self.selected_preset)
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut self.selected_preset, "Invert Colors".to_string(), "Invert Colors").clicked() {
                                    self.shader_code = r#"//!HOOK MAIN
//!BIND HOOKED
//!DESC Invert Colors
vec4 hook() {
    vec4 color = HOOKED_tex(HOOKED_pos);
    return vec4(1.0 - color.rgb, color.a);
}"#.to_string();
                                }
                                if ui.selectable_value(&mut self.selected_preset, "CRT Scanlines".to_string(), "CRT Scanlines").clicked() {
                                    self.shader_code = r#"//!HOOK MAIN
//!BIND HOOKED
//!DESC CRT Scanlines
vec4 hook() {
    vec4 color = HOOKED_tex(HOOKED_pos);
    float scanline = sin(HOOKED_pos.y * 800.0) * 0.15;
    return vec4(color.rgb - scanline, color.a);
}"#.to_string();
                                }
                                if ui.selectable_value(&mut self.selected_preset, "Sepia Film".to_string(), "Sepia Film").clicked() {
                                    self.shader_code = r#"//!HOOK MAIN
//!BIND HOOKED
//!DESC Sepia Film
vec4 hook() {
    vec4 color = HOOKED_tex(HOOKED_pos);
    float r = dot(color.rgb, vec3(0.393, 0.769, 0.189));
    float g = dot(color.rgb, vec3(0.349, 0.686, 0.168));
    float b = dot(color.rgb, vec3(0.272, 0.534, 0.131));
    return vec4(r, g, b, color.a);
}"#.to_string();
                                }
                            });
                    });

                    ui.add_space(6.0);

                    // Code text area
                    egui::ScrollArea::vertical().id_salt("shader_code_scroll").max_height(260.0).show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.shader_code)
                                .font(FontId::monospace(11.0))
                                .code_editor()
                                .desired_width(f32::INFINITY)
                        );
                    });

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new(RichText::new("⚡ Compile & Apply to Video").color(Color32::WHITE)).fill(Color32::from_rgb(80, 40, 170))).clicked() {
                            let tmp_dir = std::env::temp_dir();
                            let shader_file = tmp_dir.join("vortex_custom_shader.hook");
                            if fs::write(&shader_file, &self.shader_code).is_ok() {
                                player.apply_custom_glsl_shader(&shader_file.to_string_lossy());
                                self.status = "Shader compiled & active in video pipeline!".to_string();
                            } else {
                                self.status = "Failed to write temporary shader file.".to_string();
                            }
                        }

                        if ui.button("❌ Remove Custom Shader").clicked() {
                            player.apply_custom_glsl_shader("");
                            self.status = "Custom shader cleared.".to_string();
                        }
                    });

                    if !self.status.is_empty() {
                        ui.add_space(4.0);
                        ui.label(RichText::new(&self.status).color(Color32::from_rgb(100, 220, 140)));
                    }
                });
            });
        self.is_open = open;
    }
}
