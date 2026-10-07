use super::theme::VortexTheme;
use crate::engine::surround_eq::{
    ChannelId, SurroundEqConfig, SURROUND_EQ_FREQS, SURROUND_EQ_LABELS,
};
use crate::engine::Player;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, Sense, Stroke,
    StrokeKind, Vec2, Window,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurroundTab {
    OverviewGraph,
    Channel(ChannelId),
}

pub struct SurroundEqDialog {
    pub selected_tab: SurroundTab,
}

impl Default for SurroundEqDialog {
    fn default() -> Self {
        Self {
            selected_tab: SurroundTab::OverviewGraph,
        }
    }
}

impl SurroundEqDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn channel_color(ch: ChannelId) -> Color32 {
        match ch {
            ChannelId::FL => Color32::from_rgb(0, 225, 255),    // Cyan
            ChannelId::FR => Color32::from_rgb(45, 140, 255),   // Deep Sky Blue
            ChannelId::FC => Color32::from_rgb(255, 195, 35),   // Warm Amber/Gold
            ChannelId::LFE => Color32::from_rgb(225, 75, 255),  // Neon Magenta
            ChannelId::SL => Color32::from_rgb(50, 230, 120),   // Emerald Green
            ChannelId::SR => Color32::from_rgb(150, 245, 55),   // Lime Green
            ChannelId::BL => Color32::from_rgb(0, 210, 190),    // Mint Teal
            ChannelId::BR => Color32::from_rgb(255, 125, 50),   // Coral Orange
        }
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        config: &mut SurroundEqConfig,
        player: &Player,
    ) -> Option<Rect> {
        if !*is_open {
            return None;
        }

        let mut changed = false;
        let skin = VortexTheme::current_skin();
        let accent = skin.accent_primary;

        let window_resp = Window::new("🎛 7.1 Surround Per-Channel Equalizer")
            .open(is_open)
            .resizable(true)
            .default_width(780.0)
            .default_height(580.0)
            .min_width(680.0)
            .min_height(500.0)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(14, 16, 22))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(40, 48, 65)))
                    .corner_radius(CornerRadius::same(10))
                    .inner_margin(Margin::same(16)),
            )
            .show(ctx, |ui| {
                // ── Top Control Bar ──────────────────────────────────────────
                ui.horizontal(|ui| {
                    let is_en = config.enabled;
                    let mut en = is_en;
                    let label = RichText::new("Enable 7.1 Surround EQ")
                        .font(FontId::proportional(14.0))
                        .strong()
                        .color(if is_en { accent } else { Color32::from_rgb(150, 155, 170) });
                    if ui.checkbox(&mut en, label).changed() {
                        config.enabled = en;
                        changed = true;
                    }

                    ui.add_space(12.0);
                    ui.checkbox(&mut config.link_stereo_pairs, "🔗 Link Pairs (FL/FR, SL/SR, BL/BR)");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Flat / Reset All").clicked() {
                            config.reset_all();
                            changed = true;
                        }

                        egui::ComboBox::from_id_salt("surround_presets_combo")
                            .selected_text(format!("Preset: {}", config.profile_name))
                            .show_ui(ui, |ui| {
                                for p in [
                                    "Cinema Dialogue Focus",
                                    "Subwoofer Bass Slam",
                                    "Action Movie Immersion",
                                    "Night Mode (Quiet Bass / Clear Speech)",
                                    "Concert Hall & Live Acoustic",
                                    "Flat",
                                ] {
                                    if ui.selectable_label(config.profile_name == p, p).clicked() {
                                        config.apply_preset(p);
                                        changed = true;
                                    }
                                }
                            });
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // ── Channel Navigation Tabs ──────────────────────────────────
                ui.horizontal_wrapped(|ui| {
                    let is_overview = self.selected_tab == SurroundTab::OverviewGraph;
                    if ui
                        .selectable_label(
                            is_overview,
                            RichText::new("📊 7.1 Spectrum Overview").strong().color(if is_overview {
                                Color32::WHITE
                            } else {
                                Color32::from_rgb(180, 185, 200)
                            }),
                        )
                        .clicked()
                    {
                        self.selected_tab = SurroundTab::OverviewGraph;
                    }

                    ui.separator();

                    for &ch in &ChannelId::ALL {
                        let is_sel = self.selected_tab == SurroundTab::Channel(ch);
                        let col = Self::channel_color(ch);
                        let label = format!("{} {}", ch.short_name(), if config.channels[ch as usize].enabled { "●" } else { "○" });

                        let btn = ui.selectable_label(
                            is_sel,
                            RichText::new(label).strong().color(if is_sel {
                                Color32::WHITE
                            } else {
                                col
                            }),
                        );

                        if btn.clicked() {
                            self.selected_tab = SurroundTab::Channel(ch);
                        }
                    }
                });

                ui.add_space(8.0);

                // ── Interactive Frequency Response Visualizer Canvas ────────
                let canvas_h = 160.0;
                let (canvas_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), canvas_h), Sense::hover());
                let painter = ui.painter();

                // Canvas background & grid
                painter.rect_filled(canvas_rect, CornerRadius::same(6), Color32::from_rgb(9, 10, 14));
                painter.rect_stroke(
                    canvas_rect,
                    CornerRadius::same(6),
                    Stroke::new(1.0, Color32::from_rgb(32, 38, 52)),
                    StrokeKind::Inside,
                );

                let zero_y = canvas_rect.center().y;
                // Zero dB line
                painter.line_segment(
                    [Pos2::new(canvas_rect.left(), zero_y), Pos2::new(canvas_rect.right(), zero_y)],
                    Stroke::new(1.0, Color32::from_rgb(50, 58, 76)),
                );
                // +6 dB and -6 dB dashed guides
                let step_y = (canvas_rect.height() - 20.0) / 4.0;
                painter.line_segment(
                    [Pos2::new(canvas_rect.left(), zero_y - step_y), Pos2::new(canvas_rect.right(), zero_y - step_y)],
                    Stroke::new(0.5, Color32::from_rgb(30, 36, 48)),
                );
                painter.line_segment(
                    [Pos2::new(canvas_rect.left(), zero_y + step_y), Pos2::new(canvas_rect.right(), zero_y + step_y)],
                    Stroke::new(0.5, Color32::from_rgb(30, 36, 48)),
                );

                // Frequency grid columns (10 bands)
                let num_bands = SURROUND_EQ_FREQS.len();
                let pad_x = 36.0;
                let eff_w = canvas_rect.width() - pad_x * 2.0;

                for (idx, &label) in SURROUND_EQ_LABELS.iter().enumerate() {
                    let frac = idx as f32 / (num_bands - 1) as f32;
                    let x = canvas_rect.left() + pad_x + frac * eff_w;
                    painter.line_segment(
                        [Pos2::new(x, canvas_rect.top() + 6.0), Pos2::new(x, canvas_rect.bottom() - 18.0)],
                        Stroke::new(0.5, Color32::from_rgb(26, 32, 44)),
                    );
                    painter.text(
                        Pos2::new(x, canvas_rect.bottom() - 9.0),
                        Align2::CENTER_CENTER,
                        label,
                        FontId::proportional(9.5),
                        Color32::from_rgb(110, 118, 136),
                    );
                }

                // dB scale markings on left
                painter.text(
                    Pos2::new(canvas_rect.left() + 14.0, zero_y - step_y * 2.0 + 8.0),
                    Align2::CENTER_CENTER,
                    "+12dB",
                    FontId::proportional(8.5),
                    Color32::from_rgb(85, 92, 110),
                );
                painter.text(
                    Pos2::new(canvas_rect.left() + 14.0, zero_y),
                    Align2::CENTER_CENTER,
                    "0dB",
                    FontId::proportional(8.5),
                    Color32::from_rgb(110, 120, 140),
                );
                painter.text(
                    Pos2::new(canvas_rect.left() + 14.0, zero_y + step_y * 2.0 - 8.0),
                    Align2::CENTER_CENTER,
                    "-12dB",
                    FontId::proportional(8.5),
                    Color32::from_rgb(85, 92, 110),
                );

                // Draw curves
                let draw_channels: Vec<ChannelId> = match self.selected_tab {
                    SurroundTab::OverviewGraph => ChannelId::ALL.to_vec(),
                    SurroundTab::Channel(ch) => vec![ch],
                };

                for &ch in &draw_channels {
                    let ch_data = &config.channels[ch as usize];
                    if !ch_data.enabled {
                        continue;
                    }
                    let col = Self::channel_color(ch);
                    let is_active = match self.selected_tab {
                        SurroundTab::Channel(active) => active == ch,
                        SurroundTab::OverviewGraph => true,
                    };

                    let stroke_w = if is_active && self.selected_tab != SurroundTab::OverviewGraph { 2.5 } else { 1.5 };
                    let stroke_col = if is_active { col } else { Color32::from_rgba_unmultiplied(col.r(), col.g(), col.b(), 140) };

                    let mut pts = Vec::with_capacity(num_bands);
                    for (b_idx, &gain) in ch_data.bands.iter().enumerate() {
                        let total_gain = (gain + ch_data.gain_offset_db).clamp(-12.0, 12.0);
                        let frac_x = b_idx as f32 / (num_bands - 1) as f32;
                        let x = canvas_rect.left() + pad_x + frac_x * eff_w;
                        let frac_y = (total_gain / 12.0) as f32;
                        let y = zero_y - frac_y * (step_y * 2.0 - 8.0);
                        pts.push(Pos2::new(x, y));
                    }

                    // Draw connecting line
                    for w in pts.windows(2) {
                        painter.line_segment([w[0], w[1]], Stroke::new(stroke_w, stroke_col));
                    }

                    // Draw node circles
                    for pt in &pts {
                        painter.circle_filled(*pt, if stroke_w > 2.0 { 3.5 } else { 2.5 }, stroke_col);
                    }
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);

                // ── Controls: Either Single Channel Faders or Overview Matrix ─
                match self.selected_tab {
                    SurroundTab::Channel(ch) => {
                        let ch_idx = ch as usize;
                        let pair_idx = if config.link_stereo_pairs { ch.stereo_pair_index() } else { None };

                        ui.horizontal(|ui| {
                            let col = Self::channel_color(ch);
                            ui.label(
                                RichText::new(format!("Channel: {}", ch.display_name()))
                                    .font(FontId::proportional(15.0))
                                    .strong()
                                    .color(col),
                            );

                            ui.add_space(8.0);
                            let mut ch_en = config.channels[ch_idx].enabled;
                            if ui.checkbox(&mut ch_en, "Channel Enabled").changed() {
                                config.channels[ch_idx].enabled = ch_en;
                                if let Some(p_idx) = pair_idx {
                                    config.channels[p_idx].enabled = ch_en;
                                }
                                changed = true;
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Flat This Channel").clicked() {
                                    config.channels[ch_idx].bands = [0.0; 10];
                                    config.channels[ch_idx].gain_offset_db = 0.0;
                                    if let Some(p_idx) = pair_idx {
                                        config.channels[p_idx].bands = [0.0; 10];
                                        config.channels[p_idx].gain_offset_db = 0.0;
                                    }
                                    changed = true;
                                }

                                if let Some(p_idx) = ch.stereo_pair_index() {
                                    let p_name = ChannelId::ALL[p_idx].short_name();
                                    if ui.button(format!("Copy to {}", p_name)).clicked() {
                                        config.channels[p_idx].bands = config.channels[ch_idx].bands;
                                        config.channels[p_idx].gain_offset_db = config.channels[ch_idx].gain_offset_db;
                                        changed = true;
                                    }
                                }

                                if ui.button("Copy to All Channels").clicked() {
                                    let src = config.channels[ch_idx].clone();
                                    for target in &mut config.channels {
                                        target.bands = src.bands;
                                        target.gain_offset_db = src.gain_offset_db;
                                    }
                                    changed = true;
                                }
                            });
                        });

                        ui.add_space(8.0);

                        // Channel Preamp Offset
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Level Trim / Preamp:").color(Color32::from_rgb(170, 175, 190)));
                            let mut offset = config.channels[ch_idx].gain_offset_db;
                            if ui
                                .add(
                                    egui::Slider::new(&mut offset, -12.0..=12.0)
                                        .step_by(0.1)
                                        .suffix(" dB")
                                        .text("Trim"),
                                )
                                .changed()
                            {
                                config.channels[ch_idx].gain_offset_db = offset;
                                if let Some(p_idx) = pair_idx {
                                    config.channels[p_idx].gain_offset_db = offset;
                                }
                                changed = true;
                            }
                        });

                        ui.add_space(10.0);

                        // 10 Vertical Sliders
                        ui.horizontal(|ui| {
                            ui.spacing_mut().slider_width = 140.0;
                            let slider_h = 160.0;

                            for b_idx in 0..10 {
                                ui.vertical_centered(|ui| {
                                    let mut val = config.channels[ch_idx].bands[b_idx];
                                    let slider = egui::Slider::new(&mut val, -12.0..=12.0)
                                        .vertical()
                                        .step_by(0.1)
                                        .show_value(false);

                                    let resp = ui.add_sized([28.0, slider_h], slider);
                                    if resp.changed() {
                                        config.channels[ch_idx].bands[b_idx] = val;
                                        if let Some(p_idx) = pair_idx {
                                            config.channels[p_idx].bands[b_idx] = val;
                                        }
                                        changed = true;
                                    }

                                    // Double click to zero
                                    if resp.double_clicked() {
                                        config.channels[ch_idx].bands[b_idx] = 0.0;
                                        if let Some(p_idx) = pair_idx {
                                            config.channels[p_idx].bands[b_idx] = 0.0;
                                        }
                                        changed = true;
                                    }

                                    let gain_val = config.channels[ch_idx].bands[b_idx];
                                    let gain_col = if gain_val.abs() < 0.05 {
                                        Color32::from_rgb(130, 135, 150)
                                    } else if gain_val > 0.0 {
                                        Color32::from_rgb(60, 210, 255)
                                    } else {
                                        Color32::from_rgb(255, 120, 120)
                                    };

                                    ui.label(
                                        RichText::new(format!("{:+.1}", gain_val))
                                            .font(FontId::monospace(9.5))
                                            .color(gain_col),
                                    );
                                    ui.label(
                                        RichText::new(SURROUND_EQ_LABELS[b_idx])
                                            .font(FontId::proportional(10.0))
                                            .strong()
                                            .color(Color32::WHITE),
                                    );
                                });
                                ui.add_space(6.0);
                            }
                        });
                    }

                    SurroundTab::OverviewGraph => {
                        ui.label(
                            RichText::new("7.1 Channel Equalization Matrix Balance")
                                .font(FontId::proportional(14.0))
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.label(
                            RichText::new("Quickly adjust overall channel trims or select an individual channel tab above to tune all 10 frequency bands.")
                                .font(FontId::proportional(11.5))
                                .color(Color32::from_rgb(140, 150, 170)),
                        );

                        ui.add_space(10.0);

                        // 8-Channel Quick Level Grid
                        egui::Grid::new("7_1_quick_levels_grid")
                            .striped(true)
                            .spacing(Vec2::new(12.0, 8.0))
                            .show(ui, |ui| {
                                for &ch in &ChannelId::ALL {
                                    let ch_idx = ch as usize;
                                    let col = Self::channel_color(ch);

                                    ui.label(
                                        RichText::new(ch.display_name())
                                            .font(FontId::proportional(12.0))
                                            .strong()
                                            .color(col),
                                    );

                                    let mut ch_en = config.channels[ch_idx].enabled;
                                    if ui.checkbox(&mut ch_en, "").changed() {
                                        config.channels[ch_idx].enabled = ch_en;
                                        changed = true;
                                    }

                                    let mut trim = config.channels[ch_idx].gain_offset_db;
                                    if ui
                                        .add(
                                            egui::Slider::new(&mut trim, -12.0..=12.0)
                                                .step_by(0.1)
                                                .suffix(" dB"),
                                        )
                                        .changed()
                                    {
                                        config.channels[ch_idx].gain_offset_db = trim;
                                        changed = true;
                                    }

                                    if ui.small_button("Edit Bands ➔").clicked() {
                                        self.selected_tab = SurroundTab::Channel(ch);
                                    }

                                    ui.end_row();
                                }
                            });
                    }
                }
            });

        if changed {
            let af_str = config.build_audio_filter_string();
            player.set_audio_filter(&af_str);
        }

        window_resp.map(|w| w.response.rect)
    }
}
