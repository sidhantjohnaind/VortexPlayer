#![allow(dead_code)]

use eframe::egui::{self, Rect, Vec2};
use std::process::Command;

pub struct SubtitleLookupDialog {
    pub is_open: bool,
    pub current_text: String,
    pub target_language: String,
}

impl Default for SubtitleLookupDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            current_text: String::new(),
            target_language: "English".to_string(),
        }
    }
}

impl SubtitleLookupDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn speak_text(text: &str) {
        #[cfg(windows)]
        {
            let escaped = text.replace('"', "\\\"");
            let script = format!(r#"Add-Type -AssemblyName System.Speech; $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; $synth.Speak("{}");"#, escaped);
            let _ = crate::platform::silent_command("powershell").args(&["-NoProfile", "-Command", &script]).spawn();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("say").arg(text).spawn();
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let _ = Command::new("espeak").arg(text).spawn();
        }
    }

    pub fn open_web_dictionary(query: &str) {
        let clean = query.replace(' ', "%20");
        let url = format!("https://translate.google.com/?sl=auto&tl=en&text={}&op=translate", clean);
        #[cfg(windows)]
        {
            let _ = crate::platform::silent_command("cmd").args(&["/c", "start", &url]).spawn();
        }
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("open").arg(&url).spawn();
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let _ = Command::new("xdg-open").arg(&url).spawn();
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) -> Option<Rect> {
        if !self.is_open {
            return None;
        }

        let mut open = self.is_open;
        let resp = egui::Window::new("📖 Subtitle Word Lookup & TTS Voiceover")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_size(Vec2::new(360.0, 180.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Subtitle Text / Word:");
                    ui.text_edit_singleline(&mut self.current_text);

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("🔊 Speak Aloud (TTS)").clicked() {
                            Self::speak_text(&self.current_text);
                        }

                        if ui.button("🌐 Web Dictionary Lookup").clicked() {
                            Self::open_web_dictionary(&self.current_text);
                        }
                    });
                });
            });
        self.is_open = open;
        resp.map(|r| r.response.rect)
    }
}
