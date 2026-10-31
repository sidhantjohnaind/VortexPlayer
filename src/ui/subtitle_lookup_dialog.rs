#![allow(dead_code)]

use crate::subtitles::online::{SubtitleDownloader, SubtitleQuery, SubtitleSearchResult};
use crate::ui::theme::VortexTheme;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, Stroke, Vec2};
use std::process::Command;

/// URL encoding helper for web search queries
pub fn url_encode(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => {
                encoded.push('+');
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleLookupTab {
    OnlineSearch,
    WordLookup,
}

pub struct SubtitleLookupDialog {
    pub is_open: bool,
    pub is_pinned: bool,
    pub active_tab: SubtitleLookupTab,
    pub search_query: String,
    pub language_filter: String,
    pub search_results: Vec<SubtitleSearchResult>,
    pub is_searching: bool,
    pub status_message: String,
    pub pending_download: Option<SubtitleSearchResult>,
    pub current_text: String,
    pub target_language: String,
    pub last_hud_rect: Option<Rect>,
    pub should_resume: bool,
}

impl Default for SubtitleLookupDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_pinned: false,
            active_tab: SubtitleLookupTab::OnlineSearch,
            search_query: String::new(),
            language_filter: "en".to_string(),
            search_results: Vec::new(),
            is_searching: false,
            status_message: String::new(),
            pending_download: None,
            current_text: String::new(),
            target_language: "English".to_string(),
            last_hud_rect: None,
            should_resume: false,
        }
    }
}

impl SubtitleLookupDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    /// Set initial query from playing video filename or title
    pub fn set_default_query_if_empty(&mut self, title: &str) {
        if self.search_query.is_empty() {
            let clean = title.replace(".mkv", "").replace(".mp4", "").replace(".avi", "").replace(".webm", "");
            self.search_query = clean.trim().to_string();
        }
    }

    /// Pronounce text aloud via Windows Speech API / macOS 'say' / Linux 'espeak'
    pub fn speak_text(text: &str) {
        let clean = text.trim();
        if clean.is_empty() {
            return;
        }
        #[cfg(windows)]
        {
            let escaped = clean.replace('"', "\\\"");
            let script = format!(
                r#"Add-Type -AssemblyName System.Speech; $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; $synth.Speak("{}");"#,
                escaped
            );
            let _ = crate::platform::silent_command("powershell")
                .args(&["-NoProfile", "-Command", &script])
                .spawn();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("say").arg(clean).spawn();
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let _ = Command::new("espeak").arg(clean).spawn();
        }
    }

    /// Silently launch URL in default browser
    pub fn open_url(url: &str) {
        #[cfg(windows)]
        {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            let wide: Vec<u16> = OsStr::new(url).encode_wide().chain(std::iter::once(0)).collect();
            let op_wide: [u16; 5] = [b'o' as u16, b'p' as u16, b'e' as u16, b'n' as u16, 0];
            let res = unsafe {
                windows_sys::Win32::UI::Shell::ShellExecuteW(
                    0 as _,
                    op_wide.as_ptr(),
                    wide.as_ptr(),
                    std::ptr::null(),
                    std::ptr::null(),
                    1, // SW_SHOWNORMAL
                )
            };
            if (res as isize) <= 32 {
                let _ = crate::platform::silent_command("cmd")
                    .args(&["/c", "start", "", url])
                    .spawn();
            }
        }
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("open").arg(url).spawn();
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let _ = Command::new("xdg-open").arg(url).spawn();
        }
    }

    /// Open Google search directly for any word or sentence (supports all languages)
    pub fn open_google_search(query: &str) {
        let clean = url_encode(query.trim());
        if !clean.is_empty() {
            Self::open_url(&format!("https://www.google.com/search?q={}", clean));
        }
    }

    /// Open Google Translate with auto source language detection (supports all languages)
    pub fn open_google_translate(query: &str) {
        let clean = url_encode(query.trim());
        if !clean.is_empty() {
            Self::open_url(&format!("https://translate.google.com/?sl=auto&tl=en&text={}&op=translate", clean));
        }
    }

    /// Open web search (Google, Google Translate, DuckDuckGo, Bing, Cambridge Dictionary)
    pub fn open_web_search(query: &str, engine: &str) {
        let clean = url_encode(query.trim());
        if clean.is_empty() {
            return;
        }
        let url = match engine {
            "Google Translate" => format!("https://translate.google.com/?sl=auto&tl=en&text={}&op=translate", clean),
            "DuckDuckGo" => format!("https://duckduckgo.com/?q={}", clean),
            "Bing" => format!("https://www.bing.com/search?q={}", clean),
            "Cambridge" => format!("https://dictionary.cambridge.org/search/direct/?datasetsearch=english&q={}", clean),
            _ => format!("https://www.google.com/search?q={}", clean),
        };
        Self::open_url(&url);
    }

    /// Execute subtitle query across OpenSubtitles
    pub fn perform_search(&mut self) {
        let query_text = self.search_query.trim();
        if query_text.is_empty() {
            self.status_message = "Please enter a title or filename to search subtitles".to_string();
            return;
        }

        self.is_searching = true;
        self.status_message = format!("Searching OpenSubtitles for \"{}\"...", query_text);

        let query = SubtitleQuery {
            movie_hash: None,
            file_size: None,
            query_text: Some(query_text.to_string()),
            target_lang: self.language_filter.clone(),
        };

        self.search_results = SubtitleDownloader::search_subtitles(&query);
        self.is_searching = false;
        self.status_message = format!("Found {} subtitle matches for \"{}\"", self.search_results.len(), query_text);
    }

    /// Take pending downloaded subtitle to apply to active player
    pub fn take_pending_download(&mut self) -> Option<SubtitleSearchResult> {
        self.pending_download.take()
    }

    /// Full dialog renderer with interactive search box and results
    pub fn render(&mut self, ctx: &egui::Context, _search_engine: &str) -> Option<Rect> {
        if !self.is_open {
            return None;
        }

        let skin = VortexTheme::current_skin();
        let mut open_flag = self.is_open;
        let mut close_requested = false;
        let mut trigger_search = false;

        crate::ui::native_window::show_native_window(
            ctx,
            "vortex_subtitle_lookup_window",
            "",
            Vec2::new(640.0, 480.0),
            true,
            &mut open_flag,
            |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 6.0);

                // ── Custom Unified Header ─────────────────────────────────────
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);
                    ui.label(RichText::new("🌐 Online Subtitles").size(13.5).strong().color(skin.accent_primary));
                    ui.label(RichText::new("• Search & Lookup Studio").size(11.5).color(skin.text_muted));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 1. Close Button (Rightmost)
                        let close_btn = ui.add(
                            egui::Button::new(RichText::new("✕").size(12.0).color(skin.text_secondary))
                                .fill(Color32::TRANSPARENT)
                                .stroke(Stroke::NONE)
                        ).on_hover_text("Close Subtitle Search (Esc)");
                        if close_btn.clicked() {
                            close_requested = true;
                        }

                        ui.add_space(4.0);

                        // 2. Pin Button (Immediately adjacent to Close!)
                        let pin_icon = if self.is_pinned { "📌 Unpin" } else { "📍 Pin" };
                        let pin_btn = ui.add(
                            egui::Button::new(
                                RichText::new(pin_icon)
                                    .size(11.0)
                                    .color(if self.is_pinned { skin.accent_bright } else { Color32::from_rgb(175, 180, 195) })
                            )
                            .fill(if self.is_pinned { Color32::from_rgb(32, 40, 56) } else { Color32::from_rgb(22, 24, 30) })
                            .stroke(Stroke::new(1.0, if self.is_pinned { Color32::from_rgb(50, 70, 110) } else { Color32::from_rgb(40, 44, 56) }))
                            .corner_radius(CornerRadius::same(4))
                        ).on_hover_text(if self.is_pinned { "Unpin window" } else { "Pin window on top of playback" });
                        if pin_btn.clicked() {
                            self.is_pinned = !self.is_pinned;
                        }

                        ui.add_space(8.0);

                        // Tab Switchers
                        let is_word_tab = self.active_tab == SubtitleLookupTab::WordLookup;
                        if ui.add(
                            egui::Button::new(
                                RichText::new("📖 Word Lookup & TTS")
                                    .size(11.0)
                                    .color(if is_word_tab { skin.accent_bright } else { skin.text_secondary })
                            )
                            .fill(if is_word_tab { Color32::from_rgb(28, 36, 52) } else { Color32::from_rgb(18, 20, 26) })
                            .stroke(Stroke::new(1.0, if is_word_tab { skin.accent_primary } else { Color32::from_rgb(38, 42, 54) }))
                            .corner_radius(CornerRadius::same(4))
                        ).clicked() {
                            self.active_tab = SubtitleLookupTab::WordLookup;
                        }

                        let is_search_tab = self.active_tab == SubtitleLookupTab::OnlineSearch;
                        if ui.add(
                            egui::Button::new(
                                RichText::new("🔍 Online Search")
                                    .size(11.0)
                                    .color(if is_search_tab { skin.accent_bright } else { skin.text_secondary })
                            )
                            .fill(if is_search_tab { Color32::from_rgb(28, 36, 52) } else { Color32::from_rgb(18, 20, 26) })
                            .stroke(Stroke::new(1.0, if is_search_tab { skin.accent_primary } else { Color32::from_rgb(38, 42, 54) }))
                            .corner_radius(CornerRadius::same(4))
                        ).clicked() {
                            self.active_tab = SubtitleLookupTab::OnlineSearch;
                        }
                    });
                });

                ui.separator();

                match self.active_tab {
                    SubtitleLookupTab::OnlineSearch => {
                        // ── Subtitle Search Box ───────────────────────────────
                        let has_query = !self.search_query.is_empty();
                        let search_frame = egui::Frame::new()
                            .fill(Color32::from_rgb(14, 16, 22))
                            .stroke(Stroke::new(1.0, if has_query { skin.accent_primary } else { Color32::from_rgb(35, 40, 52) }))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::symmetric(8, 5));

                        search_frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);
                                ui.label(RichText::new("🔍").size(12.5).color(skin.accent_primary));

                                let avail_w = ui.available_width() - 240.0;
                                let text_edit = egui::TextEdit::singleline(&mut self.search_query)
                                    .hint_text("Enter movie/show title or release (e.g. Inception 2010)...")
                                    .font(FontId::proportional(12.0))
                                    .text_color(Color32::WHITE)
                                    .desired_width(avail_w.max(120.0))
                                    .frame(egui::Frame::NONE);
                                let edit_resp = ui.add(text_edit);
                                if edit_resp.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    trigger_search = true;
                                }

                                if has_query {
                                    let clear_btn = egui::Button::new(RichText::new("✕").size(10.0).color(skin.text_muted))
                                        .fill(Color32::from_rgb(26, 30, 40))
                                        .stroke(Stroke::new(1.0, Color32::from_rgb(45, 50, 65)))
                                        .corner_radius(CornerRadius::same(3));
                                    if ui.add_sized(Vec2::new(18.0, 18.0), clear_btn).on_hover_text("Clear query").clicked() {
                                        self.search_query.clear();
                                    }
                                }

                                // Language filter dropdown
                                egui::ComboBox::from_id_salt("online_sub_lang_combo")
                                    .width(80.0)
                                    .selected_text(match self.language_filter.as_str() {
                                        "all" => "🌐 All",
                                        "en" => "🇬🇧 English",
                                        "es" => "🇪🇸 Spanish",
                                        "fr" => "🇫🇷 French",
                                        "de" => "🇩🇪 German",
                                        "ja" => "🇯🇵 Japanese",
                                        "zh" => "🇨🇳 Chinese",
                                        "ko" => "🇰🇷 Korean",
                                        "it" => "🇮🇹 Italian",
                                        _ => "English",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.language_filter, "all".to_string(), "🌐 All Languages");
                                        ui.selectable_value(&mut self.language_filter, "en".to_string(), "🇬🇧 English (en)");
                                        ui.selectable_value(&mut self.language_filter, "es".to_string(), "🇪🇸 Spanish (es)");
                                        ui.selectable_value(&mut self.language_filter, "fr".to_string(), "🇫🇷 French (fr)");
                                        ui.selectable_value(&mut self.language_filter, "de".to_string(), "🇩🇪 German (de)");
                                        ui.selectable_value(&mut self.language_filter, "ja".to_string(), "🇯🇵 Japanese (ja)");
                                        ui.selectable_value(&mut self.language_filter, "zh".to_string(), "🇨🇳 Chinese (zh)");
                                        ui.selectable_value(&mut self.language_filter, "ko".to_string(), "🇰🇷 Korean (ko)");
                                        ui.selectable_value(&mut self.language_filter, "it".to_string(), "🇮🇹 Italian (it)");
                                    });

                                // Search button
                                let search_btn = egui::Button::new(
                                    RichText::new("🔍 Search")
                                        .size(11.5)
                                        .strong()
                                        .color(Color32::WHITE)
                                )
                                .fill(Color32::from_rgb(32, 55, 90))
                                .stroke(Stroke::new(1.0, skin.accent_primary))
                                .corner_radius(CornerRadius::same(4));
                                if ui.add_sized(Vec2::new(76.0, 24.0), search_btn).clicked() {
                                    trigger_search = true;
                                }
                            });
                        });

                        if !self.status_message.is_empty() {
                            ui.label(RichText::new(&self.status_message).size(11.0).color(skin.text_muted));
                        }

                        ui.add_space(2.0);

                        // ── Results List ──────────────────────────────────────
                        egui::ScrollArea::vertical()
                            .id_salt("online_sub_results_scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(0.0, 4.0);

                                if self.search_results.is_empty() {
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(40.0);
                                        ui.label(RichText::new("🔍 Search OpenSubtitles Database").size(13.5).strong().color(skin.text_primary));
                                        ui.label(RichText::new("Enter title or keywords above and click 'Search' to download synchronized subtitles.").size(11.5).color(skin.text_muted));
                                    });
                                } else {
                                    for res in &self.search_results {
                                        let card_frame = egui::Frame::new()
                                            .fill(Color32::from_rgb(16, 19, 26))
                                            .stroke(Stroke::new(1.0, Color32::from_rgb(30, 36, 48)))
                                            .corner_radius(CornerRadius::same(5))
                                            .inner_margin(Margin::symmetric(10, 8));

                                        card_frame.show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);
                                                        // Format pill
                                                        let fmt_upper = res.format.to_uppercase();
                                                        let pill_frame = egui::Frame::new()
                                                            .fill(Color32::from_rgb(24, 34, 52))
                                                            .stroke(Stroke::new(1.0, Color32::from_rgb(45, 75, 120)))
                                                            .corner_radius(CornerRadius::same(3))
                                                            .inner_margin(Margin::symmetric(5, 1));
                                                        pill_frame.show(ui, |ui| {
                                                            ui.label(RichText::new(fmt_upper).size(10.0).strong().color(Color32::from_rgb(120, 180, 255)));
                                                        });

                                                        // Language tag
                                                        ui.label(RichText::new(format!("[{}]", res.language.to_uppercase())).size(10.5).color(skin.accent_primary).strong());

                                                        // Rating badge
                                                        ui.label(RichText::new(format!("{:.1} ★", res.rating)).size(10.5).color(Color32::from_rgb(255, 205, 70)));
                                                    });

                                                    ui.add_space(2.0);
                                                    ui.label(RichText::new(&res.release_name).size(12.0).strong().color(Color32::WHITE));
                                                });

                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    let dl_btn = egui::Button::new(
                                                        RichText::new("⬇ Download & Apply")
                                                            .size(11.0)
                                                            .strong()
                                                            .color(Color32::WHITE)
                                                    )
                                                    .fill(Color32::from_rgb(28, 60, 42))
                                                    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 130, 80)))
                                                    .corner_radius(CornerRadius::same(4));

                                                    if ui.add(dl_btn).on_hover_text("Download subtitle and immediately load into video").clicked() {
                                                        self.pending_download = Some(res.clone());
                                                    }
                                                });
                                            });
                                        });
                                    }
                                }
                            });
                    }
                    SubtitleLookupTab::WordLookup => {
                        // ── Subtitle Dialogue & Word Lookup ───────────────────
                        ui.label(RichText::new("Active Subtitle Dialogue:").size(12.0).strong().color(skin.text_primary));
                        let raw_clean = SubtitleDownloader::strip_subtitle_tags(&self.current_text)
                            .replace(r"\N", " ")
                            .replace(r"\n", " ")
                            .replace('\n', " ")
                            .trim()
                            .to_string();

                        let sub_display = if raw_clean.is_empty() {
                            "(No active subtitle dialogue currently visible on video)".to_string()
                        } else {
                            raw_clean.clone()
                        };

                        let text_box_frame = egui::Frame::new()
                            .fill(Color32::from_rgb(14, 16, 22))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(35, 40, 52)))
                            .corner_radius(CornerRadius::same(6))
                            .inner_margin(Margin::symmetric(10, 8));

                        text_box_frame.show(ui, |ui| {
                            ui.label(RichText::new(&sub_display).size(13.5).color(if raw_clean.is_empty() { skin.text_muted } else { Color32::WHITE }));
                        });

                        ui.add_space(4.0);

                        // Actions row
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);

                            if ui.add(
                                egui::Button::new(RichText::new("🔍 Search Google").size(11.0).color(Color32::from_rgb(120, 200, 255)))
                                    .fill(Color32::from_rgb(20, 36, 58))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 80, 130)))
                                    .corner_radius(CornerRadius::same(4))
                            ).clicked() && !raw_clean.is_empty() {
                                Self::open_google_search(&raw_clean);
                            }

                            if ui.add(
                                egui::Button::new(RichText::new("🌐 Google Translate").size(11.0).color(Color32::from_rgb(120, 230, 180)))
                                    .fill(Color32::from_rgb(18, 42, 34))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(38, 95, 70)))
                                    .corner_radius(CornerRadius::same(4))
                            ).clicked() && !raw_clean.is_empty() {
                                Self::open_google_translate(&raw_clean);
                            }

                            if ui.add(
                                egui::Button::new(RichText::new("🔊 Listen (TTS)").size(11.0).color(Color32::from_rgb(240, 225, 120)))
                                    .fill(Color32::from_rgb(38, 34, 20))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(80, 75, 45)))
                                    .corner_radius(CornerRadius::same(4))
                            ).clicked() && !raw_clean.is_empty() {
                                Self::speak_text(&raw_clean);
                            }

                            if ui.add(
                                egui::Button::new(RichText::new("📋 Copy Text").size(11.0).color(skin.text_primary))
                                    .fill(Color32::from_rgb(24, 26, 34))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 50, 65)))
                                    .corner_radius(CornerRadius::same(4))
                            ).clicked() && !raw_clean.is_empty() {
                                ui.ctx().copy_text(raw_clean.clone());
                            }
                        });

                        ui.add_space(8.0);

                        // Word chips
                        if !raw_clean.is_empty() {
                            ui.label(RichText::new("Click any word below to search or translate:").size(11.0).color(skin.text_muted));
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                                for word in raw_clean.split_whitespace() {
                                    let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'');
                                    if clean_word.is_empty() {
                                        continue;
                                    }
                                    let word_btn = egui::Button::new(
                                        RichText::new(word).size(12.5).strong().color(Color32::WHITE)
                                    )
                                    .fill(Color32::from_rgb(22, 32, 48))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 75, 115)))
                                    .corner_radius(CornerRadius::same(4));

                                    let resp = ui.add(word_btn)
                                        .on_hover_text(format!("🔍 Left-click: Google Search \"{}\"\n🌐 Middle-click: Google Translate\n📋 Right-click: Copy", clean_word));

                                    if resp.clicked() {
                                        Self::open_google_search(clean_word);
                                    }
                                    if resp.middle_clicked() {
                                        Self::open_google_translate(clean_word);
                                    }
                                    if resp.secondary_clicked() {
                                        ui.ctx().copy_text(clean_word.to_string());
                                    }
                                }
                            });
                        }
                    }
                }
            },
        );

        if trigger_search {
            self.perform_search();
        }

        if close_requested {
            self.is_open = false;
        } else {
            self.is_open = open_flag;
        }

        None
    }

    /// Render lightweight floating hover HUD card directly above the subtitle
    pub fn render_lightweight_hud(
        &mut self,
        ctx: &egui::Context,
        pic_rect: Rect,
        sub_zone: Rect,
        _search_engine: &str,
        _pointer_pos: Option<Pos2>,
        osd_msg: &mut Option<String>,
    ) -> Option<Rect> {
        let raw_clean = SubtitleDownloader::strip_subtitle_tags(&self.current_text)
            .replace(r"\N", " ")
            .replace(r"\n", " ")
            .replace('\n', " ")
            .trim()
            .to_string();

        if raw_clean.is_empty() {
            self.last_hud_rect = None;
            return None;
        }

        if !self.is_open {
            self.last_hud_rect = None;
            return None;
        }

        let hud_pos = Pos2::new(
            sub_zone.center().x.clamp(pic_rect.min.x + 220.0, pic_rect.max.x - 220.0),
            (sub_zone.min.y - 8.0).max(pic_rect.min.y + 70.0),
        );
        let mut should_close = false;

        let area_resp = egui::Area::new(egui::Id::new("vortex_subtitle_lightweight_hover_hud"))
            .fixed_pos(hud_pos)
            .pivot(Align2::CENTER_BOTTOM)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::new()
                    .fill(Color32::from_rgba_unmultiplied(12, 16, 24, 235))
                    .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(80, 130, 200, 110)))
                    .corner_radius(CornerRadius::same(8))
                    .shadow(egui::epaint::Shadow {
                        offset: [0, 4],
                        blur: 16,
                        spread: 1,
                        color: Color32::from_black_alpha(180),
                    })
                    .inner_margin(Margin::symmetric(10, 7))
                    .show(ui, |ui| {
                        ui.set_max_width(720.0);
                        ui.vertical(|ui| {
                            // Row 1: Quick actions
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);

                                ui.label(RichText::new("📖").size(11.5));
                                ui.add_space(2.0);

                                let search_btn = egui::Button::new(
                                    RichText::new("🔍 Google Search").size(10.5).color(Color32::from_rgb(110, 200, 255))
                                )
                                .fill(Color32::from_rgb(20, 38, 62))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(45, 85, 140)))
                                .corner_radius(CornerRadius::same(4));
                                if ui.add(search_btn).on_hover_text("Search subtitle sentence on Google").clicked() {
                                    Self::open_google_search(&raw_clean);
                                    *osd_msg = Some(format!("🔍 Google: \"{}\"", raw_clean));
                                }

                                let trans_btn = egui::Button::new(
                                    RichText::new("🌐 Google Translate").size(10.5).color(Color32::from_rgb(120, 220, 175))
                                )
                                .fill(Color32::from_rgb(18, 40, 32))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(38, 95, 70)))
                                .corner_radius(CornerRadius::same(4));
                                if ui.add(trans_btn).on_hover_text("Translate with auto language detection on Google Translate").clicked() {
                                    Self::open_google_translate(&raw_clean);
                                    *osd_msg = Some("🌐 Opened in Google Translate".to_string());
                                }

                                let copy_btn = egui::Button::new(
                                    RichText::new("📋 Copy").size(10.5).color(Color32::from_rgb(200, 205, 220))
                                )
                                .fill(Color32::from_rgb(28, 32, 42))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(55, 62, 78)))
                                .corner_radius(CornerRadius::same(4));
                                if ui.add(copy_btn).on_hover_text("Copy subtitle to clipboard").clicked() {
                                    ui.ctx().copy_text(raw_clean.clone());
                                    *osd_msg = Some("📋 Copied subtitle to clipboard".to_string());
                                }

                                let speak_btn = egui::Button::new(
                                    RichText::new("🔊").size(10.5).color(Color32::from_rgb(225, 215, 120))
                                )
                                .fill(Color32::from_rgb(36, 34, 24))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(75, 70, 45)))
                                .corner_radius(CornerRadius::same(4));
                                if ui.add(speak_btn).on_hover_text("Listen to pronunciation via TTS").clicked() {
                                    Self::speak_text(&raw_clean);
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let close_btn = egui::Button::new(RichText::new("✕").size(10.5).color(crate::ui::theme::VortexTheme::current_skin().text_secondary))
                                        .fill(Color32::TRANSPARENT)
                                        .stroke(Stroke::NONE);
                                    if ui.add(close_btn).on_hover_text("Close (resumes playback)").clicked() {
                                        should_close = true;
                                    }
                                });
                            });

                            ui.add_space(4.0);

                            // Row 2: Word Chips
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                                for word in raw_clean.split_whitespace() {
                                    let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'');
                                    if clean_word.is_empty() {
                                        continue;
                                    }
                                    let word_btn = egui::Button::new(
                                        RichText::new(word).size(13.0).strong().color(Color32::WHITE)
                                    )
                                    .fill(Color32::from_rgb(24, 34, 52))
                                    .stroke(Stroke::new(1.0, Color32::from_rgb(50, 80, 125)))
                                    .corner_radius(CornerRadius::same(4));

                                    let resp = ui.add(word_btn)
                                        .on_hover_text(format!("🔍 Click: Google Search \"{}\"\n🌐 Middle-click: Google Translate\n📋 Right-click: Copy", clean_word));

                                    if resp.clicked() {
                                        Self::open_google_search(clean_word);
                                        *osd_msg = Some(format!("🔍 Google: \"{}\"", clean_word));
                                    }
                                    if resp.middle_clicked() {
                                        Self::open_google_translate(clean_word);
                                        *osd_msg = Some(format!("🌐 Translate: \"{}\"", clean_word));
                                    }
                                    if resp.secondary_clicked() {
                                        ui.ctx().copy_text(clean_word.to_string());
                                        *osd_msg = Some(format!("📋 Copied: \"{}\"", clean_word));
                                    }
                                }
                            });
                        });
                    });
            });

        if should_close {
            self.is_open = false;
            self.should_resume = true;
            self.last_hud_rect = None;
            return None;
        }

        let hud_rect = area_resp.response.rect;
        self.last_hud_rect = Some(hud_rect);
        Some(hud_rect)
    }
}
