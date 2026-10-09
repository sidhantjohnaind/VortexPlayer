//! controls.rs — Pixel-Perfect Authentic Vortex Classic Dark Bottom Control Bar with Media Dashboard

use super::icons::Icons;
use super::theme::VortexTheme;
use crate::bookmark::{format_time, BookmarkManager};
use crate::engine::{MediaStats, Player};
use crate::playlist::Playlist;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Margin, Painter, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, TextureHandle, Vec2,
};
use std::path::Path;

pub struct ControlBar;

#[allow(dead_code)]
fn safe_truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() > max_chars {
        let truncated: String = s.chars().take(max_chars.saturating_sub(3)).collect();
        format!("{}...", truncated)
    } else {
        s.to_string()
    }
}

pub struct ControlBarActions {
    pub toggle_play: bool,
    pub stop: bool,
    pub close_file: bool,
    pub next_track: bool,
    pub prev_track: bool,
    pub next_chapter: bool,
    pub prev_chapter: bool,
    pub skip_intro: bool,
    pub seek_relative: Option<f64>,
    pub frame_step: bool,
    pub open_file: bool,
    pub open_audio_channels: bool,
    pub toggle_hwdec: bool,
    pub cycle_hdr_tone_mapping: bool,
    pub toggle_playlist: bool,
    pub toggle_control_panel: bool,
    pub toggle_preferences: bool,
    pub toggle_bookmark_overlay: bool,
    pub toggle_ab_loop: bool,
    pub cycle_repeat: bool,
    pub toggle_shuffle: bool,
    pub cycle_audio: bool,
    pub cycle_subtitle: bool,
    pub open_mediainfo: bool,
    pub hover_tooltip: Option<(Pos2, String)>,
    pub hover_card_rect: Option<Rect>,
    pub open_audio_channels_pos: Option<Pos2>,
    pub take_screenshot: bool,
    pub cycle_speed: bool,
    pub reset_speed: bool,
    pub open_speed_menu: bool,
    pub open_speed_menu_pos: Option<Pos2>,
    pub open_audio_tracks_popup: bool,
    pub open_subtitle_tracks_popup: bool,
    pub toggle_wasapi_exclusive: bool,
    pub toggle_lyrics: bool,
    pub toggle_lyrics_expanded: bool,
}

impl ControlBar {
    pub const HEIGHT_COMPACT: f32 = 58.0;
    pub const HEIGHT_FULLSCREEN: f32 = 64.0;
    pub const HEIGHT_MUSIC: f32 = 104.0;

    pub fn render(
        ui: &mut egui::Ui,
        player: Option<&Player>,
        stats: &MediaStats,
        playlist: &mut Playlist,
        bookmark_mgr: &mut BookmarkManager,
        show_remaining_time: &mut bool,
        cover_texture: Option<&TextureHandle>,
        config: &crate::config::AppConfig,
        seek_preview: &mut crate::ui::SeekPreviewEngine,
        is_fullscreen: bool,
        is_playlist_open: bool,
    ) -> ControlBarActions {
        let is_song = crate::ui::music_view::MusicBackgroundView::is_song(stats);
        let total_h = if is_song {
            Self::HEIGHT_MUSIC
        } else if is_fullscreen {
            Self::HEIGHT_FULLSCREEN
        } else {
            Self::HEIGHT_COMPACT
        };

        let mut actions = ControlBarActions {
            toggle_play: false,
            stop: false,
            close_file: false,
            next_track: false,
            prev_track: false,
            next_chapter: false,
            prev_chapter: false,
            skip_intro: false,
            seek_relative: None,
            frame_step: false,
            open_file: false,
            open_audio_channels: false,
            toggle_hwdec: false,
            cycle_hdr_tone_mapping: false,
            toggle_playlist: false,
            toggle_control_panel: false,
            toggle_preferences: false,
            toggle_bookmark_overlay: false,
            toggle_ab_loop: false,
            cycle_repeat: false,
            toggle_shuffle: false,
            cycle_audio: false,
            cycle_subtitle: false,
            open_mediainfo: false,
            hover_tooltip: None,
            hover_card_rect: None,
            open_audio_channels_pos: None,
            take_screenshot: false,
            cycle_speed: false,
            reset_speed: false,
            open_speed_menu: false,
            open_speed_menu_pos: None,
            open_audio_tracks_popup: false,
            open_subtitle_tracks_popup: false,
            toggle_wasapi_exclusive: false,
            toggle_lyrics: false,
            toggle_lyrics_expanded: false,
        };

        let mut hovered_tooltip: Option<(Rect, String)> = None;

        let (total_rect, _) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), total_h),
            Sense::empty(),
        );

        let skin = VortexTheme::get_skin(config.theme_mode);
        let painter = ui.painter();

        // ── 1. Themed Surface Background & Top Highlight ──────────────
        painter.rect_filled(total_rect, CornerRadius::ZERO, skin.bg_toolbar);
        painter.line_segment(
            [total_rect.left_top(), total_rect.right_top()],
            Stroke::new(1.0, skin.border_dark),
        );

        let mut current_y = total_rect.top();

        // ── 2. Top Media Dashboard (When playing Music / Audio) ─────────────
        if is_song {
            let dash_h = 58.0;
            let dash_rect = Rect::from_min_size(
                Pos2::new(total_rect.left(), current_y),
                Vec2::new(total_rect.width(), dash_h),
            );

            // Dashboard themed panel fill & bottom hairline
            painter.rect_filled(dash_rect, CornerRadius::ZERO, skin.bg_panel);
            painter.line_segment(
                [dash_rect.left_bottom(), dash_rect.right_bottom()],
                Stroke::new(1.0, skin.border_dark),
            );

            let is_wide_dash = dash_rect.width() >= 480.0;
            let mut cur_x = dash_rect.left() + 8.0;

            // 1. Mini Cover Thumbnail (Vortex 44x44 container with preserved aspect ratio)
            if is_wide_dash {
                let thumb_size = 44.0;
                let thumb_box = Rect::from_min_size(
                    Pos2::new(cur_x, dash_rect.center().y - thumb_size / 2.0),
                    Vec2::splat(thumb_size),
                );
                painter.rect_filled(thumb_box, CornerRadius::same(2), Color32::from_rgb(18, 19, 24));

                if let Some(tex) = cover_texture {
                    let s = tex.size_vec2();
                    let aspect = s.x / s.y.max(1.0);
                    let (w, h) = if aspect > 1.0 {
                        (thumb_size, thumb_size / aspect)
                    } else {
                        (thumb_size * aspect, thumb_size)
                    };
                    let img_r = Rect::from_center_size(thumb_box.center(), Vec2::new(w, h));
                    painter.image(
                        tex.id(),
                        img_r,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                } else {
                    Icons::draw_vertex_logo(painter, thumb_box.center(), 10.0);
                }
                painter.rect_stroke(thumb_box, CornerRadius::same(2), Stroke::new(1.0, Color32::from_rgb(26, 28, 36)), StrokeKind::Inside);
                cur_x += thumb_size + 12.0;
            } else {
                cur_x += 4.0;
            }

            // 2. Digital Time Counter (Vortex exact proportional tight kerning)
            let cur_time_str = format_time(stats.time_pos);
            let tot_time_str = format_time(stats.duration);

            if is_wide_dash {
                // Wide Mode: 25px Bold White + 12px Total Duration below (proportional font = tight colons)
                painter.text(
                    Pos2::new(cur_x, dash_rect.top() + 5.0),
                    Align2::LEFT_TOP,
                    &cur_time_str,
                    FontId::proportional(25.0),
                    Color32::WHITE,
                );

                painter.text(
                    Pos2::new(cur_x + 1.0, dash_rect.top() + 34.0),
                    Align2::LEFT_TOP,
                    &tot_time_str,
                    FontId::proportional(12.0),
                    Color32::from_rgb(120, 135, 155),
                );

                let time_w = (cur_time_str.len().max(tot_time_str.len()) as f32 * 13.0).max(70.0);
                cur_x += time_w + 14.0;

                // 3. Track Title & Specs Row (Vortex tabbed specs without bullets)
                let raw_title = {
                    let name = Path::new(&stats.file_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(if stats.title.is_empty() { "Audio Track" } else { &stats.title });
                    name.to_string()
                };

                let spec_audio_codec = stats.audio_codec.split(|c: char| c.is_whitespace() || c == '(' || c == '/').next().unwrap_or("FLAC").to_uppercase();
                let spec_bitrate = if stats.audio_bitrate > 0 { format!("{:.0}kbps", stats.audio_bitrate as f64 / 1000.0) } else { "418kbps".to_string() };
                let spec_sample_rate = if stats.audio_sample_rate > 0 { format!("{:.1}kHz", stats.audio_sample_rate as f64 / 1000.0) } else { "44.1kHz".to_string() };
                let spec_line = format!("{}    {}    {}", spec_audio_codec, spec_bitrate, spec_sample_rate);

                let mut title_label = raw_title.clone();
                if let Some(idx) = playlist.current_index {
                    title_label = format!("[{}/{}] {}", idx + 1, playlist.items.len(), raw_title);
                }

                // Keep the dashboard readable at narrow window widths. The
                // action cluster is anchored to the right, so cap the title
                // to the space that is actually available instead of letting
                // it run underneath the A-B/repeat controls.
                let reserved_right = 176.0 + 10.0;
                let title_width = (dash_rect.right() - reserved_right - cur_x - 12.0).max(84.0);
                let title_chars = (title_width / 7.2).floor() as usize;
                title_label = safe_truncate(&title_label, title_chars.max(8));

                // Render track title (13.5px crisp white)
                painter.text(
                    Pos2::new(cur_x, dash_rect.top() + 8.0),
                    Align2::LEFT_TOP,
                    &title_label,
                    FontId::proportional(13.5),
                    Color32::from_rgb(245, 248, 255),
                );

                // Render specs line (Vortex 11.0px clean tabbed layout)
                painter.text(
                    Pos2::new(cur_x, dash_rect.top() + 34.0),
                    Align2::LEFT_TOP,
                    &spec_line,
                    FontId::proportional(11.0),
                    Color32::from_rgb(120, 128, 142),
                );
            } else {
                // Compact Mini Mode: Bold 24px time + 11.5px total duration below
                painter.text(
                    Pos2::new(cur_x, dash_rect.top() + 5.0),
                    Align2::LEFT_TOP,
                    &cur_time_str,
                    FontId::proportional(24.0),
                    Color32::WHITE,
                );

                painter.text(
                    Pos2::new(cur_x + 1.0, dash_rect.top() + 34.0),
                    Align2::LEFT_TOP,
                    &tot_time_str,
                    FontId::proportional(11.5),
                    Color32::from_rgb(120, 135, 155),
                );
            }

            // D. Right Block: A-B Repeat Cluster + Blue Repeat & Shuffle Pills + Lyrics
            let right_block_w = 176.0;
            let right_block_x = dash_rect.right() - right_block_w - 10.0;
            let ctrl_row_y = dash_rect.center().y - 12.0;

            // A-B Repeat Cluster plate [A] [⇄] [B]
            let ab_plate_w = 74.0;
            let ab_plate_h = 24.0;
            let ab_plate_rect = Rect::from_min_size(Pos2::new(right_block_x, ctrl_row_y), Vec2::new(ab_plate_w, ab_plate_h));
            painter.rect_filled(ab_plate_rect, CornerRadius::same(3), Color32::from_rgb(26, 28, 34));
            painter.rect_stroke(ab_plate_rect, CornerRadius::same(3), Stroke::new(1.0, Color32::from_rgb(44, 48, 58)), StrokeKind::Inside);

            let btn_w = 22.0;
            let btn_y = ab_plate_rect.top() + 1.0;

            // [A]
            let a_rect = Rect::from_min_size(Pos2::new(ab_plate_rect.left() + 2.0, btn_y), Vec2::new(btn_w, 22.0));
            let a_active = bookmark_mgr.ab_loop.point_a.is_some();
            let a_resp = ui.interact(a_rect, ui.id().with("mini_ab_a"), Sense::click());
            if a_resp.hovered() {
                hovered_tooltip = Some((a_rect, "A-B Loop: Set Start Point [A] (Shortcut: [)\nRight-click to Clear".to_string()));
            }
            if a_resp.clicked() {
                let _ = bookmark_mgr.set_loop_a(stats.time_pos);
            }
            if a_resp.secondary_clicked() {
                bookmark_mgr.clear_ab_loop();
            }
            let a_col = if a_active { skin.accent_primary } else if a_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(140, 145, 160) };
            painter.text(a_rect.center(), Align2::CENTER_CENTER, "A", FontId::monospace(11.5), a_col);

            // [⇄]
            let loop_toggle_rect = Rect::from_min_size(Pos2::new(ab_plate_rect.left() + 26.0, btn_y), Vec2::new(btn_w, 22.0));
            let loop_active = bookmark_mgr.ab_loop.is_active;
            let loop_resp = ui.interact(loop_toggle_rect, ui.id().with("mini_ab_toggle"), Sense::click());
            if loop_resp.hovered() {
                hovered_tooltip = Some((loop_toggle_rect, "A-B Loop: Toggle / Clear (Shortcut: \\)".to_string()));
            }
            if loop_resp.clicked() {
                if loop_active {
                    bookmark_mgr.clear_ab_loop();
                } else if a_active && bookmark_mgr.ab_loop.point_b.is_some() {
                    bookmark_mgr.toggle_ab_loop();
                } else {
                    let _ = bookmark_mgr.set_loop_a(stats.time_pos);
                }
            }
            if loop_resp.secondary_clicked() {
                bookmark_mgr.clear_ab_loop();
            }
            let loop_col = if loop_active { skin.accent_primary } else if loop_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(130, 135, 150) };
            let rc = loop_toggle_rect.center();
            let y1 = rc.y - 2.5;
            painter.line_segment([Pos2::new(rc.x - 4.0, y1), Pos2::new(rc.x + 2.5, y1)], Stroke::new(1.1, loop_col));
            painter.line_segment([Pos2::new(rc.x + 0.5, y1 - 2.0), Pos2::new(rc.x + 3.0, y1)], Stroke::new(1.1, loop_col));
            painter.line_segment([Pos2::new(rc.x + 0.5, y1 + 2.0), Pos2::new(rc.x + 3.0, y1)], Stroke::new(1.1, loop_col));

            let y2 = rc.y + 2.5;
            painter.line_segment([Pos2::new(rc.x - 2.5, y2), Pos2::new(rc.x + 4.0, y2)], Stroke::new(1.1, loop_col));
            painter.line_segment([Pos2::new(rc.x - 0.5, y2 - 2.0), Pos2::new(rc.x - 3.0, y2)], Stroke::new(1.1, loop_col));
            painter.line_segment([Pos2::new(rc.x - 0.5, y2 + 2.0), Pos2::new(rc.x - 3.0, y2)], Stroke::new(1.1, loop_col));

            // [B]
            let b_rect = Rect::from_min_size(Pos2::new(ab_plate_rect.left() + 50.0, btn_y), Vec2::new(btn_w, 22.0));
            let b_active = bookmark_mgr.ab_loop.point_b.is_some();
            let b_resp = ui.interact(b_rect, ui.id().with("mini_ab_b"), Sense::click());
            if b_resp.hovered() {
                hovered_tooltip = Some((b_rect, "A-B Loop: Set End Point [B] (Shortcut: ])\nRight-click to Clear".to_string()));
            }
            if b_resp.clicked() {
                let _ = bookmark_mgr.set_loop_b(stats.time_pos);
            }
            if b_resp.secondary_clicked() {
                bookmark_mgr.clear_ab_loop();
            }
            let b_col = if b_active { skin.accent_primary } else if b_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(140, 145, 160) };
            painter.text(b_rect.center(), Align2::CENTER_CENTER, "B", FontId::monospace(11.5), b_col);

            // 3. Compact dark Repeat button, matching the classic Vortex skin.
            let rep_pill_rect = Rect::from_min_size(Pos2::new(right_block_x + 80.0, ctrl_row_y), Vec2::new(26.0, 24.0));
            let rep_active = playlist.repeat_mode != crate::playlist::RepeatMode::Off;
            let rep_resp = ui.interact(rep_pill_rect, ui.id().with("dash_repeat_btn"), Sense::click());
            
            let rep_bg = if rep_resp.hovered() { skin.bg_btn_hover } else { skin.bg_btn };
            painter.rect_filled(rep_pill_rect, CornerRadius::same(3), rep_bg);
            painter.rect_stroke(rep_pill_rect, CornerRadius::same(3), Stroke::new(1.0, if rep_active { skin.accent_primary } else { skin.border_dark }), StrokeKind::Inside);

            let rep_icon_col = if rep_active { skin.accent_primary } else if rep_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 155, 170) };
            let is_one = playlist.repeat_mode == crate::playlist::RepeatMode::RepeatTrack;
            Icons::draw_repeat(painter, rep_pill_rect, rep_icon_col, is_one);

            if rep_resp.hovered() {
                let tip = match playlist.repeat_mode {
                    crate::playlist::RepeatMode::Off => "Repeat: Off (Click to cycle)",
                    crate::playlist::RepeatMode::RepeatTrack => "Repeat: Current Track (1)",
                    crate::playlist::RepeatMode::RepeatAll => "Repeat: All Tracks (All)",
                };
                hovered_tooltip = Some((rep_pill_rect, tip.to_string()));
            }
            if rep_resp.clicked() {
                playlist.cycle_repeat_mode();
            }

            // 4. Compact dark Shuffle button, matching the classic Vortex skin.
            let shuf_pill_rect = Rect::from_min_size(Pos2::new(right_block_x + 110.0, ctrl_row_y), Vec2::new(26.0, 24.0));
            let shuf_active = playlist.is_shuffle();
            let shuf_resp = ui.interact(shuf_pill_rect, ui.id().with("dash_shuffle_btn"), Sense::click());

            let shuf_bg = if shuf_resp.hovered() { skin.bg_btn_hover } else { skin.bg_btn };
            painter.rect_filled(shuf_pill_rect, CornerRadius::same(3), shuf_bg);
            painter.rect_stroke(shuf_pill_rect, CornerRadius::same(3), Stroke::new(1.0, if shuf_active { skin.accent_primary } else { skin.border_dark }), StrokeKind::Inside);

            let shuf_icon_col = if shuf_active { skin.accent_primary } else if shuf_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(150, 155, 170) };
            Icons::draw_shuffle(painter, shuf_pill_rect, shuf_icon_col);

            if shuf_resp.hovered() {
                hovered_tooltip = Some((shuf_pill_rect, if shuf_active { "Shuffle: ON (Click to toggle)" } else { "Shuffle: OFF (Click to toggle)" }.to_string()));
            }
            if shuf_resp.clicked() {
                playlist.toggle_shuffle();
            }

            // 5. Compact dark Lyrics button
            let lyr_pill_rect = Rect::from_min_size(Pos2::new(right_block_x + 140.0, ctrl_row_y), Vec2::new(26.0, 24.0));
            let lyr_resp = ui.interact(lyr_pill_rect, ui.id().with("dash_lyrics_btn"), Sense::click());
            let lyr_bg = if lyr_resp.hovered() { skin.bg_btn_hover } else { skin.bg_btn };
            painter.rect_filled(lyr_pill_rect, CornerRadius::same(3), lyr_bg);
            painter.rect_stroke(lyr_pill_rect, CornerRadius::same(3), Stroke::new(1.0, skin.border_dark), StrokeKind::Inside);
            painter.text(
                lyr_pill_rect.center(),
                Align2::CENTER_CENTER,
                "🎙",
                FontId::proportional(11.5),
                if lyr_resp.hovered() { skin.accent_primary } else { Color32::from_rgb(150, 155, 170) },
            );
            if lyr_resp.hovered() {
                hovered_tooltip = Some((lyr_pill_rect, "Synchronized Lyrics / Karaoke (Ctrl+Shift+L)\nRight-click: Full Lyrics Studio".to_string()));
            }
            if lyr_resp.clicked() {
                actions.toggle_lyrics = true;
            }
            if lyr_resp.secondary_clicked() {
                actions.toggle_lyrics_expanded = true;
            }

            current_y += dash_h;
        }

        // ── 3. Ultra-Polished Modern Dynamic Seekbar with Live Frame Hover Preview ───
        let seekbar_pad = if is_fullscreen { 16.0 } else { 12.0 };
        let seekbar_h = if is_song { 10.0 } else if is_fullscreen { 14.0 } else { 11.0 };
        let seekbar_y = current_y + 2.0;
        let seekbar_rect = Rect::from_min_size(
            Pos2::new(total_rect.left() + seekbar_pad, seekbar_y),
            Vec2::new(total_rect.width() - seekbar_pad * 2.0, seekbar_h),
        );

        let seek_resp = ui.interact(seekbar_rect, ui.id().with("main_seekbar"), Sense::click_and_drag());
        let seek_hovered = seek_resp.hovered() || seek_resp.dragged();

        let track_h = if is_fullscreen {
            if seek_hovered { 6.5 } else { 4.0 }
        } else {
            if seek_hovered { 5.5 } else { 3.5 }
        };
        let track_rect = Rect::from_min_size(
            Pos2::new(seekbar_rect.left(), seekbar_rect.center().y - track_h / 2.0),
            Vec2::new(seekbar_rect.width(), track_h),
        );

        // Visible slate track groove matching Vortex
        painter.rect_filled(track_rect, CornerRadius::same(1), Color32::from_rgb(52, 56, 68));
        painter.rect_stroke(track_rect, CornerRadius::same(1), Stroke::new(0.5, Color32::from_rgb(32, 34, 42)), StrokeKind::Inside);

        // Buffer bar with smooth corner radius
        if stats.cache_buffer_percent > 0.0 && stats.duration > 0.0 {
            let buf_w = (track_rect.width() * (stats.cache_buffer_percent / 100.0) as f32).clamp(0.0, track_rect.width());
            let buf_rect = Rect::from_min_size(track_rect.min, Vec2::new(buf_w, track_h));
            painter.rect_filled(buf_rect, CornerRadius::same(2), Color32::from_rgb(60, 65, 82));
        }

        // Played progress bar (Vibrant Gold/Amber with glow when hovered)
        let progress = if stats.duration > 0.0 {
            (stats.time_pos / stats.duration).clamp(0.0, 1.0) as f32
        } else {
            0.0
        };

        let played_w = track_rect.width() * progress;
        if played_w > 0.0 {
            let played_rect = Rect::from_min_size(track_rect.min, Vec2::new(played_w, track_h));
            painter.rect_filled(played_rect, CornerRadius::same(2), skin.accent_primary);
            if seek_hovered {
                painter.rect_filled(
                    played_rect.expand(1.0),
                    CornerRadius::same(3),
                    skin.accent_bright.gamma_multiply(0.35),
                );
            }
        }

        // A-B Loop Range Visualization (Emerald Green Highlight)
        if stats.duration > 0.0 {
            if let Some(pt_a) = bookmark_mgr.ab_loop.point_a {
                let pt_b = bookmark_mgr.ab_loop.point_b.unwrap_or(stats.time_pos);
                let start_frac = (pt_a / stats.duration).clamp(0.0, 1.0) as f32;
                let end_frac = (pt_b / stats.duration).clamp(0.0, 1.0) as f32;
                let start_x = track_rect.left() + track_rect.width() * start_frac;
                let end_x = track_rect.left() + track_rect.width() * end_frac;
                let w = (end_x - start_x).abs().max(3.0);
                let ab_rect = Rect::from_min_size(Pos2::new(start_x.min(end_x), track_rect.top() - 1.5), Vec2::new(w, track_h + 3.0));
                painter.rect_filled(ab_rect, CornerRadius::same(2), Color32::from_rgba_unmultiplied(50, 220, 100, 70));
                painter.rect_stroke(ab_rect, CornerRadius::same(2), Stroke::new(1.0, Color32::from_rgb(50, 220, 100)), StrokeKind::Inside);
            }
        }

        // Chapter Markers & Dividers (Crystal-clear segmented timeline notches)
        if stats.duration > 0.0 {
            for (_i, ch) in stats.chapters.iter().enumerate() {
                if ch.time_pos > 0.0 && ch.time_pos < stats.duration {
                    let ch_frac = (ch.time_pos / stats.duration) as f32;
                    let ch_x = track_rect.left() + track_rect.width() * ch_frac;

                    // Clean notch with subtle highlight
                    painter.line_segment(
                        [Pos2::new(ch_x, track_rect.top() - 1.5), Pos2::new(ch_x, track_rect.bottom() + 1.5)],
                        Stroke::new(1.2, Color32::from_rgb(185, 195, 220)),
                    );
                    painter.circle_filled(Pos2::new(ch_x, track_rect.top() - 1.5), 1.4, Color32::from_rgb(100, 180, 255));
                }
            }
        }

        // Bookmark Markers (Golden Amber Diamonds ◆ and Coral Red Skip Zones on seekbar)
        if stats.duration > 0.0 {
            for bm in &bookmark_mgr.bookmarks {
                if bm.time_pos >= 0.0 && bm.time_pos <= stats.duration {
                    let bm_frac = (bm.time_pos / stats.duration) as f32;
                    let bm_x = track_rect.left() + track_rect.width() * bm_frac;
                    let bm_y = track_rect.center().y;
                    let diamond_r = 3.2;
                    let is_skip = bm.title.to_uppercase().contains("[SKIP]") || bm.title.to_uppercase().contains("SKIP:");
                    let diamond_color = if is_skip {
                        Color32::from_rgb(255, 75, 75)
                    } else {
                        skin.accent_primary
                    };

                    let points = vec![
                        Pos2::new(bm_x, bm_y - diamond_r),
                        Pos2::new(bm_x + diamond_r, bm_y),
                        Pos2::new(bm_x, bm_y + diamond_r),
                        Pos2::new(bm_x - diamond_r, bm_y),
                    ];
                    painter.add(egui::epaint::PathShape::convex_polygon(
                        points,
                        diamond_color,
                        Stroke::new(0.6, Color32::BLACK),
                    ));
                }
            }
        }

        // Seek Knob: Vortex circular white scrubber knob with halo
        let knob_r = if seek_hovered { 5.5 } else { 4.0 };
        let knob_x = (track_rect.left() + played_w).clamp(track_rect.left() + knob_r, track_rect.right() - knob_r);
        let knob_y = track_rect.center().y;
        if seek_hovered {
            painter.circle_filled(Pos2::new(knob_x, knob_y), 11.0, skin.accent_bright.gamma_multiply(0.35));
            painter.circle_filled(Pos2::new(knob_x, knob_y), 7.5, skin.accent_bright.gamma_multiply(0.70));
        }
        painter.circle_filled(Pos2::new(knob_x, knob_y), knob_r, Color32::WHITE);
        painter.circle_stroke(Pos2::new(knob_x, knob_y), knob_r, Stroke::new(1.0, Color32::from_rgb(35, 38, 48)));

        // Click / Drag seeking
        if seek_resp.clicked() || seek_resp.dragged() {
            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos().or(i.pointer.latest_pos())) {
                let frac = ((mouse_pos.x - track_rect.left()) / track_rect.width()).clamp(0.0, 1.0);
                let target_time = frac as f64 * stats.duration;
                if let Some(p) = player {
                    p.seek_absolute(target_time);
                }
            }
        }

        // Storyboard Video Scrubbing & Live Thumbnail Preview Card
        if seek_hovered {
            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos().or(i.pointer.latest_pos())) {
                let frac = ((mouse_pos.x - track_rect.left()) / track_rect.width()).clamp(0.0, 1.0);
                let hover_time = frac as f64 * stats.duration;
                let time_str = format_time(hover_time);
                let pct = (frac * 100.0) as u32;

                // Find chapter if any
                let chapter_title = stats.chapters.iter().find(|c| {
                    if let Some(next_c) = stats.chapters.iter().find(|n| n.index == c.index + 1) {
                        hover_time >= c.time_pos && hover_time < next_c.time_pos
                    } else {
                        hover_time >= c.time_pos
                    }
                }).map(|c| c.title.clone());

                let has_video = !stats.file_path.is_empty() && !crate::ui::music_view::MusicBackgroundView::is_song(stats);
                let card_w = if has_video { 204.0 } else { 160.0 };
                let card_h = if has_video {
                    if chapter_title.is_some() { 172.0 } else { 144.0 }
                } else if chapter_title.is_some() {
                    72.0
                } else {
                    48.0
                };

                let min_x = track_rect.left() + card_w * 0.5 + 4.0;
                let max_x = track_rect.right() - card_w * 0.5 - 4.0;
                let card_x = if min_x <= max_x {
                    mouse_pos.x.clamp(min_x, max_x)
                } else {
                    track_rect.center().x
                };

                // Position card safely floating 16px ABOVE the seekbar track to guarantee NO overlap
                let card_bottom = track_rect.top() - 16.0;
                let card_top = card_bottom - card_h;
                let card_rect = Rect::from_min_size(Pos2::new(card_x - card_w * 0.5, card_top), Vec2::new(card_w, card_h));
                actions.hover_card_rect = Some(card_rect);

                let card_painter = ui.ctx().layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("vortex_seek_preview_tooltip")));

                // Soft Ambient Drop Shadow
                card_painter.rect_filled(card_rect.translate(Vec2::new(0.0, 4.0)).expand(2.0), CornerRadius::same(10), Color32::from_rgba_premultiplied(0, 0, 0, 140));

                // Card Frosted Obsidian Glass Background
                card_painter.rect_filled(card_rect, CornerRadius::same(8), Color32::from_rgba_unmultiplied(16, 18, 24, 248));
                // Sleek, elegant subtle slate border (avoids harsh yellow/orange borders)
                card_painter.rect_stroke(card_rect, CornerRadius::same(8), Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 32)), StrokeKind::Inside);

                let mut cur_inner_y = card_rect.top() + 8.0;

                // 1. Live Video Frame Thumbnail Preview
                if has_video {
                    let thumb_w = 188.0;
                    let thumb_h = 105.0;
                    let img_rect = Rect::from_min_size(Pos2::new(card_rect.left() + (card_w - thumb_w) * 0.5, cur_inner_y), Vec2::new(thumb_w, thumb_h));

                    let pb = std::path::Path::new(&stats.file_path);
                    let tex_opt = seek_preview.get_or_request(ui.ctx(), pb, hover_time);

                    if let Some(tex) = tex_opt {
                        card_painter.image(
                            tex.id(),
                            img_rect,
                            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                            Color32::WHITE,
                        );
                        card_painter.rect_stroke(img_rect, CornerRadius::same(5), Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 30)), StrokeKind::Inside);
                    } else {
                        // Animated shimmer loading card
                        card_painter.rect_filled(img_rect, CornerRadius::same(5), Color32::from_rgb(22, 24, 32));
                        card_painter.rect_stroke(img_rect, CornerRadius::same(5), Stroke::new(1.0, Color32::from_rgb(45, 48, 62)), StrokeKind::Inside);
                        card_painter.text(
                            img_rect.center(),
                            Align2::CENTER_CENTER,
                            "🎬 Loading...",
                            FontId::proportional(11.0),
                            Color32::from_rgb(140, 145, 165),
                        );
                    }
                    cur_inner_y += thumb_h + 8.0;
                }

                // 2. Filmstrip Timestamp Pill (Clean modern typography)
                let pill_h = 20.0;
                let pill_rect = Rect::from_min_size(Pos2::new(card_rect.left() + 8.0, cur_inner_y), Vec2::new(card_w - 16.0, pill_h));
                card_painter.rect_filled(pill_rect, CornerRadius::same(5), Color32::from_rgba_unmultiplied(255, 255, 255, 10));
                card_painter.rect_stroke(pill_rect, CornerRadius::same(5), Stroke::new(0.8, Color32::from_rgba_unmultiplied(255, 255, 255, 18)), StrokeKind::Inside);

                let header_text = format!("{}  ·  {}%", time_str, pct);
                card_painter.text(pill_rect.center(), Align2::CENTER_CENTER, header_text, FontId::proportional(11.5), Color32::from_rgb(240, 243, 250));
                cur_inner_y += pill_h + 6.0;

                // 3. Chapter title banner if present
                if let Some(ch_name) = chapter_title {
                    let ch_h = 20.0;
                    let ch_rect = Rect::from_min_size(Pos2::new(card_rect.left() + 8.0, cur_inner_y), Vec2::new(card_w - 16.0, ch_h));
                    card_painter.rect_filled(ch_rect, CornerRadius::same(5), Color32::from_rgba_unmultiplied(59, 130, 246, 28));
                    card_painter.rect_stroke(ch_rect, CornerRadius::same(5), Stroke::new(0.8, Color32::from_rgba_unmultiplied(96, 165, 250, 65)), StrokeKind::Inside);

                    let short_ch = if ch_name.chars().count() > 22 {
                        let t: String = ch_name.chars().take(19).collect();
                        format!("{}...", t)
                    } else {
                        ch_name
                    };
                    card_painter.text(ch_rect.center(), Align2::CENTER_CENTER, format!("📑 {}", short_ch), FontId::proportional(10.5), Color32::from_rgb(147, 197, 253));
                }
            }
        }

        current_y = seekbar_y + seekbar_h + 2.0;

        // Horizontal separator line under seekbar
        painter.line_segment(
            [Pos2::new(total_rect.left(), current_y), Pos2::new(total_rect.right(), current_y)],
            Stroke::new(1.0, skin.border_dark),
        );

        // ── 4. Lower Transport Bar: Tiled Segments with Dividers ─────────────
        let row_top = current_y + 2.0;
        let row_h = (total_rect.bottom() - row_top - 5.0).max(28.0);
        let mut left_x = total_rect.left();

        let draw_v_divider = |p: &Painter, x: f32| {
            p.line_segment(
                [Pos2::new(x, row_top + 3.0), Pos2::new(x, row_top + row_h - 3.0)],
                Stroke::new(1.0, skin.border_dark),
            );
        };

        let avail_w = total_rect.width();
        let is_ultra_wide = avail_w >= 1020.0;
        let is_wide = avail_w >= 800.0;
        let is_medium = avail_w >= 620.0;

        // Calculate right-side reserved width to prevent left and right controls from colliding
        let right_reserved_w = 28.0 
            + (if is_wide { 78.0 } else { 0.0 }) 
            + (if !is_song && is_medium { 42.0 } else { 0.0 }) 
            + (if is_wide { 92.0 } else if is_medium { 68.0 } else { 28.0 }) 
            + 12.0;
        let right_boundary_x = (total_rect.right() - right_reserved_w).max(left_x + 120.0);

        // 1. Play / Pause Button Tile (36px wide)
        {
            let tile_w = 36.0;
            let tile_r = Rect::from_min_size(Pos2::new(left_x, row_top), Vec2::new(tile_w, row_h));
            let resp = ui.interact(tile_r, ui.id().with("tile_play"), Sense::click());
            
            if resp.hovered() {
                painter.rect_filled(tile_r, CornerRadius::ZERO, skin.bg_btn_hover);
                let tip = if stats.is_paused || stats.is_idle { "Play (Space)" } else { "Pause (Space)" };
                hovered_tooltip = Some((tile_r, tip.to_string()));
            }

            let icon_col = if resp.hovered() { Color32::WHITE } else { Color32::from_rgb(215, 218, 228) };
            if stats.is_paused || stats.is_idle {
                Icons::draw_play(painter, tile_r, icon_col);
            } else {
                Icons::draw_pause(painter, tile_r, icon_col);
            }

            if resp.clicked() {
                actions.toggle_play = true;
            }
            left_x += tile_w;
            draw_v_divider(painter, left_x);
        }

        // Helper for standard 28px button tiles (only creates tile if space permits)
        let mut make_tile = |id_str: &str, w: f32, tip_text: &str, draw_fn: &dyn Fn(&Painter, Rect, Color32)| -> bool {
            if left_x + w > right_boundary_x - 110.0 {
                return false;
            }
            let tile_r = Rect::from_min_size(Pos2::new(left_x, row_top), Vec2::new(w, row_h));
            let resp = ui.interact(tile_r, ui.id().with(id_str), Sense::click());
            
            if resp.hovered() {
                painter.rect_filled(tile_r, CornerRadius::ZERO, skin.bg_btn_hover);
                hovered_tooltip = Some((tile_r, tip_text.to_string()));
            }

            let icon_col = if resp.hovered() { Color32::WHITE } else { Color32::from_rgb(175, 180, 192) };
            draw_fn(painter, tile_r, icon_col);

            left_x += w;
            draw_v_divider(painter, left_x);
            resp.clicked()
        };

        // 2. Stop (28px)
        if make_tile("tile_stop", 28.0, "Stop (Ctrl+S)", &|p, r, c| Icons::draw_stop(p, r, c)) {
            actions.stop = true;
        }

        // 2b. Close File & Return to Home (26px) (Wide Video mode only)
        if !is_song && is_wide && make_tile("tile_close_file", 26.0, "Close File & Return to Home (Ctrl+W / F4)", &|p, r, c| Icons::draw_close(p, r, c)) {
            actions.close_file = true;
        }

        // 3. Prev Track (26px)
        if make_tile("tile_prev", 26.0, "Previous Track (Page Up)", &|p, r, c| Icons::draw_prev(p, r, c)) {
            actions.prev_track = true;
        }

        // 3b. Prev Chapter (26px) (Ultra-wide only)
        if !is_song && is_ultra_wide && !stats.chapters.is_empty() {
            if make_tile("tile_prev_chap", 26.0, "Previous Chapter (Shift+H / [)", &|p, r, c| Icons::draw_chapter_prev(p, r, c)) {
                actions.prev_chapter = true;
            }
        }

        // 4a. Next Chapter (26px) (Ultra-wide only)
        if !is_song && is_ultra_wide && !stats.chapters.is_empty() {
            if make_tile("tile_next_chap", 26.0, "Next Chapter (H / ])", &|p, r, c| Icons::draw_chapter_next(p, r, c)) {
                actions.next_chapter = true;
            }
        }

        // 4. Next Track (26px)
        if make_tile("tile_next", 26.0, "Next Track (Page Down)", &|p, r, c| Icons::draw_next(p, r, c)) {
            actions.next_track = true;
        }

        // 4b. Skip OP / Intro (28px)
        let is_in_intro = stats.chapters.iter().any(|c| {
            let is_match = crate::config::matches_skip_keyword(&c.title, "op")
                || crate::config::matches_skip_keyword(&c.title, "opening")
                || crate::config::matches_skip_keyword(&c.title, "intro")
                || crate::config::matches_skip_keyword(&c.title, "prologue")
                || crate::config::matches_skip_keyword(&c.title, "recap");
            is_match
                && stats.time_pos >= c.time_pos
                && stats.chapters.iter().find(|n| n.index == c.index + 1).map_or(true, |n| stats.time_pos < n.time_pos)
        }) || (stats.time_pos < 90.0 && stats.duration > 180.0);

        if !is_song && is_wide && is_in_intro {
            let skip_tip = if is_in_intro { "Skip Opening / Intro (S)" } else { "Skip Forward 85s (S)" };
            if make_tile("tile_skip_intro", 28.0, skip_tip, &|p, r, c| {
                let col = if is_in_intro { skin.accent_primary } else { c };
                p.text(r.center(), Align2::CENTER_CENTER, "⏭OP", FontId::proportional(9.5), col);
            }) {
                actions.skip_intro = true;
            }
        }

        // 4c. Repeat & Shuffle Mode (Ultra-wide only)
        if !is_song && is_ultra_wide {
            let is_rep_one = playlist.repeat_mode == crate::playlist::RepeatMode::RepeatTrack;
            let is_rep_active = playlist.repeat_mode == crate::playlist::RepeatMode::RepeatAll || is_rep_one;
            let rep_tip = match playlist.repeat_mode {
                crate::playlist::RepeatMode::RepeatAll => "Repeat: All (Ctrl+R)",
                crate::playlist::RepeatMode::RepeatTrack => "Repeat: One Track (Ctrl+R)",
                _ => "Repeat: Off (Ctrl+R)",
            };
            if make_tile("tile_repeat", 26.0, rep_tip, &|p, r, c| {
                let col = if is_rep_active { skin.accent_primary } else { c };
                Icons::draw_repeat(p, r, col, is_rep_one);
            }) {
                actions.cycle_repeat = true;
            }

            let is_shuf_active = playlist.is_shuffle();
            let shuf_tip = if is_shuf_active { "Shuffle: ON (Ctrl+S / S)" } else { "Shuffle: OFF (Ctrl+S / S)" };
            if make_tile("tile_shuffle", 26.0, shuf_tip, &|p, r, c| {
                let col = if is_shuf_active { skin.accent_primary } else { c };
                Icons::draw_shuffle(p, r, col);
            }) {
                actions.toggle_shuffle = true;
            }
        }

        // 5. Open / Eject (28px) (Ultra-wide only)
        if !is_song && is_ultra_wide && make_tile("tile_open", 28.0, "Open File / Disc / URL (Ctrl+O)", &|p, r, c| Icons::draw_eject(p, r, c)) {
            actions.open_file = true;
        }

        // 5b. Frame Capture / Snapshot (26px) (Ultra-wide only)
        if !is_song && is_ultra_wide && make_tile("tile_snapshot", 26.0, "Take Snapshot / Frame Capture (Ctrl+C / Ctrl+E)", &|p, r, c| Icons::draw_camera(p, r, c)) {
            actions.take_screenshot = true;
        }

        // 5c. Subtitle Quick Selector (28px) (Ultra-wide only)
        if !is_song && is_ultra_wide && !stats.subtitle_tracks.is_empty() {
            let sub_tip = if stats.subtitles_visible { "Cycle Subtitle Track / Language (Alt+H)" } else { "Subtitles: Off (Click to enable)" };
            if make_tile("tile_sub_quick", 28.0, sub_tip, &|p, r, c| {
                let col = if stats.subtitles_visible { skin.accent_primary } else { c };
                Icons::draw_subtitles(p, r, col);
            }) {
                actions.cycle_subtitle = true;
            }
        }

        // 6. Inset LCD Time & Format Stats Display Box (Pixel-Perfect Vortex Layout)
        {
            let current_str = crate::bookmark::format_time_hms(stats.time_pos);
            let total_str = crate::bookmark::format_time_hms(stats.duration);
            let time_str = if *show_remaining_time && stats.duration > 0.0 {
                let rem = (stats.duration - stats.time_pos).max(0.0);
                format!("-{}", crate::bookmark::format_time_hms(rem))
            } else {
                current_str
            };

            #[derive(Clone)]
            enum BadgeKind {
                Hwdec,
                VideoCodec,
                AudioCodec,
                AudioChannels,
                Hdr,
            }

            struct LcdBadge {
                text: String,
                kind: BadgeKind,
            }

            let mut lcd_badges: Vec<LcdBadge> = Vec::new();
            let is_hw = (!stats.hwdec_current.is_empty() && stats.hwdec_current != "no")
                || (!stats.video_codec.is_empty() && stats.hwdec_setting != "no" && !stats.hwdec_setting.is_empty())
                || (!is_song && stats.hwdec_setting != "no" && !stats.hwdec_setting.is_empty());

            if !stats.is_idle && !stats.file_path.is_empty() {
                // 1. S/W vs H/W Badge (Only on video playback)
                if !is_song {
                    lcd_badges.push(LcdBadge {
                        text: if is_hw { "H/W".to_string() } else { "S/W".to_string() },
                        kind: BadgeKind::Hwdec,
                    });
                }

                // 2. Video Codec Badge (Compact Vortex format: AVC1, HEVC, AV1, VP9, etc.)
                if !is_song && !stats.video_codec.is_empty() {
                    let raw_v = stats.video_codec.to_lowercase();
                    let v_formatted = if raw_v.contains("h264") || raw_v.contains("avc") {
                        "AVC1".to_string()
                    } else if raw_v.contains("hevc") || raw_v.contains("h265") {
                        "HEVC".to_string()
                    } else if raw_v.contains("av1") {
                        "AV1".to_string()
                    } else if raw_v.contains("vp9") {
                        "VP9".to_string()
                    } else if raw_v.contains("mpeg2") {
                        "MPEG2".to_string()
                    } else if raw_v.contains("vc1") {
                        "VC-1".to_string()
                    } else {
                        let first = stats.video_codec.split(|c: char| c.is_whitespace() || c == '/' || c == '(').next().unwrap_or(&stats.video_codec);
                        first.to_uppercase()
                    };
                    lcd_badges.push(LcdBadge { text: v_formatted, kind: BadgeKind::VideoCodec });
                }

                // 3. Audio Codec Badge (Compact Vortex format: TrueHD, DTS-HD, FLAC, AAC, Opus, etc.)
                if !stats.audio_codec.is_empty() {
                    let raw_a = stats.audio_codec.to_lowercase();
                    let a_formatted = if raw_a.contains("truehd") {
                        "TrueHD".to_string()
                    } else if raw_a.contains("dts-hd") || raw_a.contains("dtshd") {
                        "DTS-HD".to_string()
                    } else if raw_a.contains("dts") || raw_a.contains("dca") {
                        "DTS".to_string()
                    } else if raw_a.contains("eac3") {
                        "E-AC3".to_string()
                    } else if raw_a.contains("ac3") {
                        "AC3".to_string()
                    } else if raw_a.contains("flac") {
                        "FLAC".to_string()
                    } else if raw_a.contains("opus") {
                        "Opus".to_string()
                    } else if raw_a.contains("vorbis") {
                        "Vorbis".to_string()
                    } else if raw_a.contains("aac") {
                        "AAC".to_string()
                    } else if raw_a.contains("mp3") {
                        "MP3".to_string()
                    } else {
                        let first = stats.audio_codec.split(|c: char| c.is_whitespace() || c == '/' || c == '(').next().unwrap_or(&stats.audio_codec);
                        first.to_uppercase()
                    };
                    lcd_badges.push(LcdBadge { text: a_formatted, kind: BadgeKind::AudioCodec });
                }

                // 4. Audio Channels Badge (1.0, 2.0, 5.1, 7.1, etc.)
                // Must accurately represent the active media file's audio stream layout/channels (NOT output device / soundcard config)
                let active_track = stats.audio_tracks.iter().find(|t| t.is_selected || (t.id == stats.selected_audio_track && stats.selected_audio_track > 0));

                let raw_layout = if !stats.audio_channel_layout.is_empty() {
                    stats.audio_channel_layout.to_lowercase()
                } else if let Some(t) = active_track {
                    t.channels.to_lowercase()
                } else {
                    String::new()
                };

                let ch_str = if raw_layout.contains("7.1") {
                    Some("7.1".to_string())
                } else if raw_layout.contains("6.1") {
                    Some("6.1".to_string())
                } else if raw_layout.contains("5.1") {
                    Some("5.1".to_string())
                } else if raw_layout.contains("5.0") {
                    Some("5.0".to_string())
                } else if raw_layout.contains("4.1") {
                    Some("4.1".to_string())
                } else if raw_layout.contains("4.0") || raw_layout.contains("quad") {
                    Some("4.0".to_string())
                } else if raw_layout.contains("3.1") {
                    Some("3.1".to_string())
                } else if raw_layout.contains("3.0") {
                    Some("3.0".to_string())
                } else if raw_layout.contains("2.1") {
                    Some("2.1".to_string())
                } else if raw_layout.contains("2.0") || raw_layout.contains("stereo") {
                    Some("2.0".to_string())
                } else if raw_layout.contains("1.0") || raw_layout.contains("mono") {
                    Some("1.0".to_string())
                } else {
                    let ch_count = if stats.audio_channels > 0 {
                        stats.audio_channels
                    } else if let Some(t) = active_track {
                        t.channel_count
                    } else {
                        0
                    };

                    match ch_count {
                        1 => Some("1.0".to_string()),
                        2 => Some("2.0".to_string()),
                        3 => Some("2.1".to_string()),
                        4 => Some("4.0".to_string()),
                        5 => Some("5.0".to_string()),
                        6 => Some("5.1".to_string()),
                        7 => Some("6.1".to_string()),
                        8 => Some("7.1".to_string()),
                        n if n > 0 => Some(format!("{}.0", n)),
                        _ if !stats.audio_codec.is_empty() => Some("2.0".to_string()),
                        _ => None,
                    }
                };

                if let Some(ch) = ch_str {
                    lcd_badges.push(LcdBadge { text: ch, kind: BadgeKind::AudioChannels });
                }

                // 5. HDR Badge (Golden badge when HDR content is active)
                if stats.is_hdr {
                    lcd_badges.push(LcdBadge { text: "HDR".to_string(), kind: BadgeKind::Hdr });
                }
            }

            // Calculate accurate widths
            let cur_time_w = time_str.len() as f32 * 6.5;
            let total_time_w = total_str.len() as f32 * 6.5;
            let time_content_w = cur_time_w + total_time_w + 16.0;
            let time_prefix_w = (time_content_w + 12.0).max(138.0);

            // Dynamically fit badges into the available space before right_boundary_x
            let max_badge_space = (right_boundary_x - (left_x + time_prefix_w) - 16.0).max(0.0);
            let mut fitted_badges = Vec::new();
            let mut fitted_badges_w: f32 = 0.0;
            for b in lcd_badges {
                let bw = b.text.len() as f32 * 6.8 + 8.0 + 3.0;
                if fitted_badges_w + bw <= max_badge_space {
                    fitted_badges_w += bw;
                    fitted_badges.push(b);
                }
            }
            let lcd_w = time_prefix_w + fitted_badges_w + (if fitted_badges.is_empty() { 6.0 } else { 10.0 });

            let lcd_r = Rect::from_min_size(Pos2::new(left_x, row_top), Vec2::new(lcd_w, row_h));
            
            // Background container (pure visual background plate, does NOT consume clicks)
            painter.rect_filled(lcd_r, CornerRadius::ZERO, skin.bg_canvas);

            // Tightly scoped time click area (ONLY covers the time string)
            let time_click_rect = Rect::from_min_size(
                Pos2::new(lcd_r.left() + 4.0, row_top + 2.0),
                Vec2::new(time_content_w + 6.0, row_h - 4.0),
            );
            let time_resp = ui.interact(time_click_rect, ui.id().with("tile_time_display"), Sense::click());
            if time_resp.hovered() {
                let tip = if *show_remaining_time {
                    "Remaining Time (-) active (Click to switch to Elapsed Time)"
                } else {
                    "Elapsed Time active (Click to switch to Remaining Time -)"
                };
                hovered_tooltip = Some((time_click_rect, tip.to_string()));
                painter.rect_filled(time_click_rect, CornerRadius::same(2), skin.bg_btn_hover);
            }

            if time_resp.clicked() {
                *show_remaining_time = !*show_remaining_time;
            }

            // Draw Vortex Pixel-Perfect Cyan / Dimmed Time
            let time_y = lcd_r.center().y;
            
            // Current Time (Light Cyan #64B4FF)
            painter.text(
                Pos2::new(lcd_r.left() + 7.0, time_y),
                Align2::LEFT_CENTER,
                &time_str,
                FontId::monospace(11.0),
                Color32::from_rgb(100, 185, 255),
            );

            // Slash Separator (Muted Amber / Gray)
            let slash_x = lcd_r.left() + 7.0 + cur_time_w + 3.0;
            painter.text(
                Pos2::new(slash_x, time_y),
                Align2::LEFT_CENTER,
                "/",
                FontId::monospace(11.0),
                Color32::from_rgb(160, 130, 80),
            );

            // Total Duration (Muted Steel Cyan #87AFDC)
            painter.text(
                Pos2::new(slash_x + 9.0, time_y),
                Align2::LEFT_CENTER,
                &total_str,
                FontId::monospace(11.0),
                Color32::from_rgb(135, 175, 220),
            );

            // Draw Badges (S/W, AVC1, TrueHD, 5.1, HDR)
            let mut badge_x = lcd_r.left() + time_prefix_w;

            for (idx, b) in fitted_badges.iter().enumerate() {
                let bw = b.text.len() as f32 * 6.8 + 8.0;
                let b_rect = Rect::from_center_size(
                    Pos2::new(badge_x + bw / 2.0, time_y),
                    Vec2::new(bw, 15.0),
                );

                let (b_resp, bg_col, stroke_col, text_col, tip_str) = match b.kind {
                    BadgeKind::Hwdec => {
                        let r = ui.interact(b_rect, ui.id().with("badge_hwdec"), Sense::click());
                        let active_mode = if !stats.hwdec_current.is_empty() && stats.hwdec_current != "no" {
                            &stats.hwdec_current
                        } else if !stats.hwdec_setting.is_empty() && stats.hwdec_setting != "no" {
                            &stats.hwdec_setting
                        } else {
                            "Software"
                        };
                        let tip = if is_hw {
                            format!("Hardware Decoding Active ({})\nClick to switch to Software Decoding [S/W]", active_mode)
                        } else {
                            "Software Decoding Active [S/W]\nClick to enable Hardware Acceleration [H/W]".to_string()
                        };
                        let bg = if is_hw {
                            if r.hovered() { Color32::from_rgb(26, 46, 76) } else { Color32::from_rgb(18, 32, 54) }
                        } else {
                            if r.hovered() { Color32::from_rgb(38, 42, 54) } else { Color32::from_rgb(24, 26, 34) }
                        };
                        let stroke = if is_hw { Color32::from_rgb(60, 140, 240) } else { Color32::from_rgb(50, 54, 68) };
                        let text = if is_hw { Color32::from_rgb(90, 200, 255) } else { Color32::from_rgb(160, 165, 180) };
                        (r, bg, stroke, text, tip)
                    }
                    BadgeKind::AudioChannels => {
                        let r = ui.interact(b_rect, ui.id().with("badge_audio_ch"), Sense::click());
                        let tip = if stats.audio_out_channels > 0 && stats.audio_out_channels != stats.audio_channels && stats.audio_channels > 0 {
                            format!("Audio Stream: {} ({})\nOutput Device: {} ch\nClick to configure Channels", b.text, stats.audio_channel_layout, stats.audio_out_channels)
                        } else {
                            format!("Audio Stream: {}\nClick to configure Channels", b.text)
                        };
                        let bg = if r.hovered() { Color32::from_rgb(42, 45, 58) } else { Color32::from_rgb(26, 28, 36) };
                        let stroke = Color32::from_rgb(45, 48, 58);
                        let text = if r.hovered() { skin.accent_primary } else { Color32::from_rgb(145, 150, 165) };
                        (r, bg, stroke, text, tip)
                    }
                    BadgeKind::Hdr => {
                        let r = ui.interact(b_rect, ui.id().with("badge_hdr"), Sense::click());
                        let tip = format!("HDR Active ({})\nClick to cycle Tone Mapping", stats.hdr_format);
                        let bg = if r.hovered() { Color32::from_rgb(70, 52, 15) } else { Color32::from_rgb(48, 36, 10) };
                        let stroke = if r.hovered() { Color32::from_rgb(240, 185, 40) } else { Color32::from_rgb(170, 130, 25) };
                        let text = if r.hovered() { Color32::WHITE } else { Color32::from_rgb(255, 210, 60) };
                        (r, bg, stroke, text, tip)
                    }
                    BadgeKind::VideoCodec => {
                        let r = ui.interact(b_rect, ui.id().with(format!("badge_vcodec_{}", idx)), Sense::click());
                        let tip = format!("Video Codec: {} (Click for Media Info)", stats.video_codec);
                        let bg = if r.hovered() { Color32::from_rgb(36, 40, 52) } else { Color32::from_rgb(26, 28, 36) };
                        let stroke = if r.hovered() { Color32::from_rgb(60, 90, 140) } else { Color32::from_rgb(45, 48, 58) };
                        let text = if r.hovered() { Color32::WHITE } else { Color32::from_rgb(145, 150, 165) };
                        (r, bg, stroke, text, tip)
                    }
                    BadgeKind::AudioCodec => {
                        let r = ui.interact(b_rect, ui.id().with(format!("badge_acodec_{}", idx)), Sense::click());
                        let is_exclusive = stats.is_wasapi_exclusive;
                        let tip = if is_exclusive {
                            format!("Audio: {} [WASAPI EXCLUSIVE: ON]\nClick to switch to Shared Mode\nRight-click for Media Info", stats.audio_codec)
                        } else {
                            format!("Audio: {} [Shared Mode]\nClick for Bit-Perfect WASAPI Exclusive\nRight-click for Media Info", stats.audio_codec)
                        };
                        let (bg, stroke, text) = if is_exclusive {
                            (Color32::from_rgb(52, 38, 12), Color32::from_rgb(180, 130, 24), skin.accent_primary)
                        } else if r.hovered() {
                            (Color32::from_rgb(36, 40, 52), Color32::from_rgb(60, 90, 140), Color32::WHITE)
                        } else {
                            (Color32::from_rgb(26, 28, 36), Color32::from_rgb(45, 48, 58), Color32::from_rgb(145, 150, 165))
                        };
                        (r, bg, stroke, text, tip)
                    }
                };

                if b_resp.hovered() {
                    hovered_tooltip = Some((b_rect, tip_str));
                }

                painter.rect_filled(b_rect, CornerRadius::same(2), bg_col);
                painter.rect_stroke(b_rect, CornerRadius::same(2), Stroke::new(1.0, stroke_col), StrokeKind::Inside);
                painter.text(b_rect.center(), Align2::CENTER_CENTER, &b.text, FontId::monospace(9.5), text_col);

                match b.kind {
                    BadgeKind::Hwdec if b_resp.clicked() => actions.toggle_hwdec = true,
                    BadgeKind::AudioChannels if b_resp.clicked() => {
                        actions.open_audio_channels = true;
                        actions.open_audio_channels_pos = Some(Pos2::new(b_rect.center().x, b_rect.top()));
                    }
                    BadgeKind::Hdr if b_resp.clicked() => actions.cycle_hdr_tone_mapping = true,
                    BadgeKind::AudioCodec if b_resp.clicked() => actions.toggle_wasapi_exclusive = true,
                    BadgeKind::AudioCodec if b_resp.secondary_clicked() => actions.open_mediainfo = true,
                    BadgeKind::VideoCodec if b_resp.clicked() => actions.open_mediainfo = true,
                    _ => {}
                }

                badge_x += bw + 3.0;
            }

            left_x += lcd_w;
            draw_v_divider(painter, left_x);
        }

        // 7. Right Side Control Group (Volume, Speed, Settings, Playlist)
        let mut right_x = total_rect.right();

        // Helper for right-aligned 28px buttons (strictly prevents colliding with left_x)
        let mut make_right_tile = |id_str: &str, w: f32, tip_text: &str, draw_fn: &dyn Fn(&Painter, Rect, Color32)| -> bool {
            if right_x - w < left_x + 8.0 {
                return false;
            }
            right_x -= w;
            draw_v_divider(painter, right_x);
            let tile_r = Rect::from_min_size(Pos2::new(right_x, row_top), Vec2::new(w, row_h));
            let resp = ui.interact(tile_r, ui.id().with(id_str), Sense::click());

            if resp.hovered() {
                painter.rect_filled(tile_r, CornerRadius::ZERO, Color32::from_rgb(32, 34, 42));
                hovered_tooltip = Some((tile_r, tip_text.to_string()));
            }

            let icon_col = if resp.hovered() { Color32::WHITE } else { Color32::from_rgb(170, 175, 188) };
            draw_fn(painter, tile_r, icon_col);

            resp.clicked()
        };

        // 1. Playlist Drawer ☰ (28px) — Rightmost button with vertical divider on left
        let is_pl_open = is_playlist_open;
        if make_right_tile("tile_playlist", 28.0, "Toggle Playlist (F6 / F8)", &|p, r, c| {
            let col = if is_pl_open { skin.accent_primary } else { c };
            Icons::draw_hamburger(p, r, col);
        }) {
            actions.toggle_playlist = true;
        }

        // 2, 3, 4: Secondary Tools (Always show on wide playback when space permits)
        if is_wide {
            if make_right_tile("tile_settings", 26.0, "Preferences & Configuration (F5)", &|p, r, c| Icons::draw_settings(p, r, c)) {
                actions.toggle_preferences = true;
            }

            if make_right_tile("tile_control_panel", 26.0, "Control Panel & Adjustments (F6 / F7)", &|p, r, c| Icons::draw_document_list(p, r, c)) {
                actions.toggle_control_panel = true;
            }

            if make_right_tile("tile_quick_search", 26.0, "Bookmarks & Scene Search (P / B)", &|p, r, c| Icons::draw_search_list(p, r, c)) {
                actions.toggle_bookmark_overlay = true;
            }

            if make_right_tile("tile_lyrics", 26.0, "Synchronized Lyrics / Karaoke (Ctrl+Shift+L)", &|p, r, c| {
                p.text(r.center(), Align2::CENTER_CENTER, "🎙", FontId::proportional(11.5), c);
            }) {
                actions.toggle_lyrics = true;
            }
        }

        // Speed Multiplier Pill (Video mode)
        if !is_song && is_wide && right_x - 42.0 >= left_x + 8.0 {
            let spd_w = 42.0;
            right_x -= spd_w;
            draw_v_divider(painter, right_x);

            let spd_r = Rect::from_min_size(Pos2::new(right_x, row_top), Vec2::new(spd_w, row_h));
            let pill_h = 18.0;
            let pill_r = Rect::from_center_size(spd_r.center(), Vec2::new(36.0, pill_h));
            let spd_resp = ui.interact(pill_r, ui.id().with("bottom_speed_pill"), Sense::click());

            let is_non_standard = (stats.speed - 1.0).abs() > 0.05;
            let bg_col = if is_non_standard {
                skin.accent_primary.gamma_multiply(0.25)
            } else if spd_resp.hovered() {
                skin.bg_btn_hover
            } else {
                skin.bg_btn
            };

            painter.rect_filled(pill_r, CornerRadius::same(3), bg_col);
            painter.rect_stroke(
                pill_r,
                CornerRadius::same(3),
                Stroke::new(1.0, if is_non_standard { skin.accent_primary } else { skin.border_dark }),
                StrokeKind::Inside,
            );

            let text_col = if is_non_standard { skin.accent_primary } else if spd_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(175, 180, 195) };
            let speed_str = format!("{:.1}x", stats.speed);
            painter.text(pill_r.center(), Align2::CENTER_CENTER, speed_str, FontId::monospace(10.0), text_col);

            if spd_resp.hovered() {
                hovered_tooltip = Some((pill_r, format!("Speed: {:.2}x (Click to select speed, Right-click to reset)", stats.speed)));
            }

            if spd_resp.clicked() {
                actions.open_speed_menu = true;
                actions.open_speed_menu_pos = Some(Pos2::new(pill_r.center().x, pill_r.top()));
            }
            if spd_resp.secondary_clicked() {
                actions.reset_speed = true;
            }
        }

        // Volume Slider & Mute Icon Box (Adaptive 28px to 96px, collision-proof)
        let can_fit_large_vol = right_x - 96.0 >= left_x + 8.0;
        let can_fit_med_vol = right_x - 68.0 >= left_x + 8.0;
        let can_fit_mini_vol = right_x - 28.0 >= left_x + 8.0;

        if can_fit_mini_vol {
            let vol_w = if is_wide && can_fit_large_vol {
                96.0
            } else if can_fit_med_vol {
                68.0
            } else {
                28.0
            };
            right_x -= vol_w;
            draw_v_divider(painter, right_x);

            let vol_r = Rect::from_min_size(Pos2::new(right_x, row_top), Vec2::new(vol_w, row_h));
            
            // Speaker Icon
            let spk_r = if vol_w > 32.0 {
                Rect::from_min_size(Pos2::new(vol_r.left() + 4.0, vol_r.center().y - 8.0), Vec2::splat(16.0))
            } else {
                Rect::from_center_size(vol_r.center(), Vec2::splat(16.0))
            };
            let spk_resp = ui.interact(spk_r, ui.id().with("vol_mute_btn"), Sense::click());
            let spk_col = if spk_resp.hovered() { Color32::WHITE } else { Color32::from_rgb(170, 175, 188) };
            Icons::draw_volume(painter, spk_r, spk_col, stats.is_muted, stats.volume);

            if spk_resp.hovered() {
                let tip = if stats.is_muted { "Unmute Audio (M)" } else { "Mute Audio (M) / Scroll to change volume" };
                hovered_tooltip = Some((spk_r, tip.to_string()));
            }

            if spk_resp.clicked() {
                if let Some(p) = player {
                    p.toggle_mute();
                }
            }

            let mut display_vol_pct = if stats.is_muted { 0.0 } else { (stats.volume / 100.0).clamp(0.0, 1.0) as f32 };

            // Only draw slider track if volume width > 32.0
            if vol_w > 32.0 {
                let vbar_x = vol_r.left() + 26.0;
                let vbar_w = vol_w - 36.0;
                let vbar_r = Rect::from_min_size(
                    Pos2::new(vbar_x, vol_r.center().y - 1.25),
                    Vec2::new(vbar_w, 2.5),
                );

                let v_resp = ui.interact(
                    Rect::from_center_size(vbar_r.center(), Vec2::new(vbar_w + 6.0, 22.0)),
                    ui.id().with("vol_slider_interactive"),
                    Sense::click_and_drag(),
                );

                let is_vol_scrubbing = v_resp.clicked() || v_resp.dragged();

                if is_vol_scrubbing {
                    ui.ctx().request_repaint();
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos().or_else(|| i.pointer.latest_pos())) {
                        let frac = ((pos.x - vbar_r.left()) / vbar_w).clamp(0.0, 1.0);
                        display_vol_pct = frac;
                        let target_vol = frac as f64 * 100.0;
                        if let Some(p) = player {
                            p.set_volume(target_vol);
                        }
                    }
                }

                if v_resp.hovered() || is_vol_scrubbing {
                    hovered_tooltip = Some((vbar_r, format!("Volume: {:.0}%", display_vol_pct * 100.0)));
                }

                // Groove & Fill
                painter.rect_filled(vbar_r, CornerRadius::ZERO, skin.border_dark);

                let fill_w = vbar_w * display_vol_pct;
                if fill_w > 0.0 {
                    let fill_r = Rect::from_min_size(vbar_r.min, Vec2::new(fill_w, 2.5));
                    painter.rect_filled(fill_r, CornerRadius::ZERO, skin.accent_primary);
                }

                // Volume Knob
                let vknob_x = vbar_r.left() + fill_w;
                painter.circle_filled(Pos2::new(vknob_x, vbar_r.center().y), 2.2, Color32::WHITE);
            }

            // Mouse wheel volume adjust when hovering anywhere on the volume tile
            if ui.rect_contains_pointer(vol_r) {
                let scroll_y = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll_y.abs() > 0.1 {
                    ui.ctx().request_repaint();
                    let step = if scroll_y > 0.0 { 5.0 } else { -5.0 };
                    let new_vol = (stats.volume + step).clamp(0.0, 100.0);
                    if let Some(p) = player {
                        p.set_volume(new_vol);
                    }
                }
            }
        }

        if let Some((target_r, ref tip)) = hovered_tooltip {
            actions.hover_tooltip = Some((Pos2::new(target_r.center().x, target_r.top()), tip.clone()));
        }

        actions
    }
}
