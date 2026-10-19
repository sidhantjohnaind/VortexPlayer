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
        let hud_rect = Rect::from_min_size(Pos2::new(16.0, 48.0), Vec2::new(340.0, 240.0));

        // Background
        painter.rect_filled(hud_rect, CornerRadius::same(6), Color32::from_rgba_premultiplied(10, 10, 16, 235));
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

        let avsync_ms = stats.avsync_drift * 1000.0;
        let (drop_text, drop_color) = if stats.dropped_frames > 0 {
            (
                format!(
                    "Drop: {} frames (VO: {}, Dec: {}, Mistimed: {}) ⚠",
                    stats.dropped_frames, stats.vo_drop_frame_count, stats.decoder_frame_drop_count, stats.mistimed_frame_count
                ),
                Color32::from_rgb(255, 95, 95),
            )
        } else {
            ("Drop: 0 frames (Flawless Playback ✓)".to_string(), Color32::from_rgb(90, 235, 120))
        };

        let lines: [(String, Color32); 8] = [
            (format!("Codec / HWDEC: {} ({})", stats.video_codec, stats.hwdec_current), Color32::from_rgb(200, 200, 220)),
            (format!("Resolution: {}x{} @ {:.2} FPS (Disp: {:.1}Hz)", stats.video_width, stats.video_height, stats.video_fps, stats.display_fps), Color32::from_rgb(200, 200, 220)),
            (format!("Bitrate: Video {:.1} kbps | Audio {:.1} kbps", stats.video_bitrate, stats.audio_bitrate), Color32::from_rgb(200, 200, 220)),
            (format!("Audio Stream: {} ({}ch @ {}Hz)", stats.audio_codec, stats.audio_channels, stats.audio_sample_rate), Color32::from_rgb(200, 200, 220)),
            (format!("Demuxer Cache: {:.1}%", stats.cache_buffer_percent), Color32::from_rgb(200, 200, 220)),
            (format!("A/V Sync Drift: {:+.1} ms | Jitter: {:.2} ms", avsync_ms, stats.vsync_jitter * 1000.0), Color32::from_rgb(200, 200, 220)),
            (drop_text, drop_color),
            (format!("Current Speed: {:.2}x | Vol: {:.0}%", stats.speed, stats.volume), Color32::from_rgb(200, 200, 220)),
        ];

        for (i, (line, color)) in lines.iter().enumerate() {
            painter.text(
                Pos2::new(26.0, 80.0 + (i as f32 * 18.0)),
                Align2::LEFT_TOP,
                line,
                font_mono.clone(),
                *color,
            );
        }
    }
}
