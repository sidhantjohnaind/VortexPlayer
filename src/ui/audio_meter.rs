#![allow(dead_code)]

use crate::engine::audio_levels::{ChannelMeterData, MultiChannelAudioMeterState, MIN_METER_DB, MAX_METER_DB};
use crate::engine::MediaStats;
use crate::ui::theme::VortexTheme;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Margin, Pos2, Rect, Response, RichText, Sense, Stroke,
    StrokeKind, Vec2,
};

/// High-Precision Professional Audio Level Meter Widget
pub struct AudioMeterWidget;

impl AudioMeterWidget {
    /// Render the dual-stage (Input & Output) Audio Levels panel
    pub fn render_dual_stage_panel(
        ui: &mut egui::Ui,
        meter_state: &mut MultiChannelAudioMeterState,
        stats: &MediaStats,
        on_solo_toggle: impl FnMut(usize, bool),
        on_mute_toggle: impl FnMut(usize, bool),
    ) {
        let mut on_solo = on_solo_toggle;
        let mut on_mute = on_mute_toggle;

        let _num_ch = meter_state.layout.channel_count();
        let layout_str = match meter_state.layout {
            crate::engine::ChannelLayoutMode::Mono => "1.0 Mono",
            crate::engine::ChannelLayoutMode::Stereo => "2.0 Stereo",
            crate::engine::ChannelLayoutMode::Stereo21 => "2.1 Stereo",
            crate::engine::ChannelLayoutMode::Surround30 => "3.0 Surround",
            crate::engine::ChannelLayoutMode::Surround31 => "3.1 Surround",
            crate::engine::ChannelLayoutMode::Quad => "4.0 Quad",
            crate::engine::ChannelLayoutMode::Surround50 => "5.0 Surround",
            crate::engine::ChannelLayoutMode::Surround51 => "5.1 Surround",
            crate::engine::ChannelLayoutMode::Surround70 => "7.0 Surround",
            crate::engine::ChannelLayoutMode::Surround71 => "7.1 Surround",
        };

        let codec_str = if !stats.audio_codec.is_empty() {
            stats.audio_codec.to_uppercase()
        } else {
            "PCM".to_string()
        };

        // ── Header Bar with Codec & Channel Config ───────────────────────────
        ui.horizontal(|ui| {
            ui.label(RichText::new("Multi-Channel Audio Meter").size(12.0).strong().color(Color32::from_rgb(220, 225, 235)));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{} {}", layout_str, codec_str))
                        .size(10.5)
                        .strong()
                        .color(VortexTheme::POT_YELLOW),
                );
            });
        });
        ui.add_space(4.0);

        // ── 1. INPUT METERS FRAME ────────────────────────────────────────────
        egui::Frame::new()
            .fill(Color32::from_rgb(0, 0, 0))
            .stroke(Stroke::new(1.0, Color32::from_rgb(35, 38, 48)))
            .corner_radius(CornerRadius::same(4))
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("INPUT").size(10.5).strong().color(Color32::from_rgb(140, 150, 175)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Decoded Stream (Pre-DSP)").size(9.5).color(Color32::from_rgb(100, 110, 130)));
                    });
                });
                ui.add_space(4.0);

                Self::render_meter_bank(
                    ui,
                    &mut meter_state.input_channels,
                    "input",
                    &mut on_solo,
                    &mut on_mute,
                );
            });

        ui.add_space(6.0);

        // ── 2. OUTPUT METERS FRAME ───────────────────────────────────────────
        egui::Frame::new()
            .fill(Color32::from_rgb(0, 0, 0))
            .stroke(Stroke::new(1.0, Color32::from_rgb(35, 38, 48)))
            .corner_radius(CornerRadius::same(4))
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("OUTPUT").size(10.5).strong().color(Color32::from_rgb(140, 150, 175)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("Speaker Master (Post-DSP)").size(9.5).color(Color32::from_rgb(100, 110, 130)));
                    });
                });
                ui.add_space(4.0);

                Self::render_meter_bank(
                    ui,
                    &mut meter_state.output_channels,
                    "output",
                    &mut on_solo,
                    &mut on_mute,
                );
            });

        ui.add_space(6.0);

        // ── 3. LOUDNESS & METRICS FOOTER ─────────────────────────────────────
        egui::Frame::new()
            .fill(Color32::from_rgb(0, 0, 0))
            .stroke(Stroke::new(1.0, Color32::from_rgb(30, 34, 44)))
            .corner_radius(CornerRadius::same(4))
            .inner_margin(Margin::symmetric(8, 5))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(14.0, 0.0);

                    ui.label(
                        RichText::new("Live Multichannel Studio Metering")
                            .size(10.5)
                            .color(Color32::from_rgb(130, 140, 160)),
                    );

                    // Reset clips action button
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(RichText::new("Reset Clips").size(9.5)).clicked() {
                            meter_state.reset_clips();
                        }
                    });
                });
            });
    }

    /// Render a row of vertical channel meters with dB scale ladder
    fn render_meter_bank(
        ui: &mut egui::Ui,
        channels: &mut [ChannelMeterData],
        id_prefix: &str,
        on_solo: &mut impl FnMut(usize, bool),
        on_mute: &mut impl FnMut(usize, bool),
    ) {
        let num_ch = channels.len();
        if num_ch == 0 {
            return;
        }

        let meter_height = 76.0;
        let avail_w = ui.available_width();
        let max_meter_w = avail_w;

        let (total_rect, _) = ui.allocate_exact_size(
            Vec2::new(avail_w, meter_height + 22.0),
            Sense::hover(),
        );

        let painter = ui.painter();

        // ── Draw Active Channels ──────────────────────────────────────────
        let ch_slot_w = (max_meter_w / num_ch as f32).clamp(28.0, 54.0);
        let total_channels_w = ch_slot_w * num_ch as f32;
        let channels_area_left = total_rect.left() + ((max_meter_w - total_channels_w) / 2.0).max(0.0);
        let bar_w = (ch_slot_w - 12.0).clamp(16.0, 28.0);

        for (i, ch) in channels.iter_mut().enumerate() {
            let ch_center_x = channels_area_left + (i as f32 + 0.5) * ch_slot_w;
            let bar_left = (ch_center_x - bar_w / 2.0).round();

            let bar_rect = Rect::from_min_size(
                Pos2::new(bar_left, total_rect.top()),
                Vec2::new(bar_w, meter_height),
            );

            // Background Groove
            painter.rect_filled(bar_rect, CornerRadius::same(2), Color32::from_rgb(12, 14, 18));
            painter.rect_stroke(
                bar_rect,
                CornerRadius::same(2),
                Stroke::new(1.0, Color32::from_rgb(32, 36, 48)),
                StrokeKind::Inside,
            );

            let peak_norm = ch.normalized_peak();
            let rms_norm = ch.normalized_rms();
            let peak_hold_norm = ch.normalized_peak_hold();

            // ── A. RMS Body (Darker Forest Green to Lime) ─────────────────────
            if rms_norm > 0.01 {
                let rms_h = (meter_height * rms_norm).round().clamp(2.0, meter_height);
                let rms_rect = Rect::from_min_max(
                    Pos2::new(bar_left + 1.0, bar_rect.bottom() - rms_h),
                    Pos2::new(bar_left + bar_w - 1.0, bar_rect.bottom() - 1.0),
                );

                let rms_col = if ch.rms_db > -3.0 {
                    Color32::from_rgb(220, 160, 20)
                } else {
                    Color32::from_rgb(24, 130, 42) // Solid professional green
                };
                painter.rect_filled(rms_rect, CornerRadius::ZERO, rms_col);
            }

            // ── B. Instant Peak Cap (Bright Vivid Green / Yellow) ─────────────
            if peak_norm > 0.02 {
                let peak_h = (meter_height * peak_norm).round().clamp(3.0, meter_height);
                let cap_h = 4.0f32.min(peak_h);
                let peak_rect = Rect::from_min_max(
                    Pos2::new(bar_left + 1.0, bar_rect.bottom() - peak_h),
                    Pos2::new(bar_left + bar_w - 1.0, bar_rect.bottom() - peak_h + cap_h),
                );

                let peak_col = if ch.peak_db >= -0.1 {
                    Color32::from_rgb(255, 50, 50) // Red clip
                } else if ch.peak_db >= -3.0 {
                    Color32::from_rgb(255, 215, 0) // Amber warn
                } else {
                    Color32::from_rgb(45, 210, 80) // Bright electric green
                };
                painter.rect_filled(peak_rect, CornerRadius::ZERO, peak_col);
            }

            // ── C. Peak Hold Indicator (Bright floating line) ─────────────────
            if peak_hold_norm > 0.01 {
                let hold_y = (bar_rect.bottom() - (meter_height * peak_hold_norm)).round().clamp(bar_rect.top(), bar_rect.bottom());
                painter.line_segment(
                    [Pos2::new(bar_left, hold_y), Pos2::new(bar_left + bar_w, hold_y)],
                    Stroke::new(1.5, Color32::WHITE),
                );
            }

            // ── D. Clip Indicator Light (Top 3px) ─────────────────────────────
            if ch.is_clipped {
                let clip_r = Rect::from_min_size(Pos2::new(bar_left, bar_rect.top() - 4.0), Vec2::new(bar_w, 3.0));
                painter.rect_filled(clip_r, CornerRadius::same(1), Color32::from_rgb(255, 30, 30));
            }

            // ── E. Speaker Label (L, R, C, LFE, SL, SR, BL, BR) ───────────────
            let label_y = bar_rect.bottom() + 10.0;
            let speaker_col = if ch.is_solo {
                VortexTheme::POT_YELLOW
            } else if ch.is_muted {
                Color32::from_rgb(120, 60, 60)
            } else {
                Color32::from_rgb(210, 215, 230)
            };
            painter.text(
                Pos2::new(ch_center_x, label_y),
                Align2::CENTER_CENTER,
                ch.speaker.short_name(),
                FontId::monospace(10.5),
                speaker_col,
            );

            // ── G. Channel Interaction: Click to Solo, Right-Click Context ────
            let interact_id = ui.id().with(format!("ch_meter_{}_{}", id_prefix, i));
            let ch_interact_rect = Rect::from_min_max(bar_rect.min, Pos2::new(bar_rect.right(), total_rect.bottom()));
            let resp = ui.interact(ch_interact_rect, interact_id, Sense::click());

            // Left-Click: Toggle Solo
            if resp.clicked() {
                ch.is_solo = !ch.is_solo;
                on_solo(i, ch.is_solo);
            }

            // Right-Click context menu
            resp.context_menu(|ui| {
                ui.label(RichText::new(format!("Channel: {}", ch.speaker.full_name())).strong());
                ui.separator();
                if ui.checkbox(&mut ch.is_solo, "Solo Channel").clicked() {
                    on_solo(i, ch.is_solo);
                    ui.close();
                }
                if ui.checkbox(&mut ch.is_muted, "Mute Channel").clicked() {
                    on_mute(i, ch.is_muted);
                    ui.close();
                }
                if ui.button("Reset Clip Indicator").clicked() {
                    ch.is_clipped = false;
                    ui.close();
                }
            });

            if resp.hovered() {
                let status = if ch.is_solo {
                    "[SOLO]"
                } else if ch.is_muted {
                    "[MUTED]"
                } else {
                    ""
                };
                resp.on_hover_text(format!(
                    "{} ({})\nPeak: {:+.1} dBFS\nRMS: {:+.1} dBFS\nClick to Solo • Right-Click for Options {}",
                    ch.speaker.full_name(),
                    ch.speaker.short_name(),
                    ch.peak_db,
                    ch.rms_db,
                    status
                ));
            }
        }
    }

    /// Render compact floating overlay over video
    pub fn render_compact_overlay(
        ui: &mut egui::Ui,
        meter_state: &MultiChannelAudioMeterState,
    ) {
        let num_ch = meter_state.layout.channel_count();
        let w = 180.0;
        let row_h = 16.0;
        let h = (num_ch as f32 * row_h) + 24.0;

        let (rect, _) = ui.allocate_exact_size(Vec2::new(w, h), Sense::hover());
        let painter = ui.painter();

        // Dark frosted background
        painter.rect_filled(rect, CornerRadius::same(4), Color32::from_rgba_premultiplied(12, 14, 18, 220));
        painter.rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.0, Color32::from_rgb(60, 65, 80)), StrokeKind::Inside);

        let top = rect.top() + 6.0;
        for (i, ch) in meter_state.output_channels.iter().enumerate() {
            let y = top + i as f32 * row_h;

            // Label
            painter.text(
                Pos2::new(rect.left() + 8.0, y + row_h * 0.5),
                Align2::LEFT_CENTER,
                ch.speaker.short_name(),
                FontId::monospace(9.5),
                Color32::from_rgb(180, 190, 210),
            );

            // Bar
            let bar_left = rect.left() + 36.0;
            let bar_w = 84.0;
            let bar_rect = Rect::from_min_size(Pos2::new(bar_left, y + 3.0), Vec2::new(bar_w, row_h - 6.0));
            painter.rect_filled(bar_rect, CornerRadius::ZERO, Color32::from_rgb(20, 24, 32));

            let p_norm = ch.normalized_peak();
            if p_norm > 0.01 {
                let fill_w = bar_w * p_norm;
                let fill_r = Rect::from_min_size(bar_rect.min, Vec2::new(fill_w, bar_rect.height()));
                let col = if ch.peak_db >= -0.1 {
                    Color32::from_rgb(255, 60, 60)
                } else if ch.peak_db >= -3.0 {
                    Color32::from_rgb(255, 200, 40)
                } else {
                    Color32::from_rgb(40, 200, 70)
                };
                painter.rect_filled(fill_r, CornerRadius::ZERO, col);
            }

            // Text dB
            let db_txt = if ch.peak_db <= -59.0 { "-∞".to_string() } else { format!("{:+.0} dB", ch.peak_db) };
            painter.text(
                Pos2::new(rect.right() - 8.0, y + row_h * 0.5),
                Align2::RIGHT_CENTER,
                db_txt,
                FontId::monospace(9.0),
                Color32::from_rgb(160, 170, 190),
            );
        }
    }
}
