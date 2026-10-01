//! preferences_dialog.rs — Advanced In-Depth PotPlayer-Style Preferences Studio

#![allow(dead_code)]

use super::theme::VortexTheme;
use crate::config::{AppConfig, DoubleClickAction, PlaylistEndAction, ThemeMode};
use crate::engine::audio_dsp::{FREQS, POT_EQ_PRESETS};
use crate::engine::shaders::{Anime4kPreset, VideoScaler};
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Vec2, ViewportBuilder, ViewportId};

/// Modern Fluent-style Animated Toggle Switch
pub fn fluent_switch(ui: &mut egui::Ui, value: &mut bool) -> egui::Response {
    let desired_size = Vec2::new(38.0, 20.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *value = !*value;
        response.mark_changed();
    }
    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *value);
        let track_color = if *value {
            VortexTheme::POT_YELLOW
        } else if response.hovered() {
            Color32::from_rgb(55, 60, 75)
        } else {
            Color32::from_rgb(38, 42, 54)
        };
        let painter = ui.painter();
        painter.rect_filled(rect, CornerRadius::same(10), track_color);
        painter.rect_stroke(rect, CornerRadius::same(10), Stroke::new(1.0, Color32::from_rgb(60, 66, 84)), StrokeKind::Inside);
        let radius = 7.0;
        let x = egui::lerp((rect.min.x + 10.0)..=(rect.max.x - 10.0), how_on);
        let center = Pos2::new(x, rect.center().y);
        let knob_color = if *value {
            Color32::from_rgb(18, 20, 26)
        } else {
            Color32::from_rgb(215, 220, 235)
        };
        painter.circle_filled(center, radius, knob_color);
    }
    response
}

/// Modern Fluent Settings Card Container
pub fn settings_card<R>(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::new()
        .fill(Color32::from_rgb(24, 27, 36))
        .stroke(Stroke::new(1.0, Color32::from_rgb(40, 45, 60)))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(Margin::symmetric(16, 12))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(0.0, 8.0);
            add_contents(ui)
        })
        .inner
}

/// Modern Fluent Settings Row (Title + Description on Left, Control on Right)
pub fn settings_row<R>(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    control: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new(title).size(12.5).strong().color(Color32::from_rgb(235, 240, 252)));
            if !description.is_empty() {
                ui.label(RichText::new(description).size(10.5).color(Color32::from_rgb(140, 146, 168)));
            }
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            control(ui)
        }).inner
    }).inner
}

/// Section title and subtitle header
pub fn section_header(ui: &mut egui::Ui, breadcrumb: &str, title: &str, description: &str) {
    ui.label(RichText::new(breadcrumb).size(10.5).color(Color32::from_rgb(130, 136, 155)));
    ui.add_space(1.0);
    ui.label(RichText::new(title).size(17.0).strong().color(Color32::WHITE));
    if !description.is_empty() {
        ui.label(RichText::new(description).size(11.0).color(Color32::from_rgb(145, 152, 172)));
    }
    ui.add_space(8.0);
}

pub struct PreferencesDialog {
    pub active_category: String,
    pub active_sub_category: String,
    pub search_query: String,
    pub new_preset_name: String,
    pub is_naming_preset: bool,
    pub save_feedback_time: f64,
}

impl Default for PreferencesDialog {
    fn default() -> Self {
        Self {
            active_category: "General".to_string(),
            active_sub_category: "Basic & Startup".to_string(),
            search_query: String::new(),
            new_preset_name: String::new(),
            is_naming_preset: false,
            save_feedback_time: 0.0,
        }
    }
}

impl PreferencesDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, is_open: &mut bool, config: &mut AppConfig, player: Option<&crate::engine::Player>) {
        if !*is_open {
            return;
        }

        let mut should_close = false;
        let viewport_id = ViewportId::from_hash_of("vortex_preferences_studio_window");
        let builder = ViewportBuilder::default()
            .with_title("Preferences (F5)")
            .with_inner_size([940.0, 680.0])
            .with_min_inner_size([820.0, 560.0])
            .with_resizable(true)
            .with_decorations(true);

        ctx.show_viewport_immediate(viewport_id, builder, |ctx, _class| {
            if ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                should_close = true;
            }

            egui::CentralPanel::default()
                .frame(
                    egui::Frame::new()
                        .fill(Color32::from_rgb(18, 20, 26))
                        .inner_margin(Margin::same(14)),
                )
                .show(ctx, |ui| {
                    // Top Modern Search Bar Capsule
                    egui::Frame::new()
                        .fill(Color32::from_rgb(26, 29, 38))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(42, 46, 60)))
                        .corner_radius(CornerRadius::same(8))
                        .inner_margin(Margin::symmetric(12, 8))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("🔍").size(13.0).color(Color32::from_rgb(140, 146, 168)));
                                ui.add_space(4.0);
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.search_query)
                                        .hint_text("Search settings and options (e.g. decoder, subtitles, equalizer, hotkeys, buffers)...")
                                        .frame(egui::Frame::NONE)
                                        .desired_width(ui.available_width() - 36.0),
                                );
                                if !self.search_query.is_empty() {
                                    if ui.small_button("✕").clicked() {
                                        self.search_query.clear();
                                    }
                                }
                            });
                        });
                    ui.add_space(8.0);

                let available_h = (ui.available_height() - 46.0).max(350.0);
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), available_h),
                    egui::Layout::left_to_right(egui::Align::TOP),
                    |ui| {
                        // ── Left Category Sidebar ──────────────────────────────
                        ui.vertical(|ui| {
                            ui.set_width(215.0);
                            ui.set_min_height(available_h);
                            ui.spacing_mut().item_spacing = Vec2::new(0.0, 1.0);

                            let tree_sections: &[(&str, &str, &[&str])] = &[
                                ("⚙", "General", &["Basic & Startup", "Power & Performance", "Window & Screen", "Mouse & Gestures", "OSD Diagnostics"]),
                                ("▶", "Playback", &["Time & Seeking", "Speed & Pitch", "Auto-Skip & Chapters", "Loop & End Actions"]),
                                ("🎬", "Video", &["Hardware Video Decoder", "Renderer & Presentation", "Pixel Scalers & Anime4K", "HDR & Color Management", "Aspect Ratio & Framing", "Color Adjustments"]),
                                ("🔊", "Audio", &["Output Device", "Channels & Surround Matrix", "DSP & Volume Dynamics", "18-Band Equalizer", "Soxr Resampling & ReplayGain"]),
                                ("💬", "Subtitles", &["Engine & Dual Subtitles", "Typography & Positioning", "Language Priority & Download"]),
                                ("⌨", "Input & Hotkeys", &["Keyboard & Global Hotkeys", "Gamepad & Controller"]),
                                ("🌐", "Network & Cloud", &["Stream Buffering & Protocols"]),
                                ("🧩", "Integration", &["Windows SMTC & Shell"]),
                                ("⚡", "Advanced", &["Profiles, Backup & Reset"]),
                            ];

                            egui::ScrollArea::vertical()
                                .id_salt("pref_tree_scroll")
                                .auto_shrink([false, false])
                                .min_scrolled_height(available_h)
                                .show(ui, |ui| {
                                    for (icon, section, sub_items) in tree_sections {
                                        let is_section_active = self.active_category == *section;

                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(*icon).size(12.0).color(if is_section_active { VortexTheme::POT_YELLOW } else { Color32::from_rgb(140, 146, 168) }));
                                            ui.label(
                                                RichText::new(*section)
                                                    .strong()
                                                    .size(11.5)
                                                    .color(if is_section_active { VortexTheme::POT_YELLOW } else { Color32::from_rgb(180, 186, 205) }),
                                            );
                                        });
                                        ui.add_space(2.0);

                                        for sub in *sub_items {
                                            let is_selected = is_section_active && self.active_sub_category == *sub;
                                            let text_color = if is_selected {
                                                VortexTheme::POT_YELLOW
                                            } else {
                                                Color32::from_rgb(150, 155, 172)
                                            };
                                            let bg = if is_selected {
                                                Color32::from_rgb(34, 38, 52)
                                            } else {
                                                Color32::TRANSPARENT
                                            };

                                            let btn_resp = ui.add(
                                                egui::Button::new(
                                                    RichText::new(format!("  {}", sub))
                                                        .size(11.0)
                                                        .color(text_color)
                                                )
                                                .fill(bg)
                                                .stroke(Stroke::new(1.0, if is_selected { Color32::from_rgb(55, 60, 80) } else { Color32::TRANSPARENT }))
                                                .corner_radius(CornerRadius::same(6))
                                                .min_size(Vec2::new(ui.available_width() - 8.0, 26.0))
                                            );

                                            if is_selected {
                                                let r = btn_resp.rect;
                                                ui.painter().rect_filled(
                                                    Rect::from_min_size(r.left_top() + Vec2::new(2.0, 4.0), Vec2::new(3.0, r.height() - 8.0)),
                                                    CornerRadius::same(2),
                                                    VortexTheme::POT_YELLOW,
                                                );
                                            }

                                            if btn_resp.clicked() {
                                                self.active_category = section.to_string();
                                                self.active_sub_category = sub.to_string();
                                            }
                                        }
                                    }
                                });
                        });

                        ui.separator();

                        // ── Right Detailed Content Area ─────────────────────────
                        ui.vertical(|ui| {
                            ui.set_width(ui.available_width());
                            ui.set_min_height(available_h);
                            ui.spacing_mut().item_spacing = Vec2::new(0.0, 8.0);

                            egui::ScrollArea::vertical()
                                .id_salt("pref_content_scroll")
                                .auto_shrink([false, false])
                                .min_scrolled_height(available_h)
                                .show(ui, |ui| {
                                    let query = self.search_query.trim().to_lowercase();
                                    if !query.is_empty() {
                                        ui.label(RichText::new(format!("Search Results for \"{}\":", self.search_query)).strong().color(VortexTheme::POT_YELLOW));
                                        ui.add_space(4.0);
                                    }

                                match (self.active_category.as_str(), self.active_sub_category.as_str()) {
                                    // ──────────────────────────────────────────────
                                    // 1. GENERAL
                                    // ──────────────────────────────────────────────
                                    ("General", "Basic & Startup") => {
                                        section_header(ui, "General  ›  Basic & Startup", "Basic & Startup", "Configure visual theme palette, resume playback behavior, and startup window options.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Theme Palette & Skin", "Aesthetic color theme for all window frames and dialogs", |ui| {
                                                egui::ComboBox::from_id_salt("theme_selector")
                                                    .selected_text(config.theme_mode.display_name())
                                                    .show_ui(ui, |ui| {
                                                        for mode in ThemeMode::ALL {
                                                            if ui.selectable_label(config.theme_mode == mode, mode.display_name()).clicked() {
                                                                config.theme_mode = mode;
                                                                VortexTheme::apply(ui.ctx(), mode);
                                                            }
                                                        }
                                                    });
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Auto-Resume Playback", "Remember and automatically seek to the last played timestamp upon opening media", |ui| {
                                                fluent_switch(ui, &mut config.auto_resume);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Auto-Queue Neighboring Episodes", "Automatically scan the folder for next episodes and append them to playlist", |ui| {
                                                fluent_switch(ui, &mut config.auto_load_next_episode);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Show Recent Files on Idle", "Display interactive recent media history cards when idle with no file loaded", |ui| {
                                                fluent_switch(ui, &mut config.show_recent_on_idle);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Restore Previous Playlist on Direct Launch", "Restore previous playlist only when launching player directly without opening a media file (switched OFF by default)", |ui| {
                                                fluent_switch(ui, &mut config.restore_last_playlist);
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Mouse Double-Click Action", "Primary action performed when double clicking anywhere on the video area", |ui| {
                                                egui::ComboBox::from_id_salt("double_click_pref_basic")
                                                    .selected_text(match config.double_click_action {
                                                        DoubleClickAction::ToggleFullscreen => "Toggle Fullscreen",
                                                        DoubleClickAction::PlayPause => "Play / Pause",
                                                        DoubleClickAction::MaximizeRestore => "Maximize / Restore Window",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.double_click_action, DoubleClickAction::ToggleFullscreen, "Toggle Fullscreen");
                                                        ui.selectable_value(&mut config.double_click_action, DoubleClickAction::PlayPause, "Play / Pause");
                                                        ui.selectable_value(&mut config.double_click_action, DoubleClickAction::MaximizeRestore, "Maximize / Restore Window");
                                                    });
                                            });
                                        });
                                    }
                                    ("General", "Power & Performance") => {
                                        section_header(ui, "General  ›  Power & Performance", "Power & Performance", "Control GPU frame pacing, foreground/background refresh rates, and power conservation.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Active Interaction FPS", "Target refresh rate during window dragging, hovering, and interactive UI controls", |ui| {
                                                egui::ComboBox::from_id_salt("fps_interaction_combo")
                                                    .selected_text(format!("{} FPS", config.fps_interaction))
                                                    .show_ui(ui, |ui| {
                                                        for &f in &[15, 30, 60, 120, 144] {
                                                            ui.selectable_value(&mut config.fps_interaction, f, format!("{} FPS", f));
                                                        }
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Active Playback FPS", "Foreground playback and timeline progress refresh rate", |ui| {
                                                egui::ComboBox::from_id_salt("fps_playback_combo")
                                                    .selected_text(format!("{} FPS", config.fps_playback))
                                                    .show_ui(ui, |ui| {
                                                        for &f in &[5, 10, 15, 24, 30, 60] {
                                                            ui.selectable_value(&mut config.fps_playback, f, format!("{} FPS", f));
                                                        }
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Background Playback FPS", "Refresh rate when media player is behind other active application windows", |ui| {
                                                egui::ComboBox::from_id_salt("fps_background_combo")
                                                    .selected_text(format!("{} FPS", config.fps_background))
                                                    .show_ui(ui, |ui| {
                                                        for &f in &[1, 5, 10, 15, 30] {
                                                            ui.selectable_value(&mut config.fps_background, f, format!("{} FPS", f));
                                                        }
                                                    });
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Zero-GPU Minimized Mode", "Put GPU pipeline to complete 0.0W sleep when minimized to taskbar while preserving audio", |ui| {
                                                fluent_switch(ui, &mut config.zero_gpu_minimized);
                                            });
                                        });
                                    }
                                    ("General", "Window & Screen") => {
                                        section_header(ui, "General  ›  Window & Screen", "Window & Screen", "Manage window dimensions, positioning memory, and always-on-top behavior.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Always on Top", "Keep the player window pinned above all other desktop applications", |ui| {
                                                fluent_switch(ui, &mut config.always_on_top);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Remember Window Size & Position", "Restore last closed window dimensions and desktop screen coordinates", |ui| {
                                                fluent_switch(ui, &mut config.remember_window_size);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Default Window Width", "Default width in pixels when opening without saved dimensions", |ui| {
                                                ui.add(egui::DragValue::new(&mut config.window_width).range(640.0..=3840.0).suffix(" px"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Default Window Height", "Default height in pixels when opening without saved dimensions", |ui| {
                                                ui.add(egui::DragValue::new(&mut config.window_height).range(400.0..=2160.0).suffix(" px"));
                                            });
                                        });
                                    }
                                    ("General", "Mouse & Gestures") => {
                                        section_header(ui, "General  ›  Mouse & Gestures", "Mouse & Gestures", "Map mouse buttons, wheel scrolling, and pointer gestures to player commands.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Left Double Click", "Action when double-clicking on video canvas", |ui| {
                                                egui::ComboBox::from_id_salt("double_click_pref")
                                                    .selected_text(match config.double_click_action {
                                                        DoubleClickAction::ToggleFullscreen => "Toggle Fullscreen",
                                                        DoubleClickAction::PlayPause => "Play / Pause",
                                                        DoubleClickAction::MaximizeRestore => "Maximize / Restore Window",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.double_click_action, DoubleClickAction::ToggleFullscreen, "Toggle Fullscreen");
                                                        ui.selectable_value(&mut config.double_click_action, DoubleClickAction::PlayPause, "Play / Pause");
                                                        ui.selectable_value(&mut config.double_click_action, DoubleClickAction::MaximizeRestore, "Maximize / Restore Window");
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Middle Click (Wheel Button)", "Action when clicking the mouse wheel button", |ui| {
                                                egui::ComboBox::from_id_salt("middle_click_pref")
                                                    .selected_text(&config.mouse_middle_click_action)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.mouse_middle_click_action, "Mute".to_string(), "Toggle Mute");
                                                        ui.selectable_value(&mut config.mouse_middle_click_action, "PlayPause".to_string(), "Play / Pause");
                                                        ui.selectable_value(&mut config.mouse_middle_click_action, "100PercentSize".to_string(), "Resize to 100% Video Resolution");
                                                        ui.selectable_value(&mut config.mouse_middle_click_action, "Fullscreen".to_string(), "Toggle Fullscreen");
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Mouse Wheel Action", "Action when scrolling the wheel over video viewport", |ui| {
                                                egui::ComboBox::from_id_salt("wheel_action_pref")
                                                    .selected_text(&config.mouse_wheel_action)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.mouse_wheel_action, "Volume".to_string(), "Adjust Volume (±2% / ±5%)");
                                                        ui.selectable_value(&mut config.mouse_wheel_action, "Seek".to_string(), "Seek Timeline (±5s)");
                                                        ui.selectable_value(&mut config.mouse_wheel_action, "Speed".to_string(), "Adjust Speed (±0.1x)");
                                                        ui.selectable_value(&mut config.mouse_wheel_action, "SubtitleDelay".to_string(), "Adjust Subtitle Delay (±0.05s)");
                                                    });
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Fixed Pointer Navigation").size(11.5).strong().color(Color32::from_rgb(200, 206, 225)));
                                            ui.label(RichText::new("• Left Single Click: Play / Pause on video canvas").size(10.5).color(Color32::from_rgb(140, 146, 165)));
                                            ui.label(RichText::new("• Right Click: Open Context Menu with full cascade tree").size(10.5).color(Color32::from_rgb(140, 146, 165)));
                                            ui.label(RichText::new("• Top Bar Drag in Fullscreen: Switch to windowed mode and drag window").size(10.5).color(Color32::from_rgb(140, 146, 165)));
                                        });
                                    }
                                    ("General", "OSD Diagnostics") => {
                                        section_header(ui, "General  ›  OSD Diagnostics", "OSD Diagnostics", "Configure on-screen notifications, seekbar hover scrubbing, and diagnostic HUD.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "OSD Notification Duration", "Duration in milliseconds that status alerts stay visible", |ui| {
                                                ui.add(egui::Slider::new(&mut config.osd_duration_ms, 500..=5000).suffix(" ms"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "OSD Font Size", "Typography scale for status notifications and timecode overlays", |ui| {
                                                ui.add(egui::Slider::new(&mut config.osd_font_size, 12.0..=36.0).suffix(" pt"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Seekbar Hover Thumbnail Scrubbing", "Show real-time preview card when hovering over the progress bar", |ui| {
                                                fluent_switch(ui, &mut config.show_seekbar_thumbnail);
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Real-time Diagnostics Overlay (Tab Key)").size(11.5).strong().color(VortexTheme::POT_YELLOW));
                                            ui.label(RichText::new("Pressing Tab renders the PotPlayer technical telemetry overlay with real-time stats:\n• Video Codec, Hardware Decoder, Resolution, Framerate, Dropped Frames\n• Audio Codec, Channel Layout, Sample Rate, Bitrate, Buffer Latency").size(10.5).color(Color32::from_rgb(140, 146, 165)));
                                        });
                                    }

                                    // ──────────────────────────────────────────────
                                    // 2. PLAYBACK
                                    // ──────────────────────────────────────────────
                                    ("Playback", "Time & Seeking") => {
                                        section_header(ui, "Playback  ›  Time & Seeking", "Time & Seeking", "Customize keyboard arrow seek steps and adaptive timeline pacing.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Short Seek Step (← / →)", "Fine seek step duration in seconds", |ui| {
                                                ui.add(egui::DragValue::new(&mut config.seek_step_short).range(1.0..=60.0).suffix(" s"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Medium Seek Step (Ctrl + ← / →)", "Standard scene navigation step in seconds", |ui| {
                                                ui.add(egui::DragValue::new(&mut config.seek_step_medium).range(5.0..=120.0).suffix(" s"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Long Seek Step (Shift + ← / →)", "Large chapter jump interval in seconds", |ui| {
                                                ui.add(egui::DragValue::new(&mut config.seek_step_long).range(10.0..=300.0).suffix(" s"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Huge Seek Step (Ctrl + Shift + ← / →)", "Massive timeline leap interval in seconds", |ui| {
                                                ui.add(egui::DragValue::new(&mut config.seek_step_huge).range(30.0..=600.0).suffix(" s"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Adaptive Seek Scaling", "Scale seek interval dynamically based on total media duration", |ui| {
                                                fluent_switch(ui, &mut config.adaptive_seek);
                                            });
                                        });
                                    }
                                    ("Playback", "Speed & Pitch") => {
                                        section_header(ui, "Playback  ›  Speed & Pitch", "Speed & Pitch", "Adjust playback rate and configure pitch preservation DSP.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Current Playback Speed", "Realtime playback multiplier (0.1x to 4.0x)", |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add(egui::Slider::new(&mut config.playback_speed, 0.1..=4.0).suffix("x"));
                                                    if ui.button("1.0x Reset").clicked() {
                                                        config.playback_speed = 1.0;
                                                    }
                                                });
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("💡 Intelligent Pitch Preservation").size(11.5).strong().color(VortexTheme::POT_YELLOW));
                                            ui.label(RichText::new("VortexPlayer employs Scaletempo2 and Rubberband DSP to maintain crystal-clear vocal timbre and pitch fidelity at any accelerated or decelerated speed.").size(10.5).color(Color32::from_rgb(140, 146, 165)));
                                        });
                                    }
                                    ("Playback", "Auto-Skip & Chapters") => {
                                        section_header(ui, "Playback  ›  Auto-Skip & Chapters", "Auto-Skip & Chapters", "Automatically bypass opening themes, recaps, and closing credit sequences.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Enable Intro Auto-Skip", "Skip first N seconds of anime/series episodes automatically", |ui| {
                                                fluent_switch(ui, &mut config.skip_intro_enabled);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Opening / Intro Duration", "Number of seconds to skip at start of each track", |ui| {
                                                ui.add(egui::DragValue::new(&mut config.skip_intro_sec).range(0.0..=300.0).suffix(" s"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Ending / Outro Duration", "Number of seconds to skip before file ends", |ui| {
                                                ui.add(egui::DragValue::new(&mut config.skip_outro_sec).range(0.0..=300.0).suffix(" s"));
                                            });
                                        });
                                    }
                                    ("Playback", "Loop & End Actions") => {
                                        section_header(ui, "Playback  ›  Loop & End Actions", "Loop & End Actions", "Configure player and PC power actions when playlist playback completes.");
                                        settings_card(ui, |ui| {
                                            ui.radio_value(&mut config.playlist_end_action, PlaylistEndAction::ShowHomeScreen, "Show Idle Home Screen");
                                            ui.radio_value(&mut config.playlist_end_action, PlaylistEndAction::RepeatPlaylist, "Repeat Entire Playlist Continuously");
                                            ui.radio_value(&mut config.playlist_end_action, PlaylistEndAction::StopPlayback, "Stop and Retain Final Frame");
                                            ui.radio_value(&mut config.playlist_end_action, PlaylistEndAction::ClosePlayer, "Close VortexPlayer Immediately");
                                            ui.radio_value(&mut config.playlist_end_action, PlaylistEndAction::SleepPC, "Put Computer to Sleep / Standby Mode");
                                            ui.radio_value(&mut config.playlist_end_action, PlaylistEndAction::ShutdownPC, "Shutdown Computer (with 60-second cancel countdown)");
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Restore Previous Playlist on Direct Launch", "When opening the player directly (without clicking a media file), reload items from previous session", |ui| {
                                                fluent_switch(ui, &mut config.restore_last_playlist);
                                            });
                                        });
                                    }

                                    // ──────────────────────────────────────────────
                                    // 3. VIDEO
                                    // ──────────────────────────────────────────────
                                    ("Video", "Hardware Video Decoder") => {
                                        section_header(ui, "Video  ›  Hardware Video Decoder", "Hardware Video Decoder", "Configure low-overhead GPU hardware decoding interfaces.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Hardware Decoder API", "GPU hardware acceleration backend", |ui| {
                                                egui::ComboBox::from_id_salt("hwdec_pref")
                                                    .selected_text(&config.hardware_decoding)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.hardware_decoding, "auto-safe".to_string(), "Auto Safe (D3D11 / NVDEC)");
                                                        ui.selectable_value(&mut config.hardware_decoding, "d3d11va".to_string(), "Direct3D 11 (D3D11VA - Recommended)");
                                                        ui.selectable_value(&mut config.hardware_decoding, "nvdec".to_string(), "NVIDIA NVDEC (CUDA)");
                                                        ui.selectable_value(&mut config.hardware_decoding, "dxva2".to_string(), "DirectX Video Acceleration (DXVA2)");
                                                        ui.selectable_value(&mut config.hardware_decoding, "qsv".to_string(), "Intel QuickSync (QSV)");
                                                        ui.selectable_value(&mut config.hardware_decoding, "no".to_string(), "Software Decoding (CPU)");
                                                    });
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("💡 Intelligent Decoder Failover").size(11.5).strong().color(VortexTheme::POT_YELLOW));
                                            ui.label(RichText::new("'Auto Safe' automatically selects Direct3D 11 / NVDEC hardware pipelines. If unsupported 12-bit or damaged bitstreams occur, it automatically and silently falls back to multi-threaded CPU software decoding.").size(10.5).color(Color32::from_rgb(140, 146, 165)));
                                        });
                                    }
                                    ("Video", "Renderer & Presentation") => {
                                        section_header(ui, "Video  ›  Renderer & Presentation", "Renderer & Presentation", "Direct3D 11 presentation pipeline, sync pacing, and frame drops.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "A/V Sync Mode", "Audio vs display presentation synchronization strategy", |ui| {
                                                egui::ComboBox::from_id_salt("video_sync_pref")
                                                    .selected_text(match config.video_sync_mode.as_str() {
                                                        "display-resample" => "Display Resample (Ultra-Smooth Jitter-Free)",
                                                        "display-vdrop" => "Display Video Drop",
                                                        _ => "Audio Master (Default)",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.video_sync_mode, "audio".to_string(), "Audio Master (Default)");
                                                        ui.selectable_value(&mut config.video_sync_mode, "display-resample".to_string(), "Display Resample (Ultra-Smooth Jitter-Free)");
                                                        ui.selectable_value(&mut config.video_sync_mode, "display-vdrop".to_string(), "Display Video Drop");
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Frame Dropping Behavior", "Handling of late frames under heavy system workload", |ui| {
                                                egui::ComboBox::from_id_salt("framedrop_pref")
                                                    .selected_text(match config.framedrop_mode.as_str() {
                                                        "yes" => "Aggressive Decoder Drops (Fast)",
                                                        "no" => "Never Drop Frames (Lossless)",
                                                        _ => "Video Output Drops (vo - Recommended)",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.framedrop_mode, "vo".to_string(), "Video Output Drops (vo - Recommended)");
                                                        ui.selectable_value(&mut config.framedrop_mode, "yes".to_string(), "Aggressive Decoder Drops (Fast)");
                                                        ui.selectable_value(&mut config.framedrop_mode, "no".to_string(), "Never Drop Frames (Lossless)");
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Dithering Precision", "Spatial/temporal dithering to eliminate banding on 8-bit / 10-bit displays", |ui| {
                                                egui::ComboBox::from_id_salt("dither_pref")
                                                    .selected_text(&config.dither_depth)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.dither_depth, "auto".to_string(), "Auto (Match Display)");
                                                        ui.selectable_value(&mut config.dither_depth, "8".to_string(), "8-Bit Dithering");
                                                        ui.selectable_value(&mut config.dither_depth, "10".to_string(), "10-Bit High Precision");
                                                        ui.selectable_value(&mut config.dither_depth, "no".to_string(), "Disabled (No Dither)");
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Debanding Filter", "Eliminate color banding artifacts in dark scenes and compressed gradients", |ui| {
                                                fluent_switch(ui, &mut config.video_deband);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Auto-Crop Black Bars", "Automatically detect and crop black letterbox bars (cropdetect)", |ui| {
                                                fluent_switch(ui, &mut config.auto_crop_black_bars);
                                            });
                                        });
                                    }
                                    ("Video", "Pixel Scalers & Anime4K") => {
                                        section_header(ui, "Video  ›  Pixel Scalers & Anime4K", "Pixel Scalers & Anime4K", "Neural upscaling shaders and spatial resampling algorithms.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Upscaling Algorithm", "Mathematical filter used when upscaling lower resolution content", |ui| {
                                                egui::ComboBox::from_id_salt("scaler_pref")
                                                    .selected_text(config.shaders.scale.display_name())
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.shaders.scale, VideoScaler::Spline36, "Spline36 (Sharp)");
                                                        ui.selectable_value(&mut config.shaders.scale, VideoScaler::Lanczos, "Lanczos 3-tap");
                                                        ui.selectable_value(&mut config.shaders.scale, VideoScaler::Bilinear, "Bilinear (Fast)");
                                                        ui.selectable_value(&mut config.shaders.scale, VideoScaler::Mitchell, "Mitchell-Netravali");
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Anime4K Neural Upscaling", "Deep learning shader passes for anime and line-art remastering", |ui| {
                                                egui::ComboBox::from_id_salt("anime4k_pref")
                                                    .selected_text(config.shaders.anime4k.display_name())
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.shaders.anime4k, Anime4kPreset::Off, "Off");
                                                        ui.selectable_value(&mut config.shaders.anime4k, Anime4kPreset::ModeA, "Anime4K Mode A (Fast)");
                                                        ui.selectable_value(&mut config.shaders.anime4k, Anime4kPreset::ModeB, "Anime4K Mode B (HQ)");
                                                        ui.selectable_value(&mut config.shaders.anime4k, Anime4kPreset::ModeC, "Anime4K Mode C (Ultra)");
                                                    });
                                            });
                                        });
                                    }
                                    ("Video", "HDR & Color Management") => {
                                        section_header(ui, "Video  ›  HDR & Color Management", "HDR & Color Management", "HDR10 tone mapping, BT.2020 color gamut, and 3D LUT grading.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Tone Mapping Algorithm", "HDR-to-SDR tone curve mapping strategy", |ui| {
                                                egui::ComboBox::from_id_salt("hdr_pref")
                                                    .selected_text(&config.hdr_tone_mapping)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.hdr_tone_mapping, "auto".to_string(), "Auto (BT.2390)");
                                                        ui.selectable_value(&mut config.hdr_tone_mapping, "mobius".to_string(), "Möbius");
                                                        ui.selectable_value(&mut config.hdr_tone_mapping, "reinhard".to_string(), "Reinhard");
                                                        ui.selectable_value(&mut config.hdr_tone_mapping, "hable".to_string(), "Hable (Uncharted 2)");
                                                        ui.selectable_value(&mut config.hdr_tone_mapping, "clip".to_string(), "Hard Clip");
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Compute Scene Peak Luminance", "Dynamically calculate real-time scene luminance for optimal dynamic range", |ui| {
                                                fluent_switch(ui, &mut config.hdr_compute_peak);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Send BT.2020 Gamut Hint", "Deliver wide color gamut metadata directly to display driver", |ui| {
                                                fluent_switch(ui, &mut config.hdr_target_colorspace_hint);
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("3D LUT Color Grading & Display Calibration (.cube)").size(12.5).strong().color(Color32::from_rgb(235, 240, 252)));
                                            ui.add_space(4.0);
                                            ui.horizontal(|ui| {
                                                let lut_display = config.lut_file.as_deref().unwrap_or("(None / Disabled)");
                                                ui.label(RichText::new(format!("File: {}", lut_display)).size(11.0).color(Color32::from_rgb(160, 166, 185)));
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    if config.lut_file.is_some() && ui.button("Clear").clicked() {
                                                        config.lut_file = None;
                                                    }
                                                    if ui.button("Browse LUT...").clicked() {
                                                        if let Some(path) = rfd::FileDialog::new()
                                                            .add_filter("3D LUT (*.cube)", &["cube"])
                                                            .pick_file()
                                                        {
                                                            config.lut_file = Some(path.to_string_lossy().to_string());
                                                        }
                                                    }
                                                });
                                            });
                                        });
                                    }
                                    ("Video", "Aspect Ratio & Framing") => {
                                        section_header(ui, "Video  ›  Aspect Ratio & Framing", "Aspect Ratio & Framing", "Aspect ratio constraints, letterbox bar placement, and auto-crop.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Default Aspect Ratio", "Target aspect ratio framing for loaded video streams", |ui| {
                                                egui::ComboBox::from_id_salt("ar_pref")
                                                    .selected_text(&config.aspect_ratio)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.aspect_ratio, "auto".to_string(), "Auto (Keep Aspect Ratio)");
                                                        ui.selectable_value(&mut config.aspect_ratio, "16:9".to_string(), "16:9 Wide");
                                                        ui.selectable_value(&mut config.aspect_ratio, "4:3".to_string(), "4:3 Standard");
                                                        ui.selectable_value(&mut config.aspect_ratio, "1.85:1".to_string(), "1.85:1 Cinema");
                                                        ui.selectable_value(&mut config.aspect_ratio, "2.35:1".to_string(), "2.35:1 Anamorphic");
                                                        ui.selectable_value(&mut config.aspect_ratio, "fill".to_string(), "Fit to Window (Fill)");
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Black Bar Placement", "Vertical framing position of the active video canvas", |ui| {
                                                let current_label = match config.video_align_y.as_str() {
                                                    "top" => "Bottom (Video pinned to Top)",
                                                    "bottom" => "Top (Video pinned to Bottom)",
                                                    _ => "Both Half and Half (Centered - Default)",
                                                };
                                                egui::ComboBox::from_id_salt("video_align_y_pref")
                                                    .selected_text(current_label)
                                                    .show_ui(ui, |ui| {
                                                        if ui.selectable_label(config.video_align_y == "center", "Both Half and Half (Centered - Default)").clicked() {
                                                            config.video_align_y = "center".to_string();
                                                            if let Some(p) = player {
                                                                p.set_video_align_y(&config.video_align_y);
                                                            }
                                                        }
                                                        if ui.selectable_label(config.video_align_y == "top", "Bottom (Video pinned to Top)").clicked() {
                                                            config.video_align_y = "top".to_string();
                                                            if let Some(p) = player {
                                                                p.set_video_align_y(&config.video_align_y);
                                                            }
                                                        }
                                                        if ui.selectable_label(config.video_align_y == "bottom", "Top (Video pinned to Bottom)").clicked() {
                                                            config.video_align_y = "bottom".to_string();
                                                            if let Some(p) = player {
                                                                p.set_video_align_y(&config.video_align_y);
                                                            }
                                                        }
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Auto-Crop Black Bars", "Automatically detect and crop letterbox padding (cropdetect)", |ui| {
                                                fluent_switch(ui, &mut config.auto_crop_black_bars);
                                            });
                                        });
                                    }
                                    ("Video", "Color Adjustments") => {
                                        section_header(ui, "Video  ›  Color Adjustments", "Color Adjustments", "Precision video brightness, contrast, saturation, and unsharp masking.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Brightness", "Luminance offset (-100% to +100%)", |ui| {
                                                ui.horizontal(|ui| {
                                                    let sl = ui.add(egui::Slider::new(&mut config.video_brightness, 0.0..=200.0).suffix("%"));
                                                    if sl.changed() {
                                                        if let Some(p) = player {
                                                            p.set_video_brightness(config.video_brightness - 100.0);
                                                        }
                                                        let _ = config.save();
                                                    }
                                                    if ui.small_button("↺").clicked() {
                                                        config.video_brightness = 100.0;
                                                        if let Some(p) = player {
                                                            p.set_video_brightness(0.0);
                                                        }
                                                        let _ = config.save();
                                                    }
                                                });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Contrast", "Dynamic range contrast multiplier (0% to 200%)", |ui| {
                                                ui.horizontal(|ui| {
                                                    let sl = ui.add(egui::Slider::new(&mut config.video_contrast, 0.0..=200.0).suffix("%"));
                                                    if sl.changed() {
                                                        if let Some(p) = player {
                                                            p.set_video_contrast(config.video_contrast - 100.0);
                                                        }
                                                        let _ = config.save();
                                                    }
                                                    if ui.small_button("↺").clicked() {
                                                        config.video_contrast = 100.0;
                                                        if let Some(p) = player {
                                                            p.set_video_contrast(0.0);
                                                        }
                                                        let _ = config.save();
                                                    }
                                                });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Saturation", "Chroma color saturation (0% to 200%)", |ui| {
                                                ui.horizontal(|ui| {
                                                    let sl = ui.add(egui::Slider::new(&mut config.video_saturation, 0.0..=200.0).suffix("%"));
                                                    if sl.changed() {
                                                        if let Some(p) = player {
                                                            p.set_video_saturation(config.video_saturation - 100.0);
                                                        }
                                                        let _ = config.save();
                                                    }
                                                    if ui.small_button("↺").clicked() {
                                                        config.video_saturation = 100.0;
                                                        if let Some(p) = player {
                                                            p.set_video_saturation(0.0);
                                                        }
                                                        let _ = config.save();
                                                    }
                                                });
                                            });
                                            ui.separator();
                                            settings_row(ui, "Sharpen (Unsharp Mask)", "Edge sharpening strength (0.0 to 5.0)", |ui| {
                                                let sl = ui.add(egui::Slider::new(&mut config.video_sharpen, 0.0..=5.0));
                                                if sl.changed() {
                                                    if let Some(p) = player {
                                                        p.set_property_string("vf", &format!("unsharp=5:5:{:.2}:5:5:0.0", config.video_sharpen));
                                                    }
                                                    let _ = config.save();
                                                }
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                if ui.button(RichText::new("↺ Reset All Color Levels to 100%").color(VortexTheme::POT_YELLOW)).clicked() {
                                                    config.video_brightness = 100.0;
                                                    config.video_contrast = 100.0;
                                                    config.video_saturation = 100.0;
                                                    config.video_hue = 0.0;
                                                    config.video_sharpen = 0.0;
                                                    if let Some(p) = player {
                                                        p.reset_video_colors();
                                                        p.set_property_string("vf", "");
                                                    }
                                                    let _ = config.save();
                                                }
                                            });
                                        });
                                    }

                                    // ──────────────────────────────────────────────
                                    // 4. AUDIO
                                    // ──────────────────────────────────────────────
                                    ("Audio", "Output Device & WASAPI") | ("Audio", "Output Device") => {
                                        section_header(ui, "Audio  ›  Output Device", "Output Device & WASAPI", "Configure hardware audio endpoints, WASAPI bitstream, and output formats.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Audio Output Device", "Physical audio output endpoint or DAC", |ui| {
                                                let prev_dev = config.audio_device.clone();
                                                let current_desc = if config.audio_device == "auto" || config.audio_device.is_empty() {
                                                    "Auto (System Default)".to_string()
                                                } else if let Some(p) = player {
                                                    let stats = p.stats();
                                                    stats.audio_device_list.iter()
                                                        .find(|d| d.name == config.audio_device)
                                                        .map(|d| if d.description.is_empty() { d.name.clone() } else { d.description.clone() })
                                                        .unwrap_or_else(|| config.audio_device.clone())
                                                } else {
                                                    config.audio_device.clone()
                                                };

                                                egui::ComboBox::from_id_salt("pref_audio_device")
                                                    .selected_text(current_desc)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.audio_device, "auto".to_string(), "Auto (System Default)");
                                                        if let Some(p) = player {
                                                            let stats = p.stats();
                                                            for dev in &stats.audio_device_list {
                                                                let label = if dev.description.is_empty() {
                                                                    dev.name.clone()
                                                                } else {
                                                                    dev.description.clone()
                                                                };
                                                                ui.selectable_value(&mut config.audio_device, dev.name.clone(), label);
                                                            }
                                                        }
                                                    });

                                                if prev_dev != config.audio_device {
                                                    if let Some(p) = player {
                                                        p.set_audio_device(&config.audio_device);
                                                    }
                                                }
                                            });
                                            #[cfg(windows)]
                                            {
                                                ui.separator();
                                                settings_row(ui, "WASAPI Exclusive Mode", "Bypass Windows Audio Engine for bit-perfect bitstream", |ui| {
                                                    fluent_switch(ui, &mut config.wasapi_exclusive);
                                                });
                                            }
                                            ui.separator();
                                            settings_row(ui, "Sample Rate (Resample)", "Audio sampling frequency conversion", |ui| {
                                                let prev_sr = config.resample_rate;
                                                let sr_label = match config.resample_rate {
                                                    0 => "Auto (Same as Input)",
                                                    44100 => "44.1 kHz (CD Quality)",
                                                    48000 => "48.0 kHz (Standard DVD)",
                                                    88200 => "88.2 kHz (2x CD Oversample)",
                                                    96000 => "96.0 kHz (Studio Quality)",
                                                    176400 => "176.4 kHz (4x CD Master)",
                                                    192000 => "192.0 kHz (Ultra Hi-Res Master)",
                                                    384000 => "384.0 kHz (Direct Stream Master)",
                                                    _ => "Custom Rate",
                                                };
                                                egui::ComboBox::from_id_salt("resample_rate_pref")
                                                    .selected_text(sr_label)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.resample_rate, 0, "Auto (Same as Input)");
                                                        ui.selectable_value(&mut config.resample_rate, 44100, "44.1 kHz (CD Quality)");
                                                        ui.selectable_value(&mut config.resample_rate, 48000, "48.0 kHz (Standard DVD)");
                                                        ui.selectable_value(&mut config.resample_rate, 88200, "88.2 kHz (2x CD Oversample)");
                                                        ui.selectable_value(&mut config.resample_rate, 96000, "96.0 kHz (Studio Quality)");
                                                        ui.selectable_value(&mut config.resample_rate, 176400, "176.4 kHz (4x CD Master)");
                                                        ui.selectable_value(&mut config.resample_rate, 192000, "192.0 kHz (Ultra Hi-Res Master)");
                                                        ui.selectable_value(&mut config.resample_rate, 384000, "384.0 kHz (Direct Stream Master)");
                                                    });
                                                if prev_sr != config.resample_rate {
                                                    if let Some(p) = player {
                                                        p.set_audio_samplerate(config.resample_rate);
                                                    }
                                                }
                                            });
                                            ui.separator();
                                            settings_row(ui, "Audio Bit Depth", "PCM sample representation width", |ui| {
                                                let prev_bd = config.audio_bit_depth.clone();
                                                let bd_label = match config.audio_bit_depth.as_str() {
                                                    "16" => "16-bit Integer (s16)",
                                                    "24" => "24-bit Integer (s24)",
                                                    "32" => "32-bit Integer (s32)",
                                                    "float" => "32-bit Floating Point (float)",
                                                    _ => "Auto (Bit-Perfect / 32-bit Float)",
                                                };
                                                egui::ComboBox::from_id_salt("audio_bit_depth_pref")
                                                    .selected_text(bd_label)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.audio_bit_depth, "auto".to_string(), "Auto (Bit-Perfect / 32-bit Float)");
                                                        ui.selectable_value(&mut config.audio_bit_depth, "16".to_string(), "16-bit Integer (s16)");
                                                        ui.selectable_value(&mut config.audio_bit_depth, "24".to_string(), "24-bit Integer (s24)");
                                                        ui.selectable_value(&mut config.audio_bit_depth, "32".to_string(), "32-bit Integer (s32)");
                                                        ui.selectable_value(&mut config.audio_bit_depth, "float".to_string(), "32-bit Floating Point (float)");
                                                    });
                                                if prev_bd != config.audio_bit_depth {
                                                    if let Some(p) = player {
                                                        p.set_audio_format(&config.audio_bit_depth);
                                                    }
                                                }
                                            });
                                            ui.separator();
                                            settings_row(ui, "Resampler Filter Engine", "Polyphase filter tap quality", |ui| {
                                                let prev_q = config.resampler_quality.clone();
                                                let q_label = match config.resampler_quality.as_str() {
                                                    "ultra" => "Ultra High (64-tap Polyphase Sinc)",
                                                    "fast" => "Fast / Low Latency (16-tap Sinc)",
                                                    _ => "High Quality (32-tap Polyphase SoX)",
                                                };
                                                egui::ComboBox::from_id_salt("resampler_quality_pref")
                                                    .selected_text(q_label)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.resampler_quality, "ultra".to_string(), "Ultra High (64-tap Polyphase Sinc)");
                                                        ui.selectable_value(&mut config.resampler_quality, "high".to_string(), "High Quality (32-tap Polyphase SoX)");
                                                        ui.selectable_value(&mut config.resampler_quality, "fast".to_string(), "Fast / Low Latency (16-tap Sinc)");
                                                    });
                                                if prev_q != config.resampler_quality {
                                                    if let Some(p) = player {
                                                        p.set_resampler_quality(&config.resampler_quality);
                                                    }
                                                }
                                            });
                                            ui.separator();
                                            settings_row(ui, "Maximum Volume Boost Limit", "Allow volume slider to exceed 100%", |ui| {
                                                ui.add(egui::Slider::new(&mut config.volume_boost_limit, 100.0..=200.0).suffix(" %"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Audio Delay Offset", "Lip-sync timing compensation in seconds", |ui| {
                                                ui.add(egui::DragValue::new(&mut config.audio_delay).range(-5.0..=5.0).speed(0.05).suffix(" s"));
                                            });
                                        });
                                    }
                                    ("Audio", "Channels & Surround Matrix") => {
                                        section_header(ui, "Audio  ›  Channels & Surround Matrix", "Speaker Channels & Surround Matrix", "Configure multi-channel downmixing, surround matrix, and spatial audio.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Speaker Configuration", "Output channel topology and downmixing", |ui| {
                                                let prev_ch = config.audio_channels.clone();
                                                let ch_label = match config.audio_channels.as_str() {
                                                    "auto" | "" => "Same as input (Auto 5.1/7.1)",
                                                    "5.1" => "5.1 Surround (6 Channels)",
                                                    "7.1" => "7.1 Surround (8 Channels)",
                                                    "stereo" | "2.0" => "2.0 Stereo (Downmix)",
                                                    "2.1" => "2.1 Stereo + LFE",
                                                    "3.0" => "3.0 Surround (Front L, R, C)",
                                                    "4.0" => "4.0 Quadraphonic",
                                                    "5.0" => "5.0 Surround",
                                                    "6.1" => "6.1 Surround",
                                                    "surround" => "Dolby Pro Logic II",
                                                    "bs2b" => "Virtual Headphone (BS2B)",
                                                    "sofalizer" => "Virtual 3D Surround (HRTF)",
                                                    _ => &config.audio_channels,
                                                };
                                                egui::ComboBox::from_id_salt("audio_ch_pref")
                                                    .selected_text(ch_label)
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.audio_channels, "auto".to_string(), "Same as input (Auto 5.1/7.1)");
                                                        ui.selectable_value(&mut config.audio_channels, "5.1".to_string(), "5.1 Surround (6 Channels)");
                                                        ui.selectable_value(&mut config.audio_channels, "7.1".to_string(), "7.1 Surround (8 Channels)");
                                                        ui.selectable_value(&mut config.audio_channels, "stereo".to_string(), "2.0 Stereo (Downmix)");
                                                        ui.selectable_value(&mut config.audio_channels, "2.1".to_string(), "2.1 Stereo + LFE");
                                                        ui.selectable_value(&mut config.audio_channels, "3.0".to_string(), "3.0 Surround (Front L, R, C)");
                                                        ui.selectable_value(&mut config.audio_channels, "4.0".to_string(), "4.0 Quadraphonic");
                                                        ui.selectable_value(&mut config.audio_channels, "5.0".to_string(), "5.0 Surround");
                                                        ui.selectable_value(&mut config.audio_channels, "6.1".to_string(), "6.1 Surround");
                                                        ui.selectable_value(&mut config.audio_channels, "surround".to_string(), "Dolby Pro Logic II");
                                                        ui.selectable_value(&mut config.audio_channels, "bs2b".to_string(), "Virtual Headphone (BS2B)");
                                                        ui.selectable_value(&mut config.audio_channels, "sofalizer".to_string(), "Virtual 3D Surround (HRTF)");
                                                    });
                                                if prev_ch != config.audio_channels {
                                                    if let Some(p) = player {
                                                        let ch_str = match config.audio_channels.as_str() {
                                                            "auto" | "auto-safe" | "" => "auto",
                                                            "2.0" => "stereo",
                                                            "1.0" => "mono",
                                                            other => other,
                                                        };
                                                        p.set_audio_channels(ch_str);
                                                    }
                                                }
                                            });
                                        });
                                    }
                                    ("Audio", "DSP & Volume Dynamics") => {
                                        section_header(ui, "Audio  ›  DSP & Volume Dynamics", "Audio Dynamics & DSP Filters", "Real-time acoustic filters, dialogue clarity enhancement, and stereo widening.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Dynamic Volume Normalizer", "Smooth out loud explosions and boost quiet whispering (dynaudnorm)", |ui| {
                                                fluent_switch(ui, &mut config.audio_normalize);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Karaoke Vocal Remover", "Real-time stereo phase cancellation to eliminate center vocals", |ui| {
                                                fluent_switch(ui, &mut config.vocal_remover);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Dialogue Clarity Enhancer", "Boost speech frequencies for clearer dialogue in movies", |ui| {
                                                fluent_switch(ui, &mut config.voice_enhance);
                                            });
                                            ui.separator();
                                            settings_row(ui, "Stereo Spatial Widener", "Acoustic stage width multiplier (0.0 to 2.0)", |ui| {
                                                ui.add(egui::Slider::new(&mut config.stereo_widen, 0.0..=2.0));
                                            });
                                        });
                                    }
                                    ("Audio", "18-Band Equalizer") => {
                                        let current_time = ui.input(|i| i.time);
                                        let is_recently_saved = current_time - self.save_feedback_time < 2.0;

                                        section_header(ui, "Audio  ›  18-Band Equalizer", "18-Band Graphic Equalizer", "Configure frequency responses, craft custom curves, and manage custom presets.");

                                        settings_card(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                let is_eq = config.eq_enabled;
                                                let mut eq_en = config.eq_enabled;
                                                if ui.checkbox(&mut eq_en, RichText::new("Enable Equalizer").strong().color(if is_eq { VortexTheme::POT_YELLOW } else { Color32::WHITE })).changed() {
                                                    config.eq_enabled = eq_en;
                                                    if let Some(p) = player {
                                                        p.set_equalizer(eq_en, &config.eq_bands);
                                                    }
                                                    let _ = config.save();
                                                }

                                                if ui.button("Reset (0 dB)").on_hover_text("Reset all 18 bands to 0 dB flat").clicked() {
                                                    config.eq_bands = vec![0.0; 18];
                                                    config.eq_preset = "Flat".to_string();
                                                    if let Some(p) = player {
                                                        p.set_equalizer(config.eq_enabled, &config.eq_bands);
                                                    }
                                                    let _ = config.save();
                                                }

                                                let save_txt = if is_recently_saved { "✓ Saved!" } else { "💾 Save Config" };
                                                let save_col = if is_recently_saved { Color32::from_rgb(100, 225, 140) } else { Color32::WHITE };
                                                if ui.button(RichText::new(save_txt).color(save_col)).on_hover_text("Save current equalizer bands and preset to disk").clicked() {
                                                    let _ = config.save();
                                                    self.save_feedback_time = current_time;
                                                }

                                                if ui.button("➕ Save As Custom Preset...").on_hover_text("Save current band values as a new named custom preset").clicked() {
                                                    self.is_naming_preset = !self.is_naming_preset;
                                                    if self.is_naming_preset {
                                                        self.new_preset_name = format!("Custom {}", config.custom_eq_presets.len() + 1);
                                                    }
                                                }

                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    egui::ComboBox::from_id_salt("pref_eq_preset_sel")
                                                        .selected_text(RichText::new(&config.eq_preset).color(VortexTheme::POT_YELLOW).strong())
                                                        .show_ui(ui, |ui| {
                                                            ui.label(RichText::new("── Built-in Presets ──").size(10.0).color(Color32::from_rgb(120, 125, 140)));
                                                            for preset in POT_EQ_PRESETS {
                                                                if ui.selectable_label(config.eq_preset == preset.name, preset.name).clicked() {
                                                                    config.eq_preset = preset.name.to_string();
                                                                    config.eq_bands = preset.bands.to_vec();
                                                                    if let Some(p) = player {
                                                                        p.set_equalizer(config.eq_enabled, &config.eq_bands);
                                                                    }
                                                                    let _ = config.save();
                                                                }
                                                            }

                                                            if !config.custom_eq_presets.is_empty() {
                                                                ui.separator();
                                                                ui.label(RichText::new("── Custom Presets ──").size(10.0).color(VortexTheme::POT_YELLOW));
                                                                let mut to_delete: Option<String> = None;
                                                                let custom_names: Vec<String> = config.custom_eq_presets.keys().cloned().collect();
                                                                for c_name in custom_names {
                                                                    ui.horizontal(|ui| {
                                                                        let is_sel = config.eq_preset == c_name;
                                                                        if ui.selectable_label(is_sel, format!("★ {}", c_name)).clicked() {
                                                                            if let Some(b) = config.custom_eq_presets.get(&c_name) {
                                                                                config.eq_bands = b.clone();
                                                                                config.eq_preset = c_name.clone();
                                                                                if let Some(p) = player {
                                                                                    p.set_equalizer(config.eq_enabled, &config.eq_bands);
                                                                                }
                                                                                let _ = config.save();
                                                                            }
                                                                        }
                                                                        if ui.small_button("🗑").on_hover_text("Delete custom preset").clicked() {
                                                                            to_delete = Some(c_name.clone());
                                                                        }
                                                                    });
                                                                }
                                                                if let Some(del_name) = to_delete {
                                                                    config.custom_eq_presets.remove(&del_name);
                                                                    if config.eq_preset == del_name {
                                                                        config.eq_preset = "Custom".to_string();
                                                                    }
                                                                    let _ = config.save();
                                                                }
                                                            }
                                                        });
                                                    ui.label(RichText::new("Active Preset:").color(Color32::from_rgb(160, 165, 180)));
                                                });
                                            });

                                            if self.is_naming_preset {
                                                ui.add_space(4.0);
                                                egui::Frame::new()
                                                    .fill(Color32::from_rgb(20, 22, 30))
                                                    .stroke(Stroke::new(1.0, Color32::from_rgb(50, 55, 75)))
                                                    .corner_radius(CornerRadius::same(6))
                                                    .inner_margin(Margin::symmetric(10, 6))
                                                    .show(ui, |ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label(RichText::new("New Custom Preset Name:").size(11.0).color(VortexTheme::POT_YELLOW).strong());
                                                            let text_resp = ui.add(
                                                                egui::TextEdit::singleline(&mut self.new_preset_name)
                                                                    .desired_width(200.0)
                                                                    .hint_text("e.g. Bass Extra, Vocals, Gaming...")
                                                            );
                                                            let enter_pressed = text_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                                                            if enter_pressed || ui.button("💾 Save Preset").clicked() {
                                                                let name = self.new_preset_name.trim().to_string();
                                                                if !name.is_empty() {
                                                                    config.custom_eq_presets.insert(name.clone(), config.eq_bands.clone());
                                                                    config.eq_preset = name;
                                                                    let _ = config.save();
                                                                    self.is_naming_preset = false;
                                                                    self.save_feedback_time = current_time;
                                                                    if let Some(p) = player {
                                                                        p.set_equalizer(config.eq_enabled, &config.eq_bands);
                                                                    }
                                                                }
                                                            }
                                                            if ui.button("Cancel").clicked() {
                                                                self.is_naming_preset = false;
                                                            }
                                                        });
                                                    });
                                            }

                                            ui.add_space(6.0);

                                            let freqs = ["20", "31", "50", "80", "125", "200", "315", "500", "800", "1.2k", "2k", "3.1k", "5k", "8k", "12k", "16k", "18k", "20k"];
                                            if config.eq_bands.len() != 18 {
                                                config.eq_bands = vec![0.0; 18];
                                            }

                                            egui::Frame::new()
                                                .fill(Color32::from_rgb(18, 20, 26))
                                                .stroke(Stroke::new(1.0, Color32::from_rgb(38, 42, 54)))
                                                .corner_radius(CornerRadius::same(6))
                                                .inner_margin(Margin::symmetric(10, 8))
                                                .show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);
                                                        for i in 0..18 {
                                                            ui.vertical(|ui| {
                                                                ui.spacing_mut().item_spacing = Vec2::new(0.0, 2.0);
                                                                let mut gain = config.eq_bands[i];

                                                                let gain_col = if gain > 0.1 {
                                                                    Color32::from_rgb(255, 195, 60)
                                                                } else if gain < -0.1 {
                                                                    Color32::from_rgb(100, 180, 255)
                                                                } else {
                                                                    Color32::from_rgb(120, 125, 140)
                                                                };
                                                                ui.label(RichText::new(format!("{:+.0}", gain)).size(8.5).monospace().color(gain_col));

                                                                let slider = ui.add(
                                                                    egui::Slider::new(&mut gain, -12.0..=12.0)
                                                                        .vertical()
                                                                        .show_value(false),
                                                                );
                                                                if slider.changed() {
                                                                    config.eq_bands[i] = gain;
                                                                    config.eq_preset = "Custom".to_string();
                                                                    if let Some(p) = player {
                                                                        p.set_equalizer(config.eq_enabled, &config.eq_bands);
                                                                    }
                                                                    let _ = config.save();
                                                                }

                                                                ui.label(RichText::new(freqs[i]).size(8.5).monospace().color(Color32::from_rgb(140, 145, 160)));
                                                            });
                                                        }
                                                    });
                                                });
                                        });

                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Saved Custom Presets Library").size(12.5).strong().color(VortexTheme::POT_YELLOW));
                                            if config.custom_eq_presets.is_empty() {
                                                ui.label(RichText::new("No custom EQ presets created yet. Adjust the sliders above and click \"➕ Save As Custom Preset...\" to save your personalized sound curve.").size(10.5).color(Color32::from_rgb(140, 145, 160)));
                                            } else {
                                                egui::ScrollArea::vertical()
                                                    .max_height(140.0)
                                                    .show(ui, |ui| {
                                                        let mut to_del: Option<String> = None;
                                                        let mut to_overwrite: Option<String> = None;
                                                        let mut to_load: Option<String> = None;
                                                        let custom_names: Vec<String> = config.custom_eq_presets.keys().cloned().collect();

                                                        for name in custom_names {
                                                            let is_active = config.eq_preset == name;
                                                            let row_bg = if is_active {
                                                                Color32::from_rgb(32, 36, 48)
                                                            } else {
                                                                Color32::from_rgb(20, 22, 28)
                                                            };

                                                            egui::Frame::new()
                                                                .fill(row_bg)
                                                                .stroke(Stroke::new(1.0, if is_active { Color32::from_rgb(65, 80, 115) } else { Color32::from_rgb(36, 40, 52) }))
                                                                .corner_radius(CornerRadius::same(4))
                                                                .inner_margin(Margin::symmetric(10, 6))
                                                                .show(ui, |ui| {
                                                                    ui.horizontal(|ui| {
                                                                        let star_col = if is_active { VortexTheme::POT_YELLOW } else { Color32::from_rgb(160, 165, 180) };
                                                                        ui.label(RichText::new(format!("★ {}", name)).strong().color(star_col));

                                                                        if is_active {
                                                                            ui.label(RichText::new("[Active]").size(9.5).color(VortexTheme::POT_YELLOW));
                                                                        }

                                                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                                            if ui.button(RichText::new("🗑 Delete").color(Color32::from_rgb(240, 90, 90))).clicked() {
                                                                                to_del = Some(name.clone());
                                                                            }
                                                                            if ui.button("⟳ Overwrite").on_hover_text("Update this preset with current 18-band slider values").clicked() {
                                                                                to_overwrite = Some(name.clone());
                                                                            }
                                                                            if !is_active && ui.button("Load").clicked() {
                                                                                to_load = Some(name.clone());
                                                                            }
                                                                        });
                                                                    });
                                                                });
                                                            ui.add_space(2.0);
                                                        }

                                                        if let Some(del_key) = to_del {
                                                            config.custom_eq_presets.remove(&del_key);
                                                            if config.eq_preset == del_key {
                                                                config.eq_preset = "Custom".to_string();
                                                            }
                                                            let _ = config.save();
                                                        }
                                                        if let Some(ow_key) = to_overwrite {
                                                            config.custom_eq_presets.insert(ow_key.clone(), config.eq_bands.clone());
                                                            config.eq_preset = ow_key;
                                                            let _ = config.save();
                                                            self.save_feedback_time = current_time;
                                                        }
                                                        if let Some(load_key) = to_load {
                                                            if let Some(bands) = config.custom_eq_presets.get(&load_key) {
                                                                config.eq_bands = bands.clone();
                                                                config.eq_preset = load_key;
                                                                if let Some(p) = player {
                                                                    p.set_equalizer(config.eq_enabled, &config.eq_bands);
                                                                }
                                                                let _ = config.save();
                                                            }
                                                        }
                                                    });
                                            }
                                        });
                                    }
                                    ("Audio", "Soxr Resampling & ReplayGain") => {
                                        section_header(ui, "Audio  ›  Soxr Resampling & ReplayGain", "Soxr Resampling & ReplayGain", "Audiophile rate conversion and automatic track/album volume leveling.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Sample Rate", "Target output resampling frequency", |ui| {
                                                egui::ComboBox::from_id_salt("soxr_resample_pref")
                                                    .selected_text(match config.resample_rate {
                                                        0 => "Disabled (Native Source)",
                                                        48000 => "48 kHz (Standard Studio)",
                                                        96000 => "96 kHz (Hi-Res Audiophile)",
                                                        192000 => "192 kHz (Ultra High Precision)",
                                                        _ => "Custom",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.resample_rate, 0, "Disabled (Native Source)");
                                                        ui.selectable_value(&mut config.resample_rate, 48000, "48 kHz (Standard Studio)");
                                                        ui.selectable_value(&mut config.resample_rate, 96000, "96 kHz (Hi-Res Audiophile)");
                                                        ui.selectable_value(&mut config.resample_rate, 192000, "192 kHz (Ultra High Precision)");
                                                    });
                                            });
                                            ui.separator();
                                            settings_row(ui, "ReplayGain Mode", "Automated loudness compensation standard", |ui| {
                                                egui::ComboBox::from_id_salt("replaygain_pref")
                                                    .selected_text(match config.replaygain_mode.as_str() {
                                                        "track" => "Track Gain",
                                                        "album" => "Album Gain",
                                                        _ => "Disabled",
                                                    })
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(&mut config.replaygain_mode, "no".to_string(), "Disabled");
                                                        ui.selectable_value(&mut config.replaygain_mode, "track".to_string(), "Track Gain (Per Song)");
                                                        ui.selectable_value(&mut config.replaygain_mode, "album".to_string(), "Album Gain (Per Release)");
                                                    });
                                            });
                                        });
                                    }

                                    // ──────────────────────────────────────────────
                                    // 5. SUBTITLES
                                    // ──────────────────────────────────────────────
                                    ("Subtitles", "Engine & Dual Subtitles") => {
                                        section_header(ui, "Subtitles  ›  Engine & Dual Subtitles", "Dual Subtitles & Renderer", "Simultaneous dual-track rendering and language learning support.");
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Multi-Track Subtitle Engine").size(12.5).strong().color(Color32::from_rgb(235, 240, 252)));
                                            ui.label(RichText::new("• Primary Subtitle: Rendered cleanly along the bottom of the video viewport\n• 2nd Dual Subtitle: Rendered along the top of the viewport for side-by-side language acquisition\n• Format Support: ASS, SSA, SRT, VTT, SUB, SAMI with full font attachment rendering").size(11.0).color(Color32::from_rgb(140, 146, 165)));
                                        });
                                    }
                                    ("Subtitles", "Typography & Positioning") => {
                                        section_header(ui, "Subtitles  ›  Typography & Positioning", "Typography & Positioning", "Font styling, sizing, outline weight, and letterbox canvas placement.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Font Family", "Custom typeface font name", |ui| {
                                                ui.add(egui::TextEdit::singleline(&mut config.subtitle_sub_font).hint_text("e.g. Segoe UI, Arial, Trebuchet MS"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Font Size", "Baseline typography point size", |ui| {
                                                ui.add(egui::Slider::new(&mut config.subtitle_font_size, 14.0..=72.0).suffix(" pt"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Vertical Position", "Vertical distance from screen top", |ui| {
                                                ui.add(egui::Slider::new(&mut config.subtitle_vertical_pos, 40.0..=100.0).suffix(" %"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Outline Thickness", "High-contrast shadow border width", |ui| {
                                                ui.add(egui::Slider::new(&mut config.subtitle_outline_width, 0.0..=8.0).suffix(" px"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Display Canvas Placement", "Letterbox black bar vs inside video bounds", |ui| {
                                                ui.vertical(|ui| {
                                                    ui.radio_value(&mut config.subtitle_render_to_video, false, "Render in Black Bar Letterbox (Default)");
                                                    ui.radio_value(&mut config.subtitle_render_to_video, true, "Force Render Inside Video Frame");
                                                });
                                            });
                                        });
                                    }
                                    ("Subtitles", "Language Priority & Download") => {
                                        section_header(ui, "Subtitles  ›  Language Priority & Download", "Language Priority & OpenSubtitles", "Automated track matching priority and online subtitle downloads.");
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Automatic Track Matching Priority").size(12.5).strong().color(Color32::from_rgb(235, 240, 252)));
                                            ui.label(RichText::new("VortexPlayer automatically scans embedded and external subtitle files in order of priority:\n1. Hindi (hi / hin)\n2. English (en / eng)\n3. Japanese (ja / jpn)").size(11.0).color(Color32::from_rgb(140, 146, 165)));
                                            ui.add_space(4.0);
                                            ui.label(RichText::new("💡 Press Ctrl + L at any time during playback to open the OpenSubtitles search modal and fetch matched subtitles instantly.").size(10.5).color(VortexTheme::POT_YELLOW));
                                        });
                                    }

                                    // ──────────────────────────────────────────────
                                    // 6. INPUT & HOTKEYS
                                    // ──────────────────────────────────────────────
                                    ("Input & Hotkeys", "Keyboard & Global Hotkeys") => {
                                        section_header(ui, "Input & Hotkeys  ›  Keyboard & Global Hotkeys", "Keyboard & Global Hotkeys", "Global Windows multimedia hotkeys and background control.");
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Registered System Multimedia Shortcuts").size(12.5).strong().color(Color32::from_rgb(235, 240, 252)));
                                            ui.label(RichText::new("✓ Hardware Multimedia Keys: Play / Pause, Next, Prev, Stop, Mute\n✓ Ctrl + Alt + Space : Play / Pause in background\n✓ Ctrl + Alt + Right / Left : Next / Prev file in background\n✓ Ctrl + Alt + Up / Down : Volume Up / Down in background\n✓ Ctrl + Alt + V : Focus and bring VortexPlayer to foreground").size(11.0).color(Color32::from_rgb(140, 146, 165)));
                                        });
                                    }
                                    ("Input & Hotkeys", "Gamepad & Controller") => {
                                        section_header(ui, "Input & Hotkeys  ›  Gamepad & Controller", "Gamepad & Controller", "DirectInput and XInput controller support.");
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Gamepad Controls Mapping").size(12.5).strong().color(Color32::from_rgb(235, 240, 252)));
                                            ui.label(RichText::new("✓ A Button / Cross : Play / Pause\n✓ D-Pad Left / Right : Seek Timeline ±10s\n✓ Left / Right Triggers (LT / RT) : Volume Down / Up\n✓ Start Button : Toggle Fullscreen").size(11.0).color(Color32::from_rgb(140, 146, 165)));
                                        });
                                    }

                                    // ──────────────────────────────────────────────
                                    // 7. NETWORK & CLOUD
                                    // ──────────────────────────────────────────────
                                    ("Network & Cloud", "Stream Buffering & Protocols") => {
                                        section_header(ui, "Network & Cloud  ›  Stream Buffering & Protocols", "Stream Buffering & Protocols", "Direct stream caching, network shares, and media protocols.");
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Streaming Protocols Supported").size(12.5).strong().color(Color32::from_rgb(235, 240, 252)));
                                            ui.label(RichText::new("✓ Network Shares: SMB (Windows Share), WebDAV, FTP, DLNA, UNC Share paths\n✓ Direct Web Streams: HLS (.m3u8), DASH, RTSP, YouTube / yt-dlp\n✓ Press Ctrl + N to launch the Network Media Browser Studio").size(11.0).color(Color32::from_rgb(140, 146, 165)));
                                        });
                                    }

                                    // ──────────────────────────────────────────────
                                    // 8. INTEGRATION
                                    // ──────────────────────────────────────────────
                                    ("Integration", "Windows SMTC & Shell") => {
                                        section_header(ui, "Integration  ›  Windows SMTC & Shell", "Windows Shell Integration", "System Media Transport Controls, taskbar controls, and shell associations.");
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Windows Desktop Integration Features").size(12.5).strong().color(Color32::from_rgb(235, 240, 252)));
                                            ui.label(RichText::new("✓ Windows System Media Transport Controls (SMTC overlay)\n✓ Windows Taskbar Live Scrubbing Progress and Thumbnail Toolbar Buttons\n✓ Registered File Associations: .mkv, .mp4, .avi, .webm, .ts, .mov, .flac, .mp3, .aac, .srt, .ass").size(11.0).color(Color32::from_rgb(140, 146, 165)));
                                        });
                                    }

                                    // ──────────────────────────────────────────────
                                    // 9. ADVANCED
                                    // ──────────────────────────────────────────────
                                    ("Advanced", "Profiles, Backup & Reset") => {
                                        section_header(ui, "Advanced  ›  Profiles, Backup & Reset", "Profiles, Backup & Reset", "Tune engine buffers, inject custom mpv properties, and manage configuration files.");
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Demuxer Readahead Cache", "Forward read buffer duration for instant seek response", |ui| {
                                                ui.add(egui::Slider::new(&mut config.cache_demuxer_sec, 5.0..=180.0).suffix(" sec"));
                                            });
                                            ui.separator();
                                            settings_row(ui, "Max Demuxer Memory Cache", "Maximum RAM allocated for stream buffering", |ui| {
                                                ui.add(egui::Slider::new(&mut config.cache_demuxer_mb, 32..=2048).suffix(" MB"));
                                            });
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            ui.label(RichText::new("Custom mpv.conf Parameters & Engine Direct Overrides").size(12.5).strong().color(Color32::from_rgb(235, 240, 252)));
                                            ui.label(RichText::new("Inject arbitrary key=value parameters directly into the underlying media engine pipeline:").size(10.5).color(Color32::from_rgb(140, 146, 168)));
                                            ui.add(
                                                egui::TextEdit::multiline(&mut config.custom_mpv_options)
                                                    .hint_text("e.g.\ndemuxer-max-bytes=500MiB\ninterpolation=yes\ntscale=oversample\nhwdec-codecs=all")
                                                    .desired_width(ui.available_width())
                                                    .desired_rows(5)
                                                    .font(egui::TextStyle::Monospace),
                                            );
                                        });
                                        ui.add_space(8.0);
                                        settings_card(ui, |ui| {
                                            settings_row(ui, "Configuration File", "Active config path in user AppData", |ui| {
                                                ui.label(RichText::new("%APPDATA%/vortex-player/config.json").size(11.0).color(Color32::from_rgb(160, 165, 185)));
                                            });
                                            ui.separator();
                                            ui.horizontal(|ui| {
                                                if ui.button("📁 Export Settings to JSON...").clicked() {
                                                    if let Some(dest) = rfd::FileDialog::new()
                                                        .add_filter("Configuration (*.json)", &["json"])
                                                        .set_file_name("vortex-config-backup.json")
                                                        .save_file()
                                                    {
                                                        if let Ok(json_str) = serde_json::to_string_pretty(config) {
                                                            let _ = std::fs::write(dest, json_str);
                                                        }
                                                    }
                                                }
                                                if ui.button("📂 Import Settings from JSON...").clicked() {
                                                    if let Some(src) = rfd::FileDialog::new()
                                                        .add_filter("Configuration (*.json)", &["json"])
                                                        .pick_file()
                                                    {
                                                        if let Ok(json_str) = std::fs::read_to_string(src) {
                                                            if let Ok(imported) = serde_json::from_str::<AppConfig>(&json_str) {
                                                                *config = imported;
                                                                VortexTheme::apply(ui.ctx(), config.theme_mode);
                                                            }
                                                        }
                                                    }
                                                }
                                                if ui.button("⚠️ Reset All Defaults").clicked() {
                                                    *config = AppConfig::default();
                                                    VortexTheme::apply(ui.ctx(), config.theme_mode);
                                                }
                                            });
                                        });
                                    }

                                    _ => {
                                        ui.label("Select a category from the navigation tree.");
                                    }
                                }
                            });
                    });
                });

                ui.add_space(6.0);
                ui.separator();
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(8.0, 0.0);
                    // Left status pill
                    ui.label(RichText::new("●").color(Color32::from_rgb(80, 220, 120)).size(10.0));
                    ui.label(RichText::new("Direct3D 11 Active  •  Settings auto-saved").size(11.0).color(Color32::from_rgb(140, 146, 165)));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let apply_to_player = |cfg: &AppConfig| {
                            if let Some(p) = player {
                                let ch_str = match cfg.audio_channels.as_str() {
                                    "auto" | "auto-safe" | "" => "auto",
                                    "2.0" => "stereo",
                                    "1.0" => "mono",
                                    other => other,
                                };
                                p.set_audio_channels(ch_str);
                                p.set_audio_delay(cfg.audio_delay);
                                p.set_wasapi_exclusive(cfg.wasapi_exclusive);
                                p.set_equalizer(cfg.eq_enabled, &cfg.eq_bands);
                                p.set_hwdec(&cfg.hardware_decoding);
                                p.set_aspect_ratio(&cfg.aspect_ratio);
                                p.set_video_align_y(&cfg.video_align_y);
                                p.set_subtitle_font_size(cfg.subtitle_font_size);
                                p.set_subtitle_pos(cfg.subtitle_vertical_pos);
                                p.set_subtitle_border_size(cfg.subtitle_outline_width);
                                if !cfg.subtitle_sub_font.is_empty() {
                                    p.set_subtitle_font(&cfg.subtitle_sub_font);
                                }
                                p.set_property_string("sub-use-margins", if cfg.subtitle_render_to_video { "no" } else { "yes" });
                                p.set_property_string("video-sync", &cfg.video_sync_mode);
                                p.set_property_string("framedrop", &cfg.framedrop_mode);
                                p.set_property_double("demuxer-readahead-secs", cfg.cache_demuxer_sec);
                                let cache_bytes = (cfg.cache_demuxer_mb as u64) * 1024 * 1024;
                                p.set_property_string("demuxer-max-bytes", &format!("{}", cache_bytes));
                                for line in cfg.custom_mpv_options.lines() {
                                    let trimmed = line.trim();
                                    if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
                                    if let Some((k, v)) = trimmed.split_once('=') {
                                        p.set_property_string(k.trim(), v.trim());
                                    }
                                }
                                if cfg.audio_normalize {
                                    p.set_audio_filter("dynaudnorm=g=5:f=250:r=0.9:p=0.95");
                                }
                            }
                        };

                        // Apply & Close: Primary PotPlayer Gold
                        let apply_close_btn = ui.add(
                            egui::Button::new(RichText::new("Apply & Close (Enter)").strong().color(Color32::from_rgb(18, 20, 26)))
                                .fill(VortexTheme::POT_YELLOW)
                                .corner_radius(CornerRadius::same(6))
                                .min_size(Vec2::new(140.0, 28.0))
                        );
                        if apply_close_btn.clicked() {
                            apply_to_player(config);
                            let _ = config.save();
                            should_close = true;
                        }

                        // Apply: Secondary styled
                        let apply_btn = ui.add(
                            egui::Button::new(RichText::new("Apply").color(Color32::from_rgb(220, 225, 240)))
                                .fill(Color32::from_rgb(32, 36, 48))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(55, 60, 78)))
                                .corner_radius(CornerRadius::same(6))
                                .min_size(Vec2::new(75.0, 28.0))
                        );
                        if apply_btn.clicked() {
                            apply_to_player(config);
                            let _ = config.save();
                        }

                        // Cancel
                        let cancel_btn = ui.add(
                            egui::Button::new(RichText::new("Cancel").color(Color32::from_rgb(160, 165, 180)))
                                .fill(Color32::TRANSPARENT)
                                .stroke(Stroke::new(1.0, Color32::from_rgb(45, 50, 65)))
                                .corner_radius(CornerRadius::same(6))
                                .min_size(Vec2::new(70.0, 28.0))
                        );
                        if cancel_btn.clicked() {
                            should_close = true;
                        }
                    });
                });
            });
        });

        if should_close {
            *is_open = false;
        }
    }
}
