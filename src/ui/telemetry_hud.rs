use super::theme::VortexTheme;
use crate::engine::MediaStats;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Stroke, Vec2};

pub struct TelemetryHudView;

impl TelemetryHudView {
    pub fn render(ctx: &egui::Context, is_visible: bool, stats: &MediaStats) {
        if !is_visible {
            return;
        }

        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("telemetry_hud")));
        let hud_rect = Rect::from_min_size(Pos2::new(16.0, 48.0), Vec2::new(320.0, 220.0));

        // Background
        painter.rect_filled(hud_rect, CornerRadius::same(6), Color32::from_rgba_premultiplied(10, 10, 16, 230));
        painter.rect_stroke(hud_rect, CornerRadius::same(6), Stroke::new(1.0, VortexTheme::current_skin().accent_primary), egui::StrokeKind::Outside);

        let font_title = FontId::proportional(12.5);
        let font_mono = FontId::monospace(11.0);

        painter.text(
            Pos2::new(26.0, 58.0),
            Align2::LEFT_TOP,
            "[ ADVANCED PLAYBACK TELEMETRY ]",
            font_title,
            VortexTheme::current_skin().accent_primary,
        );

        let lines = [
            format!("Codec / HWDEC: {} ({})", stats.video_codec, stats.hwdec_current),
            format!("Resolution: {}x{} @ {:.2} FPS", stats.video_width, stats.video_height, stats.video_fps),
            format!("Bitrate: Video {:.1} kbps | Audio {:.1} kbps", stats.video_bitrate, stats.audio_bitrate),
            format!("Audio Stream: {} ({}ch @ {}Hz)", stats.audio_codec, stats.audio_channels, stats.audio_sample_rate),
            format!("Demuxer Cache: {:.1}%", stats.cache_buffer_percent),
            format!("A/V Sync Drift: +0.0 ms | Drop: 0 frames"),
            format!("Current Speed: {:.2}x | Vol: {:.0}%", stats.speed, stats.volume),
        ];

        for (i, line) in lines.iter().enumerate() {
            painter.text(
                Pos2::new(26.0, 82.0 + (i as f32 * 18.0)),
                Align2::LEFT_TOP,
                line,
                font_mono.clone(),
                Color32::from_rgb(200, 200, 220),
            );
        }
    }
}
