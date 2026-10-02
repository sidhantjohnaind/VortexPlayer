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

    #[inline]
    pub fn show_center_indicator(&mut self, _icon: &'static str, _label: String, _sub_label: Option<String>, _duration_ms: u64) {
        // Disabled: user requested removing central indicator, only keep sleek top-left HUD pill
    }

    pub fn show_volume(&mut self, volume: f64, is_muted: bool) {
        if is_muted {
            self.show("Volume: MUTE".to_string(), 1200);
        } else {
            let total_bars = 16;
            let norm_vol = volume.clamp(0.0, 100.0);
            let filled_bars = ((norm_vol / 100.0) * total_bars as f64).round() as usize;
            let mut bar_str = String::new();
            for i in 0..total_bars {
                if i < filled_bars {
                    bar_str.push('|');
                } else {
                    bar_str.push('.');
                }
            }
            self.show(format!("Volume: {:3.0}%  [{}]", norm_vol, bar_str), 1200);
        }
    }

    pub fn show_seek(&mut self, target_time: f64, duration: f64, delta: Option<f64>) {
        let delta_str = match delta {
            Some(d) if d > 0.0 => format!("  [+{:.1}s]", d),
            Some(d) if d < 0.0 => format!("  [{:.1}s]", d),
            _ => String::new(),
        };
        self.show(
            format!("Seek: {} / {}{}", format_time(target_time), format_time(duration), delta_str),
            1200,
        );
    }

    pub fn show_play_pause(&mut self, is_paused: bool) {
        if is_paused {
            self.show("Paused".to_string(), 1000);
        } else {
            self.show("Playing".to_string(), 1000);
        }
    }

    pub fn show_speed(&mut self, speed: f64) {
        self.show(format!("Playback Speed: {:.2}x", speed), 1200);
    }

    pub fn show_sub_delay(&mut self, delay_secs: f64) {
        let delay_ms = (delay_secs * 1000.0).round() as i64;
        let sign = if delay_secs > 0.0001 { "+" } else { "" };
        let status_str = if delay_ms.abs() < 5 {
            "Subtitle Sync: 0.00s (Default / Synced)".to_string()
        } else {
            format!("Subtitle Sync: {}{:.3} s ({}{:+0} ms)", sign, delay_secs, sign, delay_ms)
        };
        self.show(status_str, 1500);
    }

    pub fn show_audio_delay(&mut self, delay_ms: i64) {
        self.show(format!("Audio Sync: {:+0} ms", delay_ms), 1200);
    }

    pub fn show_pan_scan(&mut self, zoom: f64, preset: &str) {
        self.show(format!("Zoom: {:.0}% [{}]", zoom * 100.0, preset), 1200);
    }

    pub fn show_jump_percent(&mut self, percent: u32, time: f64) {
        self.show(format!("Jump: {}% [{}]", percent, format_time(time)), 1200);
    }

    pub fn show_chapter(&mut self, index: usize, total: usize, title: &str) {
        self.show(format!("Chapter {}/{}: {}", index, total, title), 1800);
    }

    pub fn show_resize(&mut self, width: u32, height: u32, video_w: u32, video_h: u32) {
        let ar_str = format_aspect_ratio_str(width, height);
        let pct_str = if video_w > 0 && video_h > 0 {
            let pct = (width as f64 / video_w as f64) * 100.0;
            format!(" ({:.1}%)", pct)
        } else {
            String::new()
        };

        let msg = format!("Window Size: {} x {} [{}]{}", width, height, ar_str, pct_str);
        self.show(msg, 1200);
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

        if self.message.is_some() {
            ctx.request_repaint();
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
                    video_rect.min + Vec2::new(20.0, 48.0),
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
        // Rendered entirely by mpv ASS OSD overlay (update_osd_diagnostics_hud in player.rs)
        // No egui drawing needed here — mpv paints the full-text HUD directly on the video.
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
