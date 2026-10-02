#![allow(dead_code)]

use super::theme::VortexTheme;
use crate::bookmark::{format_time_hms, parse_time_hms, BookmarkManager};
use crate::config::{AppConfig, ConfigSkipInterval};
use crate::engine::chapters::ChapterItem;
use crate::engine::Player;
use eframe::egui::{self, Align2, Color32, CornerRadius, RichText, Stroke, Vec2};
use std::sync::Arc;

/// A single chapter-title keyword tag with its own enabled state.
#[derive(Debug, Clone)]
pub struct SkipTag {
    pub keyword: String,
    pub enabled: bool,
}

/// Default built-in keywords that PotPlayer ships.
const DEFAULT_KEYWORDS: &[&str] = &[
    "opening", "begin", "ending", "intro", "credits", "op", "ed",
    "prologue", "recap", "preview", "theme", "outro",
];

pub struct AutoSkipDialog {
    pub is_open: bool,
    was_open: bool,

    // Draft dialog state
    pub enable_skip_feature: bool,
    pub intro_at_start: bool,
    pub intro_time_str: String,
    pub ending_at_end: bool,
    pub ending_time_str: String,
    pub chapters_enabled: bool,
    pub tags: Vec<SkipTag>,
    pub new_tag_input: String,
    pub intervals: Vec<ConfigSkipInterval>,
    pub selected_index: Option<usize>,

    // Add / Edit Modal sub-state
    pub is_modal_open: bool,
    pub modal_is_edit: bool,
    pub modal_start_str: String,
    pub modal_length_str: String,
    pub modal_type_str: String,
}

impl Default for AutoSkipDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            was_open: false,
            enable_skip_feature: false,
            intro_at_start: false,
            intro_time_str: "00:00:00".to_string(),
            ending_at_end: false,
            ending_time_str: "00:00:00".to_string(),
            chapters_enabled: true,
            tags: Vec::new(),
            new_tag_input: String::new(),
            intervals: Vec::new(),
            selected_index: None,

            is_modal_open: false,
            modal_is_edit: false,
            modal_start_str: "00:00:00".to_string(),
            modal_length_str: "00:01:30".to_string(),
            modal_type_str: "Skip".to_string(),
        }
    }
}

/// Parse the semicolon-delimited config string into a Vec<SkipTag>.
fn tags_from_config_string(s: &str) -> Vec<SkipTag> {
    let mut tags: Vec<SkipTag> = Vec::new();
    for token in s.split(';') {
        let t = token.trim();
        if t.is_empty() {
            continue;
        }
        // Tokens prefixed with '!' are disabled
        let (keyword, enabled) = if let Some(stripped) = t.strip_prefix('!') {
            (stripped.to_lowercase(), false)
        } else {
            (t.to_lowercase(), true)
        };
        if !keyword.is_empty() && !tags.iter().any(|tag| tag.keyword == keyword) {
            tags.push(SkipTag { keyword, enabled });
        }
    }
    // Ensure all defaults are present
    for kw in DEFAULT_KEYWORDS {
        let lower = kw.to_lowercase();
        if !tags.iter().any(|tag| tag.keyword == lower) {
            tags.push(SkipTag {
                keyword: lower,
                enabled: false,
            });
        }
    }
    tags
}

/// Serialize tags back into a semicolon-delimited string for config storage.
/// Disabled tags are prefixed with '!'.
fn tags_to_config_string(tags: &[SkipTag]) -> String {
    tags.iter()
        .map(|tag| {
            if tag.enabled {
                format!("{};", tag.keyword)
            } else {
                format!("!{};", tag.keyword)
            }
        })
        .collect::<String>()
}

impl AutoSkipDialog {
    pub fn new() -> Self {
        Self::default()
    }

    fn init_from_config(&mut self, config: &AppConfig, bookmark_mgr: &BookmarkManager) {
        self.enable_skip_feature = config.skip_intro_enabled;
        self.intro_at_start = config.skip_intro_at_start;
        self.intro_time_str = format_time_hms(config.skip_intro_sec);
        self.ending_at_end = config.skip_ending_at_end;
        self.ending_time_str = format_time_hms(config.skip_outro_sec);
        self.chapters_enabled = config.skip_chapters_enabled;
        self.tags = tags_from_config_string(&config.skip_chapter_titles);
        self.new_tag_input.clear();

        // Combine config skip intervals with bookmark intervals if config is empty
        if !config.skip_intervals.is_empty() {
            self.intervals = config.skip_intervals.clone();
        } else if !bookmark_mgr.skip_intervals.is_empty() {
            self.intervals = bookmark_mgr
                .skip_intervals
                .iter()
                .map(|item| ConfigSkipInterval {
                    start: item.start,
                    length: (item.end - item.start).max(0.0),
                    interval_type: "Skip".to_string(),
                    enabled: item.enabled,
                })
                .collect();
        } else {
            self.intervals.clear();
        }
        self.selected_index = None;
        self.is_modal_open = false;
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        bookmark_mgr: &mut BookmarkManager,
        config: &mut AppConfig,
        chapters: &[ChapterItem],
        _player: Option<&Arc<Player>>,
        current_time: f64,
        duration: f64,
    ) -> Option<egui::Rect> {
        if !self.is_open {
            self.was_open = false;
            return None;
        }

        // Initialize state when opened
        if !self.was_open {
            self.init_from_config(config, bookmark_mgr);
            self.was_open = true;
        }

        let mut open = self.is_open;
        let mut save_and_close = false;
        let mut cancel_and_close = false;

        let window_resp = egui::Window::new("Skip Setup")
            .id(egui::Id::new("potplayer_skip_setup_window"))
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_size(Vec2::new(560.0, 520.0))
            .show(ctx, |ui| {
                ui.set_width(540.0);

                // =============================================================
                // 1. Group Box: "Skip Playback"
                // =============================================================
                ui.group(|ui| {
                    ui.set_width(540.0);
                    ui.add_space(2.0);

                    ui.label(
                        RichText::new("Skip Playback")
                            .strong()
                            .size(13.0)
                            .color(Color32::from_rgb(220, 225, 235)),
                    );
                    ui.label(
                        RichText::new(
                            "Skip feature allows you to set playback skip intervals.",
                        )
                        .size(11.5)
                        .color(Color32::from_rgb(160, 165, 175)),
                    );
                    ui.add_space(4.0);

                    // Checkbox: Enable skip feature
                    ui.checkbox(
                        &mut self.enable_skip_feature,
                        "Enable skip feature",
                    );
                    ui.add_space(3.0);

                    // Indented sub-options
                    ui.horizontal(|ui| {
                        ui.add_space(20.0);
                        ui.vertical(|ui| {
                            // Row 1: Intro (at the start)
                            ui.horizontal(|ui| {
                                ui.checkbox(
                                    &mut self.intro_at_start,
                                    "Intro (at the start)",
                                );
                                ui.add_space(30.0);
                                ui.label("Skip");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.intro_time_str)
                                        .desired_width(75.0)
                                        .font(egui::TextStyle::Monospace),
                                )
                                .on_hover_text("HH:MM:SS (e.g. 00:01:30)");
                            });
                            ui.add_space(2.0);

                            // Row 2: Ending (at the end)
                            ui.horizontal(|ui| {
                                ui.checkbox(
                                    &mut self.ending_at_end,
                                    "Ending (at the end)",
                                );
                                ui.add_space(25.0);
                                ui.label("Skip");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.ending_time_str)
                                        .desired_width(75.0)
                                        .font(egui::TextStyle::Monospace),
                                )
                                .on_hover_text("HH:MM:SS (e.g. 00:01:30)");
                            });
                            ui.add_space(4.0);

                            // Row 3: Chapter title(s) - toggleable tag chips
                            ui.horizontal(|ui| {
                                ui.checkbox(
                                    &mut self.chapters_enabled,
                                    "Chapter title(s)",
                                );
                            });

                            // Tag chip area
                            ui.add_space(2.0);
                            ui.horizontal(|ui| {
                                ui.add_space(20.0);
                                ui.vertical(|ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.spacing_mut().item_spacing = Vec2::new(4.0, 3.0);

                                        let mut remove_idx: Option<usize> = None;

                                        for (idx, tag) in
                                            self.tags.iter_mut().enumerate()
                                        {
                                            let (bg, fg, border) = if tag.enabled {
                                                (
                                                    Color32::from_rgb(35, 70, 42),
                                                    Color32::from_rgb(120, 230, 140),
                                                    Color32::from_rgb(60, 140, 80),
                                                )
                                            } else {
                                                (
                                                    Color32::from_rgb(40, 42, 48),
                                                    Color32::from_rgb(130, 135, 150),
                                                    Color32::from_rgb(65, 70, 80),
                                                )
                                            };

                                            let label_text = format!(" {} ", tag.keyword);
                                            let btn = egui::Button::new(
                                                RichText::new(&label_text)
                                                    .size(11.0)
                                                    .color(fg),
                                            )
                                            .fill(bg)
                                            .stroke(Stroke::new(1.0, border))
                                            .corner_radius(CornerRadius::same(10));

                                            let resp = ui.add(btn);

                                            if resp.clicked() {
                                                tag.enabled = !tag.enabled;
                                            }
                                            if resp.secondary_clicked() {
                                                remove_idx = Some(idx);
                                            }

                                            let tooltip = if tag.enabled {
                                                format!(
                                                    "✅ '{}' — enabled (click to disable, right-click to remove)",
                                                    tag.keyword
                                                )
                                            } else {
                                                format!(
                                                    "⬜ '{}' — disabled (click to enable, right-click to remove)",
                                                    tag.keyword
                                                )
                                            };
                                            resp.on_hover_text(tooltip);
                                        }

                                        if let Some(idx) = remove_idx {
                                            self.tags.remove(idx);
                                        }
                                    });

                                    // Add custom tag input
                                    ui.add_space(2.0);
                                    ui.horizontal(|ui| {
                                        let te = ui.add(
                                            egui::TextEdit::singleline(
                                                &mut self.new_tag_input,
                                            )
                                            .desired_width(120.0)
                                            .hint_text("New keyword..."),
                                        );
                                        let enter_pressed = te.lost_focus()
                                            && ui.input(|i| {
                                                i.key_pressed(egui::Key::Enter)
                                            });

                                        if (ui
                                            .add(
                                                egui::Button::new(
                                                    RichText::new("+ Add")
                                                        .size(10.5)
                                                        .color(
                                                            Color32::from_rgb(
                                                                100, 200, 120,
                                                            ),
                                                        ),
                                                )
                                                .min_size(Vec2::new(42.0, 18.0)),
                                            )
                                            .clicked()
                                            || enter_pressed)
                                            && !self.new_tag_input.trim().is_empty()
                                        {
                                            let kw = self
                                                .new_tag_input
                                                .trim()
                                                .to_lowercase();
                                            if !self
                                                .tags
                                                .iter()
                                                .any(|t| t.keyword == kw)
                                            {
                                                self.tags.push(SkipTag {
                                                    keyword: kw,
                                                    enabled: true,
                                                });
                                            }
                                            self.new_tag_input.clear();
                                        }

                                        ui.label(
                                            RichText::new(
                                                "(click = toggle, right-click = remove)",
                                            )
                                            .size(9.5)
                                            .color(Color32::from_rgb(110, 115, 130)),
                                        );
                                    });
                                });
                            });
                        });
                    });
                    ui.add_space(4.0);
                });

                ui.add_space(6.0);

                // =============================================================
                // 2. Table: Start | Length | Interval | Interval Type
                // =============================================================
                let table_width = ui.available_width();
                let col_start_w = 75.0;
                let col_len_w = 75.0;
                let col_int_w = 150.0;

                let frame_stroke = Stroke::new(1.0, Color32::from_rgb(60, 65, 75));
                let header_bg = Color32::from_rgb(32, 35, 43);
                let table_bg = Color32::from_rgb(22, 24, 29);

                egui::Frame::NONE
                    .fill(table_bg)
                    .stroke(frame_stroke)
                    .corner_radius(CornerRadius::same(2))
                    .show(ui, |ui| {
                        // Header
                        let (hdr_rect, _) = ui.allocate_exact_size(
                            Vec2::new(table_width, 24.0),
                            egui::Sense::hover(),
                        );
                        ui.painter().rect_filled(
                            hdr_rect,
                            CornerRadius::ZERO,
                            header_bg,
                        );
                        ui.painter().line_segment(
                            [hdr_rect.left_bottom(), hdr_rect.right_bottom()],
                            Stroke::new(1.0, Color32::from_rgb(50, 55, 65)),
                        );

                        let mut h_ui = ui
                            .new_child(egui::UiBuilder::new().max_rect(hdr_rect));
                        h_ui.horizontal(|ui| {
                            ui.set_height(24.0);
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Start")
                                    .strong()
                                    .size(11.5)
                                    .color(Color32::from_rgb(210, 215, 225)),
                            );
                            ui.add_space(col_start_w - 38.0);
                            ui.label(
                                RichText::new("Length")
                                    .strong()
                                    .size(11.5)
                                    .color(Color32::from_rgb(210, 215, 225)),
                            );
                            ui.add_space(col_len_w - 48.0);
                            ui.label(
                                RichText::new("Interval")
                                    .strong()
                                    .size(11.5)
                                    .color(Color32::from_rgb(210, 215, 225)),
                            );
                            ui.add_space(col_int_w - 58.0);
                            ui.label(
                                RichText::new("Interval Type")
                                    .strong()
                                    .size(11.5)
                                    .color(Color32::from_rgb(210, 215, 225)),
                            );
                        });

                        // Table Body
                        egui::ScrollArea::vertical()
                            .id_salt("potplayer_skip_setup_table_scroll")
                            .max_height(180.0)
                            .min_scrolled_height(140.0)
                            .show(ui, |ui| {
                                ui.set_min_width(table_width);

                                if self.intervals.is_empty() {
                                    ui.add_space(35.0);
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            RichText::new("No skip intervals added.")
                                                .color(Color32::from_rgb(
                                                    120, 125, 135,
                                                ))
                                                .size(11.0),
                                        );
                                        ui.label(
                                            RichText::new(
                                                "Click [Add...] below to create a custom skip interval.",
                                            )
                                            .color(Color32::from_rgb(90, 95, 105))
                                            .size(10.0),
                                        );
                                    });
                                    ui.add_space(35.0);
                                } else {
                                    let mut double_clicked_idx = None;

                                    for (idx, interval) in
                                        self.intervals.iter().enumerate()
                                    {
                                        let is_selected =
                                            self.selected_index == Some(idx);
                                        let row_bg = if is_selected {
                                            Color32::from_rgb(45, 75, 125)
                                        } else if idx % 2 == 1 {
                                            Color32::from_rgba_unmultiplied(
                                                255, 255, 255, 5,
                                            )
                                        } else {
                                            Color32::TRANSPARENT
                                        };

                                        let (row_rect, row_resp) =
                                            ui.allocate_exact_size(
                                                Vec2::new(table_width, 22.0),
                                                egui::Sense::click(),
                                            );
                                        if row_bg != Color32::TRANSPARENT {
                                            ui.painter().rect_filled(
                                                row_rect,
                                                CornerRadius::ZERO,
                                                row_bg,
                                            );
                                        }

                                        if row_resp.clicked() {
                                            self.selected_index = Some(idx);
                                        }
                                        if row_resp.double_clicked() {
                                            double_clicked_idx = Some(idx);
                                        }

                                        let mut r_ui = ui.new_child(
                                            egui::UiBuilder::new()
                                                .max_rect(row_rect),
                                        );
                                        r_ui.horizontal(|ui| {
                                            ui.set_height(22.0);
                                            ui.add_space(8.0);

                                            let text_col = if is_selected {
                                                Color32::WHITE
                                            } else {
                                                Color32::from_rgb(220, 225, 230)
                                            };

                                            ui.add(egui::Label::new(
                                                RichText::new(format_time_hms(
                                                    interval.start,
                                                ))
                                                .monospace()
                                                .size(11.0)
                                                .color(text_col),
                                            ));
                                            ui.add_space(col_start_w - 55.0);

                                            ui.add(egui::Label::new(
                                                RichText::new(format_time_hms(
                                                    interval.length,
                                                ))
                                                .monospace()
                                                .size(11.0)
                                                .color(text_col),
                                            ));
                                            ui.add_space(col_len_w - 55.0);

                                            let end_pos =
                                                interval.start + interval.length;
                                            ui.add(egui::Label::new(
                                                RichText::new(format!(
                                                    "{} - {}",
                                                    format_time_hms(interval.start),
                                                    format_time_hms(end_pos)
                                                ))
                                                .monospace()
                                                .size(11.0)
                                                .color(text_col),
                                            ));
                                            ui.add_space(col_int_w - 128.0);

                                            ui.add(egui::Label::new(
                                                RichText::new(
                                                    &interval.interval_type,
                                                )
                                                .size(11.0)
                                                .color(if is_selected {
                                                    Color32::WHITE
                                                } else {
                                                    VortexTheme::POT_YELLOW
                                                }),
                                            ));
                                        });
                                    }

                                    if let Some(idx) = double_clicked_idx {
                                        self.selected_index = Some(idx);
                                        let item = &self.intervals[idx];
                                        self.modal_is_edit = true;
                                        self.modal_start_str =
                                            format_time_hms(item.start);
                                        self.modal_length_str =
                                            format_time_hms(item.length);
                                        self.modal_type_str =
                                            item.interval_type.clone();
                                        self.is_modal_open = true;
                                    }
                                }
                            });
                    });

                ui.add_space(8.0);

                // =============================================================
                // 3. Action Buttons Below Table
                // =============================================================
                ui.horizontal(|ui| {
                    // Scan from current file chapters button
                    if !chapters.is_empty() {
                        let enabled_tags: Vec<String> = self
                            .tags
                            .iter()
                            .filter(|t| t.enabled)
                            .map(|t| t.keyword.clone())
                            .collect();
                        if ui
                            .button(
                                RichText::new(format!(
                                    "📥 Scan Chapters ({})",
                                    chapters.len()
                                ))
                                .color(Color32::from_rgb(70, 180, 255)),
                            )
                            .on_hover_text(
                                "Scan current file's chapters and auto-add matching intervals",
                            )
                            .clicked()
                        {
                            for (i, ch) in chapters.iter().enumerate() {
                                let matches = enabled_tags
                                    .iter()
                                    .any(|kw| crate::config::matches_skip_keyword(&ch.title, kw));
                                if matches {
                                    let start = ch.time_pos;
                                    let end = if let Some(next_c) =
                                        chapters.get(i + 1)
                                    {
                                        next_c.time_pos
                                    } else if duration > start {
                                        duration
                                    } else {
                                        start + 90.0
                                    };
                                    let length = (end - start).max(0.0);
                                    // Don't add duplicates
                                    let already_exists = self.intervals.iter().any(
                                        |iv| {
                                            (iv.start - start).abs() < 0.5
                                                && (iv.length - length).abs() < 0.5
                                        },
                                    );
                                    if !already_exists {
                                        self.intervals.push(ConfigSkipInterval {
                                            start,
                                            length,
                                            interval_type: "Skip".to_string(),
                                            enabled: true,
                                        });
                                    }
                                }
                            }
                            self.intervals.sort_by(|a, b| {
                                a.start
                                    .partial_cmp(&b.start)
                                    .unwrap_or(std::cmp::Ordering::Equal)
                            });
                        }
                    }

                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            let has_selection = self.selected_index.is_some()
                                && self.selected_index.unwrap()
                                    < self.intervals.len();

                            if ui
                                .add_enabled(
                                    has_selection,
                                    egui::Button::new("  Delete  ")
                                        .min_size(Vec2::new(72.0, 22.0)),
                                )
                                .clicked()
                            {
                                if let Some(idx) = self.selected_index {
                                    if idx < self.intervals.len() {
                                        self.intervals.remove(idx);
                                        if self.intervals.is_empty() {
                                            self.selected_index = None;
                                        } else if idx >= self.intervals.len() {
                                            self.selected_index =
                                                Some(self.intervals.len() - 1);
                                        }
                                    }
                                }
                            }

                            ui.add_space(4.0);

                            if ui
                                .add_enabled(
                                    has_selection,
                                    egui::Button::new("  Edit...  ")
                                        .min_size(Vec2::new(72.0, 22.0)),
                                )
                                .clicked()
                            {
                                if let Some(idx) = self.selected_index {
                                    if let Some(item) = self.intervals.get(idx) {
                                        self.modal_is_edit = true;
                                        self.modal_start_str =
                                            format_time_hms(item.start);
                                        self.modal_length_str =
                                            format_time_hms(item.length);
                                        self.modal_type_str =
                                            item.interval_type.clone();
                                        self.is_modal_open = true;
                                    }
                                }
                            }

                            ui.add_space(4.0);

                            if ui
                                .add(egui::Button::new("   Add...   ").min_size(Vec2::new(72.0, 22.0)))
                                .clicked()
                            {
                                self.modal_is_edit = false;
                                self.modal_start_str =
                                    format_time_hms(current_time);
                                self.modal_length_str = "00:01:30".to_string();
                                self.modal_type_str = "Skip".to_string();
                                self.is_modal_open = true;
                            }
                        },
                    );
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(4.0);

                // =============================================================
                // 4. OK / Cancel
                // =============================================================
                ui.horizontal(|ui| {
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            if ui
                                .add(egui::Button::new("  Cancel  ").min_size(Vec2::new(76.0, 24.0)))
                                .clicked()
                            {
                                cancel_and_close = true;
                            }
                            ui.add_space(8.0);
                            if ui
                                .add(egui::Button::new(RichText::new("    OK    ").strong()).min_size(Vec2::new(76.0, 24.0)))
                                .clicked()
                            {
                                save_and_close = true;
                            }
                        },
                    );
                });
            });

        // =====================================================================
        // Sub-Modal: Add / Edit Skip Interval
        // =====================================================================
        if self.is_modal_open {
            let modal_title = if self.modal_is_edit {
                "Edit Skip Interval"
            } else {
                "Add Skip Interval"
            };
            let mut modal_open = self.is_modal_open;
            let mut apply_modal = false;

            egui::Window::new(modal_title)
                .id(egui::Id::new("sub_modal_skip_interval"))
                .open(&mut modal_open)
                .resizable(false)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 20.0))
                .default_width(340.0)
                .show(ctx, |ui| {
                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.label("Start Time:    ");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.modal_start_str)
                                .desired_width(80.0)
                                .font(egui::TextStyle::Monospace),
                        );
                        if ui
                            .small_button("⏱ Curr")
                            .on_hover_text("Set to current playback time")
                            .clicked()
                        {
                            self.modal_start_str = format_time_hms(current_time);
                        }
                    });
                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.label("Length / Dur: ");
                        ui.add(
                            egui::TextEdit::singleline(
                                &mut self.modal_length_str,
                            )
                            .desired_width(80.0)
                            .font(egui::TextStyle::Monospace),
                        );
                        if ui
                            .small_button("+85s")
                            .on_hover_text("Standard Anime Opening (85s)")
                            .clicked()
                        {
                            self.modal_length_str = "00:01:25".to_string();
                        }
                        if ui
                            .small_button("+90s")
                            .on_hover_text("Standard TV Opening (90s)")
                            .clicked()
                        {
                            self.modal_length_str = "00:01:30".to_string();
                        }
                    });
                    ui.add_space(4.0);

                    // Quick Chapter Loader
                    if !chapters.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label("From Chapter:");
                            ui.menu_button("📖 Select Chapter ▾", |ui| {
                                for (i, ch) in chapters.iter().enumerate() {
                                    let label = if ch.title.trim().is_empty() {
                                        format!("Chapter {}", i + 1)
                                    } else {
                                        ch.title.trim().to_string()
                                    };
                                    let start = ch.time_pos;
                                    let end =
                                        if let Some(next_c) = chapters.get(i + 1) {
                                            next_c.time_pos
                                        } else if duration > start {
                                            duration
                                        } else {
                                            start + 90.0
                                        };
                                    let dur = (end - start).max(0.0);
                                    let title_row = format!(
                                        "{}. {} ({})",
                                        i + 1,
                                        label,
                                        format_time_hms(dur)
                                    );
                                    if ui.button(title_row).clicked() {
                                        self.modal_start_str =
                                            format_time_hms(start);
                                        self.modal_length_str =
                                            format_time_hms(dur);
                                        ui.close();
                                    }
                                }
                            });
                        });
                        ui.add_space(4.0);
                    }

                    ui.horizontal(|ui| {
                        ui.label("Interval Type: ");
                        ui.radio_value(
                            &mut self.modal_type_str,
                            "Skip".to_string(),
                            "Skip",
                        );
                        ui.radio_value(
                            &mut self.modal_type_str,
                            "Play only".to_string(),
                            "Play only",
                        );
                    });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if ui.button(" Cancel ").clicked() {
                                    self.is_modal_open = false;
                                }
                                ui.add_space(6.0);
                                if ui
                                    .button(RichText::new("   OK   ").strong())
                                    .clicked()
                                {
                                    apply_modal = true;
                                }
                            },
                        );
                    });
                });

            if apply_modal {
                let start_sec =
                    parse_time_hms(&self.modal_start_str).unwrap_or(0.0);
                let length_sec =
                    parse_time_hms(&self.modal_length_str).unwrap_or(90.0);
                let new_interval = ConfigSkipInterval {
                    start: start_sec,
                    length: length_sec,
                    interval_type: if self.modal_type_str.is_empty() {
                        "Skip".to_string()
                    } else {
                        self.modal_type_str.clone()
                    },
                    enabled: true,
                };

                if self.modal_is_edit {
                    if let Some(idx) = self.selected_index {
                        if idx < self.intervals.len() {
                            self.intervals[idx] = new_interval;
                        }
                    }
                } else {
                    self.intervals.push(new_interval);
                    self.intervals.sort_by(|a, b| {
                        a.start
                            .partial_cmp(&b.start)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    self.selected_index =
                        Some(self.intervals.len().saturating_sub(1));
                }
                self.is_modal_open = false;
            } else if !modal_open {
                self.is_modal_open = false;
            }
        }

        // =====================================================================
        // Finalize OK / Cancel
        // =====================================================================
        if save_and_close {
            // Apply all settings to AppConfig and persist to JSON
            config.skip_intro_enabled = self.enable_skip_feature;
            config.skip_intro_at_start = self.intro_at_start;
            config.skip_intro_sec =
                parse_time_hms(&self.intro_time_str).unwrap_or(0.0);
            config.skip_ending_at_end = self.ending_at_end;
            config.skip_outro_sec =
                parse_time_hms(&self.ending_time_str).unwrap_or(0.0);
            config.skip_chapters_enabled = self.chapters_enabled;
            config.skip_chapter_titles = tags_to_config_string(&self.tags);
            config.skip_intervals = self.intervals.clone();
            let _ = config.save();

            // Sync with bookmark manager
            bookmark_mgr.auto_skip_bookmarks = self.enable_skip_feature;
            bookmark_mgr.skip_intervals.clear();
            for item in &self.intervals {
                bookmark_mgr.add_skip_interval(
                    item.start,
                    item.start + item.length,
                    "Skip Interval".to_string(),
                );
            }
            bookmark_mgr.sync_skip_intervals_to_bookmarks();

            self.is_open = false;
            self.was_open = false;
        } else if cancel_and_close || !open {
            self.is_open = false;
            self.was_open = false;
        }

        window_resp.map(|r| r.response.rect)
    }
}
