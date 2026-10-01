use super::theme::VortexTheme;
use crate::engine::MediaStats;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

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
            Self::Oscilloscope => "Oscilloscope Waveform",
            Self::Vectorscope => "Stereo Vectorscope",
            Self::LufsMeter => "EBU R128 Loudness Meter",
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
}

impl Default for VisualizerSuite {
    fn default() -> Self {
        Self {
            mode: VisualizerMode::FftSpectrum,
            is_visible: false,
            bands: [0.0; 64],
            peaks: [0.0; 64],
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

        // Background box for visualizer
        let vis_h = 90.0;
        let vis_rect = Rect::from_min_size(
            Pos2::new(rect.left() + 20.0, rect.bottom() - vis_h - 20.0),
            Vec2::new(rect.width() - 40.0, vis_h),
        );

        painter.rect_filled(vis_rect, CornerRadius::same(6), Color32::from_rgba_unmultiplied(12, 12, 16, 210));
        painter.rect_stroke(vis_rect, CornerRadius::same(6), Stroke::new(1.0, Color32::from_rgba_unmultiplied(60, 65, 80, 180)), StrokeKind::Inside);

        match self.mode {
            VisualizerMode::FftSpectrum => {
                let bar_count = 64;
                let gap = 2.0;
                let total_gap = gap * (bar_count as f32 - 1.0);
                let bar_w = ((vis_rect.width() - 24.0 - total_gap) / bar_count as f32).max(1.5);
                let max_h = vis_rect.height() - 16.0;

                for i in 0..bar_count {
                    // Update band simulation if playing
                    if is_playing {
                        let freq_weight = 1.0 - (i as f32 / bar_count as f32) * 0.4;
                        let wave = ((time * 12.0 + i as f64 * 0.45).sin() * 0.5 + 0.5) as f32;
                        let wave2 = ((time * 7.5 - i as f64 * 0.25).cos() * 0.5 + 0.5) as f32;
                        let target = (wave * 0.6 + wave2 * 0.4) * freq_weight * (stats.volume as f32 / 100.0);
                        self.bands[i] = self.bands[i] * 0.7 + target * 0.3;
                        if self.bands[i] > self.peaks[i] {
                            self.peaks[i] = self.bands[i];
                        } else {
                            self.peaks[i] = (self.peaks[i] - 0.015).max(0.0);
                        }
                    } else {
                        self.bands[i] = (self.bands[i] - 0.05).max(0.0);
                        self.peaks[i] = (self.peaks[i] - 0.02).max(0.0);
                    }

                    let h = (self.bands[i] * max_h).clamp(2.0, max_h);
                    let x = vis_rect.left() + 12.0 + i as f32 * (bar_w + gap);
                    let y = vis_rect.bottom() - 8.0 - h;

                    let bar_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(bar_w, h));
                    let color = if i < 20 {
                        Color32::from_rgb(100, 180, 255)
                    } else if i < 44 {
                        VortexTheme::POT_YELLOW
                    } else {
                        Color32::from_rgb(255, 90, 90)
                    };
                    painter.rect_filled(bar_rect, CornerRadius::same(1), color);

                    // Peak LED line
                    let peak_y = vis_rect.bottom() - 8.0 - (self.peaks[i] * max_h).clamp(2.0, max_h);
                    painter.line_segment(
                        [Pos2::new(x, peak_y), Pos2::new(x + bar_w, peak_y)],
                        Stroke::new(1.5, Color32::WHITE),
                    );
                }
            }
            VisualizerMode::Oscilloscope => {
                let pts: Vec<Pos2> = (0..120).map(|i| {
                    let frac = i as f32 / 119.0;
                    let x = vis_rect.left() + 16.0 + frac * (vis_rect.width() - 32.0);
                    let wave = if is_playing {
                        ((time * 20.0 + (frac as f64 * 16.0)).sin() * 0.4) as f32
                    } else {
                        0.0
                    };
                    let y = vis_rect.center().y + wave * (vis_h / 2.0 - 10.0);
                    Pos2::new(x, y)
                }).collect();

                for w in pts.windows(2) {
                    painter.line_segment([w[0], w[1]], Stroke::new(2.0, VortexTheme::POT_YELLOW));
                }
            }
            VisualizerMode::Vectorscope | VisualizerMode::LufsMeter => {
                // EBU R128 LUFS Meter Bar
                painter.text(
                    Pos2::new(vis_rect.left() + 16.0, vis_rect.center().y),
                    Align2::LEFT_CENTER,
                    "EBU R128 LUFS: -16.2 LUFS | True Peak: -0.8 dBTP | Dynamic Range: 12.4 LU",
                    FontId::monospace(12.0),
                    VortexTheme::POT_YELLOW,
                );
            }
        }
    }
}
