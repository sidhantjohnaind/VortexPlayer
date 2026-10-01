use super::theme::VortexTheme;
use crate::bookmark::format_time;
use crate::engine::{MediaStats, Player};
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Margin, Rect, RichText, Stroke,
    Vec2, Window,
};

pub struct JumpTimeDialog {
    pub time_input: String,
    pub percent_input: f64,
    pub error_msg: Option<String>,
    pub initialized: bool,
}

impl Default for JumpTimeDialog {
    fn default() -> Self {
        Self {
            time_input: String::new(),
            percent_input: 0.0,
            error_msg: None,
            initialized: false,
        }
    }
}

impl JumpTimeDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        stats: &MediaStats,
        player: &Player,
    ) -> Option<Rect> {
        if !*is_open {
            self.initialized = false;
            return None;
        }

        if !self.initialized {
            self.time_input = format_time(stats.time_pos);
            self.percent_input = if stats.duration > 0.0 {
                (stats.time_pos / stats.duration * 100.0).clamp(0.0, 100.0)
            } else {
                0.0
            };
            self.error_msg = None;
            self.initialized = true;
        }

        let mut do_close = false;
        let mut target_time: Option<f64> = None;

        let resp = Window::new("Jump to Specific Time (G)")
            .open(is_open)
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_size(Vec2::new(380.0, 310.0))
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(18, 20, 26))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(52, 56, 72)))
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::same(14)),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Status
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Current Position:")
                                .size(11.0)
                                .color(Color32::from_rgb(160, 165, 180)),
                        );
                        ui.label(
                            RichText::new(format!(
                                "{} / {}",
                                format_time(stats.time_pos),
                                format_time(stats.duration)
                            ))
                            .size(11.5)
                            .monospace()
                            .strong()
                            .color(VortexTheme::VORTEX_CYAN),
                        );
                    });

                    ui.add_space(8.0);

                    // 1. Precise Time String Input
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(
                            RichText::new("Enter Target Time (HH:MM:SS or MM:SS or seconds):")
                                .size(11.0)
                                .color(Color32::from_rgb(200, 205, 220)),
                        );
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            let text_resp = ui.add_sized(
                                [200.0, 26.0],
                                egui::TextEdit::singleline(&mut self.time_input)
                                    .font(FontId::monospace(13.0))
                                    .hint_text("00:00:00"),
                            );

                            if text_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                if let Some(t) = Self::parse_time_str(&self.time_input) {
                                    target_time = Some(t);
                                    do_close = true;
                                } else {
                                    self.error_msg = Some("Invalid time format (e.g. 01:23:45 or 12:34)".to_string());
                                }
                            }

                            if ui.button(RichText::new("Jump").strong()).clicked() {
                                if let Some(t) = Self::parse_time_str(&self.time_input) {
                                    target_time = Some(t);
                                    do_close = true;
                                } else {
                                    self.error_msg = Some("Invalid time format (e.g. 01:23:45 or 12:34)".to_string());
                                }
                            }
                        });

                        if let Some(ref err) = self.error_msg {
                            ui.label(RichText::new(err).size(10.5).color(Color32::from_rgb(240, 80, 80)));
                        }
                    });

                    ui.add_space(6.0);

                    // 2. Percentage Jump Slider
                    ui.group(|ui| {
                        ui.set_width(ui.available_width());
                        ui.label(
                            RichText::new("Jump by Percentage (%):")
                                .size(11.0)
                                .color(Color32::from_rgb(200, 205, 220)),
                        );
                        ui.add_space(2.0);

                        ui.horizontal(|ui| {
                            let slider = egui::Slider::new(&mut self.percent_input, 0.0..=100.0)
                                .suffix("%")
                                .show_value(true);
                            if ui.add_sized([230.0, 20.0], slider).changed() {
                                if stats.duration > 0.0 {
                                    let calc_sec = (self.percent_input / 100.0) * stats.duration;
                                    self.time_input = format_time(calc_sec);
                                }
                            }

                            if ui.button(RichText::new("Apply %")).clicked() {
                                if stats.duration > 0.0 {
                                    let calc_sec = (self.percent_input / 100.0) * stats.duration;
                                    target_time = Some(calc_sec);
                                    do_close = true;
                                }
                            }
                        });
                    });

                    ui.add_space(6.0);

                    // 3. Quick Relative Jump Buttons
                    ui.label(
                        RichText::new("Quick Relative Jump:")
                            .size(10.5)
                            .color(Color32::from_rgb(160, 165, 180)),
                    );
                    ui.add_space(2.0);

                    ui.horizontal(|ui| {
                        if ui.button("-5m").clicked() {
                            target_time = Some((stats.time_pos - 300.0).max(0.0));
                            do_close = true;
                        }
                        if ui.button("-1m").clicked() {
                            target_time = Some((stats.time_pos - 60.0).max(0.0));
                            do_close = true;
                        }
                        if ui.button("-10s").clicked() {
                            target_time = Some((stats.time_pos - 10.0).max(0.0));
                            do_close = true;
                        }
                        if ui.button("+10s").clicked() {
                            target_time = Some((stats.time_pos + 10.0).min(stats.duration));
                            do_close = true;
                        }
                        if ui.button("+1m").clicked() {
                            target_time = Some((stats.time_pos + 60.0).min(stats.duration));
                            do_close = true;
                        }
                        if ui.button("+5m").clicked() {
                            target_time = Some((stats.time_pos + 300.0).min(stats.duration));
                            do_close = true;
                        }
                    });

                    // 4. Chapter Jump Row (if available)
                    if !stats.chapters.is_empty() {
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Chapter:").size(10.5).color(Color32::from_rgb(160, 165, 180)));
                            egui::ComboBox::from_id_salt("jump_dialog_chapters")
                                .selected_text(
                                    stats
                                        .current_chapter
                                        .and_then(|idx| stats.chapters.get(idx as usize))
                                        .map(|c| c.title.as_str())
                                        .unwrap_or("Select chapter..."),
                                )
                                .show_ui(ui, |ui| {
                                    for ch in &stats.chapters {
                                        if ui.selectable_label(false, &ch.title).clicked() {
                                            target_time = Some(ch.time_pos);
                                            do_close = true;
                                        }
                                    }
                                });
                        });
                    }

                    ui.add_space(10.0);

                    // Footer Dialog Actions
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("Close (Esc)").color(Color32::from_rgb(170, 175, 190))).clicked()
                            || ui.input(|i| i.key_pressed(egui::Key::Escape))
                        {
                            do_close = true;
                        }
                    });
                });
            });

        if let Some(t) = target_time {
            player.seek_absolute(t);
        }

        if do_close {
            *is_open = false;
            self.initialized = false;
        }

        resp.map(|r| r.response.rect)
    }

    fn parse_time_str(input: &str) -> Option<f64> {
        let clean = input.trim();
        if clean.is_empty() {
            return None;
        }

        if let Ok(sec) = clean.parse::<f64>() {
            return Some(sec);
        }

        let parts: Vec<&str> = clean.split(':').collect();
        match parts.len() {
            2 => {
                let m = parts[0].trim().parse::<f64>().ok()?;
                let s = parts[1].trim().parse::<f64>().ok()?;
                Some(m * 60.0 + s)
            }
            3 => {
                let h = parts[0].trim().parse::<f64>().ok()?;
                let m = parts[1].trim().parse::<f64>().ok()?;
                let s = parts[2].trim().parse::<f64>().ok()?;
                Some(h * 3600.0 + m * 60.0 + s)
            }
            _ => None,
        }
    }
}
