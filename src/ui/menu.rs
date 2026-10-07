use super::theme::{SkinColors, VortexTheme};
use crate::bookmark::{format_time, BookmarkManager};
use crate::config::{AppConfig, ThemeMode};
use crate::engine::audio_dsp::VORTEX_EQ_PRESETS;
use crate::engine::{MediaStats, Player, VideoEffectsConfig};
use crate::playlist::Playlist;
use eframe::egui::{
    self, epaint::PathShape, Align2, Color32, CornerRadius, FontId, InnerResponse, Margin, Pos2,
    Rect, Response, RichText, Sense, Stroke, Vec2,
};
use std::path::PathBuf;

pub struct VortexMenu;

pub struct MenuActions {
    pub close_menu: bool,
    pub toggle_fullscreen: bool,
    pub exit_fullscreen: bool,
    pub open_file: bool,
    pub open_folder: bool,
    pub open_url: bool,
    pub close_file: bool,
    pub load_subtitle: bool,
    pub toggle_playlist: bool,
    pub toggle_control_panel: bool,
    pub toggle_preferences: bool,
    pub toggle_bookmark_overlay: bool,
    pub toggle_media_info: bool,
    pub open_file_info: bool,
    pub toggle_stereo_3d: bool,
    pub toggle_karaoke: bool,
    pub toggle_subtitle_studio: bool,
    pub toggle_screen_capture: bool,
    pub toggle_iptv_guide: bool,
    pub toggle_logo_watermark: bool,
    pub toggle_disc_nav: bool,
    pub toggle_cd_ripper: bool,
    pub toggle_bda_tuner: bool,
    pub toggle_bdj_studio: bool,
    pub toggle_vst_winamp: bool,
    pub toggle_detached_windows: bool,
    pub toggle_discord_rpc: bool,
    pub toggle_web_remote: bool,
    pub toggle_scrobbler: bool,
    pub toggle_media_server: bool,
    pub toggle_ambilight: bool,
    pub toggle_cast_renderer: bool,
    pub toggle_transcoder: bool,
    pub toggle_zoom_magnifier: bool,
    pub toggle_radio_directory: bool,
    pub toggle_video_puzzle: bool,
    pub toggle_vr_360: bool,
    pub toggle_damaged_file_repair: bool,
    pub toggle_binaural_crossfeed: bool,
    pub toggle_video_wall: bool,
    pub toggle_chapter_marker: bool,
    pub toggle_audio_compressor: bool,
    pub toggle_video_crop: bool,
    pub toggle_goto_frame: bool,
    pub toggle_playback_history: bool,
    pub toggle_media_tag_editor: bool,
    pub toggle_deinterlace: bool,
    pub toggle_ab_repeat: bool,
    pub toggle_boss_key: bool,
    pub toggle_motion_interpolation: bool,
    pub toggle_color_lut: bool,
    pub toggle_subtitle_translator: bool,
    pub toggle_ai_subtitle: bool,
    pub toggle_ai_upscaling: bool,
    pub toggle_ai_audio: bool,
    pub toggle_loudness_radar: bool,
    pub toggle_frame_dumper: bool,
    pub toggle_color_blindness: bool,
    pub toggle_cinema_matte: bool,
    pub toggle_motion_vector_inspector: bool,
    pub toggle_rich_bookmark_notes: bool,
    pub toggle_pitch_formant: bool,
    pub toggle_auto_skip_dialog: bool,
    pub skip_intro: bool,
    pub show_about: bool,
    pub toggle_record_dialog: bool,
    pub toggle_gif_maker: bool,
    pub toggle_device_capture: bool,
    pub toggle_subtitle_lookup: bool,
    pub toggle_shader_studio: bool,
    pub toggle_broadcast: bool,
    pub toggle_contact_sheet: bool,
    pub toggle_jump_time: bool,
    pub toggle_pip: bool,
    pub minimize_window: bool,
    pub maximize_window: bool,

    pub take_screenshot: bool,
    pub play_path: Option<PathBuf>,
    pub sub_sync_changed: bool,
    pub resize_window_factor: Option<f32>,
    pub center_window: bool,
    pub open_subtitle_preferences: bool,
}







#[inline]
fn safe_icon(icon: &str) -> &str {
    // Only permit crisp, universally supported glyphs; strip all emojis, squares, and symbols
    // that render as missing glyph tofu (□ / ▭ / ■ / ⬛) in standard UI fonts.
    match icon {
        "✓" | "▶" | "⏸" | "•" | "★" | "☆" | " " => icon,
        _ => "",
    }
}

/// Reusable Standard Context Menu Item
fn menu_item(
    ui: &mut egui::Ui,
    skin: &SkinColors,
    icon: &str,
    label: &str,
    shortcut: &str,
    is_checked: bool,
    is_disabled: bool,
) -> Response {
    let icon = safe_icon(icon);
    let w = ui.available_width();
    let row_h = 19.0;
    let (_, resp) = ui.allocate_exact_size(
        Vec2::new(w, row_h),
        if is_disabled { Sense::hover() } else { Sense::click() },
    );
    let is_hovered = resp.hovered();

    // When hovering a regular menu item, clear any active child submenu at this menu level
    if is_hovered {
        let menu_state_id = ui.id().with("active_submenu_id");
        ui.ctx().data_mut(|d| d.insert_temp(menu_state_id, None::<egui::Id>));
    }

    let painter = ui.painter();
    let rect = resp.rect;

    // 1. Subtle Accent-Tinted Rounded Hover State
    if is_hovered && !is_disabled {
        let hover_rect = rect.shrink2(Vec2::new(2.0, 1.0));
        painter.rect_filled(
            hover_rect,
            CornerRadius::ZERO,
            skin.bg_btn_hover,
        );
        painter.rect_stroke(
            hover_rect,
            CornerRadius::ZERO,
            Stroke::new(1.0, skin.border_light),
            eframe::egui::StrokeKind::Inside,
        );
    } else if is_checked {
        let sel_rect = rect.shrink2(Vec2::new(2.0, 1.0));
        painter.rect_filled(
            sel_rect,
            CornerRadius::ZERO,
            skin.bg_btn_active,
        );
    }

    let cy = rect.center().y;

    // 2. Icon / Checkmark Area (Left: 4px to 18px)
    if is_checked {
        let chk_col = if is_hovered { Color32::WHITE } else { skin.accent_primary };
        crate::ui::Icons::draw_check(painter, Pos2::new(rect.left() + 10.0, cy), 10.0, chk_col);
    } else if !icon.is_empty() {
        let icon_col = if is_disabled {
            Color32::from_rgb(90, 95, 110)
        } else if is_hovered {
            Color32::WHITE
        } else {
            Color32::from_rgb(160, 165, 185)
        };
        painter.text(
            Pos2::new(rect.left() + 10.0, cy),
            Align2::CENTER_CENTER,
            icon,
            FontId::proportional(11.0),
            icon_col,
        );
    }

    // 3. Label (Left: 22px)
    let label_col = if is_disabled {
        Color32::from_rgb(100, 105, 120)
    } else if is_hovered {
        Color32::WHITE
    } else if is_checked {
        Color32::from_rgb(245, 248, 255)
    } else {
        skin.text_primary
    };

    // 3. Label & Shortcut collision-proof layout
    let shortcut_reserved_w = if !shortcut.is_empty() {
        let sc_galley = painter.layout_no_wrap(shortcut.to_string(), FontId::proportional(10.5), Color32::WHITE);
        sc_galley.size().x + 16.0
    } else {
        8.0
    };

    let label_avail_w = (rect.width() - 26.0 - shortcut_reserved_w).max(20.0);
    let full_galley = painter.layout_no_wrap(label.to_string(), FontId::proportional(11.5), label_col);
    let display_galley = if full_galley.size().x > label_avail_w {
        let mut truncated = label.to_string();
        while !truncated.is_empty() {
            truncated.pop();
            let test_str = format!("{}…", truncated.trim_end());
            let test_galley = painter.layout_no_wrap(test_str.clone(), FontId::proportional(11.5), label_col);
            if test_galley.size().x <= label_avail_w {
                break;
            }
        }
        let final_str = if truncated.is_empty() { label.to_string() } else { format!("{}…", truncated.trim_end()) };
        painter.layout_no_wrap(final_str, FontId::proportional(11.5), label_col)
    } else {
        full_galley
    };

    let label_pos = Pos2::new(rect.left() + 22.0, cy - display_galley.size().y * 0.5);
    painter.galley(label_pos, display_galley, label_col);

    // 4. Shortcut Text (Right-aligned before right edge)
    if !shortcut.is_empty() {
        let sc_col = if is_disabled {
            Color32::from_rgb(80, 85, 95)
        } else if is_hovered {
            Color32::from_rgb(200, 215, 245)
        } else {
            skin.text_secondary
        };

        painter.text(
            Pos2::new(rect.right() - 6.0, cy),
            Align2::RIGHT_CENTER,
            shortcut,
            FontId::proportional(10.5),
            sc_col,
        );
    }

    resp
}

/// Reusable Cascading Submenu Item with persistent active state and zero-flicker hover
fn submenu_item<R>(
    ui: &mut egui::Ui,
    skin: &SkinColors,
    icon: &str,
    label: &str,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> (Response, Option<InnerResponse<R>>) {
    let icon = safe_icon(icon);
    let w = ui.available_width();
    let row_h = 19.0;
    let menu_state_id = ui.id().with("active_submenu_id");
    let item_id = ui.make_persistent_id(label);

    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, row_h), Sense::click_and_drag());

    let mut current_open = ui.ctx().data(|d| d.get_temp::<Option<egui::Id>>(menu_state_id)).flatten();

    if resp.hovered() || resp.clicked() {
        if current_open != Some(item_id) {
            ui.ctx().data_mut(|d| d.insert_temp(menu_state_id, Some(item_id)));
            current_open = Some(item_id);
        }
    }

    let is_open = current_open == Some(item_id);
    let is_hovered = resp.hovered() || is_open;

    let painter = ui.painter();

    // 1. Hover / Open Highlight
    if is_open {
        let sel_rect = rect.shrink2(Vec2::new(2.0, 1.0));
        painter.rect_filled(
            sel_rect,
            CornerRadius::ZERO,
            skin.bg_btn_active,
        );
        painter.rect_stroke(
            sel_rect,
            CornerRadius::ZERO,
            Stroke::new(1.0, skin.border_light),
            eframe::egui::StrokeKind::Inside,
        );
    } else if is_hovered {
        let hover_rect = rect.shrink2(Vec2::new(2.0, 1.0));
        painter.rect_filled(
            hover_rect,
            CornerRadius::ZERO,
            skin.bg_btn_hover,
        );
        painter.rect_stroke(
            hover_rect,
            CornerRadius::ZERO,
            Stroke::new(1.0, skin.border_light),
            eframe::egui::StrokeKind::Inside,
        );
    }

    let cy = rect.center().y;

    // 2. Icon Area
    if !icon.is_empty() {
        let icon_col = if is_hovered {
            Color32::WHITE
        } else {
            Color32::from_rgb(160, 165, 185)
        };
        painter.text(
            Pos2::new(rect.left() + 10.0, cy),
            Align2::CENTER_CENTER,
            icon,
            FontId::proportional(11.0),
            icon_col,
        );
    }

    // 3. Label
    let label_col = if is_hovered {
        Color32::WHITE
    } else {
        skin.text_primary
    };

    painter.text(
        Pos2::new(rect.left() + 22.0, cy),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.5),
        label_col,
    );

    // 4. Chevron Indicator (Aligned right)
    let chevron_col = if is_hovered {
        Color32::WHITE
    } else {
        Color32::from_rgb(140, 145, 165)
    };

    let arrow_pos = Pos2::new(rect.right() - 8.0, cy);
    let p1 = arrow_pos + Vec2::new(-2.5, -3.5);
    let p2 = arrow_pos + Vec2::new(2.0, 0.0);
    let p3 = arrow_pos + Vec2::new(-2.5, 3.5);
    painter.add(PathShape::convex_polygon(
        vec![p1, p2, p3],
        chevron_col,
        Stroke::NONE,
    ));

    // 5. Render Cascading Submenu Popup Area when open
    let mut inner_res = None;
    if is_open {
        let win_size = ui.ctx().input(|i| i.raw.screen_rect.or(i.viewport().inner_rect)).map(|r| r.size()).unwrap_or(Vec2::new(1280.0, 720.0));
        let last_rect: Option<Rect> = ui.ctx().data(|d| d.get_temp(item_id.with("submenu_rect")));
        let sub_h = last_rect.map(|r| r.height()).unwrap_or(220.0).max(40.0);

        // Submenu cascading direction: if right edge overflows screen, flip to the left side
        let sub_w = last_rect.map(|r| r.width()).unwrap_or(260.0).max(180.0);
        let open_right = rect.right() + sub_w <= win_size.x - 6.0;
        let (popup_x, pivot) = if open_right {
            (rect.right() - 2.0, egui::Align2::LEFT_TOP)
        } else {
            (rect.left() + 2.0, egui::Align2::RIGHT_TOP)
        };

        let available_sub_window_h = (win_size.y - 12.0).max(100.0);
        let sub_fits_in_window = available_sub_window_h >= sub_h;

        let popup_y = if !sub_fits_in_window {
            6.0
        } else if rect.top() - 4.0 + sub_h > win_size.y - 6.0 {
            (win_size.y - 6.0 - sub_h).max(6.0)
        } else {
            (rect.top() - 4.0).max(6.0)
        };

        let max_menu_h = (win_size.y - popup_y - 6.0).max(120.0);
        let popup_pos = Pos2::new(popup_x, popup_y);

        let area_resp = egui::Area::new(item_id.with("submenu_cascade_area"))
            .order(egui::Order::Tooltip)
            .fixed_pos(popup_pos)
            .pivot(pivot)
            .show(ui.ctx(), |ui| {
                ui.style_mut().animation_time = 0.0;
                egui::Frame::new()
                    .fill(skin.bg_panel)
                    .stroke(Stroke::new(1.0, skin.border_dark))
                    .corner_radius(CornerRadius::ZERO)
                    .shadow(egui::Shadow {
                        offset: [0, 8],
                        blur: 24,
                        spread: 2,
                        color: Color32::from_black_alpha(190),
                    })
                    .inner_margin(Margin::symmetric(3, 2))
                    .show(ui, |ui| {
                        ui.set_min_width(220.0);
                        ui.set_max_width(520.0);
                        ui.spacing_mut().item_spacing = Vec2::new(0.0, 0.5);
                        ui.spacing_mut().button_padding = Vec2::new(4.0, 1.0);
                        ui.spacing_mut().menu_margin = Margin::symmetric(3, 2);
                        if sub_fits_in_window {
                            content(ui)
                        } else {
                            egui::ScrollArea::vertical()
                                .max_height(max_menu_h)
                                .auto_shrink([true, true])
                                .show(ui, |ui| {
                                    content(ui)
                                })
                                .inner
                        }
                    })
            });

        let actual_rect = area_resp.response.rect;

        // Retrieve any active child / grandchild descendant rects
        let area_menu_state_id = area_resp.response.id.with("active_submenu_id");
        let child_open_id: Option<egui::Id> = ui.ctx().data(|d| d.get_temp(area_menu_state_id)).flatten();
        let child_active_rect: Option<Rect> = if let Some(child_id) = child_open_id {
            ui.ctx().data(|d| d.get_temp(child_id.with("submenu_bridge_rect")).or_else(|| d.get_temp(child_id.with("submenu_rect"))))
        } else {
            None
        };

        let total_active_rect = if let Some(child_r) = child_active_rect {
            actual_rect.union(child_r)
        } else {
            actual_rect
        };

        ui.ctx().data_mut(|d| {
            d.insert_temp(item_id.with("submenu_rect"), actual_rect);
            d.insert_temp(item_id.with("submenu_bridge_rect"), total_active_rect);
            d.insert_temp(egui::Id::new("latest_active_submenu_rect"), Some(total_active_rect));
            let list: &mut Vec<Rect> = d.get_temp_mut_or_default(egui::Id::new("all_active_submenu_rects"));
            list.push(actual_rect);
        });

        // Submenu hover retention & bridge keeping it open seamlessly across all child levels
        if let Some(mouse_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
            let bridge_rect = rect.union(total_active_rect).expand2(Vec2::new(12.0, 8.0));
            if bridge_rect.contains(mouse_pos) {
                ui.ctx().data_mut(|d| d.insert_temp(menu_state_id, Some(item_id)));
            }
        }

        inner_res = Some(InnerResponse {
            inner: area_resp.inner.inner,
            response: area_resp.response,
        });
    }

    (resp, inner_res)
}

/// Reusable Structured Two-Line Audio Track Item
#[allow(dead_code)]
fn audio_track_item(
    ui: &mut egui::Ui,
    _skin: &SkinColors,
    title: &str,
    metadata_desc: &str,
    is_active: bool,
) -> Response {
    let w = ui.available_width();
    let row_h = 36.0;

    let btn = egui::Button::new("")
        .min_size(Vec2::new(w, row_h))
        .fill(Color32::TRANSPARENT)
        .frame(false);

    let resp = ui.add(btn);
    let is_hovered = resp.hovered();

    if is_hovered {
        let menu_state_id = ui.id().with("active_submenu_id");
        ui.ctx().data_mut(|d| d.insert_temp(menu_state_id, None::<egui::Id>));
    }
    let painter = ui.painter();
    let rect = resp.rect;

    // Background Highlight
    if is_active {
        let sel_rect = rect.shrink2(Vec2::new(3.0, 1.0));
        painter.rect_filled(
            sel_rect,
            CornerRadius::ZERO,
            if is_hovered { Color32::from_rgb(42, 60, 96) } else { Color32::from_rgb(32, 45, 72) },
        );
        painter.rect_stroke(
            sel_rect,
            CornerRadius::ZERO,
            Stroke::new(1.0, Color32::from_rgb(60, 90, 145)),
            eframe::egui::StrokeKind::Inside,
        );
    } else if is_hovered {
        let hover_rect = rect.shrink2(Vec2::new(3.0, 1.0));
        painter.rect_filled(
            hover_rect,
            CornerRadius::ZERO,
            Color32::from_rgb(34, 42, 60),
        );
    }

    let cy = rect.center().y;

    // Checkmark Area (Left)
    if is_active {
        crate::ui::Icons::draw_check(painter, Pos2::new(rect.left() + 14.0, cy), 11.0, Color32::WHITE);
    }

    // Primary Text (Track Name / Index)
    painter.text(
        Pos2::new(rect.left() + 28.0, cy - 7.0),
        Align2::LEFT_CENTER,
        title,
        FontId::proportional(12.0),
        if is_hovered || is_active { Color32::WHITE } else { Color32::from_rgb(235, 240, 252) },
    );

    // Secondary Text (Technical Specs: FLAC · 2.0 Stereo · 44.1 kHz · 481 kbps)
    painter.text(
        Pos2::new(rect.left() + 28.0, cy + 8.0),
        Align2::LEFT_CENTER,
        metadata_desc,
        FontId::proportional(10.5),
        if is_active { Color32::from_rgb(160, 200, 255) } else { Color32::from_rgb(130, 135, 150) },
    );

    resp
}

/// Reusable Context Menu Separator with Inset Padding
fn menu_separator(ui: &mut egui::Ui) {
    let w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, 4.0), Sense::hover());
    let cy = rect.center().y;
    let inset = 8.0;
    ui.painter().line_segment(
        [
            Pos2::new(rect.left() + inset, cy),
            Pos2::new(rect.right() - inset, cy),
        ],
        Stroke::new(1.0, ui.visuals().window_stroke.color),
    );
}

impl VortexMenu {
    pub fn render(
        ui: &mut egui::Ui,
        player: Option<&Player>,
        stats: &MediaStats,
        playlist: &mut Playlist,
        config: &mut AppConfig,
        bookmark_mgr: &mut BookmarkManager,
    ) -> MenuActions {
        let mut actions = MenuActions {
            close_menu: false,
            toggle_fullscreen: false,
            exit_fullscreen: false,
            open_file: false,
            open_folder: false,
            open_url: false,
            close_file: false,
            load_subtitle: false,
            toggle_playlist: false,
            toggle_control_panel: false,
            toggle_preferences: false,
            toggle_bookmark_overlay: false,
            toggle_media_info: false,
            open_file_info: false,
            toggle_stereo_3d: false,
            toggle_karaoke: false,
            toggle_subtitle_studio: false,
            toggle_screen_capture: false,
            toggle_iptv_guide: false,
            toggle_logo_watermark: false,
            toggle_disc_nav: false,
            toggle_cd_ripper: false,
            toggle_bda_tuner: false,
            toggle_bdj_studio: false,
            toggle_vst_winamp: false,
            toggle_detached_windows: false,
            toggle_discord_rpc: false,
            toggle_web_remote: false,
            toggle_scrobbler: false,
            toggle_media_server: false,
            toggle_ambilight: false,
            toggle_cast_renderer: false,
            toggle_transcoder: false,
            toggle_zoom_magnifier: false,
            toggle_radio_directory: false,
            toggle_video_puzzle: false,
            toggle_vr_360: false,
            toggle_damaged_file_repair: false,
            toggle_binaural_crossfeed: false,
            toggle_video_wall: false,
            toggle_chapter_marker: false,
            toggle_audio_compressor: false,
            toggle_video_crop: false,
            toggle_goto_frame: false,
            toggle_playback_history: false,
            toggle_media_tag_editor: false,
            toggle_deinterlace: false,
            toggle_ab_repeat: false,
            toggle_boss_key: false,
            toggle_motion_interpolation: false,
            toggle_color_lut: false,
            toggle_subtitle_translator: false,
            toggle_ai_subtitle: false,
            toggle_ai_upscaling: false,
            toggle_ai_audio: false,
            toggle_loudness_radar: false,
            toggle_frame_dumper: false,
            toggle_color_blindness: false,
            toggle_cinema_matte: false,
            toggle_motion_vector_inspector: false,
            toggle_rich_bookmark_notes: false,
            toggle_pitch_formant: false,
            toggle_auto_skip_dialog: false,
            skip_intro: false,
            show_about: false,
            toggle_record_dialog: false,
            toggle_gif_maker: false,
            toggle_device_capture: false,
            toggle_subtitle_lookup: false,
            toggle_shader_studio: false,
            toggle_broadcast: false,
            toggle_contact_sheet: false,
            toggle_jump_time: false,
            toggle_pip: false,
            minimize_window: false,
            maximize_window: false,

            take_screenshot: false,
            play_path: None,
            sub_sync_changed: false,
            resize_window_factor: None,
            center_window: false,
            open_subtitle_preferences: false,
        };






        let skin = VortexTheme::get_skin(config.theme_mode);

        // Apply Unified Global Menu Aesthetics across all popup and submenus (260px first menu width)
        ui.set_width(260.0);
        ui.set_min_width(260.0);
        ui.spacing_mut().item_spacing = Vec2::new(0.0, 0.5);
        ui.spacing_mut().button_padding = Vec2::new(4.0, 1.0);
        ui.spacing_mut().menu_margin = Margin::symmetric(3, 2);

        let visuals = ui.visuals_mut();
        visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
        visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        visuals.widgets.hovered.bg_fill = skin.bg_btn_hover;
        visuals.widgets.hovered.bg_stroke = Stroke::NONE;
        visuals.widgets.hovered.corner_radius = CornerRadius::ZERO;
        visuals.widgets.active.bg_fill = skin.bg_btn_active;
        visuals.widgets.active.bg_stroke = Stroke::NONE;
        visuals.widgets.active.corner_radius = CornerRadius::ZERO;
        visuals.menu_corner_radius = CornerRadius::ZERO;
        visuals.window_fill = skin.bg_panel;
        visuals.window_stroke = Stroke::new(1.0, skin.border_dark);
        visuals.window_shadow = egui::Shadow {
            offset: [0, 8],
            blur: 24,
            spread: 2,
            color: Color32::from_black_alpha(190),
        };

        // =====================================================================
        // 1. FILE OPERATIONS (Open File, Open Folder, Stream, Recent, Favorites)
        // =====================================================================
        if menu_item(ui, &skin, "▶", "Open File...", "F3", false, false).clicked() {
            actions.open_file = true;
            ui.close();
        }

        submenu_item(ui, &skin, "📂", "Open", |ui| {
            ui.set_min_width(330.0);
            if menu_item(ui, &skin, "📁", "Open File(s)...", "Ctrl+O", false, false).clicked() {
                actions.open_file = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📂", "Open Folder...", "Ctrl+F", false, false).clicked() {
                actions.open_folder = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🌐", "Open URL / Stream...", "Ctrl+U", false, false).clicked() {
                actions.open_url = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🖥️", "Desktop Screen Capture...", "Ctrl+Shift+S", false, false).clicked() {
                actions.toggle_screen_capture = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📺", "IPTV & EPG Broadcasts...", "Ctrl+I", false, false).clicked() {
                actions.toggle_iptv_guide = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📀", "DVD / Blu-ray Disc Browser...", "Ctrl+D", false, false).clicked() {
                actions.toggle_disc_nav = true;
                ui.close();
            }
            if menu_item(ui, &skin, "💿", "Audio CD Ripper Studio...", "Ctrl+Shift+D", false, false).clicked() {
                actions.toggle_cd_ripper = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📡", "Digital TV Tuner (BDA)...", "Ctrl+Alt+T", false, false).clicked() {
                actions.toggle_bda_tuner = true;
                ui.close();
            }
            if menu_item(ui, &skin, "☕", "Java BD-J Blu-ray Menu...", "Ctrl+Alt+B", false, false).clicked() {
                actions.toggle_bdj_studio = true;
                ui.close();
            }
            if menu_item(ui, &skin, "☁️", "Plex / Jellyfin / Emby Browser...", "Ctrl+Alt+P", false, false).clicked() {
                actions.toggle_media_server = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🔄", "Convert / Transcode Media...", "Ctrl+R", false, false).clicked() {
                actions.toggle_transcoder = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📻", "Internet Radio Directory...", "Ctrl+Alt+I", false, false).clicked() {
                actions.toggle_radio_directory = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🔧", "Damaged Media Repair & Codec Doctor...", "Ctrl+Alt+F", false, false).clicked() {
                actions.toggle_damaged_file_repair = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📜", "Playback History & Watch Stats...", "Ctrl+Alt+Y", false, false).clicked() {
                actions.toggle_playback_history = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🏷️", "Media Metadata / Tag Editor...", "Ctrl+Alt+E", false, false).clicked() {
                actions.toggle_media_tag_editor = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📹", "Webcam / Device Capture...", "Ctrl+Shift+W", false, false).clicked() {
                actions.toggle_device_capture = true;
                ui.close();
            }
            if menu_item(ui, &skin, "⏺", "Stream & Audio Recorder...", "Ctrl+Shift+R", false, false).clicked() {
                actions.toggle_record_dialog = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📡", "Live RTMP / Broadcast Stream...", "Ctrl+Alt+L", false, false).clicked() {
                actions.toggle_broadcast = true;
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "✕", "Close File & Return to Home", "Ctrl+W / F4", false, false).clicked() {
                actions.close_file = true;
                ui.close();
            }
            menu_separator(ui);

            submenu_item(ui, &skin, "🕒", "Recent Files", |ui| {
                ui.set_min_width(260.0);
                if config.recent_files.is_empty() {
                    ui.label(RichText::new("  (No recent files)").size(11.0).color(skin.text_muted));
                } else {
                    let mut chosen_path = None;
                    for path_str in &config.recent_files {
                        let pb = PathBuf::from(path_str);
                        let name = pb.file_name().and_then(|n| n.to_str()).unwrap_or(path_str);
                        if menu_item(ui, &skin, "🗎", name, "", false, false).clicked() {
                            chosen_path = Some(pb);
                            ui.close();
                        }
                    }
                    if let Some(p) = chosen_path {
                        actions.play_path = Some(p);
                    }
                }
            });
        });

        submenu_item(ui, &skin, "★", "Favorites", |ui| {
            ui.set_min_width(260.0);
            if menu_item(ui, &skin, "★", "Add Current Item to Favorites", "Alt+Ins", false, stats.file_path.is_empty()).clicked() {
                if !stats.file_path.is_empty() {
                    playlist.toggle_favorite(&PathBuf::from(&stats.file_path));
                }
                ui.close();
            }
            if menu_item(ui, &skin, "📁", "Add Current Folder to Favorites", "Ctrl+Ins", false, stats.file_path.is_empty()).clicked() {
                if !stats.file_path.is_empty() {
                    if let Some(parent) = std::path::Path::new(&stats.file_path).parent() {
                        playlist.toggle_favorite(&parent.to_path_buf());
                    }
                }
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "✕", "Clear Favorites", "", false, false).clicked() {
                playlist.favorites.clear();
                ui.close();
            }
        });

        if menu_item(ui, &skin, "⏹", "Close", "F4", false, false).clicked() {
            if let Some(p) = player {
                p.stop();
            }
            ui.close();
        }

        menu_separator(ui);

        // =====================================================================
        // 2. PLAYBACK & NAVIGATION
        // =====================================================================
        submenu_item(ui, &skin, "▷", "Playback", |ui| {
            ui.set_min_width(280.0);
            if menu_item(ui, &skin, if stats.is_paused { "▶" } else { "⏸" }, if stats.is_paused { "Play" } else { "Pause" }, "Space", false, false).clicked() {
                if let Some(p) = player {
                    p.toggle_pause();
                }
                ui.close();
            }
            if menu_item(ui, &skin, "⏹", "Stop", "Ctrl+Space", false, false).clicked() {
                if let Some(p) = player {
                    p.stop();
                }
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "⏮", "Previous File", "PageUp", false, false).clicked() {
                if let Some(p) = playlist.previous() {
                    actions.play_path = Some(p);
                }
                ui.close();
            }
            if menu_item(ui, &skin, "⏭", "Next File", "PageDown", false, false).clicked() {
                if let Some(p) = playlist.next() {
                    actions.play_path = Some(p);
                }
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "💾", "Restore Prev Playlist on Direct Launch", "", config.restore_last_playlist, false).clicked() {
                config.restore_last_playlist = !config.restore_last_playlist;
                let _ = config.save();
                ui.close();
            }
            menu_separator(ui);
            let has_chapters = !stats.chapters.is_empty();
            if menu_item(ui, &skin, "⏮", "Previous Chapter", "Shift+H", false, !has_chapters).clicked() {
                if let Some(p) = player {
                    p.prev_chapter();
                }
                ui.close();
            }
            if menu_item(ui, &skin, "⏭", "Next Chapter", "H", false, !has_chapters).clicked() {
                if let Some(p) = player {
                    p.next_chapter();
                }
                ui.close();
            }

            submenu_item(ui, &skin, "📑", "Chapters", |ui| {
                ui.set_min_width(260.0);
                if menu_item(ui, &skin, "⏮", "Previous Chapter", "Shift+H", false, !has_chapters).clicked() {
                    if let Some(p) = player {
                        p.prev_chapter();
                    }
                    ui.close();
                }
                if menu_item(ui, &skin, "⏭", "Next Chapter", "H", false, !has_chapters).clicked() {
                    if let Some(p) = player {
                        p.next_chapter();
                    }
                    ui.close();
                }
                if menu_item(ui, &skin, "📑", "Chapter Timeline Manager...", "Ctrl+Alt+J", false, false).clicked() {
                    actions.toggle_chapter_marker = true;
                    ui.close();
                }
                menu_separator(ui);

                if stats.chapters.is_empty() {
                    ui.label(RichText::new("  (No chapters)").size(11.0).color(skin.text_muted));
                } else {
                    for (i, ch) in stats.chapters.iter().enumerate() {
                        let is_current = stats.current_chapter == Some(ch.index)
                            || (stats.current_chapter.is_none() && ch.time_pos <= stats.time_pos && stats.chapters.iter().filter(|c| c.time_pos <= stats.time_pos).last().map(|c| c.index) == Some(ch.index));
                        let time_str = format_time(ch.time_pos);
                        let is_skip = config.matches_skip_chapter(&ch.title);
                        let label = if ch.title.trim().is_empty() {
                            format!("Chapter {} ({})", i + 1, time_str)
                        } else if is_skip {
                            format!("{} ({}) ⏭ [Skip]", ch.title.trim(), time_str)
                        } else {
                            format!("{} ({})", ch.title.trim(), time_str)
                        };
                        if menu_item(ui, &skin, if is_current { "✓" } else { " " }, &label, "", is_current, false).clicked() {
                            if let Some(p) = player {
                                p.set_chapter(ch.index);
                            }
                            ui.close();
                        }
                    }
                }
            });
            submenu_item(ui, &skin, "⏭", "Skip / Auto-Skip", |ui| {
                ui.set_min_width(220.0);
                let auto_skip_on = config.skip_intro_enabled;
                if menu_item(ui, &skin, if auto_skip_on { "✓" } else { " " }, "Enable Auto-Skip Engine", "Ctrl+Alt+S", auto_skip_on, false).clicked() {
                    let new_state = !config.skip_intro_enabled;
                    config.skip_intro_enabled = new_state;
                    bookmark_mgr.auto_skip_bookmarks = new_state;
                    let _ = config.save();
                    ui.close();
                }
                if menu_item(ui, &skin, if config.skip_chapters_enabled { "✓" } else { " " }, "Auto-Skip Chapters (OP/ED)", "", config.skip_chapters_enabled, false).clicked() {
                    config.skip_chapters_enabled = !config.skip_chapters_enabled;
                    if config.skip_chapters_enabled && !config.skip_intro_enabled {
                        config.skip_intro_enabled = true;
                        bookmark_mgr.auto_skip_bookmarks = true;
                    }
                    let _ = config.save();
                    ui.close();
                }
                submenu_item(ui, &skin, "🏷", "Chapter Skip Keywords", |ui| {
                    ui.set_min_width(200.0);
                    let mut tags = crate::ui::auto_skip_dialog::tags_from_config_string(&config.skip_chapter_titles);
                    let mut changed = false;

                    for tag in &mut tags {
                        let icon = if tag.enabled { "✓" } else { " " };
                        let label = tag.keyword.clone();
                        if menu_item(ui, &skin, icon, &label, "", tag.enabled, false).clicked() {
                            tag.enabled = !tag.enabled;
                            changed = true;
                        }
                    }

                    menu_separator(ui);
                    if menu_item(ui, &skin, "✓", "Enable All Keywords", "", false, false).clicked() {
                        for tag in &mut tags {
                            tag.enabled = true;
                        }
                        changed = true;
                    }
                    if menu_item(ui, &skin, "✕", "Disable All Keywords", "", false, false).clicked() {
                        for tag in &mut tags {
                            tag.enabled = false;
                        }
                        changed = true;
                    }

                    if changed {
                        config.skip_chapter_titles = crate::ui::auto_skip_dialog::tags_to_config_string(&tags);
                        let _ = config.save();
                    }
                });
                menu_separator(ui);
                if menu_item(ui, &skin, "📋", "Auto-Skip Range Table / Manager...", "Ctrl+Shift+A", false, false).clicked() {
                    actions.toggle_auto_skip_dialog = true;
                    ui.close();
                }
                if menu_item(ui, &skin, "⏭", "Skip Opening (Intro) Now", "S", false, false).clicked() {
                    actions.skip_intro = true;
                    ui.close();
                }
                menu_separator(ui);
                let intro_active = config.skip_intro_sec > 0.0;
                let intro_label = if intro_active {
                    format!("Auto-Skip Intro: {:.0}s", config.skip_intro_sec)
                } else {
                    "Auto-Skip Intro: Off".to_string()
                };
                if menu_item(ui, &skin, if intro_active { "✓" } else { " " }, &intro_label, "", intro_active, false).clicked() {
                    config.skip_intro_sec = if intro_active { 0.0 } else { 90.0 };
                    let _ = config.save();
                    ui.close();
                }
                let outro_active = config.skip_outro_sec > 0.0;
                let outro_label = if outro_active {
                    format!("Auto-Skip Outro: {:.0}s", config.skip_outro_sec)
                } else {
                    "Auto-Skip Outro: Off".to_string()
                };
                if menu_item(ui, &skin, if outro_active { "✓" } else { " " }, &outro_label, "", outro_active, false).clicked() {
                    config.skip_outro_sec = if outro_active { 0.0 } else { 90.0 };
                    let _ = config.save();
                    ui.close();
                }
                menu_separator(ui);
                if menu_item(ui, &skin, "⏩", "Skip Forward 10s", "Ctrl+→", false, false).clicked() {
                    if let Some(p) = player {
                        p.seek_relative(10.0);
                    }
                    ui.close();
                }
                if menu_item(ui, &skin, "⏪", "Skip Backward 10s", "Ctrl+←", false, false).clicked() {
                    if let Some(p) = player {
                        p.seek_relative(-10.0);
                    }
                    ui.close();
                }
                if menu_item(ui, &skin, "⏩", "Skip Anime OP (+85s)", "Shift+S", false, false).clicked() {
                    if let Some(p) = player {
                        p.seek_relative(85.0);
                    }
                    ui.close();
                }
            });
            menu_separator(ui);
            if menu_item(ui, &skin, "⏩", "Jump Forward (+5s)", "→", false, false).clicked() {
                if let Some(p) = player {
                    p.seek_relative(config.seek_step_short);
                }
                ui.close();
            }
            if menu_item(ui, &skin, "⏪", "Jump Backward (-5s)", "←", false, false).clicked() {
                if let Some(p) = player {
                    p.seek_relative(-config.seek_step_short);
                }
                ui.close();
            }
            if menu_item(ui, &skin, "⏩", "Jump Forward (+30s)", "Ctrl+→", false, false).clicked() {
                if let Some(p) = player {
                    p.seek_relative(config.seek_step_medium);
                }
                ui.close();
            }
            if menu_item(ui, &skin, "⏪", "Jump Backward (-30s)", "Ctrl+←", false, false).clicked() {
                if let Some(p) = player {
                    p.seek_relative(-config.seek_step_medium);
                }
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "⏭", "Frame Step Forward", "F", false, false).clicked() {
                if let Some(p) = player {
                    p.step_frame_forward();
                }
                ui.close();
            }
            if menu_item(ui, &skin, "⏮", "Frame Step Backward", "D", false, false).clicked() {
                if let Some(p) = player {
                    p.step_frame_backward();
                }
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "🔁", "A-B Loop: Set Start [A]", "[", false, false).clicked() {
                let _ = bookmark_mgr.set_loop_a(stats.time_pos);
                ui.close();
            }
            if menu_item(ui, &skin, "🔁", "A-B Loop: Set End [B]", "]", false, false).clicked() {
                let _ = bookmark_mgr.set_loop_b(stats.time_pos);
                ui.close();
            }
            if menu_item(ui, &skin, "✕", "A-B Loop: Clear", "\\", false, false).clicked() {
                bookmark_mgr.clear_ab_loop();
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "🎞️", "Go To Frame Number...", "Ctrl+Shift+G", false, false).clicked() {
                actions.toggle_goto_frame = true;
                ui.close();
            }
            if menu_item(ui, &skin, "⏱", "Jump to Specific Time...", "G", false, false).clicked() {
                actions.toggle_jump_time = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🔁", "A-B Repeat Studio Looper...", "Ctrl+Shift+L", false, false).clicked() {
                actions.toggle_ab_repeat = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📝", "Rich Bookmarks & Study Notes...", "Ctrl+Alt+8", false, false).clicked() {
                actions.toggle_rich_bookmark_notes = true;
                ui.close();
            }
            menu_separator(ui);



            if menu_item(ui, &skin, "⏱", "Speed: Normal (1.0×)", "Z", stats.speed == 1.0, false).clicked() {
                if let Some(p) = player {
                    p.reset_speed();
                }
                ui.close();
            }
            if menu_item(ui, &skin, "🐢", "Speed: Slower (-0.1×)", "X", false, false).clicked() {
                if let Some(p) = player {
                    p.adjust_speed(-0.1);
                }
                ui.close();
            }
            if menu_item(ui, &skin, "🐇", "Speed: Faster (+0.1×)", "C", false, false).clicked() {
                if let Some(p) = player {
                    p.adjust_speed(0.1);
                }
                ui.close();
            }
        });

        // =====================================================================
        // 3. SUBTITLES
        // =====================================================================
        submenu_item(ui, &skin, "💬", "Subtitles", |ui| {
            ui.set_min_width(175.0);
            if menu_item(ui, &skin, "👁", "Show / Hide Subtitles", "Alt+H", false, false).clicked() {
                if let Some(p) = player {
                    p.toggle_subtitles();
                }
                ui.close();
            }
            if menu_item(ui, &skin, "🔄", "Cycle Subtitle Track", "S", false, false).clicked() {
                if let Some(p) = player {
                    p.cycle_subtitle_track();
                }
                ui.close();
            }
            if menu_item(ui, &skin, "📂", "Add / Load Subtitles...", "Alt+O", false, false).clicked() {
                actions.load_subtitle = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🔍", "Online Subtitle Search & Download...", "Ctrl+Shift+O", false, false).clicked() {
                actions.toggle_subtitle_lookup = true;
                ui.close();
            }
            menu_separator(ui);

            if !stats.current_sub_text.is_empty() {
                let clean_sub = stats.current_sub_text.replace('\n', " ").trim().to_string();
                let short_sub = if clean_sub.chars().count() > 24 {
                    format!("{}...", clean_sub.chars().take(22).collect::<String>())
                } else {
                    clean_sub.clone()
                };
                let search_label = format!("Search: \"{}\" on Google", short_sub);
                if menu_item(ui, &skin, "🔍", &search_label, "Ctrl+G", false, false).clicked() {
                    crate::ui::subtitle_lookup_dialog::SubtitleLookupDialog::open_web_search(&clean_sub, &config.subtitle_search_engine);
                    ui.close();
                }
                if menu_item(ui, &skin, "🌐", "Translate Subtitle (Google Translate)", "", false, false).clicked() {
                    crate::ui::subtitle_lookup_dialog::SubtitleLookupDialog::open_web_search(&clean_sub, "Google Translate");
                    ui.close();
                }
                if menu_item(ui, &skin, "📋", "Copy Subtitle to Clipboard", "Ctrl+C", false, false).clicked() {
                    ui.ctx().copy_text(clean_sub);
                    ui.close();
                }
                menu_separator(ui);
            }

            submenu_item(ui, &skin, "🔎", "Subtitle Click Search", |ui| {
                ui.set_min_width(220.0);
                if menu_item(ui, &skin, if config.subtitle_click_search { "✓" } else { " " }, "Search on Subtitle Click / Hover", "", config.subtitle_click_search, false).clicked() {
                    config.subtitle_click_search = !config.subtitle_click_search;
                    let _ = config.save();
                    ui.close();
                }
                menu_separator(ui);
                for engine in &["Google", "Google Translate", "DuckDuckGo", "Bing", "Cambridge"] {
                    let is_sel = config.subtitle_search_engine == *engine;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, engine, "", is_sel, false).clicked() {
                        config.subtitle_search_engine = engine.to_string();
                        let _ = config.save();
                        ui.close();
                    }
                }
            });
            menu_separator(ui);

            // Subtitle Track Selection
            submenu_item(ui, &skin, "💬", "Select Subtitle Track", |ui| {
                ui.set_min_width(160.0);
                if stats.subtitle_tracks.is_empty() {
                    ui.label(RichText::new("  (No subtitle tracks)").size(11.0).color(skin.text_muted));
                } else {
                    for track in &stats.subtitle_tracks {
                        let is_sel = track.id == stats.selected_subtitle_track;
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, &track.title, "", is_sel, false).clicked() {
                            if let Some(p) = player {
                                p.set_subtitle_track(track.id);
                            }
                            ui.close();
                        }
                    }
                }
            });
            menu_separator(ui);

            // ── Subtitle Font & Style ──────────────────────────────────────────
            submenu_item(ui, &skin, "🔤", "Font & Style", |ui| {
                ui.set_min_width(170.0);
                submenu_item(ui, &skin, "📏", "Font Size", |ui| {
                    ui.set_min_width(160.0);
                    if menu_item(ui, &skin, "➕", "Increase Size (+2 pt)", "Page Up", false, false).clicked() {
                        config.subtitle_font_size = (config.subtitle_font_size + 2.0).clamp(12.0, 72.0);
                        if let Some(p) = player { p.set_subtitle_font_size(config.subtitle_font_size); }
                        let _ = config.save();
                        ui.close();
                    }
                    if menu_item(ui, &skin, "➖", "Decrease Size (-2 pt)", "Page Down", false, false).clicked() {
                        config.subtitle_font_size = (config.subtitle_font_size - 2.0).clamp(12.0, 72.0);
                        if let Some(p) = player { p.set_subtitle_font_size(config.subtitle_font_size); }
                        let _ = config.save();
                        ui.close();
                    }
                    if menu_item(ui, &skin, "↺", "Reset Size (28 pt)", "Home", false, false).clicked() {
                        config.subtitle_font_size = 28.0;
                        if let Some(p) = player { p.set_subtitle_font_size(28.0); }
                        let _ = config.save();
                        ui.close();
                    }
                    menu_separator(ui);
                    for &sz in &[16.0, 20.0, 24.0, 28.0, 32.0, 36.0, 42.0, 48.0] {
                        let is_sel = (config.subtitle_font_size - sz).abs() < 1.0;
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, &format!("{:.0} pt", sz), "", is_sel, false).clicked() {
                            config.subtitle_font_size = sz;
                            if let Some(p) = player { p.set_subtitle_font_size(sz); }
                            let _ = config.save();
                            ui.close();
                        }
                    }
                });

                submenu_item(ui, &skin, "🅰", "Font Family", |ui| {
                    ui.set_min_width(160.0);
                    for font in &["Segoe UI", "Arial", "Trebuchet MS", "Verdana", "Inter", "Roboto", "Consolas", "Georgia"] {
                        let is_sel = config.subtitle_sub_font == *font;
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, font, "", is_sel, false).clicked() {
                            config.subtitle_sub_font = font.to_string();
                            if let Some(p) = player { p.set_subtitle_font(font); }
                            let _ = config.save();
                            ui.close();
                        }
                    }
                });

                let bold_mark = if config.subtitle_bold { "✓" } else { " " };
                if menu_item(ui, &skin, bold_mark, "Bold Font Weight", "", config.subtitle_bold, false).clicked() {
                    config.subtitle_bold = !config.subtitle_bold;
                    if let Some(p) = player { p.set_subtitle_bold(config.subtitle_bold); }
                    let _ = config.save();
                    ui.close();
                }

                let italic_mark = if config.subtitle_italic { "✓" } else { " " };
                if menu_item(ui, &skin, italic_mark, "Italic Font Style", "", config.subtitle_italic, false).clicked() {
                    config.subtitle_italic = !config.subtitle_italic;
                    if let Some(p) = player { p.set_subtitle_italic(config.subtitle_italic); }
                    let _ = config.save();
                    ui.close();
                }

                submenu_item(ui, &skin, "↔", "Letter Spacing", |ui| {
                    ui.set_min_width(140.0);
                    let spacings = [
                        (-1.0, "Tight (-1 px)"),
                        (0.0, "Normal (0 px - Default)"),
                        (2.0, "Wide (+2 px)"),
                        (4.0, "Very Wide (+4 px)"),
                    ];
                    for (sp, label) in spacings {
                        let is_sel = (config.subtitle_letter_spacing - sp).abs() < 0.5;
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                            config.subtitle_letter_spacing = sp;
                            if let Some(p) = player { p.set_subtitle_letter_spacing(sp); }
                            let _ = config.save();
                            ui.close();
                        }
                    }
                });
            });

            // ── Colors, Outline & Box ─────────────────────────────────────────
            submenu_item(ui, &skin, "🎨", "Colors & Effects", |ui| {
                ui.set_min_width(170.0);
                submenu_item(ui, &skin, "🟡", "Text Color", |ui| {
                    ui.set_min_width(160.0);
                    let colors = [
                        ("#FFFFFF", "White"),
                        ("#FFE600", "Vortex Gold"),
                        ("#FFFF00", "Bright Yellow"),
                        ("#FFCC00", "Soft Amber"),
                        ("#00FFFF", "Cyan"),
                        ("#A8FFB2", "Mint Green"),
                    ];
                    for (hex, label) in colors {
                        let is_sel = config.subtitle_color.eq_ignore_ascii_case(hex);
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                            config.subtitle_color = hex.to_string();
                            if let Some(p) = player { p.set_subtitle_color(hex); }
                            let _ = config.save();
                            ui.close();
                        }
                    }
                });

                submenu_item(ui, &skin, "🖌", "Outline Thickness", |ui| {
                    ui.set_min_width(160.0);
                    let outlines = [
                        (0.0, "Off (0 px)"),
                        (1.5, "Thin (1.5 px)"),
                        (2.5, "Medium (2.5 px - Default)"),
                        (4.0, "Thick (4.0 px)"),
                        (6.0, "Extra Thick (6.0 px)"),
                    ];
                    for (w, label) in outlines {
                        let is_sel = (config.subtitle_outline_width - w).abs() < 0.5;
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                            config.subtitle_outline_width = w;
                            if let Some(p) = player { p.set_subtitle_border_size(w); }
                            let _ = config.save();
                            ui.close();
                        }
                    }
                });

                submenu_item(ui, &skin, "✨", "Outline Soft Blur", |ui| {
                    ui.set_min_width(160.0);
                    let blurs = [
                        (0.0, "Off (Crisp Edge)"),
                        (1.5, "Subtle (1.5 px)"),
                        (3.0, "Cinematic Glow (3.0 px)"),
                        (5.0, "Heavy Glow (5.0 px)"),
                    ];
                    for (b, label) in blurs {
                        let is_sel = (config.subtitle_border_blur - b).abs() < 0.5;
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                            config.subtitle_border_blur = b;
                            if let Some(p) = player { p.set_subtitle_border_blur(b); }
                            let _ = config.save();
                            ui.close();
                        }
                    }
                });

                submenu_item(ui, &skin, "🌑", "Drop Shadow", |ui| {
                    ui.set_min_width(150.0);
                    let shadows = [
                        (0.0, "Off (0 px)"),
                        (1.5, "Subtle (1.5 px)"),
                        (2.0, "Standard (2.0 px - Default)"),
                        (4.0, "Deep (4.0 px)"),
                    ];
                    for (s, label) in shadows {
                        let is_sel = (config.subtitle_shadow_offset - s).abs() < 0.5;
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                            config.subtitle_shadow_offset = s;
                            if let Some(p) = player { p.set_subtitle_shadow_offset(s); }
                            let _ = config.save();
                            ui.close();
                        }
                    }
                });

                let box_mark = if config.subtitle_background_box { "✓" } else { " " };
                if menu_item(ui, &skin, box_mark, "Background Bounding Box", "", config.subtitle_background_box, false).clicked() {
                    config.subtitle_background_box = !config.subtitle_background_box;
                    if let Some(p) = player { p.set_subtitle_background_box(config.subtitle_background_box, &config.subtitle_background_color); }
                    let _ = config.save();
                    ui.close();
                }

                if config.subtitle_background_box {
                    submenu_item(ui, &skin, "⬛", "Box Opacity", |ui| {
                        ui.set_min_width(140.0);
                        let opacities = [
                            ("#66000000", "40% Tint"),
                            ("#99000000", "60% Standard"),
                            ("#CC000000", "80% Heavy"),
                            ("#FF000000", "100% Solid"),
                        ];
                        for (hex, label) in opacities {
                            let is_sel = config.subtitle_background_color.eq_ignore_ascii_case(hex);
                            if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                                config.subtitle_background_color = hex.to_string();
                                if let Some(p) = player { p.set_subtitle_background_box(true, hex); }
                                let _ = config.save();
                                ui.close();
                            }
                        }
                    });
                }
            });

            // ── Position & Placement ──────────────────────────────────────────
            submenu_item(ui, &skin, "↕", "Position & Canvas", |ui| {
                ui.set_min_width(170.0);
                if menu_item(ui, &skin, "⬆", "Move Up (-5%)", "↑", false, false).clicked() {
                    config.subtitle_vertical_pos = (config.subtitle_vertical_pos - 5.0).clamp(0.0, 115.0);
                    if let Some(p) = player { p.set_subtitle_pos(config.subtitle_vertical_pos); }
                    let _ = config.save();
                    ui.close();
                }
                if menu_item(ui, &skin, "⬇", "Move Down (+5%)", "↓", false, false).clicked() {
                    config.subtitle_vertical_pos = (config.subtitle_vertical_pos + 5.0).clamp(0.0, 115.0);
                    if let Some(p) = player { p.set_subtitle_pos(config.subtitle_vertical_pos); }
                    let _ = config.save();
                    ui.close();
                }
                menu_separator(ui);
                let pos_presets = [
                    (100.0, "Standard Bottom (100% - Recommended)"),
                    (96.0, "Safe In-Picture (96%)"),
                    (92.0, "Floating (92%)"),
                    (50.0, "Center (50%)"),
                    (10.0, "Top (10%)"),
                ];
                for (pos, label) in pos_presets {
                    let is_sel = (config.subtitle_vertical_pos - pos).abs() < 2.0;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.subtitle_vertical_pos = pos;
                        if let Some(p) = player { p.set_subtitle_pos(pos); }
                        let _ = config.save();
                        ui.close();
                    }
                }
                menu_separator(ui);
                submenu_item(ui, &skin, "↔", "Horizontal Alignment", |ui| {
                    ui.set_min_width(140.0);
                    for &(val, label) in &[("left", "Left"), ("center", "Center (Default)"), ("right", "Right")] {
                        let is_sel = config.subtitle_align_x == val;
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                            config.subtitle_align_x = val.to_string();
                            if let Some(p) = player { p.set_subtitle_align_x(val); }
                            let _ = config.save();
                            ui.close();
                        }
                    }
                });
                menu_separator(ui);
                let lb_sel = !config.subtitle_render_to_video;
                if menu_item(ui, &skin, if lb_sel { "✓" } else { " " }, "Render in Black Bar Letterbox", "", lb_sel, false).clicked() {
                    config.subtitle_render_to_video = false;
                    if let Some(p) = player {
                        p.set_property_string("sub-use-margins", "yes");
                        p.set_property_string("sub-ass-force-margins", "yes");
                    }
                    let _ = config.save();
                    ui.close();
                }
                let vid_sel = config.subtitle_render_to_video;
                if menu_item(ui, &skin, if vid_sel { "✓" } else { " " }, "Force Render Inside Video Frame", "", vid_sel, false).clicked() {
                    config.subtitle_render_to_video = true;
                    if let Some(p) = player {
                        p.set_property_string("sub-use-margins", "no");
                        p.set_property_string("sub-ass-force-margins", "no");
                    }
                    let _ = config.save();
                    ui.close();
                }
            });

            // ── ASS / SSA Override Mode ───────────────────────────────────────
            submenu_item(ui, &skin, "⚙", "ASS Styling Override", |ui| {
                ui.set_min_width(190.0);
                let modes = [
                    ("scale", "Smart Scale (Scale to Resolution - Recommended)"),
                    ("no", "Strict / Original (Preserve All ASS Effects)"),
                    ("yes", "Allow Font & Sizing Overrides"),
                    ("force", "Force All VortexPlayer Styles"),
                    ("strip", "Strip ASS Tags (Plain Subtitles)"),
                ];
                for (mode, label) in modes {
                    let is_sel = config.subtitle_ass_override == mode;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.subtitle_ass_override = mode.to_string();
                        if let Some(p) = player { p.set_subtitle_ass_override(mode); }
                        let _ = config.save();
                        ui.close();
                    }
                }
            });
            menu_separator(ui);

            if menu_item(ui, &skin, "🌐", "Word Translator...", "Ctrl+Alt+W", false, false).clicked() {
                actions.toggle_subtitle_translator = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🧠", "AI Speech-to-Text...", "Ctrl+Alt+3", false, false).clicked() {
                actions.toggle_ai_subtitle = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📝", "Subtitle Studio...", "Ctrl+T", false, false).clicked() {
                actions.toggle_subtitle_studio = true;
                ui.close();
            }
            menu_separator(ui);
            submenu_item(ui, &skin, "⏱", "Subtitle Sync / Delay", |ui| {
                ui.set_min_width(175.0);
                let cur_delay = stats.subtitle_delay;
                let cur_ms = (cur_delay * 1000.0).round() as i64;
                ui.label(RichText::new(format!("Current Sync: {:+0} ms ({:+.2}s)", cur_ms, cur_delay)).size(11.0).color(skin.text_muted));
                menu_separator(ui);
                if menu_item(ui, &skin, "⏩", "Subtitle 0.5s Faster", "> / .", false, false).clicked() {
                    if let Some(p) = player {
                        let next = p.stats().subtitle_delay - 0.5;
                        p.set_subtitle_delay(next);
                    }
                    actions.sub_sync_changed = true;
                    ui.close();
                }
                if menu_item(ui, &skin, "⏪", "Subtitle 0.5s Slower", "< / ,", false, false).clicked() {
                    if let Some(p) = player {
                        let next = p.stats().subtitle_delay + 0.5;
                        p.set_subtitle_delay(next);
                    }
                    actions.sub_sync_changed = true;
                    ui.close();
                }
                if menu_item(ui, &skin, "⏩", "Subtitle 0.1s Faster", "Shift+>", false, false).clicked() {
                    if let Some(p) = player {
                        let next = p.stats().subtitle_delay - 0.1;
                        p.set_subtitle_delay(next);
                    }
                    actions.sub_sync_changed = true;
                    ui.close();
                }
                if menu_item(ui, &skin, "⏪", "Subtitle 0.1s Slower", "Shift+<", false, false).clicked() {
                    if let Some(p) = player {
                        let next = p.stats().subtitle_delay + 0.1;
                        p.set_subtitle_delay(next);
                    }
                    actions.sub_sync_changed = true;
                    ui.close();
                }
                menu_separator(ui);
                if menu_item(ui, &skin, "↺", "Reset Subtitle Sync (0.0s)", "/ (Slash)", false, false).clicked() {
                    if let Some(p) = player {
                        p.set_subtitle_delay(0.0);
                    }
                    actions.sub_sync_changed = true;
                    ui.close();
                }
            });
            menu_separator(ui);
            if menu_item(ui, &skin, "⚙", "Subtitle Preferences...", "F5", false, false).clicked() {
                actions.open_subtitle_preferences = true;
                ui.close();
            }
        });

        // =====================================================================
        // 4. VIDEO & DISPLAY
        // =====================================================================
        submenu_item(ui, &skin, "📺", "Video", |ui| {
            ui.set_min_width(280.0);
            submenu_item(ui, &skin, "🗖", "Aspect Ratio", |ui| {
                ui.set_min_width(280.0);
                for &ratio in &["auto", "16:9", "16:10", "4:3", "1.85:1", "2.35:1", "crop_fill", "fill"] {
                    let is_sel = config.aspect_ratio == ratio;
                    let label = match ratio {
                        "auto" => "Auto (Keep AR)",
                        "16:9" => "16:9 Wide",
                        "16:10" => "16:10 PC",
                        "4:3" => "4:3 Standard",
                        "1.85:1" => "1.85:1 Cinema",
                        "2.35:1" => "2.35:1 Anamorphic",
                        "crop_fill" => "Crop to Fill Screen (No Black Bars)",
                        "fill" => "Stretch to Fit Screen (No Black Bars)",
                        _ => ratio,
                    };
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.aspect_ratio = ratio.to_string();
                        if let Some(p) = player {
                            p.set_aspect_ratio(ratio);
                        }
                        let _ = config.save();
                        ui.close();
                    }
                }
                ui.separator();
                submenu_item(ui, &skin, "↕", "Black Bar Placement", |ui| {
                    ui.set_min_width(260.0);
                    let aligns = [
                        ("center", "Both (Half & Half - Default)"),
                        ("top", "Bottom (All bar at Bottom)"),
                        ("bottom", "Top (All bar at Top)"),
                    ];
                    for (align, label) in aligns {
                        let is_sel = config.video_align_y == align;
                        if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                            config.video_align_y = align.to_string();
                            if let Some(p) = player {
                                p.set_video_align_y(align);
                            }
                            let _ = config.save();
                            ui.close();
                        }
                    }
                });
            });

            submenu_item(ui, &skin, "⚡", "Hardware Acceleration", |ui| {
                ui.set_min_width(240.0);
                for &hw in &["auto-safe", "d3d11va", "dxva2", "nvdec", "no"] {
                    let is_sel = config.hardware_decoding == hw;
                    let label = match hw {
                        "auto-safe" => "auto-safe (Auto)",
                        "d3d11va" => "d3d11va (D3D11)",
                        "dxva2" => "dxva2 (DXVA)",
                        "nvdec" => "nvdec (NVIDIA)",
                        "no" => "no (Software)",
                        _ => hw,
                    };
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.hardware_decoding = hw.to_string();
                        if let Some(p) = player {
                            p.set_hwdec(hw);
                        }
                        let _ = config.save();
                        ui.close();
                    }
                }
            });

            submenu_item(ui, &skin, "🔄", "Rotation", |ui| {
                ui.set_min_width(220.0);
                for &deg in &[0, 90, 180, 270] {
                    let is_sel = config.video_rotation == deg;
                    let label = match deg {
                        0 => "0° (Normal)",
                        90 => "90° Clockwise",
                        180 => "180° Inverted",
                        270 => "270° Counter",
                        _ => "0°",
                    };
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.video_rotation = deg;
                        if let Some(p) = player {
                            p.set_rotation(deg);
                        }
                        let _ = config.save();
                        ui.close();
                    }
                }
            });

            submenu_item(ui, &skin, "🧊", "3D Output Modes", |ui| {
                ui.set_min_width(240.0);
                if menu_item(ui, &skin, "✓", "2D (Off / Normal)", "", true, false).clicked() {
                    if let Some(p) = player {
                        p.set_property_string("vf", "");
                    }
                    ui.close();
                }
                menu_separator(ui);
                if menu_item(ui, &skin, "👓", "3D SBS -> 2D Monoscopic", "", false, false).clicked() {
                    if let Some(p) = player {
                        p.set_property_string("vf", "crop=iw/2:ih:0:0");
                    }
                    ui.close();
                }
                if menu_item(ui, &skin, "👓", "3D TAB -> 2D Monoscopic", "", false, false).clicked() {
                    if let Some(p) = player {
                        p.set_property_string("vf", "crop=iw:ih/2:0:0");
                    }
                    ui.close();
                }
                menu_separator(ui);
                if menu_item(ui, &skin, "🔴", "3D SBS -> Red/Cyan", "", false, false).clicked() {
                    if let Some(p) = player {
                        p.set_property_string("vf", "stereo3d=sbs2l:arrc");
                    }
                    ui.close();
                }
                if menu_item(ui, &skin, "🔵", "3D SBS -> ColorCode", "", false, false).clicked() {
                    if let Some(p) = player {
                        p.set_property_string("vf", "stereo3d=sbs2l:aybc");
                    }
                    ui.close();
                }
            });

            if menu_item(ui, &skin, "🕶️", "Stereo 3D Studio...", "Ctrl+Alt+3", false, false).clicked() {
                actions.toggle_stereo_3d = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🖼️", "Watermark & Logo...", "", false, false).clicked() {
                actions.toggle_logo_watermark = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🌐", "360° VR Mode", "", false, false).clicked() {
                actions.toggle_vr_360 = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎞", "Seamless Stitching Matrix...", "", false, false).clicked() {
                actions.toggle_video_wall = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎞️", "Animated GIF / WebP Maker...", "Ctrl+G", false, false).clicked() {
                actions.toggle_gif_maker = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🖼️", "Thumbnail Contact Sheet Generator...", "Alt+N", false, false).clicked() {
                actions.toggle_contact_sheet = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎨", "GLSL Shader Studio...", "Ctrl+Alt+S", false, false).clicked() {
                actions.toggle_shader_studio = true;
                ui.close();
            }
            menu_separator(ui);

            if menu_item(ui, &skin, "✂️", "Video Crop...", "Ctrl+Shift+C", false, false).clicked() {
                actions.toggle_video_crop = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📺", "Deinterlace...", "Ctrl+Shift+I", false, false).clicked() {
                actions.toggle_deinterlace = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🏎️", "Motion Interpolation...", "Ctrl+Shift+M", false, false).clicked() {
                actions.toggle_motion_interpolation = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎨", "3D LUT Studio...", "Ctrl+Shift+T", false, false).clicked() {
                actions.toggle_color_lut = true;
                ui.close();
            }
            if menu_item(ui, &skin, "✨", "AI Super Resolution...", "Ctrl+Alt+4", false, false).clicked() {
                actions.toggle_ai_upscaling = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📸", "Frame Dumper...", "Ctrl+Alt+5", false, false).clicked() {
                actions.toggle_frame_dumper = true;
                ui.close();
            }
            if menu_item(ui, &skin, "👁️", "Color Blindness...", "Ctrl+Alt+9", false, false).clicked() {
                actions.toggle_color_blindness = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎭", "Cinema Matte...", "Ctrl+Alt+0", false, false).clicked() {
                actions.toggle_cinema_matte = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🔬", "Motion Vectors...", "", false, false).clicked() {
                actions.toggle_motion_vector_inspector = true;
                ui.close();
            }
            menu_separator(ui);

            if menu_item(ui, &skin, "📷", "Screenshot", "Ctrl+E", false, false).clicked() {
                actions.take_screenshot = true;
                ui.close();
            }
            if menu_item(ui, &skin, "↺", "Reset Colors", "Q", false, false).clicked() {
                if let Some(p) = player {
                    p.apply_video_effects(&VideoEffectsConfig::default());
                }
                config.video_brightness = 0.0;
                config.video_contrast = 0.0;
                config.video_saturation = 0.0;
                config.video_hue = 0.0;
                config.video_gamma = 0.0;
                let _ = config.save();
                ui.close();
            }
        });

        // =====================================================================
        // =====================================================================
        // 5. AUDIO
        // =====================================================================
        submenu_item(ui, &skin, "🔊", "Audio", |ui| {
            ui.set_min_width(380.0);
            if menu_item(ui, &skin, "🎤", "Karaoke Studio...", "Ctrl+K", false, false).clicked() {
                actions.toggle_karaoke = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🔄", "Cycle Audio Stream", "Alt+A", false, false).clicked() {
                if let Some(p) = player {
                    p.cycle_audio_track();
                }
                ui.close();
            }


            // EXACT VORTEX AUDIO STREAM SELECTION SUBMENU
            submenu_item(ui, &skin, "🎧", "Select Audio Stream", |ui| {
                ui.set_min_width(260.0);
                if menu_item(ui, &skin, "🔄", "Cycle Audio Stream", "Alt+A", false, false).clicked() {
                    if let Some(p) = player {
                        p.cycle_audio_track();
                    }
                    ui.close();
                }

                if stats.audio_tracks.is_empty() {
                    ui.label(RichText::new("  (No audio streams)").size(11.0).color(skin.text_muted));
                } else {
                    for track in &stats.audio_tracks {
                        let is_active = track.id == stats.selected_audio_track;
                        let prefix = if is_active { "●" } else { " " };
                        if menu_item(ui, &skin, prefix, &track.title, "", false, is_active).clicked() {
                            if let Some(p) = player {
                                p.set_audio_track(track.id);
                            }
                            ui.close();
                        }
                    }

                    menu_separator(ui);

                    let curr_ch = config.audio_channels.clone();
                    let is_both = curr_ch != "left" && curr_ch != "right";
                    let is_left = curr_ch == "left";
                    let is_right = curr_ch == "right";

                    if menu_item(ui, &skin, if is_both { "●" } else { " " }, "Both", "", false, is_both).clicked() {
                        config.audio_channels = "auto".to_string();
                        if let Some(p) = player {
                            p.set_audio_channels("auto");
                        }
                        let _ = config.save();
                        actions.close_menu = true;
                        ui.close();
                    }
                    if menu_item(ui, &skin, if is_left { "●" } else { " " }, "Left channel", "", false, is_left).clicked() {
                        config.audio_channels = "left".to_string();
                        if let Some(p) = player {
                            p.set_audio_channels("left");
                        }
                        let _ = config.save();
                        actions.close_menu = true;
                        ui.close();
                    }
                    if menu_item(ui, &skin, if is_right { "●" } else { " " }, "Right channel", "", false, is_right).clicked() {
                        config.audio_channels = "right".to_string();
                        if let Some(p) = player {
                            p.set_audio_channels("right");
                        }
                        let _ = config.save();
                        actions.close_menu = true;
                        ui.close();
                    }
                }
            });

            submenu_item(ui, &skin, "🎧", "Output Device", |ui| {
                ui.set_min_width(300.0);
                let curr_dev = config.audio_device.clone();
                let is_auto = curr_dev == "auto" || curr_dev.is_empty();
                if menu_item(ui, &skin, if is_auto { "✓" } else { " " }, "Auto (System Device)", "", is_auto, false).clicked() {
                    config.audio_device = "auto".to_string();
                    if let Some(p) = player {
                        p.set_audio_device("auto");
                    }
                    let _ = config.save();
                    actions.close_menu = true;
                    ui.close();
                }
                menu_separator(ui);
                for dev in &stats.audio_device_list {
                    if dev.name == "auto" || dev.name == "pipewire" || dev.name == "pulse" || dev.name == "alsa" || dev.name == "jack" || dev.name == "sdl" {
                        continue;
                    }
                    let is_sel = curr_dev == dev.name;
                    let icon = if dev.name.contains("surround") || dev.description.contains("Surround") || dev.description.contains("5.1") || dev.description.contains("7.1") {
                        "🔊"
                    } else if dev.name.contains("hdmi") || dev.description.contains("HDMI") {
                        "📺"
                    } else if dev.name.contains("headphone") || dev.name.contains("Headset") || dev.description.contains("boAt") {
                        "🎧"
                    } else {
                        "🖴"
                    };
                    let label = if dev.description.is_empty() { &dev.name } else { &dev.description };
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, &format!("{} {}", icon, label), "", is_sel, false).clicked() {
                        config.audio_device = dev.name.clone();
                        if let Some(p) = player {
                            p.set_audio_device(&dev.name);
                        }
                        let _ = config.save();
                        actions.close_menu = true;
                        ui.close();
                    }
                }
            });

            submenu_item(ui, &skin, "🔊", "Speaker Channels", |ui| {
                ui.set_min_width(160.0);
                let curr_ch = config.audio_channels.clone();
                let is_same = curr_ch == "auto" || curr_ch == "auto-safe" || curr_ch.is_empty();
                if menu_item(ui, &skin, if is_same { "✓" } else { " " }, "Same as input", "", is_same, false).clicked() {
                    config.audio_channels = "auto".to_string();
                    if let Some(p) = player {
                        p.set_audio_channels("auto");
                    }
                    let _ = config.save();
                    actions.close_menu = true;
                    ui.close();
                }
                menu_separator(ui);
                let ch_options = [
                    ("mono", "1.0 mono"),
                    ("stereo", "2.0 stereo"),
                    ("2.1", "2.1 stereo + LFE"),
                    ("3.0", "3.0 Surround"),
                    ("4.0", "4.0 Quad"),
                    ("5.0", "5.0 Surround"),
                    ("5.1", "5.1 Surround"),
                    ("6.1", "6.1 Surround"),
                    ("7.1", "7.1 Surround"),
                ];
                for (id, label) in ch_options {
                    let is_sel = curr_ch == id;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.audio_channels = id.to_string();
                        if let Some(p) = player {
                            p.set_audio_channels(id);
                        }
                        let _ = config.save();
                        actions.close_menu = true;
                        ui.close();
                    }
                }
                menu_separator(ui);
                let is_bs2b = curr_ch == "bs2b";
                if menu_item(ui, &skin, if is_bs2b { "✓" } else { " " }, "Virtual Headphone", "", is_bs2b, false).clicked() {
                    config.audio_channels = "bs2b".to_string();
                    if let Some(p) = player {
                        p.set_audio_channels("bs2b");
                    }
                    let _ = config.save();
                    actions.close_menu = true;
                    ui.close();
                }
                let is_sofalizer = curr_ch == "sofalizer";
                if menu_item(ui, &skin, if is_sofalizer { "✓" } else { " " }, "3D Surround (HRTF)", "", is_sofalizer, false).clicked() {
                    config.audio_channels = "sofalizer".to_string();
                    if let Some(p) = player {
                        p.set_audio_channels("sofalizer");
                    }
                    let _ = config.save();
                    actions.close_menu = true;
                    ui.close();
                }
            });

            submenu_item(ui, &skin, "🎛", "Sample Rate (Resample)", |ui| {
                ui.set_min_width(160.0);
                let curr_rate = config.resample_rate;
                let rates = [
                    (0, "Auto (Same as Input)"),
                    (44100, "44.1 kHz (CD)"),
                    (48000, "48.0 kHz (DVD)"),
                    (88200, "88.2 kHz (2x CD)"),
                    (96000, "96.0 kHz (Studio)"),
                    (176400, "176.4 kHz (4x CD)"),
                    (192000, "192.0 kHz (Hi-Res)"),
                    (384000, "384.0 kHz (Master)"),
                ];
                for (rate, label) in rates {
                    let is_sel = curr_rate == rate;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.resample_rate = rate;
                        if let Some(p) = player {
                            p.set_audio_samplerate(rate);
                        }
                        let _ = config.save();
                        actions.close_menu = true;
                        ui.close();
                    }
                }
            });

            submenu_item(ui, &skin, "💎", "Audio Bit Depth", |ui| {
                ui.set_min_width(160.0);
                let curr_bd = config.audio_bit_depth.clone();
                let depths = [
                    ("auto", "Auto (Bit-Perfect)"),
                    ("16", "16-bit Integer"),
                    ("24", "24-bit Integer"),
                    ("32", "32-bit Integer"),
                    ("float", "32-bit Float"),
                ];
                for (id, label) in depths {
                    let is_sel = curr_bd == id;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.audio_bit_depth = id.to_string();
                        if let Some(p) = player {
                            p.set_audio_format(id);
                        }
                        let _ = config.save();
                        actions.close_menu = true;
                        ui.close();
                    }
                }
            });

            submenu_item(ui, &skin, "⚡", "Resampler Quality", |ui| {
                ui.set_min_width(165.0);
                let curr_q = config.resampler_quality.clone();
                let qualities = [
                    ("ultra", "Ultra High (64-tap Sinc)"),
                    ("high", "High Quality (32-tap SoX)"),
                    ("fast", "Fast (16-tap Sinc)"),
                ];
                for (id, label) in qualities {
                    let is_sel = curr_q == id;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.resampler_quality = id.to_string();
                        if let Some(p) = player {
                            p.set_resampler_quality(id);
                        }
                        let _ = config.save();
                        actions.close_menu = true;
                        ui.close();
                    }
                }
            });

            submenu_item(ui, &skin, "📊", "18-Band Equalizer Presets", |ui| {
                ui.set_min_width(150.0);
                for preset in VORTEX_EQ_PRESETS {
                    let is_sel = config.eq_preset == preset.name;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, preset.name, "", is_sel, false).clicked() {
                        config.eq_preset = preset.name.to_string();
                        config.eq_bands = preset.bands.to_vec();
                        config.eq_enabled = true;
                        if let Some(p) = player {
                            p.set_equalizer(true, preset.bands);
                        }
                        let _ = config.save();
                        ui.close();
                    }
                }
            });

            if menu_item(ui, &skin, "📈", "Parametric Equalizer (PEQ)...", "F9", false, false).clicked() {
                actions.toggle_control_panel = true;
                ui.close();
            }

            menu_separator(ui);

            if menu_item(ui, &skin, "〰", "Volume Normalizer", "", config.audio_normalize, false).clicked() {
                config.audio_normalize = !config.audio_normalize;
                if let Some(p) = player {
                    p.set_audio_normalize(config.audio_normalize);
                }
                let _ = config.save();
                actions.close_menu = true;
                ui.close();
            }
            #[cfg(windows)]
            if menu_item(ui, &skin, "🗔", "WASAPI Exclusive", "", config.wasapi_exclusive, false).clicked() {
                config.wasapi_exclusive = !config.wasapi_exclusive;
                if let Some(p) = player {
                    p.set_wasapi_exclusive(config.wasapi_exclusive);
                }
                let _ = config.save();
                actions.close_menu = true;
                ui.close();
            }
            submenu_item(ui, &skin, "🧊", "3D Reverb Space", |ui| {
                ui.set_min_width(140.0);
                let reverb_modes = [
                    ("off", "Off (Bypass)"),
                    ("studio", "Studio Room"),
                    ("living_room", "Living Room"),
                    ("concert_hall", "Concert Hall"),
                    ("arena", "Arena / Stadium"),
                ];
                for (rev_id, label) in reverb_modes {
                    let is_sel = config.audio_reverb == rev_id;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.audio_reverb = rev_id.to_string();
                        if let Some(p) = player {
                            p.set_audio_reverb(rev_id);
                        }
                        let _ = config.save();
                        actions.close_menu = true;
                        ui.close();
                    }
                }
            });
            let passthrough_icon = if config.audio_passthrough { "✓" } else { " " };
            if menu_item(ui, &skin, passthrough_icon, "HDMI / S/PDIF Bitstream Passthrough", "", config.audio_passthrough, false).clicked() {
                config.audio_passthrough = !config.audio_passthrough;
                if let Some(p) = player {
                    p.set_audio_passthrough(config.audio_passthrough);
                }
                let _ = config.save();
                actions.close_menu = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎛️", "VST2 / VST3 & Winamp DSP Host...", "Ctrl+Alt+V", false, false).clicked() {
                actions.toggle_vst_winamp = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎧", "BS2B Crossfeed & HRTF (Headphone)...", "Ctrl+Alt+H", false, false).clicked() {
                actions.toggle_binaural_crossfeed = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🔊", "Audio Compressor & Night Mode...", "Ctrl+Shift+N", false, false).clicked() {
                actions.toggle_audio_compressor = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎙️", "AI Voice Isolation & RNNoise...", "Ctrl+Alt+6", false, false).clicked() {
                actions.toggle_ai_audio = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📊", "EBU R128 Loudness Radar...", "Ctrl+Alt+7", false, false).clicked() {
                actions.toggle_loudness_radar = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎵", "Pitch & Formant Shifter...", "Ctrl+Alt+P", false, false).clicked() {
                actions.toggle_pitch_formant = true;
                ui.close();
            }

            menu_separator(ui);






            if menu_item(ui, &skin, "🔇", "Mute / Unmute", "M", stats.is_muted, false).clicked() {
                if let Some(p) = player {
                    p.toggle_mute();
                }
                ui.close();
            }
            if menu_item(ui, &skin, "🕒", "Audio Sync -50ms", "Shift+D", false, false).clicked() {
                if let Some(p) = player {
                    p.adjust_audio_delay(-0.05);
                }
                ui.close();
            }
            if menu_item(ui, &skin, "🕒", "Audio Sync +50ms", "Shift+F", false, false).clicked() {
                if let Some(p) = player {
                    p.adjust_audio_delay(0.05);
                }
                ui.close();
            }
        });

        // =====================================================================
        // 6. FILTERS & SKINS & MISC
        // =====================================================================
        submenu_item(ui, &skin, "⚗", "Filters", |ui| {
            ui.set_min_width(145.0);
            if menu_item(ui, &skin, "⚡", "Deinterlace (Yadif 2x)", "", config.video_deinterlace, false).clicked() {
                config.video_deinterlace = !config.video_deinterlace;
                if let Some(p) = player {
                    p.set_property_string("deinterlace", if config.video_deinterlace { "yes" } else { "no" });
                }
                let _ = config.save();
                ui.close();
            }
            if menu_item(ui, &skin, "🎨", "Color Temperature", "", false, false).clicked() {
                actions.toggle_control_panel = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🧹", "Denoise (NL-Means)", "", false, false).clicked() {
                if let Some(p) = player {
                    p.set_property_string("vf", "hqdn3d");
                }
                ui.close();
            }
            if menu_item(ui, &skin, "🔍", "Sharpen (Unsharp)", "", config.video_sharpen > 0.01, false).clicked() {
                config.video_sharpen = if config.video_sharpen > 0.01 { 0.0 } else { 1.5 };
                if let Some(p) = player {
                    p.set_property_double("sharpen", config.video_sharpen as f64);
                }
                let _ = config.save();
                ui.close();
            }
            if menu_item(ui, &skin, "🌑", "Vignette Filter", "", false, false).clicked() {
                if let Some(p) = player {
                    p.set_property_string("vf", "vignette=PI/4");
                }
                ui.close();
            }
        });

        submenu_item(ui, &skin, "🖌", "Skins", |ui| {
            ui.set_min_width(140.0);
            for mode in ThemeMode::ALL {
                let is_sel = config.theme_mode == mode;
                if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, mode.display_name(), "", is_sel, false).clicked() {
                    config.theme_mode = mode;
                    let _ = config.save();
                    VortexTheme::apply(ui.ctx(), mode);
                    ui.close();
                }
            }
        });

        submenu_item(ui, &skin, "•••", "Misc", |ui| {
            ui.set_min_width(150.0);
            if menu_item(ui, &skin, if config.always_on_top { "✓" } else { " " }, "Always on Top", "Ctrl+T", config.always_on_top, false).clicked() {
                config.always_on_top = !config.always_on_top;
                let _ = config.save();
                ui.close();
            }
            if menu_item(ui, &skin, if config.remember_window_size { "✓" } else { " " }, "Remember Position", "", config.remember_window_size, false).clicked() {
                config.remember_window_size = !config.remember_window_size;
                let _ = config.save();
                ui.close();
            }
            if menu_item(ui, &skin, if config.auto_resume { "✓" } else { " " }, "Auto-Resume", "", config.auto_resume, false).clicked() {
                config.auto_resume = !config.auto_resume;
                let _ = config.save();
                ui.close();
            }
            if menu_item(ui, &skin, if config.auto_load_next_episode { "✓" } else { " " }, "Auto-Load Next", "", config.auto_load_next_episode, false).clicked() {
                config.auto_load_next_episode = !config.auto_load_next_episode;
                let _ = config.save();
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "🪟", "Detached Windows...", "Ctrl+Alt+D", false, false).clicked() {
                actions.toggle_detached_windows = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎮", "Discord RPC...", "Ctrl+Alt+R", false, false).clicked() {
                actions.toggle_discord_rpc = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📱", "Web Remote...", "Ctrl+Alt+W", false, false).clicked() {
                actions.toggle_web_remote = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🎬", "Trakt / AniList...", "Ctrl+Alt+S", false, false).clicked() {
                actions.toggle_scrobbler = true;
                ui.close();
            }
            if menu_item(ui, &skin, "💡", "Ambilight RGB...", "Ctrl+Alt+L", false, false).clicked() {
                actions.toggle_ambilight = true;
                ui.close();
            }
            if menu_item(ui, &skin, "📡", "Cast / AirPlay...", "Ctrl+Alt+C", false, false).clicked() {
                actions.toggle_cast_renderer = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🛡️", "Boss Key...", "Ctrl+Alt+K", false, false).clicked() {
                actions.toggle_boss_key = true;
                ui.close();
            }
        });

        menu_separator(ui);

        // =====================================================================
        // 7. WINDOW SIZING & FULLSCREEN
        // =====================================================================

        submenu_item(ui, &skin, "🗖", "Aspect Ratio", |ui| {
            ui.set_min_width(135.0);
            for &ratio in &["auto", "16:9", "16:10", "4:3", "1.85:1", "2.35:1", "crop_fill", "fill"] {
                let is_sel = config.aspect_ratio == ratio;
                let label = match ratio {
                    "auto" => "Auto (Keep AR)",
                    "16:9" => "16:9 Wide",
                    "16:10" => "16:10 PC",
                    "4:3" => "4:3 Standard",
                    "1.85:1" => "1.85:1 Cinema",
                    "2.35:1" => "2.35:1 Anamorphic",
                    "crop_fill" => "Crop to Fill Screen (No Black Bars)",
                    "fill" => "Stretch to Fit Screen (No Black Bars)",
                    _ => ratio,
                };
                if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                    config.aspect_ratio = ratio.to_string();
                    if let Some(p) = player {
                        p.set_aspect_ratio(ratio);
                    }
                    let _ = config.save();
                    ui.close();
                }
            }
            ui.separator();
            submenu_item(ui, &skin, "↕", "Black Bar Placement", |ui| {
                ui.set_min_width(175.0);
                let aligns = [
                    ("center", "Both (Half & Half - Default)"),
                    ("top", "Bottom (All bar at Bottom)"),
                    ("bottom", "Top (All bar at Top)"),
                ];
                for (align, label) in aligns {
                    let is_sel = config.video_align_y == align;
                    if menu_item(ui, &skin, if is_sel { "✓" } else { " " }, label, "", is_sel, false).clicked() {
                        config.video_align_y = align.to_string();
                        if let Some(p) = player {
                            p.set_video_align_y(align);
                        }
                        let _ = config.save();
                        ui.close();
                    }
                }
            });
        });

        submenu_item(ui, &skin, "🗔", "Window Size", |ui| {
            ui.set_min_width(170.0);
            if menu_item(ui, &skin, " ", "50% (Half Size)", "Alt+1", false, false).clicked() {
                actions.resize_window_factor = Some(0.5);
                ui.close();
            }
            if menu_item(ui, &skin, " ", "100% (Original Size)", "Alt+2", false, false).clicked() {
                actions.resize_window_factor = Some(1.0);
                ui.close();
            }
            if menu_item(ui, &skin, " ", "150% (1.5× Size)", "Alt+3", false, false).clicked() {
                actions.resize_window_factor = Some(1.5);
                ui.close();
            }
            if menu_item(ui, &skin, " ", "200% (Double Size)", "Alt+4", false, false).clicked() {
                actions.resize_window_factor = Some(2.0);
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "🎯", "Center Window (Fit Monitor AR)", "Ctrl+Alt+C", false, false).clicked() {
                actions.center_window = true;
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, " ", "1280 × 720 (HD)", "", false, false).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(1280.0, 720.0)));
                ui.close();
            }
            if menu_item(ui, &skin, " ", "1920 × 1080 (FHD)", "", false, false).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(1920.0, 1080.0)));
                ui.close();
            }
            if menu_item(ui, &skin, " ", "2560 × 1440 (2K)", "", false, false).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(2560.0, 1440.0)));
                ui.close();
            }
            menu_separator(ui);
            if menu_item(ui, &skin, "📌", "Picture-in-Picture (PiP)...", "Ctrl+P", false, false).clicked() {
                actions.toggle_pip = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🗕", "Minimize Window", "Alt+Space+N", false, false).clicked() {
                actions.minimize_window = true;
                ui.close();
            }
            if menu_item(ui, &skin, "🗖", "Maximize / Restore Window", "Alt+Space+X", false, false).clicked() {
                actions.maximize_window = true;
                ui.close();
            }
        });

        if menu_item(ui, &skin, "⛶", "Fullscreen", "Enter", false, false).clicked() {
            actions.toggle_fullscreen = true;
            ui.close();
        }

        menu_separator(ui);

        // =====================================================================
        // 8. PREFERENCES, PLAYLIST, CONTROL PANEL, DIAGNOSTICS & ABOUT
        // =====================================================================
        if menu_item(ui, &skin, "⚙", "Preferences...", "F5", false, false).clicked() {
            actions.toggle_preferences = true;
            ui.close();
        }

        if menu_item(ui, &skin, "≣", "Playlist...", "F6", false, false).clicked() {
            actions.toggle_playlist = true;
            ui.close();
        }

        if menu_item(ui, &skin, "⚙", "Control Panel...", "F7", false, false).clicked() {
            actions.toggle_control_panel = true;
            ui.close();
        }

        if menu_item(ui, &skin, "ⓘ", "Playback Info...", "Ctrl+F1", false, false).clicked() {
            actions.toggle_media_info = true;
            ui.close();
        }

        if menu_item(ui, &skin, "📄", "File Info (MediaInfo)...", "Ctrl+Shift+F1", false, false).clicked() {
            actions.open_file_info = true;
            ui.close();
        }

        if menu_item(ui, &skin, "ⓘ", "About...", "F1", false, false).clicked() {
            actions.show_about = true;
            ui.close();
        }

        menu_separator(ui);

        // =====================================================================
        // 9. EXIT
        // =====================================================================
        if menu_item(ui, &skin, "⏻", "Exit", "Alt+F4", false, false).clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            ui.close();
        }

        actions
    }
}
