use super::theme::VortexTheme;
use crate::engine::MediaStats;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText,
    Sense, Stroke, StrokeKind, Vec2, ViewportBuilder, ViewportId,
};

use crate::engine::audio_levels::MultiChannelAudioMeterState;
use crate::ui::audio_meter::AudioMeterWidget;

pub struct MediaInfoDialog {
    pub selected_tab: usize, // 0 = Playback Info, 1 = File Info, 2 = System Info
    pub is_pinned: bool,
    pub copy_feedback_time: f64,
    pub meter_state: MultiChannelAudioMeterState,
    pub last_update_time: f64,
    pub toggle_wasapi_exclusive: bool,
    pub word_wrap: bool,
}

impl Default for MediaInfoDialog {
    fn default() -> Self {
        Self {
            selected_tab: 0,
            is_pinned: false,
            copy_feedback_time: 0.0,
            meter_state: MultiChannelAudioMeterState::default(),
            last_update_time: 0.0,
            toggle_wasapi_exclusive: false,
            word_wrap: true,
        }
    }
}

impl MediaInfoDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, is_open: &mut bool, stats: &MediaStats) {
        if !*is_open {
            return;
        }

        let mut close_dialog = false;
        let selected_tab = self.selected_tab;
        let is_pinned = self.is_pinned;
        let copy_feedback_time = self.copy_feedback_time;

        let current_time = ctx.input(|i| i.time);
        self.last_update_time = current_time;

        // Update professional multi-channel audio meter state
        let is_playing = !stats.is_idle && !stats.is_paused;
        self.meter_state.update(
            &stats.audio_channel_levels,
            stats.audio_channels.max(1) as u32,
            is_playing,
            stats.is_muted,
            stats.volume,
        );

        let mut meter_state_clone = self.meter_state.clone();

        // Render as a floating immediate viewport so it sits above child_hwnd and never gets hidden by video
        let viewport_id = ViewportId::from_hash_of("vortex_playback_system_info_window");
        let mut builder = ViewportBuilder::default()
            .with_title("Playback/System Information")
            .with_inner_size([700.0, 720.0])
            .with_min_inner_size([480.0, 520.0])
            .with_resizable(true)
            .with_decorations(true);

        if is_pinned {
            builder = builder.with_always_on_top();
        }

        let mut new_tab = selected_tab;
        let mut toggle_pin = false;
        let mut new_copy_time = copy_feedback_time;
        let mut toggle_wasapi = false;
        let is_word_wrap = self.word_wrap;
        let mut toggle_word_wrap = false;

        ctx.show_viewport_immediate(viewport_id, builder, |ctx, _class| {
            if ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                close_dialog = true;
            }

            if is_playing {
                // Smooth 30 FPS refresh cadence (33ms interval)
                ctx.request_repaint_after(std::time::Duration::from_millis(33));
            } else if crate::engine::mediainfo::is_mediainfo_pending(&stats.file_path) {
                // Fast poll for background MediaInfo scan completion
                ctx.request_repaint_after(std::time::Duration::from_millis(50));
            }

            let current_time = ctx.input(|i| i.time);
            let is_copied = current_time - copy_feedback_time < 2.0;

            // 1. TOP TAB BAR PANEL
            egui::Panel::top("mediainfo_top_bar")
                .exact_size(32.0)
                .frame(
                    egui::Frame::new()
                        .fill(Color32::from_rgb(0, 0, 0))
                        .inner_margin(Margin::ZERO),
                )
                .show(ctx, |ui| {
                    let win_w = ui.available_width();
                    let tab_bar_h = 32.0;
                    let (tab_bar_rect, _) = ui.allocate_exact_size(
                        Vec2::new(win_w, tab_bar_h),
                        Sense::hover(),
                    );

                    let pin_w = 26.0;
                    let tabs_total_w = win_w - pin_w - 20.0;
                    let tabs = ["Playback Info", "File Info", "System Info"];
                    let tab_w = (tabs_total_w / (tabs.len() as f32)).max(80.0);
                    let mut tab_x = tab_bar_rect.left() + 8.0;

                    for (i, tab_title) in tabs.iter().enumerate() {
                        let is_active = selected_tab == i;
                        let t_rect = Rect::from_min_size(
                            Pos2::new(tab_x, tab_bar_rect.top() + 4.0),
                            Vec2::new(tab_w - 2.0, tab_bar_h - 4.0),
                        );
                        let t_resp = ui.interact(t_rect, ui.id().with(format!("vp_tab_{}", i)), Sense::click());
                        let hover = t_resp.hovered();

                        if t_resp.clicked() {
                            new_tab = i;
                        }

                        {
                            let painter = ui.painter();
                            let bg_color = if is_active {
                                Color32::from_rgb(22, 24, 30)
                            } else if hover {
                                Color32::from_rgb(16, 18, 24)
                            } else {
                                Color32::from_rgb(0, 0, 0)
                            };

                            painter.rect_filled(
                                t_rect,
                                CornerRadius { nw: 3, ne: 3, sw: 0, se: 0 },
                                bg_color,
                            );
                            painter.rect_stroke(
                                t_rect,
                                CornerRadius { nw: 3, ne: 3, sw: 0, se: 0 },
                                Stroke::new(1.0, if is_active { Color32::from_rgb(60, 65, 80) } else { Color32::from_rgb(28, 30, 40) }),
                                StrokeKind::Inside,
                            );

                            painter.text(
                                t_rect.center(),
                                Align2::CENTER_CENTER,
                                *tab_title,
                                FontId::proportional(11.5),
                                if is_active { Color32::WHITE } else if hover { Color32::from_rgb(200, 205, 220) } else { Color32::from_rgb(140, 145, 160) },
                            );
                        }

                        tab_x += tab_w;
                    }

                    // Pin Button (📌 / 📍) on the right of tab bar
                    let pin_rect = Rect::from_min_size(
                        Pos2::new(tab_bar_rect.right() - pin_w - 6.0, tab_bar_rect.center().y - 12.0),
                        Vec2::new(pin_w, 24.0),
                    );
                    let pin_resp = ui.interact(pin_rect, ui.id().with("vp_diag_pin"), Sense::click())
                        .on_hover_text(if is_pinned { "Always on top (Pinned)" } else { "Pin window on top" });
                    let pin_hover = pin_resp.hovered();
                    if pin_resp.clicked() {
                        toggle_pin = true;
                    }

                    {
                        let painter = ui.painter();
                        if is_pinned {
                            painter.rect_filled(pin_rect, CornerRadius::same(3), Color32::from_rgb(35, 40, 56));
                            painter.rect_stroke(pin_rect, CornerRadius::same(3), Stroke::new(1.0, Color32::from_rgb(60, 65, 88)), StrokeKind::Inside);
                        } else if pin_hover {
                            painter.rect_filled(pin_rect, CornerRadius::same(3), Color32::from_rgb(25, 28, 38));
                        }
                        painter.text(
                            pin_rect.center(),
                            Align2::CENTER_CENTER,
                            if is_pinned { "📌" } else { "📍" },
                            FontId::proportional(12.0),
                            if is_pinned { VortexTheme::POT_YELLOW } else if pin_hover { Color32::WHITE } else { Color32::from_rgb(160, 165, 180) },
                        );
                    }

                    // Divider below tab bar
                    ui.painter().line_segment(
                        [tab_bar_rect.left_bottom(), tab_bar_rect.right_bottom()],
                        Stroke::new(1.0, Color32::from_rgb(35, 38, 48)),
                    );
                });

            // 2. BOTTOM FOOTER TOOLBAR (Guaranteed Visible at Bottom)
            egui::Panel::bottom("mediainfo_bottom_footer")
                .exact_size(44.0)
                .frame(
                    egui::Frame::new()
                        .fill(Color32::from_rgb(0, 0, 0))
                        .inner_margin(Margin::same(6)),
                )
                .show(ctx, |ui| {
                    let footer_rect = ui.max_rect();
                    let footer_center_y = footer_rect.center().y;

                    // Copy Button (Left aligned)
                    let copy_w = 145.0;
                    let copy_h = 24.0;
                    let copy_rect = Rect::from_min_size(
                        Pos2::new(footer_rect.left() + 8.0, footer_center_y - copy_h / 2.0),
                        Vec2::new(copy_w, copy_h),
                    );
                    let copy_resp = ui.interact(copy_rect, ui.id().with("vp_diag_copy_btn"), Sense::click());
                    let copy_hover = copy_resp.hovered();

                    if copy_resp.clicked() {
                        let text = match selected_tab {
                            0 => Self::generate_playback_report(stats),
                            1 => Self::generate_file_info_text(stats),
                            2 => Self::generate_system_info_text(stats),
                            _ => String::new(),
                        };
                        ui.ctx().copy_text(text);
                        new_copy_time = current_time;
                    }

                    // Word Wrap Button (Next to Copy Button)
                    let wrap_btn_w = 125.0;
                    let wrap_btn_h = 24.0;
                    let wrap_btn_rect = Rect::from_min_size(
                        Pos2::new(copy_rect.right() + 8.0, footer_center_y - wrap_btn_h / 2.0),
                        Vec2::new(wrap_btn_w, wrap_btn_h),
                    );
                    let wrap_resp = ui.interact(wrap_btn_rect, ui.id().with("vp_diag_wrap_btn"), Sense::click())
                        .on_hover_text(if is_word_wrap { "Word Wrap is ON (Wrap long lines). Click to toggle." } else { "Word Wrap is OFF (Horizontal Scroll). Click to toggle." });
                    let wrap_hover = wrap_resp.hovered();
                    if wrap_resp.clicked() {
                        toggle_word_wrap = true;
                    }

                    // Close Button (Right aligned)
                    let close_btn_w = 75.0;
                    let close_btn_h = 24.0;
                    let close_btn_rect = Rect::from_min_size(
                        Pos2::new(footer_rect.right() - 8.0 - close_btn_w, footer_center_y - close_btn_h / 2.0),
                        Vec2::new(close_btn_w, close_btn_h),
                    );
                    let close_btn_resp = ui.interact(close_btn_rect, ui.id().with("vp_diag_close_footer"), Sense::click());
                    let cb_hover = close_btn_resp.hovered();
                    if close_btn_resp.clicked() {
                        close_dialog = true;
                    }

                    {
                        let painter = ui.painter();
                        painter.line_segment(
                            [footer_rect.left_top(), footer_rect.right_top()],
                            Stroke::new(1.0, Color32::from_rgb(35, 38, 48)),
                        );

                        // Render Copy Button
                        painter.rect_filled(
                            copy_rect,
                            CornerRadius::same(3),
                            if is_copied { Color32::from_rgb(20, 60, 32) } else if copy_hover { Color32::from_rgb(26, 30, 40) } else { Color32::from_rgb(14, 16, 22) },
                        );
                        painter.rect_stroke(
                            copy_rect,
                            CornerRadius::same(3),
                            Stroke::new(1.0, if is_copied { Color32::from_rgb(50, 160, 80) } else if copy_hover { Color32::from_rgb(60, 66, 85) } else { Color32::from_rgb(38, 42, 54) }),
                            StrokeKind::Inside,
                        );
                        painter.text(
                            copy_rect.center(),
                            Align2::CENTER_CENTER,
                            if is_copied { "✓ Copied!" } else { "Copy to the clipboard" },
                            FontId::proportional(11.0),
                            if is_copied { Color32::from_rgb(140, 245, 165) } else if copy_hover { Color32::WHITE } else { Color32::from_rgb(210, 215, 225) },
                        );

                        // Render Word Wrap Button
                        painter.rect_filled(
                            wrap_btn_rect,
                            CornerRadius::same(3),
                            if is_word_wrap { Color32::from_rgb(20, 36, 56) } else if wrap_hover { Color32::from_rgb(26, 30, 40) } else { Color32::from_rgb(14, 16, 22) },
                        );
                        painter.rect_stroke(
                            wrap_btn_rect,
                            CornerRadius::same(3),
                            Stroke::new(1.0, if is_word_wrap { Color32::from_rgb(50, 110, 190) } else if wrap_hover { Color32::from_rgb(60, 66, 85) } else { Color32::from_rgb(38, 42, 54) }),
                            StrokeKind::Inside,
                        );
                        painter.text(
                            wrap_btn_rect.center(),
                            Align2::CENTER_CENTER,
                            if is_word_wrap { "↩ Word Wrap: ON" } else { "↩ Word Wrap: OFF" },
                            FontId::proportional(11.0),
                            if is_word_wrap { Color32::from_rgb(170, 215, 255) } else if wrap_hover { Color32::WHITE } else { Color32::from_rgb(170, 175, 190) },
                        );

                        // Render Close Button
                        painter.rect_filled(
                            close_btn_rect,
                            CornerRadius::same(3),
                            if cb_hover { Color32::from_rgb(35, 38, 50) } else { Color32::from_rgb(18, 20, 26) },
                        );
                        painter.rect_stroke(
                            close_btn_rect,
                            CornerRadius::same(3),
                            Stroke::new(1.0, if cb_hover { Color32::from_rgb(65, 70, 92) } else { Color32::from_rgb(38, 42, 54) }),
                            StrokeKind::Inside,
                        );
                        painter.text(
                            close_btn_rect.center(),
                            Align2::CENTER_CENTER,
                            "Close",
                            FontId::proportional(11.0),
                            if cb_hover { Color32::WHITE } else { Color32::from_rgb(210, 215, 225) },
                        );
                    }
                });

            // 3. CENTRAL CONTENT PANEL (Fills remaining height perfectly)
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::new()
                        .fill(Color32::from_rgb(0, 0, 0))
                        .inner_margin(Margin::same(6)),
                )
                .show(ctx, |ui| {
                    let total_content_h = ui.available_height();
                    match selected_tab {
                        0 => Self::render_playback_tab_content(ui, stats, total_content_h, &mut meter_state_clone, &mut toggle_wasapi),
                        1 => Self::render_file_info_tab_content(ui, stats, is_word_wrap),
                        2 => Self::render_system_info_tab_content(ui, stats, is_word_wrap),
                        _ => {}
                    }
                });
        });

        self.selected_tab = new_tab;
        if toggle_pin {
            self.is_pinned = !self.is_pinned;
        }
        self.copy_feedback_time = new_copy_time;
        self.toggle_wasapi_exclusive = toggle_wasapi;
        if toggle_word_wrap {
            self.word_wrap = !self.word_wrap;
        }

        if close_dialog {
            *is_open = false;
        }
    }

    // =========================================================================
    // =========================================================================
    // TAB 0: PLAYBACK INFO (Exact PotPlayer Stats + Dynamic Detail Box + Professional Audio Meter)
    // =========================================================================
    fn render_playback_tab_content(
        ui: &mut egui::Ui,
        stats: &MediaStats,
        _available_h: f32,
        meter_state: &mut MultiChannelAudioMeterState,
        toggle_wasapi: &mut bool,
    ) {
        let vcodec = Self::format_short_vcodec(stats);
        let has_video = stats.video_width > 0 || !stats.video_codec.is_empty();
        let has_audio = stats.audio_channels > 0 || stats.audio_sample_rate > 0 || !stats.audio_codec.is_empty();
        let is_hw = !stats.hwdec_current.is_empty() && stats.hwdec_current != "no";

        let video_decoder_label = if !has_video {
            "No Video Stream".to_string()
        } else if is_hw {
            format!("Hardware Accelerated ({})", stats.hwdec_current.to_uppercase())
        } else if !stats.decoder_name.is_empty() {
            format!("FFmpeg Multi-Threaded ({})", stats.decoder_name)
        } else {
            format!("FFmpeg Multi-Threaded ({})", vcodec)
        };

        let video_codec_label = if !has_video {
            "--".to_string()
        } else {
            let raw = if !stats.video_format.is_empty() { &stats.video_format } else { &stats.video_codec }.to_lowercase();
            let desc = if raw.contains("h264") || raw.contains("avc") {
                "H.264 / AVC (Advanced Video Coding)"
            } else if raw.contains("hevc") || raw.contains("h265") {
                "H.265 / HEVC (High Efficiency Video Coding)"
            } else if raw.contains("av1") {
                "AV1 (AOMedia Video 1)"
            } else if raw.contains("vp9") {
                "VP9 (Google VP9 Codec)"
            } else if raw.contains("vp8") {
                "VP8 (On2 VP8 Codec)"
            } else if raw.contains("mpeg2") {
                "MPEG-2 Video"
            } else if raw.contains("vc1") {
                "VC-1 (SMPTE 421M)"
            } else if !stats.video_codec.is_empty() {
                stats.video_codec.as_str()
            } else {
                "--"
            };
            if !stats.video_format.is_empty() {
                format!("{} [{}]", desc, stats.video_format.to_uppercase())
            } else {
                desc.to_string()
            }
        };

        let input_type_str = if has_video {
            let bd = if stats.video_bit_depth > 0 { stats.video_bit_depth } else { 8 };
            if !stats.pixel_format.is_empty() {
                format!("{}({}-bit, {})", vcodec, bd, stats.pixel_format)
            } else {
                format!("{}({}-bit)", vcodec, bd)
            }
        } else {
            "--".to_string()
        };

        let output_type_str = if has_video {
            let out_pix = if !stats.video_out_pixel_format.is_empty() {
                &stats.video_out_pixel_format
            } else if is_hw {
                &stats.hwdec_current
            } else if !stats.pixel_format.is_empty() {
                &stats.pixel_format
            } else {
                "nv12"
            };
            let vo = if !stats.current_vo.is_empty() { &stats.current_vo } else { "gpu" };
            format!("{} ({})", out_pix, vo)
        } else {
            "--".to_string()
        };

        let fps_str = if stats.video_fps > 0.0 {
            format!("{:.3}", stats.video_fps)
        } else {
            "--".to_string()
        };

        let actual_fps_str = if stats.estimated_vf_fps > 0.0 {
            format!("{:.3}", stats.estimated_vf_fps)
        } else if stats.video_fps > 0.0 {
            format!("{:.3}", stats.video_fps)
        } else {
            "--".to_string()
        };

        let vbitrate_str = if stats.video_bitrate > 0 {
            format!("{} kbps", stats.video_bitrate / 1000)
        } else {
            "-- kbps".to_string()
        };

        let in_aspect = if stats.video_height > 0 {
            stats.video_width as f64 / stats.video_height as f64
        } else {
            1.78
        };
        let input_size_str = if stats.video_width > 0 && stats.video_height > 0 {
            format!("{} × {}({:.2}:1)", stats.video_width, stats.video_height, in_aspect)
        } else {
            "--".to_string()
        };

        let out_w = if stats.video_out_width > 0 { stats.video_out_width } else { stats.video_width };
        let out_h = if stats.video_out_height > 0 { stats.video_out_height } else { stats.video_height };
        let out_aspect = if out_h > 0 { out_w as f64 / out_h as f64 } else { in_aspect };
        let output_size_str = if out_w > 0 && out_h > 0 {
            format!("{} × {}({:.2}:1)", out_w, out_h, out_aspect)
        } else {
            "--".to_string()
        };

        let audio_decoder_label = if !has_audio {
            "No Audio Stream".to_string()
        } else if !stats.audio_decoder_name.is_empty() {
            format!("FFmpeg Audio Decoder ({})", stats.audio_decoder_name)
        } else if !stats.audio_codec.is_empty() {
            format!("FFmpeg Audio Decoder ({})", stats.audio_codec)
        } else {
            "FFmpeg Audio Decoder".to_string()
        };

        let audio_codec_label = if !has_audio {
            "--".to_string()
        } else {
            let raw = stats.audio_codec.to_lowercase();
            let desc = if raw.contains("flac") {
                "FLAC (Free Lossless Audio Codec)"
            } else if raw.contains("aac") {
                "AAC (Advanced Audio Coding)"
            } else if raw.contains("truehd") {
                "Dolby TrueHD Lossless"
            } else if raw.contains("eac3") || raw.contains("e-ac3") {
                "Dolby Digital Plus (E-AC-3)"
            } else if raw.contains("ac3") {
                "Dolby Digital (AC-3)"
            } else if raw.contains("dts-hd") || raw.contains("dtshd") {
                "DTS-HD Master Audio"
            } else if raw.contains("dts") || raw.contains("dca") {
                "DTS (Digital Theater Systems)"
            } else if raw.contains("opus") {
                "Opus (Interactive Audio)"
            } else if raw.contains("vorbis") {
                "Vorbis (Ogg Audio)"
            } else if raw.contains("mp3") {
                "MP3 (MPEG Audio Layer 3)"
            } else if raw.contains("pcm") {
                "Linear PCM (Uncompressed Audio)"
            } else if !stats.audio_codec.is_empty() {
                stats.audio_codec.as_str()
            } else {
                "--"
            };
            let short_ac = Self::format_short_acodec(stats);
            if !short_ac.is_empty() && short_ac != "--" {
                format!("{} [{}]", desc, short_ac)
            } else {
                desc.to_string()
            }
        };

        let sr_str = if stats.audio_sample_rate > 0 {
            let out_sr = if stats.audio_out_sample_rate > 0 { stats.audio_out_sample_rate } else { stats.audio_sample_rate };
            format!("{} -> {} samples/sec", stats.audio_sample_rate, out_sr)
        } else {
            "--".to_string()
        };

        let ch_str = if stats.audio_channels > 0 {
            let out_ch = if stats.audio_out_channels > 0 { stats.audio_out_channels } else { stats.audio_channels };
            let layout = if !stats.audio_channel_layout.is_empty() { format!(" ({})", stats.audio_channel_layout) } else { "".to_string() };
            format!("{} -> {} channels{}", stats.audio_channels, out_ch, layout)
        } else {
            "--".to_string()
        };

        let abitrate_str = if stats.audio_bitrate > 0 {
            format!("{} kbps", stats.audio_bitrate / 1000)
        } else {
            "-- kbps".to_string()
        };

        let bd_str = if stats.audio_bit_depth > 0 {
            let out_bd = if stats.audio_out_bit_depth > 0 { stats.audio_out_bit_depth } else { stats.audio_bit_depth };
            format!("{} -> {} bits/sample", stats.audio_bit_depth, out_bd)
        } else {
            "--".to_string()
        };

        let win_w = ui.available_width();

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.set_width(win_w - 16.0);

                        // ── 1. Video Info Group Box ──────────────────────────────────────────────
                        egui::Frame::new()
                            .fill(Color32::from_rgb(0, 0, 0))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(35, 38, 48)))
                            .corner_radius(CornerRadius::same(3))
                            .inner_margin(Margin::same(8))
                            .show(ui, |ui| {
                                ui.label(RichText::new("Video Info").size(11.0).strong().color(Color32::from_rgb(225, 230, 240)));
                                ui.add_space(4.0);

                                // Decoder Row
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Decoder:").size(10.5).color(Color32::from_rgb(160, 165, 180)));
                                    ui.label(RichText::new(&video_decoder_label).size(10.5).strong().color(Color32::from_rgb(220, 225, 240)));
                                });
                                ui.add_space(2.0);

                                // Codec Row
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Codec:").size(10.5).color(Color32::from_rgb(160, 165, 180)));
                                    ui.label(RichText::new(&video_codec_label).size(10.5).color(Color32::from_rgb(215, 220, 230)));
                                });
                                ui.add_space(4.0);

                                let avail_w = ui.available_width();
                                let col_w = (avail_w - 8.0) / 2.0;

                                // 2-Column Grid
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.set_width(col_w);
                                        ui.label(RichText::new(format!("Input type: {}", input_type_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                        ui.add_space(1.0);
                                        ui.label(RichText::new(format!("Output type: {}", output_type_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                        ui.add_space(1.0);
                                        ui.label(RichText::new(format!("FPS: {}", fps_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                        ui.add_space(1.0);
                                        ui.label(RichText::new(format!("Bit rate: {}", vbitrate_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                    });

                                    ui.vertical(|ui| {
                                        ui.set_width(col_w);
                                        ui.label(RichText::new(format!("Input size: {}", input_size_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                        ui.add_space(1.0);
                                        ui.label(RichText::new(format!("Output size: {}", output_size_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                        ui.add_space(1.0);
                                        ui.label(RichText::new(format!("Actual FPS: {}", actual_fps_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                    });
                                });
                            });

                        ui.add_space(6.0);

                        // ── 2. Audio Info Group Box ──────────────────────────────────────────────
                        let is_ex = stats.is_wasapi_exclusive;
                        egui::Frame::new()
                            .fill(if is_ex { Color32::from_rgb(20, 16, 8) } else { Color32::from_rgb(0, 0, 0) })
                            .stroke(Stroke::new(1.0, if is_ex { Color32::from_rgb(145, 105, 28) } else { Color32::from_rgb(35, 38, 48) }))
                            .corner_radius(CornerRadius::same(3))
                            .inner_margin(Margin::same(8))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Audio Info").size(11.0).strong().color(Color32::from_rgb(225, 230, 240)));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        let btn_txt = if is_ex { "⚡ WASAPI Exclusive: ON (Bit-Perfect)" } else { "WASAPI Shared (Click to Toggle)" };
                                        let btn_col = if is_ex { VortexTheme::POT_YELLOW } else { Color32::from_rgb(150, 155, 175) };
                                        if ui.button(RichText::new(btn_txt).size(10.0).strong().color(btn_col))
                                            .on_hover_text("Click to toggle Bit-Perfect Direct WASAPI Exclusive Output")
                                            .clicked()
                                        {
                                            *toggle_wasapi = true;
                                        }
                                    });
                                });
                                ui.add_space(4.0);

                                // Decoder Row
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Decoder:").size(10.5).color(Color32::from_rgb(160, 165, 180)));
                                    ui.label(RichText::new(&audio_decoder_label).size(10.5).strong().color(Color32::from_rgb(220, 225, 240)));
                                });
                                ui.add_space(2.0);

                                // Codec Row
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Codec:").size(10.5).color(Color32::from_rgb(160, 165, 180)));
                                    ui.label(RichText::new(&audio_codec_label).size(10.5).color(Color32::from_rgb(215, 220, 230)));
                                });
                                ui.add_space(4.0);

                                let avail_w = ui.available_width();
                                let col_w = (avail_w - 8.0) / 2.0;

                                // 2-Column Grid
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.set_width(col_w);
                                        let sr_display = if is_ex {
                                            format!("Sample rate: {} (Bit-Perfect)", sr_str)
                                        } else {
                                            format!("Sample rate: {}", sr_str)
                                        };
                                        ui.label(RichText::new(sr_display).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                        ui.add_space(1.0);
                                        ui.label(RichText::new(format!("Bit rate: {}", abitrate_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                    });

                                    ui.vertical(|ui| {
                                        ui.set_width(col_w);
                                        ui.label(RichText::new(format!("Channels: {}", ch_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                        ui.add_space(1.0);
                                        ui.label(RichText::new(format!("Bit depth: {}", bd_str)).size(10.5).color(Color32::from_rgb(205, 210, 225)));
                                    });
                                });
                            });

                        ui.add_space(6.0);

                        // ── 3. Detail Info Group Box ─────────────────────────────────────────────
                        let detail_text = Self::generate_detailed_raw_info(stats);
                        egui::Frame::new()
                            .fill(Color32::from_rgb(0, 0, 0))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(35, 38, 48)))
                            .corner_radius(CornerRadius::same(3))
                            .inner_margin(Margin::same(8))
                            .show(ui, |ui| {
                                ui.label(RichText::new("Detail Info").size(11.0).strong().color(Color32::from_rgb(225, 230, 240)));
                                ui.add_space(3.0);

                                egui::Frame::new()
                                    .fill(Color32::from_rgb(0, 0, 0))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(28, 30, 40)))
                                    .corner_radius(CornerRadius::same(2))
                                    .inner_margin(Margin::same(6))
                                    .show(ui, |ui| {
                                        let box_h = 240.0;
                                        ui.set_height(box_h);
                                        egui::ScrollArea::both()
                                            .auto_shrink([false, false])
                                            .show(ui, |ui| {
                                                ui.label(
                                                    RichText::new(&detail_text)
                                                        .monospace()
                                                        .size(10.0)
                                                        .color(Color32::from_rgb(215, 225, 240)),
                                                );
                                            });
                                    });
                            });

                        ui.add_space(6.0);

                        // ── 4. Professional Multi-Channel Audio Level Meter (Dual-Stage) ─────────
                        AudioMeterWidget::render_dual_stage_panel(
                            ui,
                            meter_state,
                            stats,
                            |_ch_idx, _is_solo| {},
                            |_ch_idx, _is_muted| {},
                        );

                        ui.add_space(8.0);
                    });
                });
            });
    }

    // =========================================================================
    // TAB 1: FILE INFO (Full Height, Complete Container & Stream Diagnostics, No Blank Space)
    // =========================================================================
    fn render_file_info_tab_content(ui: &mut egui::Ui, stats: &MediaStats, word_wrap: bool) {
        let file_info_text = Self::generate_file_info_text(stats);

        egui::Frame::new()
            .fill(Color32::from_rgb(0, 0, 0))
            .stroke(Stroke::new(1.0, Color32::from_rgb(28, 30, 40)))
            .corner_radius(CornerRadius::same(3))
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                let avail_size = ui.available_size();
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .max_width(avail_size.x)
                    .max_height(avail_size.y)
                    .show(ui, |ui| {
                        let label = egui::Label::new(
                            RichText::new(&file_info_text)
                                .monospace()
                                .size(10.5)
                                .color(Color32::from_rgb(225, 230, 242)),
                        )
                        .selectable(true);

                        if word_wrap {
                            ui.add(label.wrap_mode(egui::TextWrapMode::Wrap));
                        } else {
                            ui.add(label.wrap_mode(egui::TextWrapMode::Extend));
                        }
                    });
            });
    }

    // =========================================================================
    // TAB 2: SYSTEM INFO (Full Height, CPU, OS, Direct3D 9, DXGI, D3D11, VGA, Memory, No Blank Space)
    // =========================================================================
    fn render_system_info_tab_content(ui: &mut egui::Ui, stats: &MediaStats, word_wrap: bool) {
        let system_info_text = Self::generate_system_info_text(stats);

        egui::Frame::new()
            .fill(Color32::from_rgb(0, 0, 0))
            .stroke(Stroke::new(1.0, Color32::from_rgb(28, 30, 40)))
            .corner_radius(CornerRadius::same(3))
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                let avail_size = ui.available_size();
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .max_width(avail_size.x)
                    .max_height(avail_size.y)
                    .show(ui, |ui| {
                        let label = egui::Label::new(
                            RichText::new(&system_info_text)
                                .monospace()
                                .size(10.5)
                                .color(Color32::from_rgb(215, 225, 240)),
                        )
                        .selectable(true);

                        if word_wrap {
                            ui.add(label.wrap_mode(egui::TextWrapMode::Wrap));
                        } else {
                            ui.add(label.wrap_mode(egui::TextWrapMode::Extend));
                        }
                    });
            });
    }

    fn format_short_vcodec(stats: &MediaStats) -> String {
        let raw = if !stats.video_format.is_empty() {
            &stats.video_format
        } else {
            &stats.video_codec
        };
        let lower = raw.to_lowercase();
        if lower.contains("h264") || lower.contains("avc1") || lower.contains("avc") {
            "AVC1".to_string()
        } else if lower.contains("hevc") || lower.contains("h265") || lower.contains("hvc1") {
            "HEVC".to_string()
        } else if lower.contains("av1") || lower.contains("av01") {
            "AV1".to_string()
        } else if lower.contains("vp9") {
            "VP9".to_string()
        } else if lower.contains("vp8") {
            "VP8".to_string()
        } else if lower.contains("mpeg2") {
            "MPEG2".to_string()
        } else if lower.contains("vc1") {
            "VC1".to_string()
        } else if !stats.video_format.is_empty() {
            stats.video_format.to_uppercase()
        } else if !stats.video_codec.is_empty() {
            stats.video_codec.split(|ch: char| ch.is_whitespace() || ch == '/' || ch == '(').next().unwrap_or("RAW").to_uppercase()
        } else {
            "--".to_string()
        }
    }

    fn format_short_acodec(stats: &MediaStats) -> String {
        let raw = if !stats.audio_codec.is_empty() {
            &stats.audio_codec
        } else if !stats.audio_format_str.is_empty() {
            &stats.audio_format_str
        } else {
            ""
        };
        let lower = raw.to_lowercase();
        if lower.contains("flac") {
            "FLAC".to_string()
        } else if lower.contains("truehd") {
            "TrueHD".to_string()
        } else if lower.contains("aac") {
            "AAC".to_string()
        } else if lower.contains("dts") {
            "DTS".to_string()
        } else if lower.contains("eac3") || lower.contains("e-ac3") {
            "EAC3".to_string()
        } else if lower.contains("ac3") {
            "AC3".to_string()
        } else if lower.contains("opus") {
            "Opus".to_string()
        } else if lower.contains("vorbis") {
            "Vorbis".to_string()
        } else if lower.contains("mp3") {
            "MP3".to_string()
        } else if !raw.is_empty() {
            raw.to_uppercase()
        } else {
            "--".to_string()
        }
    }

    pub fn generate_detailed_raw_info(stats: &MediaStats) -> String {
        if stats.file_path.is_empty() {
            return "No active media file loaded.".to_string();
        }

        let vcodec = Self::format_short_vcodec(stats);
        let acodec = Self::format_short_acodec(stats);
        let sr = stats.audio_sample_rate;
        let ch = stats.audio_channels;
        let bd = stats.audio_bit_depth;
        let fps = stats.video_fps;
        let w = stats.video_width;
        let h = stats.video_height;
        let in_aspect = if h > 0 { (w as f64) / (h as f64) } else { 1.78 };

        let ext = std::path::Path::new(&stats.file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let container_source = if !stats.file_format.is_empty() {
            format!("Built-in Demuxer Engine ({})", stats.file_format)
        } else if !ext.is_empty() {
            format!("Built-in {} Container Source", ext.to_uppercase())
        } else {
            "Built-in FFmpeg Media Source".to_string()
        };

        let mut lines = Vec::new();
        lines.push("[Used Filter Pipeline]".to_string());
        lines.push(format!("  (1) {}", container_source));

        let mut filter_idx = 2;
        if w > 0 || !stats.video_codec.is_empty() {
            let is_hw = !stats.hwdec_current.is_empty() && stats.hwdec_current != "no";
            let video_decoder = if is_hw {
                format!("Native {} Hardware Video Decoder", stats.hwdec_current.to_uppercase())
            } else if !stats.decoder_name.is_empty() {
                format!("FFmpeg Software Video Decoder ({})", stats.decoder_name)
            } else {
                format!("FFmpeg Software Video Decoder ({})", vcodec)
            };
            lines.push(format!("  ({}) {}", filter_idx, video_decoder));
            filter_idx += 1;

            let vo_name = if !stats.current_vo.is_empty() { &stats.current_vo } else { "gpu" };
            let session = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
            let surface_type = if cfg!(windows) {
                "Direct3D11 / WGPU Hardware Surface"
            } else if session.contains("wayland") {
                "EGL / Wayland Hardware Surface"
            } else {
                "OpenGL / GLX Hardware Surface"
            };
            lines.push(format!("  ({}) Video Output: {} ({})", filter_idx, vo_name, surface_type));
            filter_idx += 1;
        }

        if ch > 0 || !stats.audio_codec.is_empty() {
            let audio_dec = if !stats.audio_decoder_name.is_empty() {
                format!("Built-in Audio Decoder ({})", stats.audio_decoder_name)
            } else {
                format!("Built-in {} Audio Decoder Engine", acodec)
            };
            lines.push(format!("  ({}) {}", filter_idx, audio_dec));
            filter_idx += 1;

            let ao = if !stats.current_ao.is_empty() {
                stats.current_ao.clone()
            } else if cfg!(windows) {
                "wasapi".to_string()
            } else {
                "pipewire".to_string()
            };
            let ao_device = if !stats.active_audio_device.is_empty() && stats.active_audio_device != "auto" {
                stats.active_audio_device.clone()
            } else {
                "auto".to_string()
            };
            lines.push(format!("  ({}) Audio Output: {} ({})", filter_idx, ao, ao_device));
        }

        if w > 0 {
            lines.push(String::new());
            lines.push("[Video Information]".to_string());
            let codec_full = if !stats.video_format.is_empty() && stats.video_format != stats.video_codec {
                format!("{} ({})", stats.video_codec.to_uppercase(), stats.video_format)
            } else {
                stats.video_codec.to_uppercase()
            };
            lines.push(format!("  Codec: {}", codec_full));
            lines.push(format!("  Codec ID: {}", if !stats.video_format.is_empty() { &stats.video_format } else { &stats.video_codec }));
            lines.push(format!("  Input Size: {} × {} ({:.2}:1)", w, h, in_aspect));

            let out_w = if stats.video_out_width > 0 { stats.video_out_width } else { w };
            let out_h = if stats.video_out_height > 0 { stats.video_out_height } else { h };
            let out_aspect = if out_h > 0 { out_w as f64 / out_h as f64 } else { in_aspect };
            if out_w > 0 && out_h > 0 {
                lines.push(format!("  Output Size: {} × {} ({:.2}:1)", out_w, out_h, out_aspect));
            }
            if stats.video_bit_depth > 0 {
                lines.push(format!("  Bit Depth: {}-bit", stats.video_bit_depth));
            }
            if fps > 0.0 {
                lines.push(format!("  Frame Rate: {:.3} FPS", fps));
            }
            if stats.estimated_vf_fps > 0.0 {
                lines.push(format!("  Actual FPS: {:.3}", stats.estimated_vf_fps));
            }
            if stats.video_bitrate > 0 {
                lines.push(format!("  Bitrate: {} kbps", stats.video_bitrate / 1000));
            }
            if !stats.pixel_format.is_empty() {
                lines.push(format!("  Pixel Format: {}", stats.pixel_format));
            }
            if !stats.primaries.is_empty() {
                lines.push(format!("  Color Primaries: {}", stats.primaries));
            }
            if !stats.gamma.is_empty() {
                lines.push(format!("  Transfer: {}", stats.gamma));
            }
            if !stats.colormatrix.is_empty() {
                lines.push(format!("  Color Matrix: {}", stats.colormatrix));
            }
            if stats.is_hdr {
                lines.push(format!("  HDR Format: {}", stats.hdr_format));
            }
            let is_hw = !stats.hwdec_current.is_empty() && stats.hwdec_current != "no";
            lines.push(format!("  Hardware Decode: {}", if is_hw { &stats.hwdec_current } else { "off (software)" }));
            lines.push(format!("  Dropped Frames: {}", stats.dropped_frames));
        }

        if ch > 0 || sr > 0 || !stats.audio_codec.is_empty() {
            lines.push(String::new());
            lines.push("[Audio Information]".to_string());
            let acodec_full = if !stats.audio_format_str.is_empty() && stats.audio_format_str != stats.audio_codec {
                format!("{} ({})", stats.audio_codec.to_uppercase(), stats.audio_format_str)
            } else {
                stats.audio_codec.to_uppercase()
            };
            lines.push(format!("  Codec: {}", acodec_full));
            if sr > 0 {
                lines.push(format!("  Sampling Rate: {} Hz", sr));
            }
            if bd > 0 {
                lines.push(format!("  Bit Depth: {} bits/sample", bd));
            }
            if ch > 0 {
                let layout = if !stats.audio_channel_layout.is_empty() {
                    &stats.audio_channel_layout
                } else {
                    match ch {
                        1 => "Mono",
                        2 => "Stereo",
                        6 => "5.1",
                        8 => "7.1",
                        _ => "Multi-Channel",
                    }
                };
                lines.push(format!("  Channels: {} ({})", ch, layout));
            }
            if stats.audio_bitrate > 0 {
                lines.push(format!("  Bitrate: {} kbps", stats.audio_bitrate / 1000));
            }
            if stats.audio_out_sample_rate > 0 {
                lines.push(format!("  Output Sample Rate: {} Hz", stats.audio_out_sample_rate));
            }
            if stats.audio_out_channels > 0 {
                lines.push(format!("  Output Channels: {}", stats.audio_out_channels));
            }
            if stats.audio_out_bit_depth > 0 {
                lines.push(format!("  Output Bit Depth: {} bits/sample", stats.audio_out_bit_depth));
            }
        }

        lines.push(String::new());
        lines.push("[Playback Engine]".to_string());
        lines.push(format!("  Engine: libmpv (mpv media player core)"));
        lines.push(format!("  Video Output: {}", if !stats.current_vo.is_empty() { &stats.current_vo } else { "gpu" }));
        lines.push(format!("  Audio Output: {}", if !stats.current_ao.is_empty() { &stats.current_ao } else { "auto" }));
        lines.push(format!("  Display FPS: {:.1} Hz", stats.display_fps));
        lines.push(format!("  VSync Jitter: {:.2} ms", stats.vsync_jitter * 1000.0));
        lines.push(format!("  A/V Delay: {:.3} s", stats.audio_delay));

        lines.join("\n")
    }

    pub fn generate_file_info_text(stats: &MediaStats) -> String {
        crate::engine::mediainfo::get_mediainfo_text(&stats.file_path, stats)
    }

    pub fn generate_system_info_text(stats: &MediaStats) -> String {
        let threads = std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(8);
        let arch = std::env::consts::ARCH;

        // 1. CPU Model & Flags Discovery
        let mut cpu_model = String::new();
        let mut physical_cores = threads / 2;
        let mut cpu_mhz = String::new();
        let mut simd_flags = "AVX2 / FMA3 / SSE4.2 / SSE4.1 / SSSE3 / SSE3 / SSE2 / MMX / AES-NI".to_string();

        #[cfg(target_os = "linux")]
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("model name") && cpu_model.is_empty() {
                    if let Some(val) = line.split(':').nth(1) {
                        cpu_model = val.trim().to_string();
                    }
                } else if line.starts_with("cpu cores") {
                    if let Some(val) = line.split(':').nth(1) {
                        if let Ok(c) = val.trim().parse::<usize>() {
                            physical_cores = c;
                        }
                    }
                } else if line.starts_with("cpu MHz") && cpu_mhz.is_empty() {
                    if let Some(val) = line.split(':').nth(1) {
                        cpu_mhz = format!("{} MHz", val.trim());
                    }
                } else if line.starts_with("flags") {
                    let mut flags = Vec::new();
                    let f_line = line.to_lowercase();
                    if f_line.contains("avx512") { flags.push("AVX-512"); }
                    if f_line.contains("avx2") { flags.push("AVX2"); }
                    if f_line.contains("avx") && !flags.contains(&"AVX2") { flags.push("AVX"); }
                    if f_line.contains("fma") { flags.push("FMA3"); }
                    if f_line.contains("sse4_2") { flags.push("SSE4.2"); }
                    if f_line.contains("sse4_1") { flags.push("SSE4.1"); }
                    if f_line.contains("ssse3") { flags.push("SSSE3"); }
                    if f_line.contains("sse3") { flags.push("SSE3"); }
                    if f_line.contains("sse2") { flags.push("SSE2"); }
                    if f_line.contains("mmx") { flags.push("MMX"); }
                    if f_line.contains("aes") { flags.push("AES-NI"); }
                    if f_line.contains("bmi2") { flags.push("BMI2"); }
                    if f_line.contains("sha") { flags.push("SHA-NI"); }
                    if !flags.is_empty() {
                        simd_flags = flags.join(" / ");
                    }
                }
            }
        }

        if cpu_model.is_empty() {
            cpu_model = if cfg!(target_arch = "x86_64") {
                format!("x86_64 High Performance Multi-Core Processor ({} Cores)", physical_cores)
            } else {
                format!("ARM64 Silicon Multi-Core Processor ({} Cores)", physical_cores)
            };
        }

        // 2. Memory Information
        let mut mem_total_str = "16.00 GiB (16384 MB)".to_string();
        let mut mem_avail_str = "12.50 GiB (12800 MB)".to_string();

        #[cfg(target_os = "linux")]
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            let mut total_kb = 0u64;
            let mut avail_kb = 0u64;
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        total_kb = val.parse::<u64>().unwrap_or(0);
                    }
                } else if line.starts_with("MemAvailable:") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        avail_kb = val.parse::<u64>().unwrap_or(0);
                    }
                }
            }
            if total_kb > 0 {
                let total_gib = total_kb as f64 / (1024.0 * 1024.0);
                let total_mb = total_kb / 1024;
                mem_total_str = format!("{:.2} GiB ({} MB)", total_gib, total_mb);
            }
            if avail_kb > 0 {
                let avail_gib = avail_kb as f64 / (1024.0 * 1024.0);
                let avail_mb = avail_kb / 1024;
                mem_avail_str = format!("{:.2} GiB ({} MB)", avail_gib, avail_mb);
            }
        }

        // 3. Operating System & Desktop Environment
        let mut os_name = if cfg!(windows) {
            "Windows 11 / 10 (64-bit Direct3D11 / DXGI DWM)".to_string()
        } else if cfg!(target_os = "macos") {
            "macOS Darwin (Metal / Cocoa Window Server)".to_string()
        } else {
            "Linux / Unix".to_string()
        };

        #[cfg(target_os = "linux")]
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    let name = line.trim_start_matches("PRETTY_NAME=").trim_matches('"');
                    os_name = name.to_string();
                    break;
                }
            }
        }

        let desktop_session = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| if cfg!(windows) { "Windows Explorer Shell".to_string() } else { "X11 Desktop".to_string() });

        let session_type = std::env::var("XDG_SESSION_TYPE")
            .unwrap_or_else(|_| if std::env::var("WAYLAND_DISPLAY").is_ok() { "wayland".to_string() } else if cfg!(windows) { "win32".to_string() } else { "x11".to_string() });

        let native_subsystem = if session_type.to_lowercase().contains("wayland") {
            "Wayland Native Compositor / EGL Hardware Surface".to_string()
        } else if cfg!(windows) {
            "Win32 Direct3D11 Flip Discard / DXGI 1.6 DWM".to_string()
        } else {
            "X11 Direct Client (DISPLAY=:0) / GLX 1.4 Hardware Surface".to_string()
        };

        // 4. Graphics & GPU Adapter
        let mut gpu_adapter = String::new();
        let mut driver_model = if cfg!(windows) { "WDDM 3.1+ (Direct3D 11.4 / DXGI 1.6)" } else { "Direct Rendering Infrastructure (DRI3 / DRM Kernel)" }.to_string();

        #[cfg(target_os = "linux")]
        {
            if let Ok(nv_ver) = std::fs::read_to_string("/proc/driver/nvidia/version") {
                if let Some(first_line) = nv_ver.lines().next() {
                    if let Some(pos) = first_line.find("Module") {
                        driver_model = format!("NVIDIA UNIX Open Driver {}", &first_line[pos + 6..].split("Release").next().unwrap_or("").trim());
                    }
                }
            }
            if let Ok(output) = std::process::Command::new("lspci").output() {
                let lspci_str = String::from_utf8_lossy(&output.stdout);
                for line in lspci_str.lines() {
                    if line.contains("VGA compatible controller") || line.contains("3D controller") || line.contains("Display controller") {
                        if let Some(part) = line.split(':').nth(2) {
                            gpu_adapter = part.trim().to_string();
                            break;
                        }
                    }
                }
            }
        }

        if gpu_adapter.is_empty() {
            gpu_adapter = if !stats.hwdec_current.is_empty() && stats.hwdec_current != "no" {
                format!("Hardware Video Adapter ({})", stats.hwdec_current.to_uppercase())
            } else {
                "Direct3D11 / WGPU / OpenGL Hardware Surface".to_string()
            };
        }

        let presentation_api = if cfg!(windows) {
            "Direct3D 11.4 / DXGI 1.6 Hardware Surface"
        } else if session_type.contains("wayland") {
            "EGL 1.5 / OpenGL Core 4.6 / Wayland Presentation"
        } else {
            "OpenGL Core 4.6 / GLX 1.4 Hardware Acceleration"
        };

        let hwdec_name = if !stats.hwdec_current.is_empty() && stats.hwdec_current != "no" {
            format!("{} (Active Hardware Acceleration)", stats.hwdec_current.to_uppercase())
        } else {
            format!("{} (Multi-Threaded Decoding)", if !stats.hwdec_setting.is_empty() { &stats.hwdec_setting } else { "auto-safe" })
        };

        // 5. Audio Pipeline
        let ao = if !stats.current_ao.is_empty() { &stats.current_ao } else { if cfg!(windows) { "wasapi" } else { "pipewire" } };
        let audio_subsystem = match ao {
            "pipewire" => "PipeWire Low-Latency Audio Server (PulseAudio / ALSA Bridge)",
            "pulse" => "PulseAudio Sound Server (Low-Latency Shared Buffer)",
            "alsa" => "ALSA Linux Sound Architecture (Direct Hardware PCM)",
            "jack" => "JACK Audio Connection Kit (Pro Audio Low-Latency)",
            "wasapi" => "WASAPI Exclusive / Shared Audio Session",
            _ => "Low-Latency High-Definition Audio Pipeline",
        };

        let audio_device = if !stats.active_audio_device.is_empty() && stats.active_audio_device != "auto" {
            stats.active_audio_device.as_str()
        } else {
            "Default System Audio Endpoint"
        };

        let audio_channels_str = if stats.audio_channels > 0 {
            match stats.audio_channels {
                1 => "1.0 Mono (Single Channel)".to_string(),
                2 => "2.0 Stereo (Left / Right Channels)".to_string(),
                6 => "5.1 Surround Sound (FL, FR, FC, LFE, SL, SR)".to_string(),
                8 => "7.1 Surround Sound (FL, FR, FC, LFE, BL, BR, SL, SR)".to_string(),
                ch => format!("{} Channels Multi-Channel Stream", ch),
            }
        } else {
            "2.0 Stereo (Default Hardware Layout)".to_string()
        };

        let audio_sr = if stats.audio_sample_rate > 0 { stats.audio_sample_rate } else { 48000 };
        let audio_bd = if stats.audio_bit_depth > 0 { stats.audio_bit_depth } else { 32 };

        let mut lines = Vec::new();
        lines.push("[CPU & Processor Architecture]".to_string());
        lines.push(format!("  Processor Model         : {}", cpu_model));
        lines.push(format!("  Physical Cores          : {}", physical_cores));
        lines.push(format!("  Logical Cores / Threads : {}", threads));
        lines.push(format!("  Architecture            : {}", arch));
        if !cpu_mhz.is_empty() {
            lines.push(format!("  Core Clock Speed        : {}", cpu_mhz));
        }
        lines.push(format!("  SIMD Acceleration       : {}", simd_flags));

        lines.push(String::new());
        lines.push("[System Memory & Resource Allocation]".to_string());
        lines.push(format!("  Total Physical RAM      : {}", mem_total_str));
        lines.push(format!("  Available System RAM    : {}", mem_avail_str));
        lines.push(format!("  Memory Page Size        : 4,096 Bytes (4 KB Standard)"));
        lines.push(format!("  Demuxer Stream Buffer   : 150 MB Ring Buffer (Read-Ahead Cache)"));

        lines.push(String::new());
        lines.push("[Operating System & Desktop Subsystem]".to_string());
        lines.push(format!("  OS Distribution         : {}", os_name));
        lines.push(format!("  Desktop Environment     : {}", desktop_session));
        lines.push(format!("  Windowing Subsystem     : {} ({})", session_type.to_uppercase(), native_subsystem));
        lines.push(format!("  Graphics Driver Model   : {}", driver_model));
        lines.push(format!("  UI Framework & Engine   : Egui 0.31.1 Immediate Mode (eframe glow)"));

        lines.push(String::new());
        lines.push("[Graphics & Display Hardware]".to_string());
        lines.push(format!("  Active Video Adapter    : {}", gpu_adapter));
        lines.push(format!("  Presentation API        : {}", presentation_api));
        lines.push(format!("  Hardware Decoder        : {}", hwdec_name));
        if stats.display_fps > 0.0 {
            lines.push(format!("  Display Refresh Rate    : {:.1} Hz (High-Precision VSync)", stats.display_fps));
        }
        let color_pipeline = if stats.is_hdr {
            format!("{} / {} -> Tone Mapping ({})", stats.hdr_format, if !stats.colormatrix.is_empty() { &stats.colormatrix } else { "BT.2020" }, stats.hdr_tone_mapping)
        } else if !stats.pixel_format.is_empty() || !stats.colormatrix.is_empty() {
            format!("{} / {} -> Standard RGB Output", if !stats.pixel_format.is_empty() { &stats.pixel_format } else { "YUV" }, if !stats.colormatrix.is_empty() { &stats.colormatrix } else { "BT.709" })
        } else {
            "Standard RGB Conversion".to_string()
        };
        lines.push(format!("  Color Pipeline          : {}", color_pipeline));
        if !stats.current_vo.is_empty() {
            lines.push(format!("  Video Output Renderer   : {}", stats.current_vo));
        }

        lines.push(String::new());
        lines.push("[Audio Engine & Output Pipeline]".to_string());
        lines.push(format!("  Audio Subsystem         : {}", audio_subsystem));
        lines.push(format!("  Output Driver           : {}", ao));
        lines.push(format!("  Output Hardware Device  : {}", audio_device));
        if audio_bd > 0 {
            lines.push(format!("  Audio Pipeline Format   : {}-bit Floating Point Resampler", audio_bd));
        }
        if audio_sr > 0 {
            lines.push(format!("  Output Sample Rate      : {} Hz", audio_sr));
        }
        lines.push(format!("  Channel Configuration   : {}", audio_channels_str));

        lines.push(String::new());
        lines.push("[Demuxer & Media Pipeline]".to_string());
        lines.push(format!("  Demuxer Format          : {}", if !stats.file_format.is_empty() { &stats.file_format } else { "libavformat" }));
        lines.push(format!("  Video Decoder Core      : {}", if !stats.decoder_name.is_empty() { &stats.decoder_name } else { if !stats.video_codec.is_empty() { &stats.video_codec } else { "None" } }));
        lines.push(format!("  Audio Decoder Core      : {}", if !stats.audio_decoder_name.is_empty() { &stats.audio_decoder_name } else { if !stats.audio_codec.is_empty() { &stats.audio_codec } else { "None" } }));
        if stats.audio_delay != 0.0 {
            lines.push(format!("  Clock Offset (A/V Sync) : {:.3} s", stats.audio_delay));
        }

        lines.join("\n")
    }

    pub fn generate_playback_report(stats: &MediaStats) -> String {
        let vcodec = Self::format_short_vcodec(stats);
        let acodec = Self::format_short_acodec(stats);
        let sr = stats.audio_sample_rate;
        let ch = stats.audio_channels;
        let in_br = if stats.audio_bitrate > 0 { stats.audio_bitrate / 1000 } else { 0 };
        let bd = stats.audio_bit_depth;

        format!(
            "Playback Information Report\n=============================\nVideo Codec: {}\nDimensions: {}x{}\nFPS: {:.3}\nAudio Codec: {}\nSample Rate: {} Hz\nBit Depth: {}-bit\nChannels: {} ch\nBitrate: {} kbps\n\n{}",
            vcodec,
            stats.video_width,
            stats.video_height,
            stats.video_fps,
            acodec,
            sr,
            bd,
            ch,
            in_br,
            Self::generate_detailed_raw_info(stats)
        )
    }
}
