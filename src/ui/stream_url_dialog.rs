use super::theme::VortexTheme;
use crate::engine::Player;
use eframe::egui::{self, Align, Color32, Layout, Rect, RichText, Vec2};

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

        let viewport_id = egui::ViewportId::from_hash_of("vortex_stream_url_window");
        let builder = egui::ViewportBuilder::default()
            .with_title("Open Network Stream / URL (Ctrl+U)")
            .with_inner_size([460.0, 240.0])
            .with_min_inner_size([380.0, 180.0])
            .with_resizable(true)
            .with_decorations(true);

        ctx.show_viewport_immediate(viewport_id, builder, |ctx, _class| {
            if ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                should_close = true;
            }
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::new()
                        .fill(Color32::from_rgb(14, 16, 22))
                        .inner_margin(egui::Margin::same(12)),
                )
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

        });

        if should_close {
            *is_open = false;
        }

        None
    }
}
