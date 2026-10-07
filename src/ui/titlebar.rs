//! titlebar.rs — Pixel-perfect Vortex titlebar matching media_1787017395599.png.

use super::icons::Icons;
use super::theme::VortexTheme;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, Vec2};

#[derive(Default, Clone, Debug)]
pub struct TitleBarResponse {
    pub tooltip: Option<(Pos2, String)>,
    pub drag_started: bool,
    pub double_clicked: bool,
    pub toggle_maximize: bool,
}

pub struct TitleBar;

impl TitleBar {
    pub const HEIGHT: f32 = 30.0;

    /// Render the custom title bar.
    pub fn render(
        ui: &mut egui::Ui,
        title: &str,
        codec_badge: Option<&str>,
        always_on_top: &mut bool,
        show_menu: &mut bool,
        menu_anchor_pos: &mut Option<(i32, i32)>,
        menu_pos_out: &mut Option<Pos2>,
        is_fullscreen: bool,
        is_maximized: bool,
        theme_mode: crate::config::ThemeMode,
    ) -> TitleBarResponse {
        let mut hovered_tooltip: Option<(Pos2, String)> = None;
        let mut drag_started = false;
        let mut double_clicked = false;
        let mut toggle_maximize = false;

        let (rect, _bar_resp) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), Self::HEIGHT),
            Sense::hover(),
        );

        let skin = VortexTheme::get_skin(theme_mode);
        let painter = ui.painter();

        // ── 1. Themed Background & Divider ──────────────────────────────
        painter.rect_filled(rect, CornerRadius::ZERO, skin.bg_titlebar);
        
        // Bottom fine divider
        painter.line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            Stroke::new(1.0, skin.border_dark),
        );

        // ── 2. Left: VortexPlayer ⌵ Brand Button ───────────────────────────────
        let mut cursor_left = rect.left() + 6.0;

        let brand_w = 122.0;
        let brand_rect = Rect::from_min_size(
            Pos2::new(cursor_left, rect.top() + 2.0),
            Vec2::new(brand_w, Self::HEIGHT - 4.0),
        );
        let brand_resp = ui.interact(brand_rect, ui.id().with("tb_brand_btn"), Sense::click());
        
        if brand_resp.hovered() {
            painter.rect_filled(
                brand_rect,
                CornerRadius::same(2),
                skin.bg_btn_hover,
            );
            hovered_tooltip = Some((Pos2::new(brand_rect.center().x, brand_rect.bottom() + 4.0), "VortexPlayer Main Menu".to_string()));
        }

        // Gold Vortex Logo Icon
        Icons::draw_vertex_logo(
            painter,
            Pos2::new(brand_rect.left() + 11.0, brand_rect.center().y),
            6.5,
        );

        // "VortexPlayer" Text
        painter.text(
            Pos2::new(brand_rect.left() + 22.0, brand_rect.center().y),
            Align2::LEFT_CENTER,
            "VortexPlayer",
            FontId::proportional(12.0),
            if brand_resp.hovered() {
                Color32::WHITE
            } else {
                skin.text_primary
            },
        );

        // Chevron ⌵
        Icons::draw_chevron_down(
            painter,
            Pos2::new(brand_rect.right() - 10.0, brand_rect.center().y),
            3.0,
            if brand_resp.hovered() {
                Color32::WHITE
            } else {
                skin.text_secondary
            },
        );

        if brand_resp.clicked() {
            *show_menu = !*show_menu;
            *menu_pos_out = Some(Pos2::new(brand_rect.left(), brand_rect.bottom() + 2.0));
            // Screen coords for menu anchoring (convert egui logical points to physical client pixels)
            let ppp = ui.ctx().pixels_per_point();
            let client_pos = brand_rect.left_bottom();
            *menu_anchor_pos = Some(((client_pos.x * ppp).round() as i32, (client_pos.y * ppp).round() as i32));
        }

        cursor_left += brand_w + 6.0;

        // ── Right Side Window Controls Geometry Calculation ────────────────
        let btn_w = 32.0;
        let btn_h = Self::HEIGHT - 4.0;
        let btn_y = rect.top() + 2.0;
        let right_controls_width = 4.0 * btn_w + 3.0 * 2.0 + 8.0;
        let drag_right = rect.right() - right_controls_width;

        // Middle Drag Region: Only drags when clicking empty title area (never on buttons!)
        let drag_rect = Rect::from_min_max(
            Pos2::new(cursor_left, rect.top()),
            Pos2::new(drag_right, rect.bottom()),
        );
        let drag_resp = ui.interact(drag_rect, ui.id().with("tb_drag_area"), Sense::click_and_drag());
        if drag_resp.double_clicked() {
            double_clicked = true;
        } else if drag_resp.drag_started() {
            drag_started = true;
        }

        let is_idle_title = title.is_empty() || title == "VertexPlayer" || title == "Ready" || title == "VortexPlayer";

        if !is_idle_title {
            // ── 3. Vertical Separator ───────────────────────────────────────────
            painter.line_segment(
                [
                    Pos2::new(cursor_left, rect.top() + 6.0),
                    Pos2::new(cursor_left, rect.bottom() - 6.0),
                ],
                Stroke::new(1.0, skin.border_dark),
            );
            cursor_left += 8.0;

            // ── 4. Codec Badge (FLAC, HEVC, AVC, etc.) ──────────────────────────
            if let Some(badge) = codec_badge {
                let badge_text = badge.trim();
                if !badge_text.is_empty() {
                    let badge_w = badge_text.len() as f32 * 7.0 + 8.0;
                    painter.text(
                        Pos2::new(cursor_left, rect.center().y),
                        Align2::LEFT_CENTER,
                        badge_text,
                        FontId::monospace(11.0),
                        skin.accent_primary,
                    );
                    cursor_left += badge_w + 8.0;

                    // Secondary Divider
                    painter.line_segment(
                        [
                            Pos2::new(cursor_left, rect.top() + 6.0),
                            Pos2::new(cursor_left, rect.bottom() - 6.0),
                        ],
                        Stroke::new(1.0, skin.border_dark),
                    );
                    cursor_left += 8.0;
                }
            }

            // ── 5. Media Title with Track Index ([8/22] Title.ext) ──────────────
            let max_title_w = (drag_right - cursor_left - 8.0).max(50.0);

            let mut clipped_title = title.to_string();
            let max_chars = ((max_title_w / 7.2) as usize).max(8);
            if clipped_title.chars().count() > max_chars {
                clipped_title = format!("{}...", clipped_title.chars().take(max_chars.saturating_sub(3)).collect::<String>());
            }

            painter.text(
                Pos2::new(cursor_left, rect.center().y),
                Align2::LEFT_CENTER,
                clipped_title,
                FontId::proportional(12.0),
                skin.text_primary,
            );
        }

        // ── 6. Right Side Window Controls (Pin, Min, Max, Close) ────────────
        let mut btn_right = rect.right() - 4.0;

        // Close Button (✕)
        let close_rect = Rect::from_min_size(Pos2::new(btn_right - btn_w, btn_y), Vec2::new(btn_w, btn_h));
        let close_resp = ui.interact(close_rect, ui.id().with("tb_close"), Sense::click());
        if close_resp.hovered() {
            painter.rect_filled(close_rect, CornerRadius::same(2), Color32::from_rgb(205, 45, 45));
            hovered_tooltip = Some((Pos2::new(close_rect.center().x, close_rect.bottom() + 4.0), "Close (Alt+F4)".to_string()));
        }
        Icons::draw_close(
            painter,
            close_rect,
            if close_resp.hovered() { Color32::WHITE } else { skin.text_primary },
        );
        if close_resp.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
        btn_right -= btn_w + 2.0;

        // Maximize Button (◻)
        let is_max = is_fullscreen || is_maximized || ui.input(|i| i.viewport().maximized.unwrap_or(false));
        let max_rect = Rect::from_min_size(Pos2::new(btn_right - btn_w, btn_y), Vec2::new(btn_w, btn_h));
        let max_resp = ui.interact(max_rect, ui.id().with("tb_max"), Sense::click());
        if max_resp.hovered() {
            painter.rect_filled(max_rect, CornerRadius::same(2), skin.bg_btn_hover);
            hovered_tooltip = Some((Pos2::new(max_rect.center().x, max_rect.bottom() + 4.0), if is_max { "Restore Window".to_string() } else { "Maximize Window".to_string() }));
        }
        Icons::draw_maximize(
            painter,
            max_rect,
            if max_resp.hovered() { Color32::WHITE } else { skin.text_secondary },
            is_max,
        );
        if max_resp.clicked() {
            toggle_maximize = true;
        }
        btn_right -= btn_w + 2.0;

        // Minimize Button (―)
        let min_rect = Rect::from_min_size(Pos2::new(btn_right - btn_w, btn_y), Vec2::new(btn_w, btn_h));
        let min_resp = ui.interact(min_rect, ui.id().with("tb_min"), Sense::click());
        if min_resp.hovered() {
            painter.rect_filled(min_rect, CornerRadius::same(2), skin.bg_btn_hover);
            hovered_tooltip = Some((Pos2::new(min_rect.center().x, min_rect.bottom() + 4.0), "Minimize Window".to_string()));
        }
        Icons::draw_minimize(
            painter,
            min_rect,
            if min_resp.hovered() { Color32::WHITE } else { skin.text_secondary },
        );
        if min_resp.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
        btn_right -= btn_w + 2.0;

        // Always On Top Pin Button (📌)
        let pin_rect = Rect::from_min_size(Pos2::new(btn_right - btn_w, btn_y), Vec2::new(btn_w, btn_h));
        let pin_resp = ui.interact(pin_rect, ui.id().with("tb_pin"), Sense::click());
        if pin_resp.hovered() {
            painter.rect_filled(pin_rect, CornerRadius::same(2), skin.bg_btn_hover);
            hovered_tooltip = Some((Pos2::new(pin_rect.center().x, pin_rect.bottom() + 4.0), if *always_on_top { "Always On Top: ON (Click to toggle)".to_string() } else { "Always On Top: OFF (Click to toggle)".to_string() }));
        }
        let pin_color = if *always_on_top {
            skin.accent_primary
        } else if pin_resp.hovered() {
            Color32::WHITE
        } else {
            skin.text_muted
        };
        Icons::draw_pin(painter, pin_rect, pin_color, *always_on_top);
        if pin_resp.clicked() {
            *always_on_top = !*always_on_top;
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                if *always_on_top { egui::WindowLevel::AlwaysOnTop } else { egui::WindowLevel::Normal }
            ));
        }

        TitleBarResponse {
            tooltip: hovered_tooltip,
            drag_started,
            double_clicked,
            toggle_maximize,
        }
    }
}
