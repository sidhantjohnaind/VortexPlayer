#![allow(dead_code)]

use eframe::egui::{self, Color32, CornerRadius, Painter, Pos2, Rect, Stroke, StrokeKind, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkerType {
    Chapter,
    Bookmark,
    AutoSkip,
    SceneCut,
    AbLoopPoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineMarker {
    pub time_sec: f64,
    pub end_sec: Option<f64>,
    pub marker_type: MarkerType,
    pub label: String,
}

pub struct TimelineMarkerRenderer;

impl TimelineMarkerRenderer {
    pub fn get_marker_color(marker_type: MarkerType) -> Color32 {
        match marker_type {
            MarkerType::Chapter => Color32::from_rgb(180, 140, 255), // Light purple
            MarkerType::Bookmark => Color32::from_rgb(255, 215, 0),  // Amber gold
            MarkerType::AutoSkip => Color32::from_rgb(255, 80, 60),  // Coral red
            MarkerType::SceneCut => Color32::from_rgb(50, 220, 220),  // Cyan
            MarkerType::AbLoopPoint => Color32::from_rgb(50, 220, 100), // Emerald
        }
    }

    /// Render subtitle density track along bottom edge of seekbar groove
    pub fn render_subtitle_density(
        painter: &Painter,
        track_rect: Rect,
        subtitle_times: &[f64],
        duration: f64,
    ) {
        if duration <= 0.0 || subtitle_times.is_empty() {
            return;
        }

        let density_color = Color32::from_rgba_unmultiplied(200, 210, 255, 60);
        let bottom_y = track_rect.bottom();
        for &t in subtitle_times {
            if t >= 0.0 && t <= duration {
                let frac = (t / duration) as f32;
                let x = track_rect.left() + track_rect.width() * frac;
                painter.line_segment(
                    [Pos2::new(x, bottom_y - 1.5), Pos2::new(x, bottom_y)],
                    Stroke::new(1.0, density_color),
                );
            }
        }
    }

    /// Render a single diamond marker
    pub fn render_diamond(painter: &Painter, center: Pos2, radius: f32, color: Color32) {
        let points = vec![
            Pos2::new(center.x, center.y - radius),
            Pos2::new(center.x + radius, center.y),
            Pos2::new(center.x, center.y + radius),
            Pos2::new(center.x - radius, center.y),
        ];
        painter.add(egui::epaint::PathShape::convex_polygon(
            points,
            color,
            Stroke::new(0.6, Color32::BLACK),
        ));
    }

    /// Render an interval range (e.g. Auto-Skip or A-B loop)
    pub fn render_range(
        painter: &Painter,
        track_rect: Rect,
        start_sec: f64,
        end_sec: f64,
        duration: f64,
        color: Color32,
    ) {
        if duration <= 0.0 || end_sec <= start_sec {
            return;
        }

        let start_frac = (start_sec / duration).clamp(0.0, 1.0) as f32;
        let end_frac = (end_sec / duration).clamp(0.0, 1.0) as f32;

        let start_x = track_rect.left() + track_rect.width() * start_frac;
        let width = (track_rect.width() * (end_frac - start_frac)).max(2.0);

        let range_rect = Rect::from_min_size(
            Pos2::new(start_x, track_rect.top() - 1.0),
            Vec2::new(width, track_rect.height() + 2.0),
        );

        painter.rect_filled(range_rect, CornerRadius::same(1), color);
        painter.rect_stroke(range_rect, CornerRadius::same(1), Stroke::new(0.8, color.gamma_multiply(1.5)), StrokeKind::Inside);
    }
}
