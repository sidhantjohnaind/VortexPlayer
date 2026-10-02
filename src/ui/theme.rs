#![allow(dead_code)]

use eframe::egui::{self, Color32, CornerRadius, Margin, Stroke, Vec2};
pub use crate::config::ThemeMode;

#[derive(Debug, Clone, Copy)]
pub struct SkinColors {
    pub bg_canvas: Color32,
    pub bg_titlebar: Color32,
    pub bg_toolbar: Color32,
    pub bg_panel: Color32,
    pub bg_btn: Color32,
    pub bg_btn_hover: Color32,
    pub bg_btn_active: Color32,
    pub accent_primary: Color32,
    pub accent_bright: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    pub border_dark: Color32,
    pub border_light: Color32,
}

pub struct VortexTheme;

impl VortexTheme {
    pub const POT_YELLOW: Color32        = Color32::from_rgb(245, 166, 35);
    pub const POT_YELLOW_BRIGHT: Color32 = Color32::from_rgb(255, 185, 45);
    pub const POT_CYAN: Color32          = Color32::from_rgb(60, 165, 240);
    pub const POT_LIME_OSD: Color32      = Color32::from_rgb(0, 255, 80);
    pub const POT_RED: Color32           = Color32::from_rgb(235, 40, 40);

    // Compatibility aliases
    pub const VORTEX_PURPLE: Color32     = Self::POT_YELLOW;
    pub const VORTEX_CYAN: Color32       = Self::POT_CYAN;
    pub const VORTEX_BLUE: Color32       = Color32::from_rgb(60, 140, 240);
    pub const TEXT_PRIMARY: Color32      = Color32::from_rgb(230, 233, 240);
    pub const TEXT_SECONDARY: Color32    = Color32::from_rgb(155, 160, 175);
    pub const TEXT_MUTED: Color32        = Color32::from_rgb(100, 105, 118);
    pub const BORDER_DARK: Color32       = Color32::from_rgb(32, 34, 42);
    pub const BORDER_BASE: Color32       = Self::BORDER_DARK;
    pub const BG_CARD: Color32           = Color32::from_rgb(32, 34, 40);
    pub const BG_ACTIVE: Color32         = Color32::from_rgb(58, 62, 75);
    pub const BG_HOVER: Color32          = Color32::from_rgb(46, 50, 60);

    pub fn get_skin(mode: ThemeMode) -> SkinColors {
        match mode {
            ThemeMode::PotPlayerClassic => SkinColors {
                bg_canvas: Color32::from_rgb(12, 13, 15),
                bg_titlebar: Color32::from_rgb(22, 23, 27),
                bg_toolbar: Color32::from_rgb(24, 25, 30),
                bg_panel: Color32::from_rgb(18, 19, 23),
                bg_btn: Color32::from_rgb(32, 34, 40),
                bg_btn_hover: Color32::from_rgb(46, 50, 60),
                bg_btn_active: Color32::from_rgb(60, 65, 78),
                accent_primary: Color32::from_rgb(245, 166, 35),
                accent_bright: Color32::from_rgb(255, 185, 45),
                text_primary: Color32::from_rgb(230, 233, 240),
                text_secondary: Color32::from_rgb(155, 160, 175),
                text_muted: Color32::from_rgb(100, 105, 118),
                border_dark: Color32::from_rgb(32, 34, 42),
                border_light: Color32::from_rgb(48, 52, 64),
            },
            ThemeMode::OnyxDiamond => SkinColors {
                bg_canvas: Color32::from_rgb(0, 0, 0),
                bg_titlebar: Color32::from_rgb(10, 10, 10),
                bg_toolbar: Color32::from_rgb(12, 12, 12),
                bg_panel: Color32::from_rgb(8, 8, 8),
                bg_btn: Color32::from_rgb(20, 20, 20),
                bg_btn_hover: Color32::from_rgb(35, 35, 35),
                bg_btn_active: Color32::from_rgb(50, 50, 50),
                accent_primary: Color32::from_rgb(0, 230, 118),
                accent_bright: Color32::from_rgb(50, 255, 150),
                text_primary: Color32::from_rgb(240, 240, 240),
                text_secondary: Color32::from_rgb(160, 160, 160),
                text_muted: Color32::from_rgb(90, 90, 90),
                border_dark: Color32::from_rgb(24, 24, 24),
                border_light: Color32::from_rgb(40, 40, 40),
            },
            ThemeMode::MidnightNavy => SkinColors {
                bg_canvas: Color32::from_rgb(8, 12, 22),
                bg_titlebar: Color32::from_rgb(14, 20, 35),
                bg_toolbar: Color32::from_rgb(16, 24, 42),
                bg_panel: Color32::from_rgb(10, 16, 28),
                bg_btn: Color32::from_rgb(22, 34, 60),
                bg_btn_hover: Color32::from_rgb(32, 48, 82),
                bg_btn_active: Color32::from_rgb(45, 65, 110),
                accent_primary: Color32::from_rgb(0, 180, 255),
                accent_bright: Color32::from_rgb(50, 210, 255),
                text_primary: Color32::from_rgb(225, 235, 250),
                text_secondary: Color32::from_rgb(145, 165, 195),
                text_muted: Color32::from_rgb(90, 105, 130),
                border_dark: Color32::from_rgb(24, 38, 65),
                border_light: Color32::from_rgb(38, 56, 95),
            },
            ThemeMode::CyberpunkNeon => SkinColors {
                bg_canvas: Color32::from_rgb(14, 8, 26),
                bg_titlebar: Color32::from_rgb(24, 14, 44),
                bg_toolbar: Color32::from_rgb(28, 16, 52),
                bg_panel: Color32::from_rgb(18, 10, 34),
                bg_btn: Color32::from_rgb(40, 20, 75),
                bg_btn_hover: Color32::from_rgb(58, 28, 105),
                bg_btn_active: Color32::from_rgb(80, 38, 145),
                accent_primary: Color32::from_rgb(236, 72, 153),
                accent_bright: Color32::from_rgb(244, 114, 182),
                text_primary: Color32::from_rgb(245, 230, 255),
                text_secondary: Color32::from_rgb(185, 155, 215),
                text_muted: Color32::from_rgb(120, 95, 150),
                border_dark: Color32::from_rgb(45, 25, 80),
                border_light: Color32::from_rgb(68, 38, 120),
            },
            ThemeMode::TitaniumSilver => SkinColors {
                bg_canvas: Color32::from_rgb(18, 20, 24),
                bg_titlebar: Color32::from_rgb(30, 33, 40),
                bg_toolbar: Color32::from_rgb(34, 38, 46),
                bg_panel: Color32::from_rgb(24, 27, 33),
                bg_btn: Color32::from_rgb(44, 49, 60),
                bg_btn_hover: Color32::from_rgb(60, 66, 80),
                bg_btn_active: Color32::from_rgb(76, 84, 102),
                accent_primary: Color32::from_rgb(56, 189, 248),
                accent_bright: Color32::from_rgb(125, 211, 252),
                text_primary: Color32::from_rgb(240, 243, 248),
                text_secondary: Color32::from_rgb(165, 175, 190),
                text_muted: Color32::from_rgb(110, 120, 135),
                border_dark: Color32::from_rgb(45, 50, 62),
                border_light: Color32::from_rgb(65, 72, 88),
            },
            ThemeMode::EmeraldForest => SkinColors {
                bg_canvas: Color32::from_rgb(10, 16, 14),
                bg_titlebar: Color32::from_rgb(16, 28, 24),
                bg_toolbar: Color32::from_rgb(20, 34, 28),
                bg_panel: Color32::from_rgb(13, 22, 19),
                bg_btn: Color32::from_rgb(26, 46, 38),
                bg_btn_hover: Color32::from_rgb(38, 66, 54),
                bg_btn_active: Color32::from_rgb(50, 86, 70),
                accent_primary: Color32::from_rgb(16, 185, 129),
                accent_bright: Color32::from_rgb(52, 211, 153),
                text_primary: Color32::from_rgb(230, 245, 238),
                text_secondary: Color32::from_rgb(150, 185, 170),
                text_muted: Color32::from_rgb(95, 125, 112),
                border_dark: Color32::from_rgb(28, 48, 40),
                border_light: Color32::from_rgb(42, 70, 58),
            },
            ThemeMode::CrimsonRuby => SkinColors {
                bg_canvas: Color32::from_rgb(18, 8, 12),
                bg_titlebar: Color32::from_rgb(30, 14, 20),
                bg_toolbar: Color32::from_rgb(36, 16, 24),
                bg_panel: Color32::from_rgb(24, 10, 16),
                bg_btn: Color32::from_rgb(50, 20, 32),
                bg_btn_hover: Color32::from_rgb(72, 28, 46),
                bg_btn_active: Color32::from_rgb(96, 36, 60),
                accent_primary: Color32::from_rgb(244, 63, 94),
                accent_bright: Color32::from_rgb(251, 113, 133),
                text_primary: Color32::from_rgb(250, 230, 235),
                text_secondary: Color32::from_rgb(195, 150, 165),
                text_muted: Color32::from_rgb(135, 95, 108),
                border_dark: Color32::from_rgb(52, 24, 35),
                border_light: Color32::from_rgb(78, 35, 52),
            },
            ThemeMode::NordicFrost => SkinColors {
                bg_canvas: Color32::from_rgb(14, 17, 23),
                bg_titlebar: Color32::from_rgb(24, 28, 38),
                bg_toolbar: Color32::from_rgb(28, 34, 46),
                bg_panel: Color32::from_rgb(19, 23, 31),
                bg_btn: Color32::from_rgb(36, 44, 58),
                bg_btn_hover: Color32::from_rgb(50, 60, 80),
                bg_btn_active: Color32::from_rgb(65, 78, 104),
                accent_primary: Color32::from_rgb(125, 211, 252),
                accent_bright: Color32::from_rgb(186, 230, 253),
                text_primary: Color32::from_rgb(241, 245, 249),
                text_secondary: Color32::from_rgb(160, 175, 195),
                text_muted: Color32::from_rgb(105, 118, 136),
                border_dark: Color32::from_rgb(38, 46, 62),
                border_light: Color32::from_rgb(55, 68, 90),
            },
        }
    }

    pub fn apply(ctx: &egui::Context, mode: ThemeMode) {
        let skin = Self::get_skin(mode);
        let mut visuals = egui::Visuals::dark();

        visuals.override_text_color                  = Some(skin.text_primary);
        visuals.panel_fill                           = skin.bg_canvas;
        visuals.window_fill                          = Color32::from_rgb(0, 0, 0);
        visuals.extreme_bg_color                     = Color32::from_rgb(0, 0, 0);
        visuals.window_stroke                        = Stroke::new(1.0, Color32::from_rgb(45, 50, 65));
        visuals.window_corner_radius                 = CornerRadius::same(4);
        visuals.menu_corner_radius                   = CornerRadius::same(4);

        visuals.window_shadow = egui::Shadow {
            offset: [0, 6],
            blur: 24,
            spread: 2,
            color: Color32::from_black_alpha(200),
        };
        visuals.popup_shadow = egui::Shadow {
            offset: [0, 6],
            blur: 24,
            spread: 2,
            color: Color32::from_black_alpha(200),
        };

        // Tooltip & Non-interactive widgets styling
        visuals.widgets.noninteractive.bg_fill       = Color32::from_rgb(10, 10, 14);
        visuals.widgets.noninteractive.bg_stroke     = Stroke::new(1.0, Color32::from_rgb(45, 50, 65));
        visuals.widgets.noninteractive.fg_stroke     = Stroke::new(1.0, Color32::from_rgb(240, 242, 248));
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(4);

        visuals.widgets.inactive.bg_fill             = skin.bg_btn;
        visuals.widgets.inactive.bg_stroke           = Stroke::new(1.0, skin.border_dark);
        visuals.widgets.inactive.fg_stroke           = Stroke::new(1.0, skin.text_primary);
        visuals.widgets.inactive.corner_radius       = CornerRadius::same(3);

        visuals.widgets.hovered.bg_fill              = Color32::from_rgb(34, 42, 60);
        visuals.widgets.hovered.bg_stroke            = Stroke::new(1.0, skin.border_light);
        visuals.widgets.hovered.fg_stroke            = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.hovered.corner_radius        = CornerRadius::same(3);

        visuals.widgets.active.bg_fill               = Color32::from_rgb(42, 58, 90);
        visuals.widgets.active.bg_stroke             = Stroke::new(1.0, skin.accent_primary);
        visuals.widgets.active.fg_stroke             = Stroke::new(1.0, Color32::WHITE);
        visuals.widgets.active.corner_radius         = CornerRadius::same(3);

        visuals.selection.bg_fill                    = Color32::from_rgb(38, 70, 130);
        visuals.selection.stroke                     = Stroke::new(1.0, skin.accent_primary);

        ctx.set_visuals(visuals);

        ctx.style_mut_of(egui::Theme::Dark, |style| {
            style.animation_time         = 0.0; // Instant snappy native response - zero sliding motion
            style.spacing.item_spacing   = Vec2::new(4.0, 4.0);
            style.spacing.window_margin  = Margin::symmetric(4, 4);
            style.spacing.button_padding = Vec2::new(6.0, 3.0);
            style.spacing.tooltip_width  = 380.0;
            style.spacing.icon_spacing   = 4.0;
        });
    }
}
