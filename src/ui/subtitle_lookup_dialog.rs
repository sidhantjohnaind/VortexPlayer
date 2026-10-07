#![allow(dead_code)]

use eframe::egui::{self, Align2, Color32, CornerRadius, Pos2, Rect, RichText, Stroke, Vec2};
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

pub struct SubtitleLookupDialog {
    pub is_open: bool,
    pub current_text: String,
    pub target_language: String,
    pub last_hud_rect: Option<Rect>,
    pub should_resume: bool,
}

impl Default for SubtitleLookupDialog {
    fn default() -> Self {
        Self {
            is_open: false,
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

    /// Render lightweight floating hover HUD card directly above the subtitle.
    /// Not a popup window — a sleek, frosted glass on-screen hover HUD.
    pub fn render_lightweight_hud(
        &mut self,
        ctx: &egui::Context,
        pic_rect: Rect,
        sub_zone: Rect,
        _search_engine: &str,
        _pointer_pos: Option<Pos2>,
        osd_msg: &mut Option<String>,
    ) -> Option<Rect> {
        let raw_clean = crate::subtitles::online::SubtitleDownloader::strip_subtitle_tags(&self.current_text)
            .replace("\\N", " ")
            .replace("\\n", " ")
            .replace('\n', " ")
            .trim()
            .to_string();

        if raw_clean.is_empty() {
            self.last_hud_rect = None;
            return None;
        }

        // Combined hit-test zone: dynamic subtitle zone + HUD area (expanded with 14px margin so zero gaps)
        let mut hover_zone = sub_zone;
        if let Some(hud_r) = self.last_hud_rect {
            hover_zone = hover_zone.union(hud_r);
        }
        let _hover_zone = hover_zone.expand(14.0);

        // ONLY show when explicitly opened via clicking the subtitle!
        if !self.is_open {
            self.last_hud_rect = None;
            return None;
        }

        // Anchor HUD directly above the dynamic subtitle zone (touches top of subtitle text with clean 8px gap)
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
                    .inner_margin(egui::Margin::symmetric(10, 7))
                    .show(ui, |ui| {
                        ui.set_max_width(720.0);
                        ui.vertical(|ui| {
                            // Row 1: Header quick actions
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
                                    let close_btn = egui::Button::new(RichText::new("✕").size(10.5).color(Color32::from_rgb(160, 165, 180)))
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

    /// Compatibility wrapper for render
    pub fn render(&mut self, _ctx: &egui::Context, _search_engine: &str) -> Option<Rect> {
        None
    }
}
