#![allow(dead_code)]

use super::theme::VortexTheme;
use eframe::egui::{Align2, Color32, CornerRadius, FontId, Painter, Pos2, Rect, Shape, Stroke, StrokeKind, Vec2};

pub struct Icons;

impl Icons {
    /// Cinema Aperture Iris Logo — Precision camera lens iris with concentric gold/cyan tracks and play prism core.
    pub fn draw_vertex_logo(painter: &Painter, center: Pos2, radius: f32) {
        // Outer dark titanium lens barrel
        painter.circle_filled(center, radius, Color32::from_rgb(22, 24, 30));
        painter.circle_stroke(center, radius, Stroke::new(1.0, Color32::from_rgb(62, 66, 78)));

        // Concentric precision track rings (gold & cyan)
        painter.circle_stroke(center, radius * 0.82, Stroke::new(0.8, Color32::from_rgb(230, 168, 42)));
        painter.circle_stroke(center, radius * 0.68, Stroke::new(0.6, Color32::from_rgb(45, 180, 240)));

        // Inner shutter pupil
        painter.circle_filled(center, radius * 0.54, Color32::from_rgb(10, 12, 18));
        painter.circle_stroke(center, radius * 0.54, Stroke::new(0.6, Color32::from_rgb(70, 205, 255)));

        // Crystalline glowing play prism core
        let tri_w = radius * 0.40;
        let tri_h = radius * 0.35;
        let tri_center = center + Vec2::new(radius * 0.04, 0.0);
        let p1 = tri_center + Vec2::new(-tri_w * 0.5, -tri_h);
        let p2 = tri_center + Vec2::new(-tri_w * 0.5, tri_h);
        let p3 = tri_center + Vec2::new(tri_w, 0.0);
        painter.add(Shape::convex_polygon(
            vec![p1, p3, p2],
            Color32::from_rgb(255, 225, 80),
            Stroke::new(0.7, Color32::from_rgb(90, 220, 255)),
        ));
    }

    pub fn draw_vortex_logo(painter: &Painter, center: Pos2, radius: f32) {
        Self::draw_vertex_logo(painter, center, radius);
    }

    /// Large Hero Cinema Aperture Iris Emblem with multi-track rings, shutter blades, and glowing optical core.
    pub fn draw_vortex_hero_logo(painter: &Painter, center: Pos2, radius: f32) {
        // 1. Ambient outer aura
        painter.circle_filled(center, radius * 1.8, Color32::from_rgba_unmultiplied(40, 140, 255, 14));
        painter.circle_filled(center, radius * 1.45, Color32::from_rgba_unmultiplied(245, 166, 35, 18));

        // 2. Knurled outer barrel & titanium rim
        painter.circle_filled(center, radius * 1.15, Color32::from_rgb(18, 20, 26));
        painter.circle_stroke(center, radius * 1.15, Stroke::new(2.0, Color32::from_rgb(72, 78, 92)));
        painter.circle_stroke(center, radius * 1.08, Stroke::new(1.0, Color32::from_rgb(42, 46, 56)));

        // 3. Concentric precision cinema tracks (gold & cyan)
        painter.circle_stroke(center, radius * 0.95, Stroke::new(1.4, Color32::from_rgb(230, 170, 45)));
        painter.circle_stroke(center, radius * 0.88, Stroke::new(0.8, Color32::from_rgb(34, 38, 48)));
        painter.circle_stroke(center, radius * 0.80, Stroke::new(1.2, Color32::from_rgb(50, 185, 245)));
        painter.circle_stroke(center, radius * 0.72, Stroke::new(1.0, Color32::from_rgb(210, 150, 35)));

        // 4. Shutter aperture blades
        painter.circle_filled(center, radius * 0.65, Color32::from_rgb(14, 16, 22));
        for i in 0..8 {
            let angle = (i as f32) * (std::f32::consts::PI / 4.0);
            let r1 = radius * 0.35;
            let r2 = radius * 0.65;
            let p_start = center + Vec2::new(angle.cos() * r1, angle.sin() * r1);
            let p_end = center + Vec2::new((angle + 0.6).cos() * r2, (angle + 0.6).sin() * r2);
            painter.line_segment([p_start, p_end], Stroke::new(1.0, Color32::from_rgb(46, 54, 70)));
        }

        // 5. Deep optical vortex iris pupil
        painter.circle_filled(center, radius * 0.35, Color32::from_rgb(10, 14, 28));
        painter.circle_stroke(center, radius * 0.35, Stroke::new(1.6, Color32::from_rgb(70, 205, 255)));

        // 6. Crystalline play prism core
        let tri_w = radius * 0.28;
        let tri_h = radius * 0.24;
        let tri_center = center + Vec2::new(radius * 0.03, 0.0);
        let p1 = tri_center + Vec2::new(-tri_w * 0.5, -tri_h);
        let p2 = tri_center + Vec2::new(-tri_w * 0.5, tri_h);
        let p3 = tri_center + Vec2::new(tri_w, 0.0);
        painter.add(Shape::convex_polygon(
            vec![p1, p3, p2],
            Color32::from_rgb(255, 225, 90),
            Stroke::new(1.4, Color32::from_rgb(100, 220, 255)),
        ));
        painter.circle_filled(tri_center, radius * 0.04, Color32::WHITE);
    }

    /// Crisp equilateral triangle — Play button (9px wide x 11px tall).
    pub fn draw_play(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let p1 = c + Vec2::new(-4.5, -5.5);
        let p2 = c + Vec2::new(-4.5,  5.5);
        let p3 = c + Vec2::new( 5.5,  0.0);
        painter.add(eframe::egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            color,
            Stroke::NONE,
        ));
    }

    /// Two crisp vertical bars — Pause button (10px wide x 11px tall).
    pub fn draw_pause(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let r1 = Rect::from_center_size(c - Vec2::new(3.0, 0.0), Vec2::new(2.5, 11.0));
        let r2 = Rect::from_center_size(c + Vec2::new(3.0, 0.0), Vec2::new(2.5, 11.0));
        painter.rect_filled(r1, 0.0, color);
        painter.rect_filled(r2, 0.0, color);
    }

    /// Crisp square — Stop button (9px x 9px).
    pub fn draw_stop(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let r = Rect::from_center_size(c, Vec2::new(8.5, 8.5));
        painter.rect_filled(r, 0.0, color);
    }

    /// Vertical bar + left triangle — Previous track (10px wide x 10px tall).
    pub fn draw_prev(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        // Bar
        let bar = Rect::from_center_size(c - Vec2::new(4.5, 0.0), Vec2::new(1.8, 10.0));
        painter.rect_filled(bar, 0.0, color);

        // Left triangle
        let p1 = c + Vec2::new( 4.5, -5.0);
        let p2 = c + Vec2::new( 4.5,  5.0);
        let p3 = c + Vec2::new(-2.5,  0.0);
        painter.add(eframe::egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            color,
            Stroke::NONE,
        ));
    }

    /// Right triangle + vertical bar — Next track (10px wide x 10px tall).
    pub fn draw_next(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        // Right triangle
        let p1 = c + Vec2::new(-4.5, -5.0);
        let p2 = c + Vec2::new(-4.5,  5.0);
        let p3 = c + Vec2::new( 2.5,  0.0);
        painter.add(eframe::egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            color,
            Stroke::NONE,
        ));

        // Bar
        let bar = Rect::from_center_size(c + Vec2::new(4.5, 0.0), Vec2::new(1.8, 10.0));
        painter.rect_filled(bar, 0.0, color);
    }

    /// Chapter previous (Bar + double left triangles: |<<).
    pub fn draw_chapter_prev(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let bar = Rect::from_center_size(c - Vec2::new(5.5, 0.0), Vec2::new(1.6, 9.0));
        painter.rect_filled(bar, 0.0, color);

        let p1 = c + Vec2::new(-0.5, -4.5);
        let p2 = c + Vec2::new(-0.5,  4.5);
        let p3 = c + Vec2::new(-4.5,  0.0);
        painter.add(eframe::egui::Shape::convex_polygon(vec![p1, p2, p3], color, Stroke::NONE));

        let q1 = c + Vec2::new( 4.5, -4.5);
        let q2 = c + Vec2::new( 4.5,  4.5);
        let q3 = c + Vec2::new( 0.5,  0.0);
        painter.add(eframe::egui::Shape::convex_polygon(vec![q1, q2, q3], color, Stroke::NONE));
    }

    /// Chapter next (Double right triangles + bar: >>|).
    pub fn draw_chapter_next(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let p1 = c + Vec2::new(-4.5, -4.5);
        let p2 = c + Vec2::new(-4.5,  4.5);
        let p3 = c + Vec2::new(-0.5,  0.0);
        painter.add(eframe::egui::Shape::convex_polygon(vec![p1, p2, p3], color, Stroke::NONE));

        let q1 = c + Vec2::new(-0.5, -4.5);
        let q2 = c + Vec2::new(-0.5,  4.5);
        let q3 = c + Vec2::new( 3.5,  0.0);
        painter.add(eframe::egui::Shape::convex_polygon(vec![q1, q2, q3], color, Stroke::NONE));

        let bar = Rect::from_center_size(c + Vec2::new(5.5, 0.0), Vec2::new(1.6, 9.0));
        painter.rect_filled(bar, 0.0, color);
    }

    /// Frame step forward (Small triangle + bar).
    pub fn draw_step_forward(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let p1 = c + Vec2::new(-4.0, -4.5);
        let p2 = c + Vec2::new(-4.0,  4.5);
        let p3 = c + Vec2::new( 1.5,  0.0);
        painter.add(eframe::egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            color,
            Stroke::NONE,
        ));
        let bar = Rect::from_center_size(c + Vec2::new(3.5, 0.0), Vec2::new(1.5, 9.0));
        painter.rect_filled(bar, 0.0, color);
    }

    /// Eject / Open icon (Triangle over horizontal bar).
    pub fn draw_eject(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        // Upward triangle
        let p1 = c + Vec2::new( 0.0, -4.5);
        let p2 = c + Vec2::new(-4.5,  1.0);
        let p3 = c + Vec2::new( 4.5,  1.0);
        painter.add(eframe::egui::Shape::convex_polygon(
            vec![p1, p2, p3],
            color,
            Stroke::NONE,
        ));
        // Bottom bar
        let bar = Rect::from_center_size(c + Vec2::new(0.0, 3.8), Vec2::new(9.5, 1.8));
        painter.rect_filled(bar, 0.0, color);
    }

    /// Speaker volume icon with sound waves.
    pub fn draw_volume(painter: &Painter, rect: Rect, color: Color32, is_muted: bool, volume: f64) {
        let c = rect.center();
        let sc = c - Vec2::new(3.0, 0.0);

        // Speaker cone
        let p1 = sc + Vec2::new(-3.5, -2.5);
        let p2 = sc + Vec2::new(-1.5, -2.5);
        let p3 = sc + Vec2::new( 1.5, -5.0);
        let p4 = sc + Vec2::new( 1.5,  5.0);
        let p5 = sc + Vec2::new(-1.5,  2.5);
        let p6 = sc + Vec2::new(-3.5,  2.5);
        painter.add(eframe::egui::Shape::convex_polygon(
            vec![p1, p2, p3, p4, p5, p6],
            color,
            Stroke::NONE,
        ));

        if is_muted || volume < 0.5 {
            // Red 'X' for mute
            let x_c = c + Vec2::new(4.5, 0.0);
            painter.line_segment([x_c + Vec2::new(-2.5, -2.5), x_c + Vec2::new(2.5, 2.5)], Stroke::new(1.4, Color32::from_rgb(235, 60, 60)));
            painter.line_segment([x_c + Vec2::new(2.5, -2.5), x_c + Vec2::new(-2.5, 2.5)], Stroke::new(1.4, Color32::from_rgb(235, 60, 60)));
        } else {
            // Wave 1
            let w1_c = c + Vec2::new(3.0, 0.0);
            painter.line_segment([w1_c + Vec2::new(0.0, -2.5), w1_c + Vec2::new(1.5, 0.0)], Stroke::new(1.2, color));
            painter.line_segment([w1_c + Vec2::new(1.5, 0.0), w1_c + Vec2::new(0.0, 2.5)], Stroke::new(1.2, color));

            if volume > 40.0 {
                // Wave 2
                let w2_c = c + Vec2::new(6.0, 0.0);
                painter.line_segment([w2_c + Vec2::new(0.0, -4.5), w2_c + Vec2::new(2.0, 0.0)], Stroke::new(1.2, color));
                painter.line_segment([w2_c + Vec2::new(2.0, 0.0), w2_c + Vec2::new(0.0, 4.5)], Stroke::new(1.2, color));
            }
        }
    }

    /// Quick Search / Scene Browser icon (Horizontal scan lines + magnifying glass).
    pub fn draw_search_list(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        // 3 scan lines on left
        painter.line_segment([c + Vec2::new(-6.2, -3.5), c + Vec2::new(-1.8, -3.5)], Stroke::new(1.2, color));
        painter.line_segment([c + Vec2::new(-6.2,  0.0), c + Vec2::new(-1.8,  0.0)], Stroke::new(1.2, color));
        painter.line_segment([c + Vec2::new(-6.2,  3.5), c + Vec2::new(-1.8,  3.5)], Stroke::new(1.2, color));

        // Magnifying glass on right
        let gc = c + Vec2::new(1.8, -0.8);
        painter.circle_stroke(gc, 3.2, Stroke::new(1.3, color));
        painter.line_segment([gc + Vec2::new(2.3, 2.3), gc + Vec2::new(5.0, 5.0)], Stroke::new(1.6, color));
    }

    /// Control Center / Bookmark List document icon (Rounded sheet with 3 lines).
    pub fn draw_document_list(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let sheet_r = Rect::from_center_size(c, Vec2::new(11.5, 12.5));
        painter.rect_stroke(sheet_r, CornerRadius::same(2), Stroke::new(1.2, color), StrokeKind::Inside);
        painter.line_segment([c + Vec2::new(-3.2, -2.6), c + Vec2::new(3.2, -2.6)], Stroke::new(1.0, color));
        painter.line_segment([c + Vec2::new(-3.2,  0.0), c + Vec2::new(3.2,  0.0)], Stroke::new(1.0, color));
        painter.line_segment([c + Vec2::new(-3.2,  2.6), c + Vec2::new(3.2,  2.6)], Stroke::new(1.0, color));
    }

    /// Settings 8-tooth gear icon.
    pub fn draw_settings(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let r_outer = 4.2;
        painter.circle_stroke(c, r_outer, Stroke::new(1.3, color));
        painter.circle_filled(c, 1.6, color);

        for step in 0..8 {
            let angle = (step as f32) * (std::f32::consts::PI / 4.0);
            let p1 = c + Vec2::new(angle.cos() * 3.6, angle.sin() * 3.6);
            let p2 = c + Vec2::new(angle.cos() * 5.6, angle.sin() * 5.6);
            painter.line_segment([p1, p2], Stroke::new(1.4, color));
        }
    }

    /// 3-line hamburger menu icon.
    pub fn draw_hamburger(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let w = 11.5;
        painter.line_segment([c + Vec2::new(-w/2.0, -3.5), c + Vec2::new(w/2.0, -3.5)], Stroke::new(1.4, color));
        painter.line_segment([c + Vec2::new(-w/2.0,  0.0), c + Vec2::new(w/2.0,  0.0)], Stroke::new(1.4, color));
        painter.line_segment([c + Vec2::new(-w/2.0,  3.5), c + Vec2::new(w/2.0,  3.5)], Stroke::new(1.4, color));
    }

    /// Down chevron ▾.
    pub fn draw_chevron_down(painter: &Painter, center: Pos2, size: f32, color: Color32) {
        let p1 = center + Vec2::new(-size, -size * 0.4);
        let p2 = center + Vec2::new(0.0,   size * 0.6);
        let p3 = center + Vec2::new(size,  -size * 0.4);
        painter.line_segment([p1, p2], Stroke::new(1.2, color));
        painter.line_segment([p2, p3], Stroke::new(1.2, color));
    }

    /// Pin icon.
    pub fn draw_pin(painter: &Painter, rect: Rect, color: Color32, is_pinned: bool) {
        let c = rect.center();
        // Pin head bar
        painter.line_segment([c + Vec2::new(-3.5, -3.0), c + Vec2::new(3.5, -3.0)], Stroke::new(1.3, color));
        // Pin body
        painter.line_segment([c + Vec2::new(-2.0, -3.0), c + Vec2::new(-1.0, 1.0)], Stroke::new(1.1, color));
        painter.line_segment([c + Vec2::new(2.0, -3.0), c + Vec2::new(1.0, 1.0)], Stroke::new(1.1, color));
        painter.line_segment([c + Vec2::new(-2.5, 1.0), c + Vec2::new(2.5, 1.0)], Stroke::new(1.3, color));
        // Pin needle
        painter.line_segment([c + Vec2::new(0.0, 1.0), c + Vec2::new(0.0, 5.5)], Stroke::new(1.2, color));
        if is_pinned {
            painter.circle_filled(c + Vec2::new(0.0, -1.0), 1.5, color);
        }
    }

    /// Window Minimize.
    pub fn draw_minimize(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        painter.line_segment([c + Vec2::new(-4.5, 0.0), c + Vec2::new(4.5, 0.0)], Stroke::new(1.2, color));
    }

    /// Window Maximize / Restore.
    pub fn draw_maximize(painter: &Painter, rect: Rect, color: Color32, is_max: bool) {
        let c = rect.center();
        if is_max {
            let r1 = Rect::from_center_size(c + Vec2::new(1.5, -1.5), Vec2::new(7.5, 7.5));
            painter.rect_stroke(r1, CornerRadius::ZERO, Stroke::new(1.1, color), StrokeKind::Inside);
            let r2 = Rect::from_center_size(c + Vec2::new(-1.5, 1.5), Vec2::new(7.5, 7.5));
            painter.rect_filled(r2, CornerRadius::ZERO, Color32::from_rgb(18, 18, 20));
            painter.rect_stroke(r2, CornerRadius::ZERO, Stroke::new(1.1, color), StrokeKind::Inside);
        } else {
            let r = Rect::from_center_size(c, Vec2::new(9.0, 9.0));
            painter.rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.1, color), StrokeKind::Inside);
        }
    }

    /// Window Close '✕'.
    pub fn draw_close(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let d = 4.5;
        painter.line_segment([c + Vec2::new(-d, -d), c + Vec2::new(d, d)], Stroke::new(1.2, color));
        painter.line_segment([c + Vec2::new(d, -d), c + Vec2::new(-d, d)], Stroke::new(1.2, color));
    }

    /// Repeat icon (looping rectangular arrows with optional '1' badge).
    pub fn draw_repeat(painter: &Painter, rect: Rect, color: Color32, is_one: bool) {
        let c = rect.center();
        let w = 11.0;
        let h = 7.5;
        let left = c.x - w / 2.0;
        let right = c.x + w / 2.0;
        let top = c.y - h / 2.0;
        let bottom = c.y + h / 2.0;

        // Top line going right with arrowhead
        painter.line_segment([Pos2::new(left + 2.0, top), Pos2::new(right - 1.0, top)], Stroke::new(1.2, color));
        // Top right arrowhead
        painter.line_segment([Pos2::new(right - 3.5, top - 2.5), Pos2::new(right - 1.0, top)], Stroke::new(1.2, color));
        painter.line_segment([Pos2::new(right - 3.5, top + 2.5), Pos2::new(right - 1.0, top)], Stroke::new(1.2, color));

        // Right down curve
        painter.line_segment([Pos2::new(right, top + 2.0), Pos2::new(right, bottom - 1.0)], Stroke::new(1.2, color));

        // Bottom line going left with arrowhead
        painter.line_segment([Pos2::new(right - 2.0, bottom), Pos2::new(left + 1.0, bottom)], Stroke::new(1.2, color));
        // Bottom left arrowhead
        painter.line_segment([Pos2::new(left + 3.5, bottom - 2.5), Pos2::new(left + 1.0, bottom)], Stroke::new(1.2, color));
        painter.line_segment([Pos2::new(left + 3.5, bottom + 2.5), Pos2::new(left + 1.0, bottom)], Stroke::new(1.2, color));

        // Left up line
        painter.line_segment([Pos2::new(left, bottom - 2.0), Pos2::new(left, top + 1.0)], Stroke::new(1.2, color));

        if is_one {
            painter.text(c, Align2::CENTER_CENTER, "1", FontId::monospace(8.0), color);
        }
    }

    /// Shuffle icon (intertwined crossing arrows).
    pub fn draw_shuffle(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let w = 11.0;
        let h = 8.0;
        let left = c.x - w / 2.0;
        let right = c.x + w / 2.0;
        let top = c.y - h / 2.0;
        let bottom = c.y + h / 2.0;

        // Top-left to bottom-right arrow
        painter.line_segment([Pos2::new(left, top), Pos2::new(left + 3.0, top)], Stroke::new(1.2, color));
        painter.line_segment([Pos2::new(left + 3.0, top), Pos2::new(right - 3.0, bottom)], Stroke::new(1.2, color));
        painter.line_segment([Pos2::new(right - 3.0, bottom), Pos2::new(right, bottom)], Stroke::new(1.2, color));
        // Bottom-right arrowhead
        painter.line_segment([Pos2::new(right - 2.5, bottom - 2.5), Pos2::new(right, bottom)], Stroke::new(1.2, color));
        painter.line_segment([Pos2::new(right - 2.5, bottom + 2.5), Pos2::new(right, bottom)], Stroke::new(1.2, color));

        // Bottom-left to top-right arrow
        painter.line_segment([Pos2::new(left, bottom), Pos2::new(left + 3.0, bottom)], Stroke::new(1.2, color));
        painter.line_segment([Pos2::new(left + 3.0, bottom), Pos2::new(left + 4.5, bottom - 2.5)], Stroke::new(1.2, color));
        painter.line_segment([Pos2::new(right - 4.5, top + 2.5), Pos2::new(right - 3.0, top)], Stroke::new(1.2, color));
        painter.line_segment([Pos2::new(right - 3.0, top), Pos2::new(right, top)], Stroke::new(1.2, color));
        // Top-right arrowhead
        painter.line_segment([Pos2::new(right - 2.5, top - 2.5), Pos2::new(right, top)], Stroke::new(1.2, color));
        painter.line_segment([Pos2::new(right - 2.5, top + 2.5), Pos2::new(right, top)], Stroke::new(1.2, color));
    }

    /// Crisp vector checkmark ✓
    pub fn draw_check(painter: &Painter, center: Pos2, size: f32, color: Color32) {
        let p1 = center + Vec2::new(-size * 0.42, -size * 0.05);
        let p2 = center + Vec2::new(-size * 0.08,  size * 0.35);
        let p3 = center + Vec2::new( size * 0.42, -size * 0.35);
        painter.line_segment([p1, p2], Stroke::new(1.6, color));
        painter.line_segment([p2, p3], Stroke::new(1.6, color));
    }

    /// Crisp circular radio button indicator
    pub fn draw_radio(painter: &Painter, center: Pos2, radius: f32, is_selected: bool, is_hovered: bool, color: Color32) {
        let border_col = if is_selected {
            color
        } else if is_hovered {
            Color32::from_rgb(130, 140, 165)
        } else {
            Color32::from_rgb(70, 75, 92)
        };
        painter.circle_stroke(center, radius, Stroke::new(1.2, border_col));
        if is_selected {
            painter.circle_filled(center, radius * 0.52, color);
        }
    }

    /// Crisp camera snapshot icon
    pub fn draw_camera(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let body = Rect::from_center_size(c + Vec2::new(0.0, 1.0), Vec2::new(13.0, 9.0));
        painter.rect_stroke(body, CornerRadius::same(1), Stroke::new(1.0, color), StrokeKind::Inside);
        let lens_c = c + Vec2::new(0.0, 1.0);
        painter.circle_stroke(lens_c, 2.2, Stroke::new(1.0, color));
        let flash = Rect::from_center_size(c + Vec2::new(3.0, -4.5), Vec2::new(3.0, 2.0));
        painter.rect_filled(flash, CornerRadius::ZERO, color);
    }

    /// Crisp subtitle CC icon
    pub fn draw_subtitles(painter: &Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let box_r = Rect::from_center_size(c, Vec2::new(14.0, 10.0));
        painter.rect_stroke(box_r, CornerRadius::same(1), Stroke::new(1.0, color), StrokeKind::Inside);
        painter.line_segment([box_r.left_top() + Vec2::new(2.5, 3.5), box_r.left_top() + Vec2::new(6.0, 3.5)], Stroke::new(1.0, color));
        painter.line_segment([box_r.left_top() + Vec2::new(7.5, 3.5), box_r.left_top() + Vec2::new(11.5, 3.5)], Stroke::new(1.0, color));
        painter.line_segment([box_r.left_top() + Vec2::new(2.5, 6.5), box_r.left_top() + Vec2::new(10.0, 6.5)], Stroke::new(1.0, color));
    }
}
