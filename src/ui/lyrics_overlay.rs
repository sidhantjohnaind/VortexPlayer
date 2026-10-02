use super::theme::VortexTheme;
use crate::engine::lyrics::LyricTrack;
use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Vec2};

pub struct LyricsOverlay {
    pub is_enabled: bool,
    pub track: Option<LyricTrack>,
}

impl Default for LyricsOverlay {
    fn default() -> Self {
        Self {
            is_enabled: true,
            track: None,
        }
    }
}

impl LyricsOverlay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&self, ui: &mut egui::Ui, video_rect: Rect, current_time: f64) {
        if !self.is_enabled {
            return;
        }

        if let Some(ref lrc) = self.track {
            if let Some((idx, active_line)) = lrc.get_current_line(current_time) {
                let painter = ui.painter();
                let center_y = video_rect.center().y + 40.0;

                // Draw Current Active Line (Highlighted Gold)
                painter.text(
                    Pos2::new(video_rect.center().x, center_y),
                    Align2::CENTER_CENTER,
                    &active_line.text,
                    FontId::proportional(22.0),
                    VortexTheme::VORTEX_YELLOW,
                );

                // Draw Next Line (Muted Grey)
                if let Some(next_line) = lrc.lines.get(idx + 1) {
                    painter.text(
                        Pos2::new(video_rect.center().x, center_y + 32.0),
                        Align2::CENTER_CENTER,
                        &next_line.text,
                        FontId::proportional(15.0),
                        Color32::from_rgb(140, 145, 160),
                    );
                }
            }
        }
    }
}
