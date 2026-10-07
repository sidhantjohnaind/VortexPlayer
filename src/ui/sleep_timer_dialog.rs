use super::theme::VortexTheme;
use crate::engine::sleep_timer::{PlaybackCompletionAction, SleepTimerEngine};
use eframe::egui::{
    self, Align2, Color32, CornerRadius, Margin, ProgressBar, Rect, RichText, Stroke,
    Vec2, Window,
};

pub struct SleepTimerDialog {
    pub selected_minutes: u64,
    pub selected_action: PlaybackCompletionAction,
    pub trigger_on_playlist_end: bool,
}

impl Default for SleepTimerDialog {
    fn default() -> Self {
        Self {
            selected_minutes: 30,
            selected_action: PlaybackCompletionAction::PausePlayback,
            trigger_on_playlist_end: false,
        }
    }
}

impl SleepTimerDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        engine: &mut SleepTimerEngine,
    ) -> Option<Rect> {
        if !*is_open {
            return None;
        }

        let accent = VortexTheme::current_skin().accent_primary;
        let mut do_close = false;

        let resp = Window::new("🌙 Sleep Timer")
            .open(is_open)
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_size(Vec2::new(400.0, 360.0))
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(18, 20, 26))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 52, 68)))
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::same(16)),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Title and Description
                    ui.label(
                        RichText::new("Automatic Playback & PC Sleep Timer")
                            .font(egui::FontId::proportional(15.0))
                            .strong()
                            .color(Color32::WHITE),
                    );
                    ui.label(
                        RichText::new("Automatically pause playback or shut down your PC after a set period.")
                            .font(egui::FontId::proportional(11.5))
                            .color(Color32::from_rgb(150, 160, 180)),
                    );

                    ui.add_space(10.0);

                    // If active, show active banner
                    if engine.active {
                        let remaining_str = if engine.trigger_on_playlist_end {
                            "Triggering at the end of current media/playlist".to_string()
                        } else if let Some(rem) = engine.remaining() {
                            let total_secs = rem.as_secs();
                            let mins = total_secs / 60;
                            let secs = total_secs % 60;
                            format!("⏳ {:02}:{:02} remaining", mins, secs)
                        } else {
                            "Timer expired".to_string()
                        };

                        ui.group(|ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("ACTIVE TIMER:")
                                        .strong()
                                        .color(accent),
                                );
                                ui.label(
                                    RichText::new(&remaining_str)
                                        .color(Color32::WHITE)
                                        .strong(),
                                );
                            });

                            if !engine.trigger_on_playlist_end {
                                ui.add(
                                    ProgressBar::new(engine.progress())
                                        .show_percentage()
                                        .fill(accent),
                                );
                            }

                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!("Action: {} {}", engine.completion_action.icon(), engine.completion_action.label()))
                                        .color(Color32::from_rgb(180, 190, 210))
                                        .font(egui::FontId::proportional(12.0)),
                                );
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button(RichText::new("Cancel Timer").color(Color32::from_rgb(255, 100, 100))).clicked() {
                                        engine.cancel();
                                    }
                                });
                            });
                        });

                        ui.add_space(12.0);
                    }

                    // Timer Duration Presets
                    ui.label(
                        RichText::new("Choose Duration:")
                            .strong()
                            .color(Color32::from_rgb(200, 210, 230))
                            .font(egui::FontId::proportional(12.5)),
                    );

                    ui.horizontal_wrapped(|ui| {
                        let presets = [15, 30, 45, 60, 90, 120];
                        for p in presets {
                            let selected = !self.trigger_on_playlist_end && self.selected_minutes == p;
                            let btn_text = RichText::new(format!("{} min", p))
                                .color(if selected { Color32::WHITE } else { Color32::from_rgb(180, 190, 210) });
                            let btn = egui::Button::new(btn_text)
                                .fill(if selected { accent } else { Color32::from_rgb(28, 32, 42) });
                            if ui.add(btn).clicked() {
                                self.selected_minutes = p;
                                self.trigger_on_playlist_end = false;
                            }
                        }
                    });

                    ui.add_space(4.0);

                    // Custom slider
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Custom:").color(Color32::from_rgb(160, 170, 190)));
                        ui.add(
                            egui::Slider::new(&mut self.selected_minutes, 1..=360)
                                .suffix(" min")
                                .text(""),
                        );
                    });

                    ui.checkbox(&mut self.trigger_on_playlist_end, "At end of current video / playlist");

                    ui.add_space(10.0);

                    // Action to execute
                    ui.label(
                        RichText::new("Action when timer finishes:")
                            .strong()
                            .color(Color32::from_rgb(200, 210, 230))
                            .font(egui::FontId::proportional(12.5)),
                    );

                    let actions = [
                        PlaybackCompletionAction::PausePlayback,
                        PlaybackCompletionAction::ExitPlayer,
                        PlaybackCompletionAction::SleepPc,
                        PlaybackCompletionAction::HibernatePc,
                        PlaybackCompletionAction::ShutdownPc,
                    ];

                    let cur_action = self.selected_action;
                    for act in actions {
                        ui.radio_value(
                            &mut self.selected_action,
                            act,
                            RichText::new(format!("{} {}", act.icon(), act.label()))
                                .color(if cur_action == act { Color32::WHITE } else { Color32::from_rgb(170, 180, 200) }),
                        );
                    }

                    ui.add_space(14.0);

                    // Bottom Buttons
                    ui.horizontal(|ui| {
                        let start_btn = egui::Button::new(
                            RichText::new(if engine.active { "Restart Timer" } else { "Start Timer" })
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(accent)
                        .min_size(Vec2::new(110.0, 30.0));

                        if ui.add(start_btn).clicked() {
                            if self.trigger_on_playlist_end {
                                engine.start_at_playlist_end(self.selected_action);
                            } else {
                                engine.start(self.selected_minutes, self.selected_action);
                            }
                            do_close = true;
                        }

                        if engine.active {
                            if ui.button(RichText::new("Stop Timer").color(Color32::from_rgb(255, 120, 120))).clicked() {
                                engine.cancel();
                            }
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Close").clicked() {
                                do_close = true;
                            }
                        });
                    });
                });
            });

        if do_close {
            *is_open = false;
        }

        resp.map(|r| r.response.rect)
    }
}
