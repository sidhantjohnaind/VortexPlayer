use super::theme::VortexTheme;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

pub struct SplitCompareView {
    pub enabled: bool,
    pub split_pos: f32, // 0.0 to 1.0 (divider horizontal fraction)
    pub is_dragging: bool,
}

impl Default for SplitCompareView {
    fn default() -> Self {
        Self {
            enabled: false,
            split_pos: 0.5,
            is_dragging: false,
        }
    }
}

impl SplitCompareView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ui: &mut egui::Ui, video_rect: Rect) {
        if !self.enabled {
            return;
        }

        let painter = ui.painter();
        let split_x = video_rect.left() + video_rect.width() * self.split_pos;

        // Draw vertical divider line
        painter.line_segment(
            [Pos2::new(split_x, video_rect.top()), Pos2::new(split_x, video_rect.bottom())],
            Stroke::new(2.5, VortexTheme::VORTEX_YELLOW),
        );

        // Divider handle
        let handle_rect = Rect::from_center_size(
            Pos2::new(split_x, video_rect.center().y),
            Vec2::new(24.0, 48.0),
        );
        painter.rect_filled(handle_rect, CornerRadius::same(6), Color32::from_rgb(30, 30, 40));
        painter.rect_stroke(handle_rect, CornerRadius::same(6), Stroke::new(1.5, VortexTheme::VORTEX_YELLOW), StrokeKind::Inside);
        painter.text(
            handle_rect.center(),
            Align2::CENTER_CENTER,
            "◀▶",
            FontId::proportional(11.0),
            VortexTheme::VORTEX_YELLOW,
        );

        // Labels: Left = Original, Right = Processed (Shaders / HDR)
        let left_label_rect = Rect::from_min_size(
            Pos2::new(video_rect.left() + 16.0, video_rect.top() + 16.0),
            Vec2::new(110.0, 24.0),
        );
        painter.rect_filled(left_label_rect, CornerRadius::same(4), Color32::from_rgba_unmultiplied(10, 10, 15, 200));
        painter.text(
            left_label_rect.center(),
            Align2::CENTER_CENTER,
            "ORIGINAL (RAW)",
            FontId::proportional(11.0),
            Color32::from_rgb(200, 200, 210),
        );

        let right_label_rect = Rect::from_min_size(
            Pos2::new(video_rect.right() - 150.0, video_rect.top() + 16.0),
            Vec2::new(134.0, 24.0),
        );
        painter.rect_filled(right_label_rect, CornerRadius::same(4), Color32::from_rgba_unmultiplied(10, 10, 15, 200));
        painter.text(
            right_label_rect.center(),
            Align2::CENTER_CENTER,
            "PROCESSED (SHADERS)",
            FontId::proportional(11.0),
            VortexTheme::VORTEX_YELLOW,
        );

        // Interaction for dragging divider
        let response = ui.interact(video_rect, ui.id().with("split_slider"), Sense::click_and_drag());
        if response.dragged() {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                self.split_pos = ((mouse_pos.x - video_rect.left()) / video_rect.width()).clamp(0.05, 0.95);
            }
        }
    }
}
