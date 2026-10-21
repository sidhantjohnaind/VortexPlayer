use super::theme::VortexTheme;
use crate::bookmark::BookmarkManager;
use crate::config::AppConfig;
use crate::engine::audio_dsp::VORTEX_EQ_PRESETS;
use crate::engine::{MediaStats, Player};
use crate::playlist::Playlist;
use eframe::egui::{self, Align, Color32, CornerRadius, Layout, Rect, RichText, Stroke, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlPanelTab {
    Video,
    Audio,
    Playback,
    Subtitles,
}

impl Default for ControlPanelTab {
    fn default() -> Self {
        Self::Video
    }
}

pub struct ControlPanel {
    pub active_tab: ControlPanelTab,
    pub new_preset_name: String,
    pub is_naming_preset: bool,
    pub save_feedback_time: f64,
    pub drag_offset: Option<Vec2>,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self {
            active_tab: ControlPanelTab::Video,
            new_preset_name: String::new(),
            is_naming_preset: false,
            save_feedback_time: 0.0,
            drag_offset: None,
        }
    }
}

/// Helper to render a consistent obsidian card frame
fn card_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(Color32::from_rgb(22, 25, 34))
        .stroke(Stroke::new(1.0, Color32::from_rgb(38, 43, 58)))
        .corner_radius(CornerRadius::same(7))
        .inner_margin(egui::Margin::symmetric(14, 10))
}

/// Helper to render a selectable pill button
fn pill_button(ui: &mut egui::Ui, label: &str, is_selected: bool) -> egui::Response {
    let text_col = if is_selected {
        VortexTheme::current_skin().accent_primary
    } else {
        Color32::from_rgb(165, 170, 185)
    };
    let bg_col = if is_selected {
        Color32::from_rgb(42, 38, 18)
    } else {
        Color32::from_rgb(28, 31, 42)
    };
    let stroke_col = if is_selected {
        VortexTheme::current_skin().accent_primary
    } else {
        Color32::from_rgb(46, 51, 68)
    };

    ui.add(
        egui::Button::new(RichText::new(label).color(text_col).size(11.0).strong())
            .fill(bg_col)
            .stroke(Stroke::new(1.0, stroke_col))
            .corner_radius(CornerRadius::same(4)),
    )
}

impl ControlPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        player: &Player,
        stats: &MediaStats,
        config: &mut AppConfig,
        bookmark_mgr: &mut BookmarkManager,
        playlist: &mut Playlist,
    ) -> Option<Rect> {
        if !*is_open {
            self.drag_offset = None;
            return None;
        }

        let anchor_offset = self.drag_offset.unwrap_or(Vec2::ZERO);
        let mut close_requested = false;
        let mut drag_delta = Vec2::ZERO;

        let resp = egui::Window::new("vortex_control_panel_win")
            .title_bar(false)
            .collapsible(false)
            .resizable(true)
            .default_width(580.0)
            .min_width(550.0)
            .anchor(egui::Align2::CENTER_CENTER, anchor_offset)
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(16, 18, 25))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(42, 47, 64)))
                    .corner_radius(CornerRadius::same(10))
                    .inner_margin(egui::Margin::symmetric(14, 12)),
            )
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 9.0);

                // ── 0. Custom Sleek Titlebar (Draggable) ─────────────────────
                let title_resp = ui.horizontal(|ui| {
                    ui.label(RichText::new("Vortex Control Panel").size(13.0).strong().color(Color32::WHITE));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let close_btn = ui.add(
                            egui::Button::new(RichText::new("✕").size(12.0).color(Color32::from_rgb(175, 180, 195)))
                                .fill(Color32::TRANSPARENT)
                                .stroke(Stroke::NONE)
                        );
                        if close_btn.clicked() {
                            close_requested = true;
                        }
                    });
                });

                let drag_interact = ui.interact(title_resp.response.rect, ui.id().with("cp_title_bar_drag"), egui::Sense::drag());
                if drag_interact.dragged() {
                    drag_delta = drag_interact.drag_delta();
                }

                // ── 1. Modern Segmented Tab Bar ──────────────────────────────
                egui::Frame::new()
                    .fill(Color32::from_rgb(11, 12, 18))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(28, 33, 46)))
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(egui::Margin::symmetric(4, 4))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
                            let tabs = [
                                (ControlPanelTab::Video, "Video & Colors"),
                                (ControlPanelTab::Audio, "Audio & EQ"),
                                (ControlPanelTab::Playback, "Playback & Loop"),
                                (ControlPanelTab::Subtitles, "Subtitles"),
                            ];

                            let tab_w = ((ui.available_width() - 12.0) / 4.0).max(85.0);
                            for (tab, label) in tabs {
                                let is_active = self.active_tab == tab;
                                let text_col = if is_active { VortexTheme::current_skin().accent_primary } else { Color32::from_rgb(165, 170, 185) };
                                let bg_col = if is_active { Color32::from_rgb(34, 38, 54) } else { Color32::TRANSPARENT };
                                let stroke_col = if is_active { Color32::from_rgb(58, 68, 94) } else { Color32::TRANSPARENT };

                                let tab_btn = ui.add_sized(
                                    Vec2::new(tab_w, 28.0),
                                    egui::Button::new(RichText::new(label).color(text_col).size(11.5).strong())
                                        .fill(bg_col)
                                        .stroke(Stroke::new(1.0, stroke_col))
                                        .corner_radius(CornerRadius::same(4)),
                                );
                                if tab_btn.clicked() {
                                    self.active_tab = tab;
                                }
                            }
                        });
                    });

                match self.active_tab {
                    // =========================================================================
                    // 1. VIDEO TAB: REAL-TIME COLOR GRADING, ASPECT RATIO, DEINTERLACE, SHARPEN
                    // =========================================================================
                    ControlPanelTab::Video => {
                        // Card 1: Media Info Stream Card
                        card_frame().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let v_codec = if stats.video_codec.is_empty() { "No Media Loaded" } else { &stats.video_codec };
                                ui.label(RichText::new("CODEC:").size(9.5).strong().color(Color32::from_rgb(130, 138, 158)));
                                ui.label(RichText::new(v_codec).size(11.5).strong().color(VortexTheme::current_skin().accent_primary));

                                if stats.video_width > 0 && stats.video_height > 0 {
                                    ui.add_space(8.0);
                                    ui.separator();
                                    ui.add_space(8.0);
                                    ui.label(RichText::new(format!("{} × {}  ·  {:.2} fps", stats.video_width, stats.video_height, stats.video_fps)).size(11.0).color(Color32::from_rgb(155, 195, 255)));
                                }
                                if stats.is_hdr {
                                    ui.add_space(8.0);
                                    ui.separator();
                                    ui.add_space(8.0);
                                    ui.label(RichText::new(format!("⭐ {}", stats.hdr_format)).size(10.5).color(VortexTheme::VORTEX_LIME_OSD).strong());
                                }
                            });
                        });

                        // Card 2: Color Adjustments with Precision Grid Alignment
                        card_frame().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Color Adjustments").strong().size(12.5).color(Color32::WHITE));
                                ui.label(RichText::new("(Default: 100% Neutral)").size(10.5).color(Color32::from_rgb(120, 126, 142)));

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.button(RichText::new("↺ Reset Colors (Q)").size(11.0).strong().color(VortexTheme::current_skin().accent_primary)).clicked() {
                                        config.video_brightness = 100.0;
                                        config.video_contrast = 100.0;
                                        config.video_saturation = 100.0;
                                        config.video_hue = 0.0;
                                        player.reset_video_colors();
                                        let _ = config.save();
                                    }
                                });
                            });

                            ui.add_space(6.0);

                            // Brightness
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Brightness:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 310.0;
                                let slider = ui.add(egui::Slider::new(&mut config.video_brightness, 0.0..=200.0).suffix("%").show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:.0}%", config.video_brightness)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if ui.small_button("↺").on_hover_text("Reset Brightness to default 100%").clicked() {
                                    config.video_brightness = 100.0;
                                    player.set_video_brightness(0.0);
                                    let _ = config.save();
                                }

                                if slider.changed() {
                                    player.set_video_brightness(config.video_brightness - 100.0);
                                    let _ = config.save();
                                }
                            });

                            // Contrast
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Contrast:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 310.0;
                                let slider = ui.add(egui::Slider::new(&mut config.video_contrast, 0.0..=200.0).suffix("%").show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:.0}%", config.video_contrast)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if ui.small_button("↺").on_hover_text("Reset Contrast to default 100%").clicked() {
                                    config.video_contrast = 100.0;
                                    player.set_video_contrast(0.0);
                                    let _ = config.save();
                                }

                                if slider.changed() {
                                    player.set_video_contrast(config.video_contrast - 100.0);
                                    let _ = config.save();
                                }
                            });

                            // Saturation
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Saturation:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 310.0;
                                let slider = ui.add(egui::Slider::new(&mut config.video_saturation, 0.0..=200.0).suffix("%").show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:.0}%", config.video_saturation)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if ui.small_button("↺").on_hover_text("Reset Saturation to default 100%").clicked() {
                                    config.video_saturation = 100.0;
                                    player.set_video_saturation(0.0);
                                    let _ = config.save();
                                }

                                if slider.changed() {
                                    player.set_video_saturation(config.video_saturation - 100.0);
                                    let _ = config.save();
                                }
                            });

                            // Hue
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Hue:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 310.0;
                                let slider = ui.add(egui::Slider::new(&mut config.video_hue, -180.0..=180.0).suffix("°").show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:.0}°", config.video_hue)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if ui.small_button("↺").on_hover_text("Reset Hue to default 0°").clicked() {
                                    config.video_hue = 0.0;
                                    player.set_video_hue(0.0);
                                    let _ = config.save();
                                }

                                if slider.changed() {
                                    player.set_video_hue(config.video_hue);
                                    let _ = config.save();
                                }
                            });
                        });

                        // Card 3: Framing, Display & Enhancement
                        card_frame().show(ui, |ui| {
                            // Aspect Ratio Row
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 22.0], egui::Label::new(RichText::new("Aspect Ratio:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                let ar_presets = [
                                    ("auto", "Auto"),
                                    ("16:9", "16:9"),
                                    ("4:3", "4:3"),
                                    ("21:9", "21:9"),
                                    ("2.35:1", "2.35:1"),
                                ];
                                for (val, label) in ar_presets {
                                    if pill_button(ui, label, config.aspect_ratio == val).clicked() {
                                        config.aspect_ratio = val.to_string();
                                        player.set_aspect_ratio(val);
                                        let _ = config.save();
                                    }
                                }
                            });

                            ui.add_space(5.0);

                            // Rotation Row
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 22.0], egui::Label::new(RichText::new("Rotation:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                for &deg in &[0, 90, 180, 270] {
                                    let label = format!("{}°", deg);
                                    if pill_button(ui, &label, config.video_rotation == deg).clicked() {
                                        config.video_rotation = deg;
                                        player.set_rotation(deg);
                                        let _ = config.save();
                                    }
                                }
                            });

                            ui.add_space(6.0);

                            // Deinterlacing & Sharpening
                            ui.horizontal(|ui| {
                                let mut deint = config.video_deinterlace;
                                if ui.checkbox(&mut deint, RichText::new("Hardware Deinterlacing (Yadif)").size(11.5)).changed() {
                                    config.video_deinterlace = deint;
                                    player.set_deinterlace(deint);
                                    let _ = config.save();
                                }

                                ui.add_space(16.0);
                                ui.label(RichText::new("Sharpen:").size(11.5).color(Color32::from_rgb(175, 180, 195)));
                                let slider = ui.add_sized([120.0, 20.0], egui::Slider::new(&mut config.video_sharpen, 0.0..=5.0));
                                if slider.changed() {
                                    player.set_property_string("vf", &format!("unsharp=5:5:{:.2}:5:5:0.0", config.video_sharpen));
                                    let _ = config.save();
                                }
                            });
                        });
                    }

                    // =========================================================================
                    // 2. AUDIO TAB: 18-BAND GRAPHIC EQUALIZER, PRESETS, NORMALIZER, DELAY
                    // =========================================================================
                    ControlPanelTab::Audio => {
                        let current_time = ctx.input(|i| i.time);
                        let is_recently_saved = current_time - self.save_feedback_time < 2.0;

                        // Card 1: Equalizer Mode, Presets & Management
                        card_frame().show(ui, |ui| {
                            // Row 1: Master Enable, Status Badge & Preset Selector
                            ui.horizontal(|ui| {
                                let mut eq_en = config.eq_enabled;
                                let is_eq = eq_en;
                                if ui.checkbox(&mut eq_en, RichText::new("Equalizer Active").strong().color(if is_eq { VortexTheme::current_skin().accent_primary } else { Color32::WHITE })).changed() {
                                    config.eq_enabled = eq_en;
                                    player.set_equalizer(eq_en, &config.eq_bands);
                                    let _ = config.save();
                                }

                                ui.add_space(4.0);

                                if is_eq {
                                    ui.label(RichText::new("● RUNNING").size(9.5).strong().color(VortexTheme::VORTEX_LIME_OSD));
                                } else {
                                    ui.label(RichText::new("○ BYPASS").size(9.5).color(Color32::from_rgb(130, 135, 150)));
                                }

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.button("↺ Flat (0 dB)").on_hover_text("Reset all 18 equalizer bands to 0 dB flat").clicked() {
                                        config.eq_bands = vec![0.0; 18];
                                        config.eq_preset = "Flat".to_string();
                                        player.set_equalizer(config.eq_enabled, &config.eq_bands);
                                        let _ = config.save();
                                    }

                                    egui::ComboBox::from_id_salt("cp_eq_preset")
                                        .selected_text(RichText::new(&config.eq_preset).color(VortexTheme::current_skin().accent_primary).strong())
                                        .show_ui(ui, |ui| {
                                            ui.label(RichText::new("── Built-in Presets ──").size(10.0).color(Color32::from_rgb(120, 125, 140)));
                                            for preset in VORTEX_EQ_PRESETS {
                                                if ui.selectable_label(config.eq_preset == preset.name, preset.name).clicked() {
                                                    config.eq_preset = preset.name.to_string();
                                                    config.eq_bands = preset.bands.to_vec();
                                                    player.set_equalizer(config.eq_enabled, &config.eq_bands);
                                                    let _ = config.save();
                                                }
                                            }

                                            if !config.custom_eq_presets.is_empty() {
                                                ui.separator();
                                                ui.label(RichText::new("── Custom Presets ──").size(10.0).color(VortexTheme::current_skin().accent_primary));
                                                let mut to_delete: Option<String> = None;
                                                let custom_names: Vec<String> = config.custom_eq_presets.keys().cloned().collect();
                                                for c_name in custom_names {
                                                    ui.horizontal(|ui| {
                                                        let is_sel = config.eq_preset == c_name;
                                                        if ui.selectable_label(is_sel, format!("★ {}", c_name)).clicked() {
                                                            if let Some(b) = config.custom_eq_presets.get(&c_name) {
                                                                config.eq_bands = b.clone();
                                                                config.eq_preset = c_name.clone();
                                                                player.set_equalizer(config.eq_enabled, &config.eq_bands);
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

                                    ui.label(RichText::new("Preset:").color(Color32::from_rgb(150, 155, 170)));
                                });
                            });

                            ui.add_space(4.0);

                            // Row 2: Action Toolbar (Save to disk, Save As Custom Preset, Overwrite)
                            ui.horizontal(|ui| {
                                let save_txt = if is_recently_saved { "✓ Saved to disk!" } else { "💾 Save Config" };
                                let save_col = if is_recently_saved { Color32::from_rgb(100, 225, 140) } else { Color32::WHITE };
                                if ui.button(RichText::new(save_txt).color(save_col)).on_hover_text("Save current equalizer configuration to config.json").clicked() {
                                    let _ = config.save();
                                    self.save_feedback_time = current_time;
                                }

                                if ui.button("➕ Save As Preset...").on_hover_text("Save current band values as a new named custom preset").clicked() {
                                    self.is_naming_preset = !self.is_naming_preset;
                                    if self.is_naming_preset {
                                        self.new_preset_name = format!("Custom {}", config.custom_eq_presets.len() + 1);
                                    }
                                }

                                if config.custom_eq_presets.contains_key(&config.eq_preset) {
                                    if ui.button("⟳ Overwrite").on_hover_text("Update selected custom preset with current bands").clicked() {
                                        config.custom_eq_presets.insert(config.eq_preset.clone(), config.eq_bands.clone());
                                        let _ = config.save();
                                        self.save_feedback_time = current_time;
                                    }
                                    if ui.button(RichText::new("🗑 Delete").color(Color32::from_rgb(240, 90, 90))).on_hover_text("Delete selected custom preset").clicked() {
                                        config.custom_eq_presets.remove(&config.eq_preset);
                                        config.eq_preset = "Flat".to_string();
                                        let _ = config.save();
                                    }
                                }
                            });

                            // Inline Save Preset Dialog
                            if self.is_naming_preset {
                                ui.add_space(4.0);
                                egui::Frame::new()
                                    .fill(Color32::from_rgb(14, 15, 20))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(46, 52, 70)))
                                    .corner_radius(CornerRadius::same(5))
                                    .inner_margin(egui::Margin::symmetric(10, 6))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new("Preset Name:").size(11.0).color(VortexTheme::current_skin().accent_primary).strong());
                                            let text_resp = ui.add(
                                                egui::TextEdit::singleline(&mut self.new_preset_name)
                                                    .desired_width(180.0)
                                                    .hint_text("e.g. Acoustic Master")
                                            );
                                            if (text_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) || ui.button("💾 Save").clicked() {
                                                let name = self.new_preset_name.trim().to_string();
                                                if !name.is_empty() {
                                                    config.custom_eq_presets.insert(name.clone(), config.eq_bands.clone());
                                                    config.eq_preset = name;
                                                    let _ = config.save();
                                                    self.is_naming_preset = false;
                                                    self.save_feedback_time = current_time;
                                                }
                                            }
                                            if ui.button("Cancel").clicked() {
                                                self.is_naming_preset = false;
                                            }
                                        });
                                    });
                            }
                        });

                        // Card 2: 18-Band Precision Graphic Equalizer Faders
                        card_frame().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("18-Band Precision Graphic Equalizer").strong().size(12.0).color(Color32::WHITE));
                                ui.label(RichText::new("  (±12 dB range)").size(10.5).color(Color32::from_rgb(130, 136, 155)));
                            });
                            ui.add_space(4.0);

                            let freqs = ["20", "31", "50", "80", "125", "200", "315", "500", "800", "1.2k", "2k", "3.1k", "5k", "8k", "12k", "16k", "18k", "20k"];
                            if config.eq_bands.len() != 18 {
                                config.eq_bands = vec![0.0; 18];
                            }

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(5.0, 0.0);
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

                                        let slider = ui.add_sized(
                                            Vec2::new(23.0, 110.0),
                                            egui::Slider::new(&mut gain, -12.0..=12.0)
                                                .vertical()
                                                .show_value(false),
                                        );
                                        if slider.changed() {
                                            config.eq_bands[i] = gain;
                                            config.eq_preset = "Custom".to_string();
                                            player.set_equalizer(config.eq_enabled, &config.eq_bands);
                                            let _ = config.save();
                                        }

                                        ui.label(RichText::new(freqs[i]).size(8.0).monospace().color(Color32::from_rgb(140, 145, 160)));
                                    });
                                }
                            });
                        });

                        // Card 3: Audio Sync Delay & Volume Normalizer
                        card_frame().show(ui, |ui| {
                            // Audio Delay
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Audio Delay:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 190.0;
                                let mut delay = config.audio_delay;
                                let slider = ui.add(egui::Slider::new(&mut delay, -5.0..=5.0).suffix("s").show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:+.2}s", config.audio_delay)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if slider.changed() {
                                    config.audio_delay = delay;
                                    player.set_audio_delay(delay);
                                    let _ = config.save();
                                }

                                if ui.small_button("-50ms").clicked() {
                                    config.audio_delay -= 0.05;
                                    player.adjust_audio_delay(-0.05);
                                    let _ = config.save();
                                }
                                if ui.small_button("+50ms").clicked() {
                                    config.audio_delay += 0.05;
                                    player.adjust_audio_delay(0.05);
                                    let _ = config.save();
                                }
                                if ui.small_button("↺ 0s").clicked() {
                                    config.audio_delay = 0.0;
                                    player.set_audio_delay(0.0);
                                    let _ = config.save();
                                }
                            });

                            ui.add_space(5.0);

                            // Dynamic Volume Normalizer
                            ui.horizontal(|ui| {
                                let mut norm = config.audio_normalize;
                                let is_norm = norm;
                                if ui.checkbox(&mut norm, RichText::new("Dynamic Volume Normalizer (Night Mode)").size(11.5).color(if is_norm { VortexTheme::current_skin().accent_primary } else { Color32::WHITE })).changed() {
                                    config.audio_normalize = norm;
                                    player.set_audio_normalize(norm);
                                    let _ = config.save();
                                }
                                ui.label(RichText::new("— Compresses dynamic range for dialogue clarity").size(10.5).color(Color32::from_rgb(130, 136, 150)));
                            });
                        });
                    }

                    // =========================================================================
                    // 3. PLAYBACK TAB: SPEED MULTIPLIER, PRESETS, A-B LOOP, REPEAT & SHUFFLE
                    // =========================================================================
                    ControlPanelTab::Playback => {
                        // Card 1: Speed Multiplier & Presets
                        card_frame().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Playback Rate & Transport").strong().size(12.5).color(Color32::WHITE));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.button(RichText::new("1.0x (Normal)").color(VortexTheme::current_skin().accent_primary).size(11.0).strong()).clicked() {
                                        player.set_speed(1.0);
                                    }
                                });
                            });

                            ui.add_space(6.0);

                            // Speed Slider
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Speed:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 330.0;
                                let mut spd = stats.speed;
                                let slider = ui.add(egui::Slider::new(&mut spd, 0.2..=4.0).show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:.2}x", stats.speed)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if slider.changed() {
                                    player.set_speed(spd);
                                }
                            });

                            ui.add_space(4.0);

                            // Speed Presets Row
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 22.0], egui::Label::new(RichText::new("Presets:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                let speed_presets = [0.5, 0.8, 1.0, 1.2, 1.5, 2.0, 3.0];
                                for &preset in &speed_presets {
                                    let is_active = (stats.speed - preset).abs() < 0.05;
                                    let label = format!("{:.1}x", preset);
                                    if pill_button(ui, &label, is_active).clicked() {
                                        player.set_speed(preset);
                                    }
                                }
                            });

                            ui.add_space(4.0);

                            // Quick Seek Row
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 22.0], egui::Label::new(RichText::new("Quick Seek:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                if ui.button("⏪ -30s").clicked() {
                                    player.seek_relative(-30.0);
                                }
                                if ui.button("⏪ -5s").clicked() {
                                    player.seek_relative(-5.0);
                                }
                                if ui.button("⏩ +5s").clicked() {
                                    player.seek_relative(5.0);
                                }
                                if ui.button("⏩ +30s").clicked() {
                                    player.seek_relative(30.0);
                                }
                            });
                        });

                        // Card 2: Sequence & Order
                        card_frame().show(ui, |ui| {
                            ui.label(RichText::new("Playlist Sequence & Order").strong().size(12.5).color(Color32::WHITE));
                            ui.add_space(6.0);

                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 22.0], egui::Label::new(RichText::new("Repeat:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                let r_mode = playlist.repeat_mode;

                                if pill_button(ui, "Off", r_mode == crate::playlist::RepeatMode::Off).clicked() {
                                    playlist.repeat_mode = crate::playlist::RepeatMode::Off;
                                    config.playlist_repeat_mode = "Off".to_string();
                                    let _ = config.save();
                                }
                                if pill_button(ui, "🔁 All", r_mode == crate::playlist::RepeatMode::RepeatAll).clicked() {
                                    playlist.repeat_mode = crate::playlist::RepeatMode::RepeatAll;
                                    config.playlist_repeat_mode = "RepeatAll".to_string();
                                    let _ = config.save();
                                }
                                if pill_button(ui, "🔂 Track", r_mode == crate::playlist::RepeatMode::RepeatTrack).clicked() {
                                    playlist.repeat_mode = crate::playlist::RepeatMode::RepeatTrack;
                                    config.playlist_repeat_mode = "RepeatTrack".to_string();
                                    let _ = config.save();
                                }

                                ui.add_space(20.0);

                                let mut shuf = playlist.is_shuffle();
                                let is_shuf = shuf;
                                if ui.checkbox(&mut shuf, RichText::new("🔀 Shuffle").size(11.5).color(if is_shuf { VortexTheme::current_skin().accent_primary } else { Color32::WHITE })).changed() {
                                    playlist.set_shuffle(shuf);
                                    config.playlist_shuffle = shuf;
                                    let _ = config.save();
                                }
                            });
                        });

                        // Card 3: A-B Repeat Loop Segment
                        card_frame().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("A-B Segment Loop").strong().size(12.5).color(Color32::WHITE));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let has_loop = bookmark_mgr.ab_loop.point_a.is_some() && bookmark_mgr.ab_loop.point_b.is_some();
                                    if has_loop {
                                        ui.label(RichText::new("● LOOP ACTIVE").size(9.5).strong().color(VortexTheme::current_skin().accent_primary));
                                    } else {
                                        ui.label(RichText::new("○ INACTIVE").size(9.5).color(Color32::from_rgb(130, 135, 150)));
                                    }
                                });
                            });

                            ui.add_space(6.0);

                            let a_text = bookmark_mgr.ab_loop.point_a.map(|t| crate::bookmark::format_time(t)).unwrap_or_else(|| "--:--:--".to_string());
                            let b_text = bookmark_mgr.ab_loop.point_b.map(|t| crate::bookmark::format_time(t)).unwrap_or_else(|| "--:--:--".to_string());

                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 22.0], egui::Label::new(RichText::new("Range:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.label(RichText::new(format!("Point A: [ {} ]    Point B: [ {} ]", a_text, b_text)).size(11.5).monospace().color(Color32::from_rgb(210, 215, 230)));
                            });

                            ui.add_space(4.0);

                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 22.0], egui::Label::new(RichText::new("Actions:").size(11.5).color(Color32::from_rgb(175, 180, 195))));

                                let a_is_set = bookmark_mgr.ab_loop.point_a.is_some();
                                if ui.button(RichText::new("[ Set Point A").color(if a_is_set { VortexTheme::current_skin().accent_primary } else { Color32::WHITE })).clicked() {
                                    bookmark_mgr.set_loop_a(stats.time_pos);
                                }

                                let b_is_set = bookmark_mgr.ab_loop.point_b.is_some();
                                if ui.button(RichText::new("] Set Point B").color(if b_is_set { VortexTheme::current_skin().accent_primary } else { Color32::WHITE })).clicked() {
                                    let _ = bookmark_mgr.set_loop_b(stats.time_pos);
                                }

                                if ui.button("↺ Clear Loop (\\)").clicked() {
                                    bookmark_mgr.clear_ab_loop();
                                }
                            });
                        });
                    }

                    // =========================================================================
                    // 4. SUBTITLES TAB: SYNC DELAY, POSITION OFFSET, FONT SIZE, TRACK SELECT
                    // =========================================================================
                    ControlPanelTab::Subtitles => {
                        // Card 1: Subtitle Synchronization & Visibility
                        card_frame().show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Subtitle Synchronization").strong().size(12.5).color(Color32::WHITE));
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let mut sub_vis = stats.subtitles_visible;
                                    if ui.checkbox(&mut sub_vis, RichText::new("Show Subtitles (Alt+H)").size(11.5)).changed() {
                                        player.toggle_subtitles();
                                    }
                                });
                            });

                            ui.add_space(6.0);

                            // Sync Delay Slider & Steppers
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Sync Delay:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 175.0;
                                let mut delay = stats.subtitle_delay;
                                let slider = ui.add(egui::Slider::new(&mut delay, -10.0..=10.0).suffix("s").show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:+.2}s", stats.subtitle_delay)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if slider.changed() {
                                    player.set_subtitle_delay(delay);
                                }

                                if ui.small_button("-0.5s").clicked() {
                                    player.adjust_subtitle_delay(-0.5);
                                }
                                if ui.small_button("-0.05s").clicked() {
                                    player.adjust_subtitle_delay(-0.05);
                                }
                                if ui.small_button("+0.05s").clicked() {
                                    player.adjust_subtitle_delay(0.05);
                                }
                                if ui.small_button("+0.5s").clicked() {
                                    player.adjust_subtitle_delay(0.5);
                                }
                                if ui.small_button("↺ 0s").clicked() {
                                    player.set_subtitle_delay(0.0);
                                }
                            });
                        });

                        // Card 2: Typography & Sizing
                        card_frame().show(ui, |ui| {
                            ui.label(RichText::new("Typography & Placement").strong().size(12.5).color(Color32::WHITE));
                            ui.add_space(6.0);

                            // Font Size
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Font Size:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 300.0;
                                let mut sz = config.subtitle_font_size;
                                let slider = ui.add(egui::Slider::new(&mut sz, 12.0..=72.0).show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:.0} pt", config.subtitle_font_size)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if slider.changed() {
                                    config.subtitle_font_size = sz;
                                    player.set_subtitle_font_size(sz);
                                    let _ = config.save();
                                }

                                if ui.small_button("A-").clicked() {
                                    config.subtitle_font_size = (config.subtitle_font_size - 2.0).max(12.0);
                                    player.set_subtitle_font_size(config.subtitle_font_size);
                                    let _ = config.save();
                                }
                                if ui.small_button("A+").clicked() {
                                    config.subtitle_font_size = (config.subtitle_font_size + 2.0).min(72.0);
                                    player.set_subtitle_font_size(config.subtitle_font_size);
                                    let _ = config.save();
                                }
                            });

                            // Vertical Position
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Vertical Pos:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 230.0;
                                let mut pos = config.subtitle_vertical_pos;
                                let slider = ui.add(egui::Slider::new(&mut pos, 0.0..=115.0).show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:.0}%", config.subtitle_vertical_pos)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if slider.changed() {
                                    config.subtitle_vertical_pos = pos;
                                    player.set_subtitle_pos(pos);
                                    let _ = config.save();
                                }

                                if ui.small_button("▲ Up").clicked() {
                                    config.subtitle_vertical_pos = (config.subtitle_vertical_pos - 5.0).max(0.0);
                                    player.set_subtitle_pos(config.subtitle_vertical_pos);
                                    let _ = config.save();
                                }
                                if ui.small_button("▼ Down").clicked() {
                                    config.subtitle_vertical_pos = (config.subtitle_vertical_pos + 5.0).min(115.0);
                                    player.set_subtitle_pos(config.subtitle_vertical_pos);
                                    let _ = config.save();
                                }
                                if ui.small_button("↺ 102%").clicked() {
                                    config.subtitle_vertical_pos = 102.0;
                                    player.set_subtitle_pos(102.0);
                                    let _ = config.save();
                                }
                            });

                            // Outline Width
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Outline:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                ui.spacing_mut().slider_width = 360.0;
                                let mut ow = config.subtitle_outline_width;
                                let slider = ui.add(egui::Slider::new(&mut ow, 0.0..=8.0).show_value(false));
                                ui.add_sized([52.0, 20.0], egui::Label::new(RichText::new(format!("{:.1} px", config.subtitle_outline_width)).size(11.5).strong().color(Color32::from_rgb(220, 225, 238))));

                                if slider.changed() {
                                    config.subtitle_outline_width = ow;
                                    player.set_subtitle_border_size(ow);
                                    let _ = config.save();
                                }
                            });

                            ui.add_space(4.0);

                            // Font Styles
                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 20.0], egui::Label::new(RichText::new("Options:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                if ui.checkbox(&mut config.subtitle_bold, "Bold Font").changed() {
                                    player.set_subtitle_bold(config.subtitle_bold);
                                    let _ = config.save();
                                }

                                ui.add_space(12.0);

                                if ui.checkbox(&mut config.subtitle_background_box, "Background Box (Improves Legibility)").changed() {
                                    player.set_subtitle_background_box(config.subtitle_background_box, &config.subtitle_background_color);
                                    let _ = config.save();
                                }
                            });
                        });

                        // Card 3: Advanced Rendering & ASS Override
                        card_frame().show(ui, |ui| {
                            ui.label(RichText::new("Subtitle Rendering Engine").strong().size(12.5).color(Color32::WHITE));
                            ui.add_space(6.0);

                            ui.horizontal(|ui| {
                                ui.add_sized([82.0, 22.0], egui::Label::new(RichText::new("ASS Override:").size(11.5).color(Color32::from_rgb(175, 180, 195))));
                                let cur_ov = config.subtitle_ass_override.clone();
                                egui::ComboBox::from_id_salt("cp_ass_override")
                                    .selected_text(match cur_ov.as_str() {
                                        "no" => "Strict (Keep Original ASS)",
                                        "scale" => "Smart Scale (Recommended)",
                                        "yes" => "Allow Font Overrides",
                                        "force" => "Force All Overrides",
                                        "strip" => "Strip All ASS Tags",
                                        _ => "Smart Scale (Recommended)",
                                    })
                                    .show_ui(ui, |ui| {
                                        if ui.selectable_label(config.subtitle_ass_override == "scale", "Smart Scale (Recommended)").clicked() {
                                            config.subtitle_ass_override = "scale".to_string();
                                            player.set_subtitle_ass_override(&config.subtitle_ass_override);
                                            let _ = config.save();
                                        }
                                        if ui.selectable_label(config.subtitle_ass_override == "force", "Force All Overrides").clicked() {
                                            config.subtitle_ass_override = "force".to_string();
                                            player.set_subtitle_ass_override(&config.subtitle_ass_override);
                                            let _ = config.save();
                                        }
                                        if ui.selectable_label(config.subtitle_ass_override == "no", "Strict (Keep Original ASS)").clicked() {
                                            config.subtitle_ass_override = "no".to_string();
                                            player.set_subtitle_ass_override(&config.subtitle_ass_override);
                                            let _ = config.save();
                                        }
                                        if ui.selectable_label(config.subtitle_ass_override == "strip", "Strip All ASS Tags").clicked() {
                                            config.subtitle_ass_override = "strip".to_string();
                                            player.set_subtitle_ass_override(&config.subtitle_ass_override);
                                            let _ = config.save();
                                        }
                                    });
                            });
                        });
                    }
                }
            });

        if drag_delta != Vec2::ZERO {
            let cur = self.drag_offset.unwrap_or(Vec2::ZERO);
            self.drag_offset = Some(cur + drag_delta);
        }

        if close_requested {
            *is_open = false;
            self.drag_offset = None;
        }

        resp.map(|r| r.response.rect)
    }
}
