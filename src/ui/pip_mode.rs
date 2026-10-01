use super::theme::VortexTheme;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, Vec2};

pub struct PipModeView {
    pub is_pip: bool,
    pub opacity: f32, // 0.2 to 1.0
}

impl Default for PipModeView {
    fn default() -> Self {
        Self {
            is_pip: false,
            opacity: 0.95,
        }
    }
}

impl PipModeView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_controls(&mut self, ui: &mut egui::Ui) {
        if !self.is_pip {
            return;
        }

        ui.horizontal(|ui| {
            ui.label("PiP Opacity:");
            ui.add(egui::Slider::new(&mut self.opacity, 0.2..=1.0).text("Transparency"));
        });
    }
}
