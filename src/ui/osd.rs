//! osd.rs — Ultra-Polished Modern Glassmorphism OSD Engine with Central Transport Animations

#![allow(dead_code)]

use super::theme::VortexTheme;
use crate::bookmark::format_time;
use crate::engine::MediaStats;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Stroke, StrokeKind, Vec2,
};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct CentralHudIndicator {
    pub icon: &'static str,
    pub label: String,
    pub sub_label: Option<String>,
    pub created_at: Instant,
    pub duration: Duration,
}

pub struct OsdEngine {
    pub message: Option<String>,
    pub is_new_message: bool,
    pub created_at: Instant,
    pub expires_at: Instant,
    pub duration: Duration,
    pub show_media_info: bool,
    pub center_indicator: Option<CentralHudIndicator>,
}

impl Default for OsdEngine {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            message: None,
            is_new_message: false,
            created_at: now,
            expires_at: now,
            duration: Duration::from_millis(1200),
            show_media_info: false,
            center_indicator: None,
        }
    }
}

impl OsdEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, text: String, duration_ms: u64) {
        let now = Instant::now();
        let dur = Duration::from_millis(duration_ms);
        self.message = Some(text);
        self.is_new_message = true;
        self.created_at = now;
        self.duration = dur;
        self.expires_at = now + dur;
    }

    pub fn show_center_indicator(&mut self, icon: &'static str, label: String, sub_label: Option<String>, duration_ms: u64) {
        let now = Instant::now();
        self.center_indicator = Some(CentralHudIndicator {
            icon,
            label,
            sub_label,
            created_at: now,
            duration: Duration::from_millis(duration_ms),
        });
    }

    pub fn show_volume(&mut self, volume: f64, is_muted: bool) {
        if is_muted {
            self.show("🔇 Volume: MUTE".to_string(), 1200);
            self.show_center_indicator("🔇", "MUTE".to_string(), None, 600);
        } else {
            let total_bars = 16;
            let norm_vol = volume.clamp(0.0, 100.0);
            let filled_bars = ((norm_vol / 100.0) * total_bars as f64).round() as usize;
            let mut bar_str = String::new();
            for i in 0..total_bars {
                if i < filled_bars {
                    bar_str.push('█');
                } else {
                    bar_str.push('░');
                }
            }
            self.show(format!("🔊 Volume: {:3.0}%  {}", norm_vol, bar_str), 1200);
            self.show_center_indicator("🔊", format!("{:.0}%", norm_vol), Some(bar_str), 600);
        }
    }

    pub fn show_seek(&mut self, target_time: f64, duration: f64, delta: Option<f64>) {
        let (icon, delta_str) = match delta {
            Some(d) if d > 0.0 => ("⏩", format!("  [+{:.1}s]", d)),
            Some(d) if d < 0.0 => ("⏪", format!("  [{:.1}s]", d)),
            _ => ("⏱", String::new()),
        };
        self.show(
            format!("{} {} / {}{}", icon, format_time(target_time), format_time(duration), delta_str),
            1200,
        );
        if let Some(d) = delta {
            let sign = if d >= 0.0 { "+" } else { "" };
            self.show_center_indicator(
                if d >= 0.0 { "⏩" } else { "⏪" },
                format!("{}{:.1}s", sign, d),
                Some(format!("{} / {}", format_time(target_time), format_time(duration))),
                600,
            );
        }
    }

    pub fn show_play_pause(&mut self, is_paused: bool) {
        if is_paused {
            self.show("⏸ Paused".to_string(), 1000);
            self.show_center_indicator("⏸", "Paused".to_string(), None, 500);
        } else {
            self.show("▶ Playing".to_string(), 1000);
            self.show_center_indicator("▶", "Playing".to_string(), None, 500);
        }
    }

    pub fn show_speed(&mut self, speed: f64) {
        self.show(format!("⚡ Playback Speed: {:.2}×", speed), 1200);
        self.show_center_indicator("⚡", format!("{:.2}× Speed", speed), None, 600);
    }

    pub fn show_sub_delay(&mut self, delay_secs: f64) {
        let delay_ms = (delay_secs * 1000.0).round() as i64;
        let sign = if delay_secs > 0.0001 { "+" } else { "" };
        let status_str = if delay_ms.abs() < 5 {
            "💬 Subtitle Sync: 0.00s (Default / Synced)".to_string()
        } else {
            format!("💬 Subtitle Sync: {}{:.3} s ({}{:+0} ms)", sign, delay_secs, sign, delay_ms)
        };
        self.show(status_str, 1500);

        let icon = if delay_ms == 0 { "💬" } else if delay_secs > 0.0 { "⏳" } else { "⚡" };
        let center_main = if delay_ms == 0 {
            "SYNC 0.0s".to_string()
        } else {
            format!("{}{:.2}s", sign, delay_secs)
        };
        let center_sub = if delay_ms == 0 {
            "Default Sync".to_string()
        } else {
            format!("Subtitle Delay: {:+0} ms", delay_ms)
        };
        self.show_center_indicator(icon, center_main, Some(center_sub), 700);
    }

    pub fn show_audio_delay(&mut self, delay_ms: i64) {
        self.show(format!("🎧 Audio Sync: {:+0} ms", delay_ms), 1200);
    }

    pub fn show_pan_scan(&mut self, zoom: f64, preset: &str) {
        self.show(format!("🔍 Zoom: {:.0}% [{}]", zoom * 100.0, preset), 1200);
    }

    pub fn show_jump_percent(&mut self, percent: u32, time: f64) {
        self.show(format!("🎯 Jump: {}% [{}]", percent, format_time(time)), 1200);
    }

    pub fn show_chapter(&mut self, index: usize, total: usize, title: &str) {
        self.show(format!("📑 Chapter {}/{}: {}", index, total, title), 1800);
    }

    pub fn show_resize(&mut self, width: u32, height: u32, video_w: u32, video_h: u32) {
        let ar_str = format_aspect_ratio_str(width, height);
        let pct_str = if video_w > 0 && video_h > 0 {
            let pct = (width as f64 / video_w as f64) * 100.0;
            format!(" ({:.1}%)", pct)
        } else {
            String::new()
        };

        let msg = format!("📐 {} × {} [{}]{}", width, height, ar_str, pct_str);
        self.show(msg, 1200);
        self.show_center_indicator(
            "📐",
            format!("{} × {}", width, height),
            Some(if !pct_str.is_empty() {
                format!("{} {}", ar_str, pct_str.trim())
            } else {
                ar_str
            }),
            800,
        );
    }

    pub fn toggle_media_info(&mut self) {
        self.show_media_info = !self.show_media_info;
    }

    pub fn render(&mut self, ui: &egui::Ui, video_rect: Rect, stats: &MediaStats) {
        self.render_ctx(ui.ctx(), video_rect, stats);
    }

    pub fn render_ctx(&mut self, ctx: &egui::Context, video_rect: Rect, stats: &MediaStats) {
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("vortex_osd_layer")));
        let now = Instant::now();

        if self.center_indicator.is_some() || self.message.is_some() {
            ctx.request_repaint();
        }

        // ── 1. Center Floating Transport Badge (Animated Fade & Scale) ────────
        if let Some(ref ind) = self.center_indicator {
            let elapsed = now.duration_since(ind.created_at).as_secs_f32();
            let total_sec = ind.duration.as_secs_f32();
            if elapsed < total_sec {
                let remaining = total_sec - elapsed;
                let alpha_in = (elapsed / 0.08).min(1.0);
                let alpha_out = (remaining / 0.20).min(1.0);
                let alpha_factor = alpha_in.min(alpha_out);
                let alpha = (alpha_factor * 255.0) as u8;

                let center_x = video_rect.center().x;
                let center_y = video_rect.center().y;

                let badge_w = 200.0;
                let badge_h = if ind.sub_label.is_some() { 100.0 } else { 88.0 };
                let badge_rect = Rect::from_center_size(Pos2::new(center_x, center_y), Vec2::new(badge_w, badge_h));

                // Frosted Glass Dark Background
                let bg_col = Color32::from_rgba_unmultiplied(12, 14, 20, (alpha_factor * 225.0) as u8);
                let stroke_col = Color32::from_rgba_unmultiplied(245, 166, 35, (alpha_factor * 190.0) as u8);
                painter.rect_filled(badge_rect, CornerRadius::same(14), bg_col);
                painter.rect_stroke(badge_rect, CornerRadius::same(14), Stroke::new(1.4, stroke_col), StrokeKind::Inside);

                // Icon (Big 30px)
                let icon_y = if ind.sub_label.is_some() { badge_rect.top() + 24.0 } else { badge_rect.top() + 28.0 };
                painter.text(
                    Pos2::new(center_x, icon_y),
                    Align2::CENTER_CENTER,
                    ind.icon,
                    FontId::proportional(28.0),
                    Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                );

                // Primary Label
                let label_y = icon_y + 24.0;
                painter.text(
                    Pos2::new(center_x, label_y),
                    Align2::CENTER_CENTER,
                    &ind.label,
                    FontId::proportional(13.5),
                    Color32::from_rgba_unmultiplied(240, 245, 255, alpha),
                );

                // Sub Label (if any)
                if let Some(ref sub) = ind.sub_label {
                    let sub_y = label_y + 18.0;
                    painter.text(
                        Pos2::new(center_x, sub_y),
                        Align2::CENTER_CENTER,
                        sub,
                        FontId::monospace(10.5),
                        Color32::from_rgba_unmultiplied(140, 190, 255, alpha),
                    );
                }
            } else {
                self.center_indicator = None;
            }
        }

        // ── 2. Top-Left Modern Glassmorphism Pill Toast Notification ──────────
        if let Some(ref msg) = self.message {
            if now < self.expires_at {
                let elapsed = now.duration_since(self.created_at).as_secs_f32();
                let remaining = self.expires_at.duration_since(now).as_secs_f32();

                let alpha_in = (elapsed / 0.10).min(1.0);
                let alpha_out = (remaining / 0.25).min(1.0);
                let alpha_factor = alpha_in.min(alpha_out);
                let alpha = (alpha_factor * 255.0) as u8;

                let font = FontId::proportional(13.5);
                let text_color = Color32::from_rgba_unmultiplied(245, 248, 255, alpha);
                let galley = painter.layout_no_wrap(msg.clone(), font.clone(), text_color);
                let text_size = galley.size();

                let padding = Vec2::new(14.0, 7.0);
                let toast_rect = Rect::from_min_size(
                    video_rect.min + Vec2::new(20.0, 20.0),
                    text_size + padding * 2.0,
                );

                // Glassmorphism background
                let bg_alpha = (alpha_factor * 220.0) as u8;
                painter.rect_filled(
                    toast_rect,
                    CornerRadius::same(8),
                    Color32::from_rgba_unmultiplied(14, 16, 22, bg_alpha),
                );
                // Clean subtle border (no bright blue accent border)
                painter.rect_stroke(
                    toast_rect,
                    CornerRadius::same(8),
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(48, 52, 65, (alpha_factor * 180.0) as u8)),
                    StrokeKind::Inside,
                );

                // Render Text
                painter.text(
                    toast_rect.left_top() + padding,
                    Align2::LEFT_TOP,
                    msg,
                    font,
                    text_color,
                );
            } else {
                self.message = None;
            }
        }

        // ── 3. Playback Info Diagnostics HUD (Tab / Ctrl+F1) ─────────────────
        if self.show_media_info {
            let hud_rect = Rect::from_min_size(
                video_rect.min + Vec2::new(16.0, 16.0),
                Vec2::new(480.0, 280.0),
            );

            // Dark semi-transparent HUD background
            painter.rect_filled(hud_rect, CornerRadius::same(8), Color32::from_rgba_unmultiplied(10, 12, 16, 240));
            painter.rect_stroke(hud_rect, CornerRadius::same(8), Stroke::new(1.0, Color32::from_rgb(45, 65, 95)), StrokeKind::Inside);

            let font_head = FontId::proportional(13.0);
            let font_mono = FontId::monospace(10.5);

            painter.text(
                hud_rect.left_top() + Vec2::new(14.0, 12.0),
                Align2::LEFT_TOP,
                "⚡ VortexPlayer Playback Diagnostics & Codec HUD",
                font_head,
                VortexTheme::POT_YELLOW,
            );

            let bit_depth_str = if stats.pixel_format.contains("10") || stats.pixel_format.contains("p010") {
                "10-bit"
            } else if stats.pixel_format.contains("12") {
                "12-bit"
            } else {
                "8-bit"
            };

            let v_codec = if stats.video_codec.is_empty() { "None".to_string() } else { stats.video_codec.to_uppercase() };
            let a_codec = if stats.audio_codec.is_empty() { "None".to_string() } else { stats.audio_codec.to_uppercase() };

            let lines = [
                format!("File: {}", if stats.file_path.is_empty() { "No Media Loaded" } else { &stats.file_path }),
                format!("Video Codec: {} ({}x{} @ {:.2} fps, {})", v_codec, stats.video_width, stats.video_height, stats.video_fps, bit_depth_str),
                format!("Color Matrix / HDR: {} | {} ({})", if stats.colormatrix.is_empty() { "BT.709" } else { &stats.colormatrix }, stats.hdr_format, if stats.is_hdr { "Wide Gamut HDR" } else { "SDR" }),
                format!("Video Bitrate: {} kbps | PixFmt: {}", stats.video_bitrate / 1000, if stats.pixel_format.is_empty() { "yuv420p" } else { &stats.pixel_format }),
                format!("Audio Codec: {} ({} ch, {} Hz, {} kbps)", a_codec, stats.audio_channels, stats.audio_sample_rate, stats.audio_bitrate / 1000),
                format!("Hardware Decoder: {}", stats.hwdec_current),
                format!("HDR Tone Mapping: {}", stats.hdr_tone_mapping),
                format!("Playback Time: {} / {} ({:.1}%)", format_time(stats.time_pos), format_time(stats.duration), stats.percent_pos),
                format!("Speed: {:.2}× | Volume: {:.0}%", stats.speed, stats.volume),
                format!("Dropped Frames: {} | Buffer Cache: {:.1}%", stats.dropped_frames, stats.cache_buffer_percent),
            ];

            let mut y = hud_rect.top() + 36.0;
            for line in lines {
                painter.text(
                    Pos2::new(hud_rect.left() + 14.0, y),
                    Align2::LEFT_TOP,
                    line,
                    font_mono.clone(),
                    Color32::from_rgb(130, 230, 160),
                );
                y += 18.0;
            }
        }
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    if a == 0 {
        1
    } else {
        a
    }
}

pub fn format_aspect_ratio_str(w: u32, h: u32) -> String {
    if w == 0 || h == 0 {
        return String::new();
    }
    let ratio = w as f64 / h as f64;
    if (ratio - 16.0 / 9.0).abs() < 0.03 {
        "16:9".to_string()
    } else if (ratio - 16.0 / 10.0).abs() < 0.03 {
        "16:10".to_string()
    } else if (ratio - 4.0 / 3.0).abs() < 0.03 {
        "4:3".to_string()
    } else if (ratio - 21.0 / 9.0).abs() < 0.04 || (ratio - 2.35).abs() < 0.04 {
        "21:9".to_string()
    } else if (ratio - 1.85).abs() < 0.03 {
        "1.85:1".to_string()
    } else {
        let g = gcd(w, h);
        let aw = w / g;
        let ah = h / g;
        if aw <= 32 && ah <= 32 {
            format!("{}:{}", aw, ah)
        } else {
            format!("{:.2}:1", ratio)
        }
    }
}
