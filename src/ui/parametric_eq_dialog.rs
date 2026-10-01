use super::theme::VortexTheme;
use crate::engine::parametric_eq::{FilterType, ParametricEqConfig, ParametricNode};
use crate::engine::Player;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Vec2};

pub struct ParametricEqDialog {
    pub import_text: String,
    pub show_import_modal: bool,
}

impl Default for ParametricEqDialog {
    fn default() -> Self {
        Self {
            import_text: String::new(),
            show_import_modal: false,
        }
    }
}

impl ParametricEqDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        config: &mut ParametricEqConfig,
        player: &Player,
    ) -> Option<Rect> {
        if !*is_open {
            return None;
        }

        let resp = egui::Window::new("Parametric Equalizer (PEQ)")
            .open(is_open)
            .resizable(false)
            .default_width(620.0)
            .default_height(480.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let mut en = config.enabled;
                    if ui.checkbox(&mut en, "Enable Parametric EQ").changed() {
                        config.enabled = en;
                        let af_str = config.build_audio_filter_string();
                        player.set_audio_filter(&af_str);
                    }

                    ui.label(RichText::new(&config.profile_name).color(VortexTheme::POT_YELLOW).strong());

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("📥 Import AutoEQ / APO").clicked() {
                            self.show_import_modal = true;
                        }
                        if ui.button("➕ Add Node").clicked() {
                            config.nodes.push(ParametricNode::default());
                        }
                    });
                });

                ui.separator();

                // ── 1. Interactive Frequency Response Graph Canvas ────────────
                let (canvas_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 160.0), Sense::hover());
                let painter = ui.painter();

                painter.rect_filled(canvas_rect, CornerRadius::same(3), Color32::from_rgb(12, 13, 16));
                painter.rect_stroke(canvas_rect, CornerRadius::same(3), Stroke::new(1.0, Color32::from_rgb(32, 35, 44)), StrokeKind::Inside);

                // Zero line (0 dB)
                let zero_y = canvas_rect.center().y;
                painter.line_segment(
                    [Pos2::new(canvas_rect.left(), zero_y), Pos2::new(canvas_rect.right(), zero_y)],
                    Stroke::new(1.0, Color32::from_rgb(45, 50, 65)),
                );

                // Grid lines & labels (20Hz, 100Hz, 1kHz, 10kHz, 20kHz)
                let grid_freqs: [(f64, &str); 5] = [(20.0, "20Hz"), (100.0, "100Hz"), (1000.0, "1kHz"), (10000.0, "10kHz"), (20000.0, "20kHz")];
                for &(f, label) in &grid_freqs {
                    let frac = (f.log10() - 20.0_f64.log10()) / (20000.0_f64.log10() - 20.0_f64.log10());
                    let x = canvas_rect.left() + (frac as f32 * canvas_rect.width());
                    painter.line_segment(
                        [Pos2::new(x, canvas_rect.top()), Pos2::new(x, canvas_rect.bottom())],
                        Stroke::new(1.0, Color32::from_rgb(24, 26, 34)),
                    );
                    painter.text(Pos2::new(x, canvas_rect.bottom() - 10.0), Align2::CENTER_CENTER, label, FontId::monospace(8.5), Color32::from_rgb(80, 85, 100));
                }

                // Draw Nodes
                for (idx, node) in config.nodes.iter().enumerate() {
                    if !node.enabled { continue; }
                    let frac_x = ((node.freq.log10() - 20.0_f64.log10()) / (20000.0_f64.log10() - 20.0_f64.log10())).clamp(0.0, 1.0);
                    let node_x = canvas_rect.left() + (frac_x as f32 * canvas_rect.width());
                    let frac_y = (node.gain.clamp(-24.0, 24.0) / 24.0) as f32;
                    let node_y = zero_y - (frac_y * (canvas_rect.height() / 2.0 - 12.0));

                    let pt = Pos2::new(node_x, node_y);
                    painter.circle_filled(pt, 5.0, VortexTheme::POT_YELLOW);
                    painter.circle_stroke(pt, 5.0, Stroke::new(1.5, Color32::WHITE));
                    painter.text(pt - Vec2::new(0.0, 9.0), Align2::CENTER_CENTER, format!("#{}", idx + 1), FontId::monospace(9.0), Color32::WHITE);
                }

                ui.separator();

                // ── 2. Nodes Table with Real-time Sliders ───────────────────────
                let mut changed = false;
                let mut node_to_remove = None;

                egui::ScrollArea::vertical().id_salt("peq_nodes_scroll").max_height(200.0).show(ui, |ui| {
                    for (i, node) in config.nodes.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(format!("#{}", i + 1)).strong().color(VortexTheme::POT_YELLOW));
                            if ui.checkbox(&mut node.enabled, "").changed() { changed = true; }

                            egui::ComboBox::from_id_salt(format!("peq_type_{}", i))
                                .selected_text(node.filter_type.display_name())
                                .show_ui(ui, |ui| {
                                    for t in FilterType::ALL {
                                        if ui.selectable_value(&mut node.filter_type, t, t.display_name()).clicked() {
                                            changed = true;
                                        }
                                    }
                                });

                            ui.label("Freq:");
                            if ui.add(egui::DragValue::new(&mut node.freq).range(20.0..=20000.0).suffix("Hz").speed(5.0)).changed() {
                                changed = true;
                            }

                            ui.label("Gain:");
                            if ui.add(egui::DragValue::new(&mut node.gain).range(-24.0..=24.0).suffix("dB").speed(0.1)).changed() {
                                changed = true;
                            }

                            ui.label("Q:");
                            if ui.add(egui::DragValue::new(&mut node.q).range(0.1..=10.0).speed(0.05)).changed() {
                                changed = true;
                            }

                            if ui.button("🗑").clicked() {
                                node_to_remove = Some(i);
                            }
                        });
                    }
                });

                if let Some(idx) = node_to_remove {
                    config.nodes.remove(idx);
                    changed = true;
                }

                if changed {
                    let af_str = config.build_audio_filter_string();
                    player.set_audio_filter(&af_str);
                }
            });

        // AutoEQ Importer Modal
        let mut close_modal = false;
        let mut apply_import = false;
        if self.show_import_modal {
            let mut is_open = true;
            egui::Window::new("Import AutoEQ / Equalizer APO Profile")
                .open(&mut is_open)
                .default_width(420.0)
                .show(ctx, |ui| {
                    ui.label("Paste your AutoEQ or Equalizer APO text configuration below:");
                    ui.text_edit_multiline(&mut self.import_text);
                    ui.horizontal(|ui| {
                        if ui.button("Apply Profile").clicked() {
                            apply_import = true;
                            close_modal = true;
                        }
                        if ui.button("Cancel").clicked() {
                            close_modal = true;
                        }
                    });
                });
            if !is_open || close_modal {
                self.show_import_modal = false;
            }
            if apply_import {
                if let Some(parsed) = ParametricEqConfig::parse_autoeq(&self.import_text) {
                    *config = parsed;
                    let af_str = config.build_audio_filter_string();
                    player.set_audio_filter(&af_str);
                }
            }
        }
        resp.map(|r| r.response.rect)
    }
}
