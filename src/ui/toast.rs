#![allow(dead_code)]

use super::theme::VortexTheme;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Stroke, StrokeKind, Vec2};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

pub struct Toast {
    pub message: String,
    pub kind: ToastKind,
    pub created_at: Instant,
    pub duration: Duration,
}

pub struct ToastManager {
    pub toasts: Vec<Toast>,
}

impl Default for ToastManager {
    fn default() -> Self {
        Self {
            toasts: Vec::new(),
        }
    }
}

impl ToastManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, message: String, kind: ToastKind, duration_ms: u64) {
        self.toasts.push(Toast {
            message,
            kind,
            created_at: Instant::now(),
            duration: Duration::from_millis(duration_ms),
        });
    }

    pub fn info(&mut self, msg: impl Into<String>) {
        self.push(msg.into(), ToastKind::Info, 3000);
    }

    pub fn success(&mut self, msg: impl Into<String>) {
        self.push(msg.into(), ToastKind::Success, 2500);
    }

    pub fn warning(&mut self, msg: impl Into<String>) {
        self.push(msg.into(), ToastKind::Warning, 4000);
    }

    pub fn warn(&mut self, msg: impl Into<String>) {
        self.warning(msg);
    }

    pub fn error_toast(&mut self, msg: impl Into<String>) {
        self.push(msg.into(), ToastKind::Error, 5000);
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        self.toasts.retain(|t| t.created_at.elapsed() < t.duration);
        if self.toasts.is_empty() {
            return;
        }

        let (win_w, win_h) = ctx.input(|i| {
            i.viewport()
                .inner_rect
                .map(|r| (r.width(), r.height()))
                .or_else(|| i.raw.screen_rect.map(|r| (r.width(), r.height())))
                .unwrap_or((1100.0, 680.0))
        });
        let screen_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(win_w, win_h));

        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Tooltip,
            egui::Id::new("vortex_toasts"),
        ));

        let toast_h = 32.0;
        let gap = 6.0;

        for (i, toast) in self.toasts.iter().rev().take(5).enumerate() {
            let elapsed = toast.created_at.elapsed().as_secs_f32();
            let total_dur = toast.duration.as_secs_f32();
            let remaining = (total_dur - elapsed).max(0.0);

            let fade_in = (elapsed / 0.15).min(1.0);
            let fade_out = (remaining / 0.25).min(1.0);
            let alpha_factor = fade_in.min(fade_out);
            let alpha = (alpha_factor * 250.0) as u8;

            if alpha == 0 {
                continue;
            }

            // Dynamically calculate toast width based on message length
            let text_w = toast.message.len() as f32 * 7.0;
            let toast_w = (text_w + 52.0).clamp(220.0, (screen_rect.width() - 40.0).max(220.0));

            let x = screen_rect.right() - toast_w - 18.0;
            let y = screen_rect.bottom() - 85.0 - (i as f32 * (toast_h + gap));
            let rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(toast_w, toast_h));

            let (accent_color, icon) = match toast.kind {
                ToastKind::Info => (VortexTheme::POT_CYAN, "ℹ"),
                ToastKind::Success => (Color32::from_rgb(0, 220, 100), "✓"),
                ToastKind::Warning => (Color32::from_rgb(255, 185, 35), "⚠"),
                ToastKind::Error => (Color32::from_rgb(245, 50, 60), "✕"),
            };

            // Card background (translucent dark with alpha)
            painter.rect_filled(
                rect,
                CornerRadius::same(4),
                Color32::from_rgba_premultiplied(16, 18, 24, alpha),
            );
            painter.rect_stroke(
                rect,
                CornerRadius::same(4),
                Stroke::new(
                    1.0,
                    Color32::from_rgba_premultiplied(48, 52, 65, alpha),
                ),
                StrokeKind::Inside,
            );

            // Left accent vertical bar
            let bar_rect = Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height()));
            painter.rect_filled(
                bar_rect,
                CornerRadius {
                    nw: 4,
                    ne: 0,
                    sw: 4,
                    se: 0,
                },
                Color32::from_rgba_premultiplied(
                    accent_color.r(),
                    accent_color.g(),
                    accent_color.b(),
                    alpha,
                ),
            );

            // Icon
            painter.text(
                rect.left_center() + Vec2::new(13.0, 0.0),
                Align2::LEFT_CENTER,
                icon,
                FontId::proportional(13.0),
                Color32::from_rgba_premultiplied(
                    accent_color.r(),
                    accent_color.g(),
                    accent_color.b(),
                    alpha,
                ),
            );

            // Message text
            painter.text(
                rect.left_center() + Vec2::new(30.0, 0.0),
                Align2::LEFT_CENTER,
                &toast.message,
                FontId::proportional(12.0),
                Color32::from_rgba_premultiplied(235, 238, 245, alpha),
            );
        }
    }
}

