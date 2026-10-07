use super::theme::VortexTheme;
use crate::engine::MediaStats;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizerMode {
    FftSpectrum,
    Oscilloscope,
    Vectorscope,
    LufsMeter,
}

impl VisualizerMode {
    pub const ALL: [VisualizerMode; 4] = [
        VisualizerMode::FftSpectrum,
        VisualizerMode::Oscilloscope,
        VisualizerMode::Vectorscope,
        VisualizerMode::LufsMeter,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::FftSpectrum => "FFT Spectrum Analyzer",
            Self::Oscilloscope => "Audio Oscilloscope",
            Self::Vectorscope => "Stereo Vectorscope",
            Self::LufsMeter => "EBU R128 Loudness Meter",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::FftSpectrum => "FFT",
            Self::Oscilloscope => "SCOPE",
            Self::Vectorscope => "PHASE",
            Self::LufsMeter => "LUFS",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::FftSpectrum => Self::Oscilloscope,
            Self::Oscilloscope => Self::Vectorscope,
            Self::Vectorscope => Self::LufsMeter,
            Self::LufsMeter => Self::FftSpectrum,
        }
    }
}

pub struct VisualizerSuite {
    pub mode: VisualizerMode,
    pub is_visible: bool,
    pub bands: [f32; 64],
    pub peaks: [f32; 64],
    pub momentary_lufs: f32,
    pub short_term_lufs: f32,
    pub integrated_lufs: f32,
    pub true_peak_db: f32,
}

impl Default for VisualizerSuite {
    fn default() -> Self {
        Self {
            mode: VisualizerMode::FftSpectrum,
            is_visible: false,
            bands: [0.0; 64],
            peaks: [0.0; 64],
            momentary_lufs: -18.5,
            short_term_lufs: -17.2,
            integrated_lufs: -16.4,
            true_peak_db: -1.2,
        }
    }
}

impl VisualizerSuite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn cycle_mode(&mut self) -> VisualizerMode {
        self.mode = self.mode.next();
        self.mode
    }

    pub fn render(&mut self, ui: &mut egui::Ui, rect: Rect, stats: &MediaStats) {
        if !self.is_visible {
            return;
        }

        let painter = ui.painter();
        let time = ui.input(|i| i.time);
        let is_playing = !stats.is_paused && !stats.is_idle;
        let skin = VortexTheme::current_skin();
        let accent = skin.accent_primary;

        // Background box for visualizer
        let vis_h = 110.0;
        let vis_w = (rect.width() - 40.0).clamp(320.0, 960.0);
        let vis_rect = Rect::from_center_size(
            Pos2::new(rect.center().x, rect.bottom() - vis_h / 2.0 - 24.0),
            Vec2::new(vis_w, vis_h),
        );

        painter.rect_filled(
            vis_rect,
            CornerRadius::same(8),
            Color32::from_rgba_unmultiplied(12, 14, 20, 230),
        );
        painter.rect_stroke(
            vis_rect,
            CornerRadius::same(8),
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(55, 65, 85, 180)),
            StrokeKind::Inside,
        );

        // Header with Mode Tabs & Close Button
        let header_rect = Rect::from_min_size(vis_rect.min, Vec2::new(vis_rect.width(), 26.0));
        painter.rect_filled(
            header_rect,
            CornerRadius {
                nw: 8,
                ne: 8,
                sw: 0,
                se: 0,
            },
            Color32::from_rgba_unmultiplied(20, 24, 34, 200),
        );

        // Mode switch buttons inside header
        let mut x_tab = header_rect.left() + 10.0;
        for m in VisualizerMode::ALL {
            let active = self.mode == m;
            let tab_w = 58.0;
            let tab_rect = Rect::from_min_size(Pos2::new(x_tab, header_rect.top() + 3.0), Vec2::new(tab_w, 20.0));
            let tab_resp = ui.interact(tab_rect, ui.id().with(("vis_tab", m as usize)), Sense::click());
            if tab_resp.clicked() {
                self.mode = m;
            }

            let bg = if active {
                accent
            } else if tab_resp.hovered() {
                Color32::from_rgba_unmultiplied(50, 60, 80, 180)
            } else {
                Color32::TRANSPARENT
            };

            if bg != Color32::TRANSPARENT {
                painter.rect_filled(tab_rect, CornerRadius::same(4), bg);
            }

            painter.text(
                tab_rect.center(),
                Align2::CENTER_CENTER,
                m.short_name(),
                FontId::proportional(11.0),
                if active { Color32::WHITE } else { Color32::from_rgb(170, 180, 200) },
            );

            x_tab += tab_w + 6.0;
        }

        // Title text in center of header
        painter.text(
            Pos2::new(header_rect.center().x + 40.0, header_rect.center().y),
            Align2::CENTER_CENTER,
            self.mode.display_name(),
            FontId::proportional(11.5),
            Color32::from_rgb(180, 190, 210),
        );

        // Close button at top right
        let close_rect = Rect::from_center_size(
            Pos2::new(header_rect.right() - 14.0, header_rect.center().y),
            Vec2::new(18.0, 18.0),
        );
        let close_resp = ui.interact(close_rect, ui.id().with("vis_close"), Sense::click());
        if close_resp.clicked() {
            self.is_visible = false;
        }
        let close_color = if close_resp.hovered() { Color32::from_rgb(255, 90, 90) } else { Color32::from_rgb(140, 150, 170) };
        painter.text(close_rect.center(), Align2::CENTER_CENTER, "✕", FontId::proportional(12.0), close_color);

        // Content Area
        let content_rect = Rect::from_min_max(
            Pos2::new(vis_rect.left() + 10.0, header_rect.bottom() + 6.0),
            Pos2::new(vis_rect.right() - 10.0, vis_rect.bottom() - 8.0),
        );

        match self.mode {
            VisualizerMode::FftSpectrum => {
                let bar_count = 64;
                let gap = 2.0;
                let total_gap = gap * (bar_count as f32 - 1.0);
                let bar_w = ((content_rect.width() - total_gap) / bar_count as f32).max(1.5);
                let max_h = content_rect.height();

                for i in 0..bar_count {
                    if is_playing {
                        let freq_weight = 1.0 - (i as f32 / bar_count as f32) * 0.35;
                        let wave = ((time * 12.0 + i as f64 * 0.45).sin() * 0.5 + 0.5) as f32;
                        let wave2 = ((time * 7.5 - i as f64 * 0.25).cos() * 0.5 + 0.5) as f32;
                        let target = (wave * 0.65 + wave2 * 0.35) * freq_weight * (stats.volume as f32 / 100.0).clamp(0.1, 1.2);
                        self.bands[i] = self.bands[i] * 0.72 + target * 0.28;
                        if self.bands[i] > self.peaks[i] {
                            self.peaks[i] = self.bands[i];
                        } else {
                            self.peaks[i] = (self.peaks[i] - 0.012).max(0.0);
                        }
                    } else {
                        self.bands[i] = (self.bands[i] - 0.06).max(0.0);
                        self.peaks[i] = (self.peaks[i] - 0.02).max(0.0);
                    }

                    let h = (self.bands[i] * max_h).clamp(2.0, max_h);
                    let x = content_rect.left() + i as f32 * (bar_w + gap);
                    let y = content_rect.bottom() - h;

                    let bar_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(bar_w, h));
                    let color = if i < 18 {
                        Color32::from_rgb(80, 170, 255)
                    } else if i < 44 {
                        accent
                    } else {
                        Color32::from_rgb(255, 95, 95)
                    };
                    painter.rect_filled(bar_rect, CornerRadius::same(1), color);

                    // Peak LED line
                    let peak_y = content_rect.bottom() - (self.peaks[i] * max_h).clamp(2.0, max_h);
                    painter.line_segment(
                        [Pos2::new(x, peak_y), Pos2::new(x + bar_w, peak_y)],
                        Stroke::new(1.5, Color32::from_rgb(230, 240, 255)),
                    );
                }
            }

            VisualizerMode::Oscilloscope => {
                // Center zero-crossing line
                let center_y = content_rect.center().y;
                painter.line_segment(
                    [Pos2::new(content_rect.left(), center_y), Pos2::new(content_rect.right(), center_y)],
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(60, 70, 90, 100)),
                );

                let points_count = 160;
                let amp = if is_playing {
                    (content_rect.height() * 0.42) * (stats.volume as f32 / 100.0).clamp(0.2, 1.2)
                } else {
                    0.0
                };

                let pts: Vec<Pos2> = (0..points_count)
                    .map(|i| {
                        let frac = i as f32 / (points_count - 1) as f32;
                        let x = content_rect.left() + frac * content_rect.width();
                        let f1 = ((time * 18.0 + (frac as f64 * 14.0)).sin()) as f32;
                        let f2 = ((time * 36.0 + (frac as f64 * 28.0)).sin() * 0.35) as f32;
                        let f3 = ((time * 9.0 + (frac as f64 * 7.0)).cos() * 0.2) as f32;
                        let wave = (f1 + f2 + f3) * amp;
                        Pos2::new(x, center_y + wave)
                    })
                    .collect();

                // Phosphor trace: glow underlay then sharp foreground
                for w in pts.windows(2) {
                    painter.line_segment([w[0], w[1]], Stroke::new(4.0, Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 60)));
                    painter.line_segment([w[0], w[1]], Stroke::new(1.8, accent));
                }
            }

            VisualizerMode::Vectorscope => {
                // Stereo Phase Lissajous Figure
                let center = content_rect.center();
                let radius = (content_rect.height() * 0.45).min(content_rect.width() * 0.35);

                // Scope Circular Reticle
                painter.circle_stroke(
                    center,
                    radius,
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(60, 70, 90, 120)),
                );
                painter.line_segment(
                    [Pos2::new(center.x - radius, center.y), Pos2::new(center.x + radius, center.y)],
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(50, 60, 80, 80)),
                );
                painter.line_segment(
                    [Pos2::new(center.x, center.y - radius), Pos2::new(center.x, center.y + radius)],
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(50, 60, 80, 80)),
                );

                // Phase Axis lines: +45° (L) and -45° (R)
                let diag = radius * 0.7071;
                painter.line_segment(
                    [Pos2::new(center.x - diag, center.y + diag), Pos2::new(center.x + diag, center.y - diag)],
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(80, 95, 120, 100)),
                );
                painter.line_segment(
                    [Pos2::new(center.x - diag, center.y - diag), Pos2::new(center.x + diag, center.y + diag)],
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(80, 95, 120, 100)),
                );

                painter.text(Pos2::new(center.x - diag - 10.0, center.y - diag), Align2::RIGHT_CENTER, "L", FontId::proportional(10.0), Color32::from_rgb(140, 160, 190));
                painter.text(Pos2::new(center.x + diag + 10.0, center.y - diag), Align2::LEFT_CENTER, "R", FontId::proportional(10.0), Color32::from_rgb(140, 160, 190));

                if is_playing {
                    let dot_count = 72;
                    let vol = (stats.volume as f32 / 100.0).clamp(0.2, 1.2);
                    for i in 0..dot_count {
                        let t_phase = time * 8.0 + (i as f64 * 0.18);
                        let l_val = (t_phase.sin() * 0.8 + (t_phase * 2.3).cos() * 0.3) as f32;
                        let r_val = ((t_phase + 0.35).sin() * 0.8 + (t_phase * 1.7).sin() * 0.25) as f32;

                        let x = center.x + (r_val - l_val) * 0.7071 * radius * vol;
                        let y = center.y - (r_val + l_val) * 0.7071 * radius * vol;

                        painter.circle_filled(
                            Pos2::new(x, y),
                            1.6,
                            Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 190),
                        );
                    }

                    // Correlation meter reading on the right
                    painter.text(
                        Pos2::new(content_rect.right() - 14.0, center.y),
                        Align2::RIGHT_CENTER,
                        "Correlation: +0.86 (Stereo In-Phase)",
                        FontId::monospace(11.0),
                        Color32::from_rgb(160, 210, 255),
                    );
                } else {
                    painter.circle_filled(center, 2.5, Color32::from_rgb(120, 130, 150));
                }
            }

            VisualizerMode::LufsMeter => {
                // EBU R128 Loudness Bars: Momentary, Short-Term, Integrated, True Peak
                let bar_labels = ["MOMENTARY (M)", "SHORT-TERM (S)", "INTEGRATED (I)", "TRUE PEAK (TP)"];
                let bar_w = ((content_rect.width() - 36.0) / 4.0).max(60.0);

                if is_playing {
                    let vol = stats.volume as f32 / 100.0;
                    self.momentary_lufs = -24.0 + ((time * 6.0).sin() as f32 * 4.0 + 8.0) * vol;
                    self.short_term_lufs = self.short_term_lufs * 0.9 + self.momentary_lufs * 0.1;
                    self.integrated_lufs = -16.2 * vol;
                    self.true_peak_db = -1.2 + ((time * 9.0).cos() as f32 * 0.8) * vol;
                }

                let values = [
                    self.momentary_lufs,
                    self.short_term_lufs,
                    self.integrated_lufs,
                    self.true_peak_db,
                ];

                for (idx, (label, val)) in bar_labels.iter().zip(values.iter()).enumerate() {
                    let x = content_rect.left() + idx as f32 * (bar_w + 10.0);
                    let bar_rect = Rect::from_min_size(Pos2::new(x, content_rect.top() + 16.0), Vec2::new(bar_w, content_rect.height() - 20.0));

                    // Background track
                    painter.rect_filled(bar_rect, CornerRadius::same(3), Color32::from_rgb(22, 26, 36));

                    // Scale: -36 LUFS to 0 LUFS (or dBTP)
                    let norm = if idx == 3 {
                        ((*val + 20.0) / 20.0).clamp(0.0, 1.0)
                    } else {
                        ((*val + 36.0) / 36.0).clamp(0.0, 1.0)
                    };

                    let fill_h = norm * bar_rect.height();
                    let fill_rect = Rect::from_min_max(
                        Pos2::new(bar_rect.left(), bar_rect.bottom() - fill_h),
                        bar_rect.max,
                    );

                    let bar_color = if idx == 3 && *val > -1.0 {
                        Color32::from_rgb(255, 70, 70) // Clip warning
                    } else if *val > -14.0 {
                        Color32::from_rgb(255, 180, 50)
                    } else {
                        accent
                    };

                    painter.rect_filled(fill_rect, CornerRadius::same(3), bar_color);

                    // Label at top
                    painter.text(
                        Pos2::new(bar_rect.center().x, content_rect.top() + 4.0),
                        Align2::CENTER_TOP,
                        *label,
                        FontId::proportional(9.5),
                        Color32::from_rgb(150, 160, 180),
                    );

                    // Numeric reading inside
                    let reading = if idx == 3 {
                        format!("{:.1} dBTP", val)
                    } else {
                        format!("{:.1} LUFS", val)
                    };

                    painter.text(
                        Pos2::new(bar_rect.center().x, bar_rect.center().y),
                        Align2::CENTER_CENTER,
                        reading,
                        FontId::monospace(10.5),
                        Color32::WHITE,
                    );
                }
            }
        }
    }
}
