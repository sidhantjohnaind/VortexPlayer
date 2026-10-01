#![allow(dead_code)]

use crate::bookmark::format_time;
use crate::engine::MediaStats;
use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct NotificationNavBar {
    pub is_enabled: bool,
    pub is_pinned: bool,
    pub banner_until: Instant,
    pub last_title: String,
    pub slide_progress: f32, // 0.0 = hidden, 1.0 = fully down
    pub last_frame_time: Instant,
}

impl Default for NotificationNavBar {
    fn default() -> Self {
        Self {
            is_enabled: true,
            is_pinned: false,
            banner_until: Instant::now(),
            last_title: String::new(),
            slide_progress: 0.0,
            last_frame_time: Instant::now(),
        }
    }
}

#[derive(Default)]
pub struct NotificationNavActions {
    pub toggle_play: bool,
    pub prev_track: bool,
    pub next_track: bool,
    pub seek_target: Option<f64>,
    pub toggle_pin: bool,
    pub close: bool,
}

impl NotificationNavBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn notify_track(&mut self, title: &str) {
        self.last_title = title.to_string();
        self.banner_until = Instant::now() + Duration::from_millis(4500);
    }

    pub fn is_active(&self, mouse_pos: Option<Pos2>, window_rect: Rect) -> bool {
        if !self.is_enabled {
            return false;
        }
        if self.is_pinned {
            return true;
        }
        if Instant::now() < self.banner_until {
            return true;
        }
        if let Some(pos) = mouse_pos {
            // Hovering near top center
            if pos.y <= window_rect.min.y + 70.0 
                && pos.x >= window_rect.center().x - 240.0 
                && pos.x <= window_rect.center().x + 240.0 
            {
                return true;
            }
        }
        false
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        stats: &MediaStats,
        is_playing: bool,
    ) -> NotificationNavActions {
        let mut actions = NotificationNavActions::default();
        if !self.is_enabled || stats.file_path.is_empty() {
            return actions;
        }

        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        let win_size = ctx.input(|i| i.viewport().inner_rect.or(i.raw.screen_rect)).map(|r| r.size()).unwrap_or(Vec2::new(1280.0, 720.0));
        let screen_rect = Rect::from_min_size(Pos2::ZERO, win_size);
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos());
        let target_active = self.is_active(mouse_pos, screen_rect);

        // Smooth animate slide
        let target_progress = if target_active { 1.0 } else { 0.0 };
        let speed = 8.0;
        self.slide_progress += (target_progress - self.slide_progress) * (speed * dt).min(1.0);

        if self.slide_progress < 0.01 {
            return actions;
        }

        let bar_width = 460.0_f32.min(screen_rect.width() - 40.0);
        let bar_height = 48.0;
        let top_y = screen_rect.min.y + 12.0 - (1.0 - self.slide_progress) * (bar_height + 20.0);
        let center_x = screen_rect.center().x;
        let nav_rect = Rect::from_center_size(
            Pos2::new(center_x, top_y + bar_height * 0.5),
            Vec2::new(bar_width, bar_height),
        );

        let alpha = (self.slide_progress * 255.0).clamp(0.0, 255.0) as u8;
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("notification_nav_bar")));

        // Drop shadow
        let shadow_rect = nav_rect.translate(Vec2::new(0.0, 4.0));
        painter.rect_filled(shadow_rect, CornerRadius::same(12), Color32::from_rgba_premultiplied(0, 0, 0, (alpha as f32 * 0.45) as u8));

        // Background pill
        painter.rect_filled(
            nav_rect,
            CornerRadius::same(12),
            Color32::from_rgba_premultiplied(18, 14, 26, (alpha as f32 * 0.96) as u8),
        );

        // Glow Border
        painter.rect_stroke(
            nav_rect,
            CornerRadius::same(12),
            Stroke::new(1.0, Color32::from_rgba_premultiplied(90, 55, 140, (alpha as f32 * 0.8) as u8)),
            StrokeKind::Inside,
        );

        // Interactive controls Area
        egui::Area::new(egui::Id::new("notification_media_nav_interactive"))
            .fixed_pos(nav_rect.min)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                ui.set_width(bar_width);
                ui.set_height(bar_height);

                ui.horizontal(|ui| {
                    ui.add_space(10.0);

                    // Mini Spectrum Icon
                    let (spec_rect, _) = ui.allocate_exact_size(Vec2::new(18.0, 22.0), Sense::hover());
                    let p = ui.painter();
                    let t = ctx.input(|i| i.time) as f32;
                    for i in 0..4 {
                        let bar_x = spec_rect.min.x + i as f32 * 4.5;
                        let h = if is_playing {
                            ((t * 6.0 + i as f32 * 1.5).sin() * 0.5 + 0.5) * 16.0 + 4.0
                        } else {
                            3.0
                        };
                        p.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(bar_x, spec_rect.max.y - h),
                                Pos2::new(bar_x + 3.0, spec_rect.max.y),
                            ),
                            CornerRadius::same(1),
                            Color32::from_rgb(180, 140, 255),
                        );
                    }

                    ui.add_space(4.0);

                    // Title & Time info column
                    ui.vertical(|ui| {
                        ui.add_space(4.0);
                        let file_stem = std::path::Path::new(&stats.file_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("VertexPlayer");
                        let title_text = if !stats.title.is_empty() {
                            &stats.title
                        } else {
                            file_stem
                        };
                        
                        let max_title_w = bar_width - 230.0;
                        ui.allocate_ui(Vec2::new(max_title_w, 18.0), |ui| {
                            let label = ui.label(
                                egui::RichText::new(title_text)
                                    .size(12.0)
                                    .color(Color32::from_rgb(230, 230, 245))
                                    .strong(),
                            );
                            if label.hovered() {
                                label.on_hover_text(title_text);
                            }
                        });

                        // Mini progress bar + time
                        let time_str = format!("{} / {}", format_time(stats.time_pos), format_time(stats.duration));
                        ui.label(
                            egui::RichText::new(time_str)
                                .size(10.0)
                                .color(Color32::from_rgb(140, 140, 170))
                                .monospace(),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);

                        // Pin button
                        let pin_color = if self.is_pinned {
                            Color32::from_rgb(220, 160, 255)
                        } else {
                            Color32::from_rgb(120, 120, 140)
                        };
                        if ui.add(egui::Button::new(egui::RichText::new(if self.is_pinned { "📌" } else { "📍" }).size(11.0).color(pin_color)).frame(false)).clicked() {
                            self.is_pinned = !self.is_pinned;
                            actions.toggle_pin = true;
                        }

                        ui.add_space(4.0);

                        // Transport Buttons: Next Track
                        let next_btn = ui.add(
                            egui::Button::new(egui::RichText::new("⏭").size(14.0).color(Color32::from_rgb(210, 210, 230)))
                                .frame(false)
                        );
                        if next_btn.clicked() {
                            actions.next_track = true;
                        }
                        if next_btn.hovered() {
                            next_btn.on_hover_text("Next Track");
                        }

                        // Transport Buttons: Play / Pause (Accent Button)
                        let play_icon = if is_playing { "⏸" } else { "▶" };
                        let play_btn = ui.add(
                            egui::Button::new(
                                egui::RichText::new(play_icon)
                                    .size(15.0)
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .fill(Color32::from_rgb(90, 50, 170))
                            .corner_radius(CornerRadius::same(14))
                            .min_size(Vec2::new(28.0, 28.0)),
                        );
                        if play_btn.clicked() {
                            actions.toggle_play = true;
                        }
                        if play_btn.hovered() {
                            play_btn.on_hover_text(if is_playing { "Pause (Space)" } else { "Play (Space)" });
                        }

                        // Transport Buttons: Prev Track
                        let prev_btn = ui.add(
                            egui::Button::new(egui::RichText::new("⏮").size(14.0).color(Color32::from_rgb(210, 210, 230)))
                                .frame(false)
                        );
                        if prev_btn.clicked() {
                            actions.prev_track = true;
                        }
                        if prev_btn.hovered() {
                            prev_btn.on_hover_text("Previous Track");
                        }
                    });
                });
            });

        actions
    }
}
