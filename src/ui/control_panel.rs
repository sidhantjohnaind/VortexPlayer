use super::theme::VortexTheme;
use crate::bookmark::BookmarkManager;
use crate::config::AppConfig;
use crate::engine::audio_dsp::POT_EQ_PRESETS;
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
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self {
            active_tab: ControlPanelTab::Video,
            new_preset_name: String::new(),
            is_naming_preset: false,
            save_feedback_time: 0.0,
        }
    }
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
            return None;
        }

        let resp = egui::Window::new("⚙ PotPlayer Control Panel (F6)")
            .open(is_open)
            .collapsible(false)
            .resizable(true)
            .default_width(550.0)
            .min_width(530.0)
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(0, 0, 0))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 50, 65)))
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(egui::Margin::same(12)),
            )
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 8.0);

                // ── 1. Modern Segmented Tab Bar ──────────────────────────────
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(5.0, 0.0);
                    let tabs = [
                        (ControlPanelTab::Video, "🎬 Video & Colors"),
                        (ControlPanelTab::Audio, "🎧 Audio & EQ"),
                        (ControlPanelTab::Playback, "⏩ Playback & Loop"),
                        (ControlPanelTab::Subtitles, "💬 Subtitles"),
                    ];

                    let tab_w = ((ui.available_width() - 15.0) / 4.0).max(85.0);
                    for (tab, label) in tabs {
                        let is_active = self.active_tab == tab;
                        let text_col = if is_active { VortexTheme::POT_YELLOW } else { Color32::from_rgb(175, 180, 195) };
                        let bg_col = if is_active { Color32::from_rgb(32, 28, 14) } else { Color32::from_rgb(14, 15, 20) };
                        let stroke_col = if is_active { VortexTheme::POT_YELLOW } else { Color32::from_rgb(38, 42, 54) };

                        let tab_btn = ui.add_sized(
                            Vec2::new(tab_w, 26.0),
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

                ui.add(egui::Separator::default().spacing(6.0));

                match self.active_tab {
                    // =========================================================================
                    // 1. VIDEO TAB: REAL-TIME COLOR GRADING, ASPECT RATIO, DEINTERLACE, SHARPEN
                    // =========================================================================
                    ControlPanelTab::Video => {
                        // Media Info strip
                        ui.horizontal(|ui| {
                            let v_codec = if stats.video_codec.is_empty() { "None" } else { &stats.video_codec };
                            ui.label(RichText::new(format!("Codec: {}", v_codec)).strong().color(VortexTheme::POT_YELLOW));
                            if stats.video_width > 0 && stats.video_height > 0 {
                                ui.label(RichText::new(format!("{}×{} @ {:.2}fps", stats.video_width, stats.video_height, stats.video_fps)).size(10.5).color(Color32::from_rgb(140, 190, 255)));
                            }
                            if stats.is_hdr {
                                ui.label(RichText::new(format!("⭐ {}", stats.hdr_format)).color(VortexTheme::POT_LIME_OSD).strong());
                            }
                        });

                        ui.separator();

                        // Color sliders with 100% baseline default (PotPlayer style)
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Color Adjustments").strong().color(VortexTheme::TEXT_PRIMARY));
                            ui.label(RichText::new("(Default: 100% Neutral)").size(10.5).color(Color32::from_rgb(130, 135, 150)));
                        });

                        // Brightness (0% - 200%, default 100%)
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Brightness:").size(11.0));
                            let slider = ui.add(egui::Slider::new(&mut config.video_brightness, 0.0..=200.0).suffix("%"));
                            if slider.changed() {
                                player.set_video_brightness(config.video_brightness - 100.0);
                                let _ = config.save();
                            }
                            if ui.small_button("↺").on_hover_text("Reset Brightness to default 100%").clicked() {
                                config.video_brightness = 100.0;
                                player.set_video_brightness(0.0);
                                let _ = config.save();
                            }
                        });

                        // Contrast (0% - 200%, default 100%)
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Contrast:").size(11.0));
                            let slider = ui.add(egui::Slider::new(&mut config.video_contrast, 0.0..=200.0).suffix("%"));
                            if slider.changed() {
                                player.set_video_contrast(config.video_contrast - 100.0);
                                let _ = config.save();
                            }
                            if ui.small_button("↺").on_hover_text("Reset Contrast to default 100%").clicked() {
                                config.video_contrast = 100.0;
                                player.set_video_contrast(0.0);
                                let _ = config.save();
                            }
                        });

                        // Saturation (0% - 200%, default 100%)
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Saturation:").size(11.0));
                            let slider = ui.add(egui::Slider::new(&mut config.video_saturation, 0.0..=200.0).suffix("%"));
                            if slider.changed() {
                                player.set_video_saturation(config.video_saturation - 100.0);
                                let _ = config.save();
                            }
                            if ui.small_button("↺").on_hover_text("Reset Saturation to default 100%").clicked() {
                                config.video_saturation = 100.0;
                                player.set_video_saturation(0.0);
                                let _ = config.save();
                            }
                        });

                        // Hue (-180° - +180°, default 0°)
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Hue:").size(11.0));
                            let slider = ui.add(egui::Slider::new(&mut config.video_hue, -180.0..=180.0).suffix("°"));
                            if slider.changed() {
                                player.set_video_hue(config.video_hue);
                                let _ = config.save();
                            }
                            if ui.small_button("↺").on_hover_text("Reset Hue to default 0°").clicked() {
                                config.video_hue = 0.0;
                                player.set_video_hue(0.0);
                                let _ = config.save();
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.button(RichText::new("↺ Reset All Colors to 100% (Q)").color(VortexTheme::POT_YELLOW)).clicked() {
                                config.video_brightness = 100.0;
                                config.video_contrast = 100.0;
                                config.video_saturation = 100.0;
                                config.video_hue = 0.0;
                                player.reset_video_colors();
                                let _ = config.save();
                            }
                        });

                        ui.separator();

                        // Aspect Ratio Presets
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Aspect Ratio:").size(11.0));
                            let ar_presets = [
                                ("auto", "Auto (Original)"),
                                ("16:9", "16:9"),
                                ("4:3", "4:3"),
                                ("21:9", "21:9"),
                                ("2.35:1", "2.35:1 (Cinema)"),
                            ];
                            for (val, label) in ar_presets {
                                if ui.button(label).clicked() {
                                    player.set_aspect_ratio(val);
                                }
                            }
                        });

                        // Deinterlace & Sharpening
                        ui.horizontal(|ui| {
                            let mut deint = config.video_deinterlace;
                            if ui.checkbox(&mut deint, "Hardware Deinterlacing (Yadif)").changed() {
                                config.video_deinterlace = deint;
                                player.set_deinterlace(deint);
                                let _ = config.save();
                            }

                            ui.add_space(10.0);
                            ui.label("Sharpen:");
                            let slider = ui.add(egui::Slider::new(&mut config.video_sharpen, 0.0..=5.0));
                            if slider.changed() {
                                player.set_property_string("vf", &format!("unsharp=5:5:{:.2}:5:5:0.0", config.video_sharpen));
                                let _ = config.save();
                            }
                        });

                        // Rotation
                        ui.horizontal(|ui| {
                            ui.label("Rotation:");
                            for &deg in &[0, 90, 180, 270] {
                                if ui.selectable_label(config.video_rotation == deg, format!("{}°", deg)).clicked() {
                                    config.video_rotation = deg;
                                    player.set_rotation(deg);
                                    let _ = config.save();
                                }
                            }
                        });
                    }

                    // =========================================================================
                    // 2. AUDIO TAB: 18-BAND GRAPHIC EQUALIZER, PRESETS, NORMALIZER, DELAY
                    // =========================================================================
                    ControlPanelTab::Audio => {
                        let current_time = ctx.input(|i| i.time);
                        let is_recently_saved = current_time - self.save_feedback_time < 2.0;

                        // ── Row 1: Enable Toggle, Reset, and Preset Selection ──
                        ui.horizontal(|ui| {
                            let mut eq_en = config.eq_enabled;
                            let is_eq = eq_en;
                            if ui.checkbox(&mut eq_en, RichText::new("Enable Equalizer").strong().color(if is_eq { VortexTheme::POT_YELLOW } else { Color32::WHITE })).changed() {
                                config.eq_enabled = eq_en;
                                player.set_equalizer(eq_en, &config.eq_bands);
                                let _ = config.save();
                            }

                            if ui.button("↺ Reset (0 dB)").on_hover_text("Reset all 18 bands to 0 dB flat").clicked() {
                                config.eq_bands = vec![0.0; 18];
                                config.eq_preset = "Flat".to_string();
                                player.set_equalizer(config.eq_enabled, &config.eq_bands);
                                let _ = config.save();
                            }

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                egui::ComboBox::from_id_salt("cp_eq_preset")
                                    .selected_text(RichText::new(&config.eq_preset).color(VortexTheme::POT_YELLOW).strong())
                                    .show_ui(ui, |ui| {
                                        // Built-in presets
                                        ui.label(RichText::new("── Built-in Presets ──").size(10.0).color(Color32::from_rgb(120, 125, 140)));
                                        for preset in POT_EQ_PRESETS {
                                            if ui.selectable_label(config.eq_preset == preset.name, preset.name).clicked() {
                                                config.eq_preset = preset.name.to_string();
                                                config.eq_bands = preset.bands.to_vec();
                                                player.set_equalizer(config.eq_enabled, &config.eq_bands);
                                                let _ = config.save();
                                            }
                                        }

                                        // Custom presets
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

                        // ── Row 2: Action Toolbar (Save to disk, Save As Custom Preset, Overwrite) ──
                        ui.horizontal(|ui| {
                            let save_txt = if is_recently_saved { "✓ Saved to disk!" } else { "💾 Save Config" };
                            let save_col = if is_recently_saved { Color32::from_rgb(100, 225, 140) } else { Color32::WHITE };
                            if ui.button(RichText::new(save_txt).color(save_col)).on_hover_text("Save current equalizer configuration to config.json").clicked() {
                                let _ = config.save();
                                self.save_feedback_time = current_time;
                            }

                            if ui.button("➕ Save As Custom Preset...").on_hover_text("Save current band values as a new named custom preset").clicked() {
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

                        // ── Inline Save Preset Dialog ──
                        if self.is_naming_preset {
                            egui::Frame::new()
                                .fill(Color32::from_rgb(10, 10, 14))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(40, 45, 60)))
                                .corner_radius(CornerRadius::same(4))
                                .inner_margin(egui::Margin::symmetric(10, 6))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Preset Name:").size(11.0).color(VortexTheme::POT_YELLOW).strong());
                                        let text_resp = ui.add(
                                            egui::TextEdit::singleline(&mut self.new_preset_name)
                                                .desired_width(180.0)
                                                .hint_text("e.g. Bass Boost 2")
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

                        // ── 18-Band Graphic Equalizer Fader Plate ──
                        let freqs = ["20", "31", "50", "80", "125", "200", "315", "500", "800", "1.2k", "2k", "3.1k", "5k", "8k", "12k", "16k", "18k", "20k"];
                        if config.eq_bands.len() != 18 {
                            config.eq_bands = vec![0.0; 18];
                        }

                        let fader_card = egui::Frame::new()
                            .fill(Color32::from_rgb(0, 0, 0))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(32, 35, 46)))
                            .corner_radius(CornerRadius::same(4))
                            .inner_margin(egui::Margin::symmetric(10, 8));

                        fader_card.show(ui, |ui| {
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
                                            player.set_equalizer(config.eq_enabled, &config.eq_bands);
                                            let _ = config.save();
                                        }

                                        ui.label(RichText::new(freqs[i]).size(8.0).monospace().color(Color32::from_rgb(140, 145, 160)));
                                    });
                                }
                            });
                        });

                        ui.separator();

                        // ── Audio Delay & Dynamic Normalizer Card ──
                        egui::Frame::new()
                            .fill(Color32::from_rgb(0, 0, 0))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(32, 35, 46)))
                            .corner_radius(CornerRadius::same(4))
                            .inner_margin(egui::Margin::symmetric(10, 8))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Audio Delay:").strong().color(Color32::from_rgb(220, 225, 235)));
                                    let mut delay = config.audio_delay;
                                    let slider = ui.add(egui::Slider::new(&mut delay, -5.0..=5.0).suffix("s"));
                                    if slider.changed() {
                                        config.audio_delay = delay;
                                        player.set_audio_delay(delay);
                                        let _ = config.save();
                                    }

                                    if ui.button("-50ms").clicked() {
                                        config.audio_delay -= 0.05;
                                        player.adjust_audio_delay(-0.05);
                                        let _ = config.save();
                                    }
                                    if ui.button("+50ms").clicked() {
                                        config.audio_delay += 0.05;
                                        player.adjust_audio_delay(0.05);
                                        let _ = config.save();
                                    }
                                    if ui.button("↺ Reset").clicked() {
                                        config.audio_delay = 0.0;
                                        player.set_audio_delay(0.0);
                                        let _ = config.save();
                                    }
                                });

                                ui.add_space(4.0);

                                ui.horizontal(|ui| {
                                    let mut norm = config.audio_normalize;
                                    let is_norm = norm;
                                    if ui.checkbox(&mut norm, RichText::new("Dynamic Volume Normalizer (Night Mode)").color(if is_norm { VortexTheme::POT_YELLOW } else { Color32::WHITE })).changed() {
                                        config.audio_normalize = norm;
                                        player.set_audio_normalize(norm);
                                        let _ = config.save();
                                    }
                                });
                            });
                    }

                    // =========================================================================
                    // 3. PLAYBACK TAB: SPEED MULTIPLIER, PRESETS, A-B LOOP, REPEAT & SHUFFLE
                    // =========================================================================
                    ControlPanelTab::Playback => {
                        // Playback Sequence & Repeat/Shuffle Controls
                        ui.label(RichText::new("Playback Sequence & Order").strong().color(VortexTheme::TEXT_PRIMARY));
                        ui.horizontal(|ui| {
                            ui.label("Repeat:");
                            let r_mode = playlist.repeat_mode;
                            if ui.selectable_label(r_mode == crate::playlist::RepeatMode::Off, "Off").clicked() {
                                playlist.repeat_mode = crate::playlist::RepeatMode::Off;
                                config.playlist_repeat_mode = "Off".to_string();
                                let _ = config.save();
                            }
                            if ui.selectable_label(r_mode == crate::playlist::RepeatMode::RepeatAll, "🔁 All").clicked() {
                                playlist.repeat_mode = crate::playlist::RepeatMode::RepeatAll;
                                config.playlist_repeat_mode = "RepeatAll".to_string();
                                let _ = config.save();
                            }
                            if ui.selectable_label(r_mode == crate::playlist::RepeatMode::RepeatTrack, "🔂 Track").clicked() {
                                playlist.repeat_mode = crate::playlist::RepeatMode::RepeatTrack;
                                config.playlist_repeat_mode = "RepeatTrack".to_string();
                                let _ = config.save();
                            }

                            ui.add_space(12.0);

                            let mut shuf = playlist.is_shuffle();
                            let is_shuf = shuf;
                            if ui.checkbox(&mut shuf, RichText::new("🔀 Shuffle").color(if is_shuf { VortexTheme::POT_YELLOW } else { Color32::WHITE })).changed() {
                                playlist.set_shuffle(shuf);
                                config.playlist_shuffle = shuf;
                                let _ = config.save();
                            }
                        });

                        ui.separator();

                        // Speed multiplier with fine slider
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Speed:").strong().color(VortexTheme::TEXT_PRIMARY));
                            let mut spd = stats.speed;
                            let slider = ui.add(egui::Slider::new(&mut spd, 0.2..=4.0).suffix("x"));
                            if slider.changed() {
                                player.set_speed(spd);
                            }
                            if ui.button(RichText::new("1.0x (Normal)").color(VortexTheme::POT_YELLOW)).clicked() {
                                player.set_speed(1.0);
                            }
                        });

                        // Speed presets
                        ui.horizontal(|ui| {
                            let speed_presets = [0.5, 0.8, 1.0, 1.2, 1.5, 2.0, 3.0];
                            for &preset in &speed_presets {
                                let is_active = (stats.speed - preset).abs() < 0.05;
                                let btn_text = format!("{:.1}x", preset);
                                let btn = egui::Button::new(RichText::new(btn_text).color(if is_active { VortexTheme::POT_YELLOW } else { Color32::WHITE }));
                                if ui.add(btn).clicked() {
                                    player.set_speed(preset);
                                }
                            }
                        });

                        ui.separator();

                        // Quick Jump steps
                        ui.label(RichText::new("Quick Seek & Jump Interval").strong().color(VortexTheme::TEXT_PRIMARY));
                        ui.horizontal(|ui| {
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

                        ui.separator();

                        // A-B Repeat Loop Segment Controls
                        ui.label(RichText::new("A-B Repeat Loop Segment").strong().color(VortexTheme::TEXT_PRIMARY));
                        let a_text = bookmark_mgr.ab_loop.point_a.map(|t| crate::bookmark::format_time(t)).unwrap_or_else(|| "--:--".to_string());
                        let b_text = bookmark_mgr.ab_loop.point_b.map(|t| crate::bookmark::format_time(t)).unwrap_or_else(|| "--:--".to_string());
                        ui.label(format!("Point A: {} | Point B: {}", a_text, b_text));

                        ui.horizontal(|ui| {
                            if ui.button(RichText::new("[ Set Point A").color(if bookmark_mgr.ab_loop.point_a.is_some() { VortexTheme::POT_YELLOW } else { Color32::WHITE })).clicked() {
                                bookmark_mgr.set_loop_a(stats.time_pos);
                            }
                            if ui.button(RichText::new("] Set Point B").color(if bookmark_mgr.ab_loop.point_b.is_some() { VortexTheme::POT_YELLOW } else { Color32::WHITE })).clicked() {
                                let _ = bookmark_mgr.set_loop_b(stats.time_pos);
                            }
                            if ui.button("↺ Clear Loop (\\)").clicked() {
                                bookmark_mgr.clear_ab_loop();
                            }
                        });
                    }

                    // =========================================================================
                    // 4. SUBTITLES TAB: SYNC DELAY, POSITION OFFSET, FONT SIZE, TRACK SELECT
                    // =========================================================================
                    ControlPanelTab::Subtitles => {
                        ui.label(RichText::new("Subtitle Sync & Positioning").strong().color(VortexTheme::TEXT_PRIMARY));

                        let mut sub_vis = stats.subtitles_visible;
                        if ui.checkbox(&mut sub_vis, "Show Subtitles (Alt+H)").changed() {
                            player.toggle_subtitles();
                        }

                        // Subtitle Delay Sync with fine and coarse buttons
                        ui.horizontal(|ui| {
                            ui.label("Sync Delay:");
                            let mut delay = stats.subtitle_delay;
                            let slider = ui.add(egui::Slider::new(&mut delay, -10.0..=10.0).suffix("s"));
                            if slider.changed() {
                                player.set_subtitle_delay(delay);
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.button("-0.5s (<)").clicked() {
                                player.adjust_subtitle_delay(-0.5);
                            }
                            if ui.button("-0.05s").clicked() {
                                player.adjust_subtitle_delay(-0.05);
                            }
                            if ui.button("+0.05s").clicked() {
                                player.adjust_subtitle_delay(0.05);
                            }
                            if ui.button("+0.5s (>)").clicked() {
                                player.adjust_subtitle_delay(0.5);
                            }
                            if ui.button("↺ Reset (0.00s)").clicked() {
                                player.set_subtitle_delay(0.0);
                            }
                        });

                        ui.separator();

                        // Font Size & Position
                        ui.horizontal(|ui| {
                            ui.label("Font Size:");
                            let mut sz = config.subtitle_font_size;
                            let slider = ui.add(egui::Slider::new(&mut sz, 12.0..=72.0).suffix("pt"));
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

                        ui.horizontal(|ui| {
                            ui.label("Vertical Position:");
                            let mut pos = config.subtitle_vertical_pos;
                            let slider = ui.add(egui::Slider::new(&mut pos, 0.0..=100.0).suffix("%"));
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
                                config.subtitle_vertical_pos = (config.subtitle_vertical_pos + 5.0).min(100.0);
                                player.set_subtitle_pos(config.subtitle_vertical_pos);
                                let _ = config.save();
                            }
                            if ui.small_button("↺ Default").clicked() {
                                config.subtitle_vertical_pos = 95.0;
                                player.set_subtitle_pos(95.0);
                                let _ = config.save();
                            }
                        });

                        ui.separator();

                        // Outline & Quick Toggles
                        ui.horizontal(|ui| {
                            ui.label("Outline Width:");
                            let mut ow = config.subtitle_outline_width;
                            let slider = ui.add(egui::Slider::new(&mut ow, 0.0..=8.0).suffix("px"));
                            if slider.changed() {
                                config.subtitle_outline_width = ow;
                                player.set_subtitle_border_size(ow);
                                let _ = config.save();
                            }
                        });

                        ui.horizontal(|ui| {
                            if ui.checkbox(&mut config.subtitle_bold, "Bold Font").changed() {
                                player.set_subtitle_bold(config.subtitle_bold);
                                let _ = config.save();
                            }
                            if ui.checkbox(&mut config.subtitle_background_box, "Background Box").changed() {
                                player.set_subtitle_background_box(config.subtitle_background_box, &config.subtitle_background_color);
                                let _ = config.save();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("ASS Override:");
                            let cur_ov = config.subtitle_ass_override.clone();
                            egui::ComboBox::from_id_salt("cp_ass_override")
                                .selected_text(match cur_ov.as_str() {
                                    "no" => "Strict (Keep ASS)",
                                    "scale" => "Smart Scale",
                                    "yes" => "Allow Font Overrides",
                                    "force" => "Force All Overrides",
                                    "strip" => "Strip ASS Tags",
                                    _ => "Smart Scale",
                                })
                                .show_ui(ui, |ui| {
                                    if ui.selectable_label(config.subtitle_ass_override == "scale", "Smart Scale (Default)").clicked() {
                                        config.subtitle_ass_override = "scale".to_string();
                                        player.set_subtitle_ass_override(&config.subtitle_ass_override);
                                        let _ = config.save();
                                    }
                                    if ui.selectable_label(config.subtitle_ass_override == "force", "Force All Overrides").clicked() {
                                        config.subtitle_ass_override = "force".to_string();
                                        player.set_subtitle_ass_override(&config.subtitle_ass_override);
                                        let _ = config.save();
                                    }
                                    if ui.selectable_label(config.subtitle_ass_override == "no", "Strict (Keep ASS)").clicked() {
                                        config.subtitle_ass_override = "no".to_string();
                                        player.set_subtitle_ass_override(&config.subtitle_ass_override);
                                        let _ = config.save();
                                    }
                                    if ui.selectable_label(config.subtitle_ass_override == "strip", "Strip ASS Tags").clicked() {
                                        config.subtitle_ass_override = "strip".to_string();
                                        player.set_subtitle_ass_override(&config.subtitle_ass_override);
                                        let _ = config.save();
                                    }
                                });
                        });
                    }
                }
            });
        resp.map(|r| r.response.rect)
    }
}
