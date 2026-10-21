use super::theme::VortexTheme;
use crate::engine::Player;
use eframe::egui::{self, Align, Layout, Rect, RichText, Vec2};

pub struct StreamUrlDialog {
    pub url: String,
    pub user_agent: String,
    pub referer: String,
    pub ytdl_format: String,
}

impl Default for StreamUrlDialog {
    fn default() -> Self {
        Self {
            url: String::new(),
            user_agent: String::new(),
            referer: String::new(),
            ytdl_format: "bestvideo+bestaudio/best".to_string(),
        }
    }
}

impl StreamUrlDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        player: &Player,
    ) -> Option<Rect> {
        if !*is_open {
            return None;
        }

        let mut should_close = false;

        let resp = egui::Window::new("Open Network Stream / URL (Ctrl+U)")
            .open(is_open)
            .resizable(false)
            .default_width(450.0)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 8.0);

                ui.label(RichText::new("Enter direct URL, HLS, DASH, RTSP, or Online Stream:").color(VortexTheme::current_skin().text_secondary));

                ui.add(
                    egui::TextEdit::singleline(&mut self.url)
                        .desired_width(430.0)
                        .hint_text("https://... or rtsp://... or m3u8 stream"),
                );

                ui.collapsing("Advanced Network & Stream Settings", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Stream Quality / Format:");
                        ui.add(egui::TextEdit::singleline(&mut self.ytdl_format).desired_width(260.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Custom User-Agent:");
                        ui.add(egui::TextEdit::singleline(&mut self.user_agent).desired_width(260.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label("HTTP Referer:");
                        ui.add(egui::TextEdit::singleline(&mut self.referer).desired_width(260.0));
                    });
                });

                ui.separator();

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }

                    if ui.button(RichText::new("▶ Play Stream").strong().color(VortexTheme::current_skin().accent_primary)).clicked() {
                        let trimmed = self.url.trim();
                        if !trimmed.is_empty() {
                            player.load_file(trimmed);
                            should_close = true;
                        }
                    }
                });
            });

        if should_close {
            *is_open = false;
        }

        resp.map(|r| r.response.rect)
    }
}
