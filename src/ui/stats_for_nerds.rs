use std::collections::VecDeque;
use std::time::Instant;
use eframe::egui::{self, Color32, CornerRadius, FontId, Pos2, Rect, Stroke, StrokeKind, Vec2, Sense};
use crate::engine::MediaStats;
use super::theme::VortexTheme;

/// A single timestamped sample for the Stats for Nerds historical graphs
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NerdStatPoint {
    pub time: Instant,
    pub net_speed_kbps: f64,
    pub buffer_duration_sec: f64,
    pub buffer_bytes_mb: f64,
    pub fps: f64,
    pub frame_time_ms: f64,
    pub dropped_delta: i64,
    pub dropped_total: i64,
}

pub struct StatsForNerdsDialog {
    pub is_open: bool,
    pub is_pinned: bool,
    pub history: VecDeque<NerdStatPoint>,
    pub last_sample_time: Option<Instant>,
    pub last_dropped_frames: i64,
    pub max_samples: usize,
}

impl Default for StatsForNerdsDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_pinned: false,
            history: VecDeque::with_capacity(120),
            last_sample_time: None,
            last_dropped_frames: 0,
            max_samples: 90,
        }
    }
}

impl StatsForNerdsDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    /// Sample incoming playback metrics into the rolling historical buffer
    pub fn update_sample(&mut self, stats: &MediaStats) {
        let now = Instant::now();
        if let Some(last) = self.last_sample_time {
            // Sample every 120ms for smooth 8.3Hz rolling graphs
            if now.duration_since(last) < std::time::Duration::from_millis(120) {
                return;
            }
        }
        self.last_sample_time = Some(now);

        let dropped_current = stats.dropped_frames;
        let dropped_delta = if dropped_current >= self.last_dropped_frames {
            dropped_current - self.last_dropped_frames
        } else {
            0
        };
        self.last_dropped_frames = dropped_current;

        // Calculate network speed in Kbps
        let net_speed_kbps = if stats.cache_speed > 0 {
            (stats.cache_speed as f64 * 8.0) / 1000.0
        } else if !stats.file_path.starts_with("http") {
            // For local disk files, synthesize transfer read rate based on video+audio bitrate and buffer delta
            let active_bitrate = (stats.video_bitrate + stats.audio_bitrate).max(0) as f64;
            if active_bitrate > 0.0 && !stats.is_paused {
                active_bitrate / 1000.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        let buffer_duration_sec = stats.demuxer_cache_duration.max(0.0);
        let buffer_bytes_mb = (stats.demuxer_cache_bytes as f64) / (1024.0 * 1024.0);

        let current_fps = if stats.estimated_vf_fps > 0.0 {
            stats.estimated_vf_fps
        } else if stats.video_fps > 0.0 {
            stats.video_fps
        } else {
            0.0
        };

        let frame_time_ms = if current_fps > 0.0 {
            (1000.0 / current_fps).min(200.0)
        } else {
            0.0
        };

        self.history.push_back(NerdStatPoint {
            time: now,
            net_speed_kbps,
            buffer_duration_sec,
            buffer_bytes_mb,
            fps: current_fps,
            frame_time_ms,
            dropped_delta,
            dropped_total: dropped_current,
        });

        if self.history.len() > self.max_samples {
            self.history.pop_front();
        }
    }

    /// Render the YouTube-style "Stats for Nerds" window with 3 live sparkline graphs
    pub fn render(
        &mut self,
        ctx: &egui::Context,
        stats: &MediaStats,
        viewport_w: f32,
        viewport_h: f32,
        ppp: f32,
    ) {
        if !self.is_open {
            return;
        }

        self.update_sample(stats);

        let mut is_open = self.is_open;
        let mut close_requested = false;
        let mut copy_debug_info = false;

        let skin = VortexTheme::current_skin();

        let window = egui::Window::new("🤓 Stats for Nerds")
            .open(&mut is_open)
            .resizable(true)
            .collapsible(true)
            .default_size([460.0, 560.0])
            .min_size([400.0, 420.0])
            .default_pos([24.0, 48.0])
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgba_premultiplied(12, 14, 18, 235))
                    .stroke(Stroke::new(1.0, Color32::from_rgba_premultiplied(70, 160, 240, 120)))
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(egui::Margin::symmetric(14, 12))
            );

        window.show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(0.0, 6.0);

            // ── Top Toolbar Header ──────────────────────────────────────────
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Stats for Nerds")
                        .strong()
                        .size(14.0)
                        .color(skin.accent_bright),
                );
                ui.colored_label(skin.text_muted, "• Vortex Engine");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("✕").on_hover_text("Close Stats for Nerds (Ctrl+Shift+N)").clicked() {
                        close_requested = true;
                    }

                    let pin_icon = if self.is_pinned { "📌 Unpin" } else { "📍 Pin" };
                    if ui.button(pin_icon).on_hover_text("Pin overlay on top of video").clicked() {
                        self.is_pinned = !self.is_pinned;
                    }

                    if ui.button("📋 Copy Debug Info").on_hover_text("Copy formatted JSON debug statistics to clipboard").clicked() {
                        copy_debug_info = true;
                    }
                });
            });

            ui.add_space(2.0);
            ui.separator();
            ui.add_space(2.0);

            // ── Classic YouTube "Stats for Nerds" Text Rows ─────────────────
            let filename = if !stats.file_path.is_empty() {
                std::path::Path::new(&stats.file_path)
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_else(|| stats.file_path.clone())
            } else {
                "No Media Playing".to_string()
            };

            let drop_pct = if stats.total_frames_estimated > 0 {
                (stats.dropped_frames as f64 / stats.total_frames_estimated as f64) * 100.0
            } else {
                0.0
            };

            let is_hw = !stats.hwdec_current.is_empty() && stats.hwdec_current != "no";
            let hw_desc = if is_hw {
                format!("{} (Hardware)", stats.hwdec_current)
            } else {
                "Software (CPU)".to_string()
            };

            let color_desc = format!(
                "{} / {} / {}-bit ({})",
                if stats.primaries.is_empty() { "bt709" } else { &stats.primaries },
                if stats.colormatrix.is_empty() { "bt709" } else { &stats.colormatrix },
                if stats.video_bit_depth > 0 { stats.video_bit_depth } else { 8 },
                if stats.is_hdr { &stats.hdr_format } else { "SDR" }
            );

            let host_desc = if stats.file_path.starts_with("http://") || stats.file_path.starts_with("https://") {
                "HTTPS / Network Stream".to_string()
            } else if !stats.file_path.is_empty() {
                "Local NVMe / Storage Drive".to_string()
            } else {
                "Idle".to_string()
            };

            let v_codec = if stats.video_codec.is_empty() { "N/A" } else { &stats.video_codec };
            let a_codec = if stats.audio_codec.is_empty() { "N/A" } else { &stats.audio_codec };

            egui::Grid::new("nerd_stats_grid")
                .num_columns(2)
                .spacing([12.0, 4.0])
                .striped(false)
                .show(ui, |ui| {
                    ui.colored_label(Color32::from_rgb(175, 185, 200), "Video ID / sCPN");
                    let mut file_disp = filename.clone();
                    if file_disp.chars().count() > 36 {
                        let truncated: String = file_disp.chars().take(34).collect();
                        file_disp = format!("{}...", truncated);
                    }
                    ui.label(
                        egui::RichText::new(file_disp)
                            .monospace()
                            .size(11.0)
                            .color(Color32::from_rgb(240, 240, 245)),
                    ).on_hover_text(&stats.file_path);
                    ui.end_row();

                    ui.colored_label(Color32::from_rgb(175, 185, 200), "Viewport / Frames");
                    let drop_color = if stats.dropped_frames > 0 {
                        Color32::from_rgb(255, 95, 95)
                    } else {
                        Color32::from_rgb(100, 240, 140)
                    };
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("{:.0}x{:.0}*{:.1} / ", viewport_w, viewport_h, ppp))
                                .monospace()
                                .size(11.0)
                                .color(Color32::from_rgb(240, 240, 245)),
                        );
                        ui.label(
                            egui::RichText::new(format!("{} dropped of {} ({:.2}%)", stats.dropped_frames, stats.total_frames_estimated, drop_pct))
                                .monospace()
                                .size(11.0)
                                .color(drop_color),
                        );
                    });
                    ui.end_row();

                    ui.colored_label(Color32::from_rgb(175, 185, 200), "Optimal Res");
                    ui.label(
                        egui::RichText::new(format!(
                            "{}x{}@{:.2}fps / Display {:.1}Hz",
                            stats.video_width, stats.video_height, stats.video_fps, stats.display_fps
                        ))
                        .monospace()
                        .size(11.0)
                        .color(Color32::from_rgb(240, 240, 245)),
                    );
                    ui.end_row();

                    ui.colored_label(Color32::from_rgb(175, 185, 200), "Codecs");
                    ui.label(
                        egui::RichText::new(format!("{} ({}) / {} ({}ch @ {}Hz)", v_codec, hw_desc, a_codec, stats.audio_channels, stats.audio_sample_rate))
                            .monospace()
                            .size(11.0)
                            .color(Color32::from_rgb(240, 240, 245)),
                    );
                    ui.end_row();

                    ui.colored_label(Color32::from_rgb(175, 185, 200), "Color / Space");
                    ui.label(
                        egui::RichText::new(color_desc)
                            .monospace()
                            .size(11.0)
                            .color(Color32::from_rgb(240, 240, 245)),
                    );
                    ui.end_row();

                    ui.colored_label(Color32::from_rgb(175, 185, 200), "Host / Protocol");
                    ui.label(
                        egui::RichText::new(host_desc)
                            .monospace()
                            .size(11.0)
                            .color(Color32::from_rgb(140, 210, 255)),
                    );
                    ui.end_row();
                });

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // ── Graph 1: Connection Speed & Network Buffer Graph ────────────
            let latest_speed = self.history.back().map(|p| p.net_speed_kbps).unwrap_or(0.0);
            let peak_speed = self.history.iter().map(|p| p.net_speed_kbps).fold(0.0f64, f64::max);
            let speed_badge = if latest_speed > 1000.0 {
                format!("{:.2} Mbps", latest_speed / 1000.0)
            } else {
                format!("{:.1} Kbps", latest_speed)
            };

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Connection Speed & Network Buffer")
                        .strong()
                        .size(11.5)
                        .color(Color32::from_rgb(140, 210, 255)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(Color32::from_rgb(90, 200, 255), egui::RichText::new(&speed_badge).monospace().strong().size(12.0));
                    ui.colored_label(skin.text_muted, format!("Peak: {:.1} Kbps", peak_speed));
                });
            });

            let speed_values: Vec<f64> = self.history.iter().map(|p| p.net_speed_kbps).collect();
            Self::draw_sparkline_graph(
                ui,
                48.0,
                &speed_values,
                None,
                Color32::from_rgb(60, 180, 255),
                Color32::from_rgba_premultiplied(30, 130, 220, 60),
                "Kbps",
                None,
            );

            ui.add_space(6.0);

            // ── Graph 2: Buffer Graph (Demuxer Cache & Buffer Health) ────────
            let latest_buf_sec = self.history.back().map(|p| p.buffer_duration_sec).unwrap_or(0.0);
            let latest_buf_mb = self.history.back().map(|p| p.buffer_bytes_mb).unwrap_or(0.0);
            let health_color = if latest_buf_sec < 2.0 && !stats.is_paused && !stats.is_idle {
                Color32::from_rgb(255, 100, 80)
            } else if latest_buf_sec < 8.0 {
                Color32::from_rgb(255, 195, 60)
            } else {
                Color32::from_rgb(80, 230, 140)
            };

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Buffer Health (Demuxer Cache)")
                        .strong()
                        .size(11.5)
                        .color(Color32::from_rgb(110, 235, 150)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(health_color, egui::RichText::new(format!("{:.2} s", latest_buf_sec)).monospace().strong().size(12.0));
                    ui.colored_label(skin.text_muted, format!("{:.1} MB ({:.1}%)", latest_buf_mb, stats.cache_buffer_percent));
                });
            });

            let buffer_values: Vec<f64> = self.history.iter().map(|p| p.buffer_duration_sec).collect();
            Self::draw_sparkline_graph(
                ui,
                48.0,
                &buffer_values,
                None,
                Color32::from_rgb(70, 230, 130),
                Color32::from_rgba_premultiplied(40, 180, 90, 55),
                "s",
                Some(20.0), // Target line: 20s buffer health
            );

            ui.add_space(6.0);

            // ── Graph 3: Frame Graph (Framerate & Dropped Frame Spikes) ─────
            let latest_fps = self.history.back().map(|p| p.fps).unwrap_or(stats.video_fps);
            let drops_count = stats.dropped_frames;
            let drops_label_color = if drops_count > 0 {
                Color32::from_rgb(255, 95, 95)
            } else {
                Color32::from_rgb(120, 230, 150)
            };

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Frame Timing & Presentation Jitter")
                        .strong()
                        .size(11.5)
                        .color(Color32::from_rgb(255, 190, 80)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(drops_label_color, egui::RichText::new(format!("{:.2} FPS (Drops: {})", latest_fps, drops_count)).monospace().strong().size(12.0));
                    ui.colored_label(skin.text_muted, format!("Jitter: {:.2}ms | VO:{} Dec:{}", stats.vsync_jitter * 1000.0, stats.vo_drop_frame_count, stats.decoder_frame_drop_count));
                });
            });

            let fps_values: Vec<f64> = self.history.iter().map(|p| p.fps).collect();
            let drop_spikes: Vec<i64> = self.history.iter().map(|p| p.dropped_delta).collect();
            Self::draw_sparkline_graph(
                ui,
                54.0,
                &fps_values,
                Some(&drop_spikes),
                Color32::from_rgb(255, 195, 60),
                Color32::from_rgba_premultiplied(220, 140, 30, 50),
                "FPS",
                Some(stats.video_fps.max(24.0)),
            );

            ui.add_space(6.0);
            ui.separator();
            ui.add_space(2.0);

            // ── Mystery Text Footer (Authentic YouTube Stats for Nerds) ──────
            let mystery_text = format!(
                "s:4 t:{:.2} b:{:.1}-{:.1} p:{} d:{} e:0 | A/V: {:+.2}ms | Vol: {:.0}%",
                stats.time_pos,
                stats.demuxer_cache_duration,
                stats.duration,
                if stats.is_paused { 0 } else { 1 },
                stats.dropped_frames,
                stats.avsync_drift * 1000.0,
                stats.volume
            );
            ui.horizontal(|ui| {
                ui.colored_label(Color32::from_rgb(140, 150, 165), egui::RichText::new(mystery_text).monospace().size(10.5));
            });
        });

        if copy_debug_info {
            let escaped_path = stats.file_path.replace('\\', "\\\\").replace('\"', "\\\"");
            let debug_json = format!(
                "{{\n  \"file\": \"{}\",\n  \"viewport\": \"{:.0}x{:.0}\",\n  \"optimal_res\": \"{}x{}@{:.2}\",\n  \"video_codec\": \"{}\",\n  \"hwdec\": \"{}\",\n  \"audio_codec\": \"{}\",\n  \"dropped_frames\": {},\n  \"vo_drop_frames\": {},\n  \"decoder_drop_frames\": {},\n  \"buffer_health_sec\": {:.2},\n  \"cache_bytes\": {},\n  \"cache_speed_bps\": {},\n  \"avsync_drift_ms\": {:.2},\n  \"vsync_jitter_ms\": {:.2}\n}}",
                escaped_path,
                viewport_w,
                viewport_h,
                stats.video_width,
                stats.video_height,
                stats.video_fps,
                stats.video_codec,
                stats.hwdec_current,
                stats.audio_codec,
                stats.dropped_frames,
                stats.vo_drop_frame_count,
                stats.decoder_frame_drop_count,
                stats.demuxer_cache_duration,
                stats.demuxer_cache_bytes,
                stats.cache_speed,
                stats.avsync_drift * 1000.0,
                stats.vsync_jitter * 1000.0
            );
            ctx.copy_text(debug_json);
        }

        if close_requested {
            is_open = false;
        }
        self.is_open = is_open;
    }

    /// Reusable Sparkline / Area graph renderer with grid lines, scale readouts, and drop spike indicators
    fn draw_sparkline_graph(
        ui: &mut egui::Ui,
        height: f32,
        values: &[f64],
        drop_spikes: Option<&[i64]>,
        line_color: Color32,
        fill_color: Color32,
        unit: &str,
        target_line: Option<f64>,
    ) {
        let (response, painter) = ui.allocate_painter(Vec2::new(ui.available_width(), height), Sense::hover());
        let rect = response.rect;

        // Background box
        painter.rect_filled(rect, CornerRadius::same(4), Color32::from_rgba_premultiplied(8, 10, 14, 210));
        painter.rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.0, Color32::from_white_alpha(24)), StrokeKind::Inside);

        if values.len() < 2 {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Buffering telemetry metrics...",
                FontId::monospace(10.5),
                Color32::from_white_alpha(80),
            );
            return;
        }

        let mut max_val = values.iter().cloned().fold(0.0f64, f64::max);
        if let Some(target) = target_line {
            max_val = max_val.max(target * 1.15);
        }
        if max_val <= 0.001 {
            max_val = 1.0;
        }

        // Draw horizontal grid lines (25%, 50%, 75%)
        let grid_stroke = Stroke::new(0.5, Color32::from_white_alpha(15));
        for frac in &[0.25f32, 0.50f32, 0.75f32] {
            let y = rect.bottom() - rect.height() * frac;
            painter.line_segment([Pos2::new(rect.left() + 2.0, y), Pos2::new(rect.right() - 2.0, y)], grid_stroke);
        }

        // Target baseline line (e.g. 20s buffer target, 60fps target)
        if let Some(target) = target_line {
            let target_frac = ((target / max_val) as f32).clamp(0.0, 1.0);
            let y = rect.bottom() - rect.height() * target_frac;
            painter.line_segment(
                [Pos2::new(rect.left() + 2.0, y), Pos2::new(rect.right() - 2.0, y)],
                Stroke::new(1.0, Color32::from_rgba_premultiplied(200, 200, 255, 60)),
            );
            painter.text(
                Pos2::new(rect.right() - 4.0, y - 2.0),
                egui::Align2::RIGHT_BOTTOM,
                format!("{:.0} {}", target, unit),
                FontId::monospace(8.5),
                Color32::from_rgba_premultiplied(200, 200, 255, 80),
            );
        }

        // Convert values to screen points
        let count = values.len();
        let step_x = rect.width() / (count - 1) as f32;
        let mut points: Vec<Pos2> = Vec::with_capacity(count);

        for (i, &v) in values.iter().enumerate() {
            let x = rect.left() + i as f32 * step_x;
            let norm_y = ((v / max_val) as f32).clamp(0.0, 1.0);
            let y = rect.bottom() - (norm_y * (rect.height() - 4.0) + 2.0);
            points.push(Pos2::new(x, y));
        }

        // Draw filled area underneath curve using convex vertical trapezoid segments
        for pair in points.windows(2) {
            let p0 = pair[0];
            let p1 = pair[1];
            let quad = vec![
                p0,
                p1,
                Pos2::new(p1.x, rect.bottom()),
                Pos2::new(p0.x, rect.bottom()),
            ];
            painter.add(egui::Shape::convex_polygon(quad, fill_color, Stroke::NONE));
        }

        // Draw curve outline
        for pair in points.windows(2) {
            painter.line_segment([pair[0], pair[1]], Stroke::new(1.5, line_color));
        }

        // Draw red drop spikes if frame drop markers exist
        if let Some(spikes) = drop_spikes {
            for (i, &delta) in spikes.iter().enumerate() {
                if delta > 0 && i < points.len() {
                    let pt = points[i];
                    // Red vertical drop line
                    painter.line_segment(
                        [Pos2::new(pt.x, rect.bottom()), Pos2::new(pt.x, rect.top() + 2.0)],
                        Stroke::new(1.5, Color32::from_rgb(255, 50, 50)),
                    );
                    // Glowing indicator head
                    painter.circle_filled(Pos2::new(pt.x, rect.top() + 4.0), 3.0, Color32::from_rgb(255, 70, 70));
                    painter.circle_stroke(
                        Pos2::new(pt.x, rect.top() + 4.0),
                        4.5,
                        Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 50, 50, 120)),
                    );
                }
            }
        }

        // Min and Max scale markers
        painter.text(
            Pos2::new(rect.left() + 4.0, rect.top() + 2.0),
            egui::Align2::LEFT_TOP,
            format!("{:.1} {}", max_val, unit),
            FontId::monospace(9.0),
            Color32::from_white_alpha(100),
        );
        painter.text(
            Pos2::new(rect.left() + 4.0, rect.bottom() - 2.0),
            egui::Align2::LEFT_BOTTOM,
            format!("0.0 {}", unit),
            FontId::monospace(9.0),
            Color32::from_white_alpha(70),
        );

        // Hover interactive crosshair & tooltip
        if let Some(hover_pos) = response.hover_pos() {
            if rect.contains(hover_pos) {
                let frac = ((hover_pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
                let idx = ((frac * (count - 1) as f32).round() as usize).min(count - 1);
                let pt = points[idx];
                let val = values[idx];

                painter.line_segment(
                    [Pos2::new(pt.x, rect.top()), Pos2::new(pt.x, rect.bottom())],
                    Stroke::new(1.0, Color32::from_white_alpha(90)),
                );
                painter.circle_filled(pt, 3.5, Color32::WHITE);

                response.on_hover_text_at_pointer(format!("{:.2} {}", val, unit));
            }
        }
    }
}
