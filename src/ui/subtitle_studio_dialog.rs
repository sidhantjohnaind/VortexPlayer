//! subtitle_studio_dialog.rs — Subtitle Timeline GUI Editor & Multi-Engine Machine Translation / TTS Studio

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};
use std::fs;
use std::path::PathBuf;
#[cfg(windows)]
use std::os::windows::process::CommandExt;


#[derive(Debug, Clone)]
pub struct SubtitleCue {
    pub id: usize,
    pub start_time: f64,
    pub end_time: f64,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleStudioTab {
    TimelineEditor,
    TranslationEngine,
    TtsVoiceReader,
}

pub struct SubtitleStudioDialog {
    pub is_open: bool,
    pub active_tab: SubtitleStudioTab,
    pub cues: Vec<SubtitleCue>,
    pub selected_cue_idx: Option<usize>,
    pub shift_ms: i64,
    pub search_query: String,
    pub export_path: Option<PathBuf>,

    // Translation settings
    pub translation_provider: String,
    pub translation_api_key: String,
    pub target_language: String,
    pub auto_translate_dual: bool,

    // TTS Subtitle Reader
    pub tts_enabled: bool,
    pub tts_rate: i32,
    pub tts_volume: u32,
    pub tts_voice: String,
    last_spoken_text: String,
}

impl Default for SubtitleStudioDialog {
    fn default() -> Self {
        let sample_cues = vec![
            SubtitleCue {
                id: 1,
                start_time: 0.0,
                end_time: 3.5,
                text: "Welcome to VortexPlayer — High Performance Media Experience".to_string(),
            },
            SubtitleCue {
                id: 2,
                start_time: 4.0,
                end_time: 8.2,
                text: "Studio-grade D3D11 rendering with native HDR10 passthrough.".to_string(),
            },
            SubtitleCue {
                id: 3,
                start_time: 8.8,
                end_time: 14.0,
                text: "Full Vortex feature parity with memory-safe Rust foundation.".to_string(),
            },
        ];

        Self {
            is_open: false,
            active_tab: SubtitleStudioTab::TimelineEditor,
            cues: sample_cues,
            selected_cue_idx: None,
            shift_ms: 0,
            search_query: String::new(),
            export_path: None,
            translation_provider: "DeepL API".to_string(),
            translation_api_key: String::new(),
            target_language: "English (en)".to_string(),
            auto_translate_dual: false,
            tts_enabled: false,
            tts_rate: 0,
            tts_volume: 100,
            tts_voice: "Microsoft David".to_string(),
            last_spoken_text: String::new(),
        }
    }
}

impl SubtitleStudioDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format_timestamp(seconds: f64) -> String {
        let total_ms = (seconds * 1000.0).max(0.0) as u64;
        let hours = total_ms / 3_600_000;
        let minutes = (total_ms % 3_600_000) / 60_000;
        let secs = (total_ms % 60_000) / 1000;
        let ms = total_ms % 1000;
        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, secs, ms)
    }

    pub fn render(&mut self, ctx: &egui::Context, player: &crate::engine::Player) {
        if !self.is_open {
            return;
        }

        let mut open = self.is_open;
        egui::Window::new("📝 Subtitle Studio & Timeline Editor (Ctrl+T)")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(760.0, 520.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Subtitle Studio & AI Translation Engine")
                                .strong()
                                .size(16.0)
                                .color(Color32::from_rgb(255, 215, 100)),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.selectable_value(&mut self.active_tab, SubtitleStudioTab::TtsVoiceReader, "🗣️ TTS Reader");
                            ui.selectable_value(&mut self.active_tab, SubtitleStudioTab::TranslationEngine, "🌐 Live Translation");
                            ui.selectable_value(&mut self.active_tab, SubtitleStudioTab::TimelineEditor, "⏱️ Timeline Editor");
                        });
                    });

                    ui.separator();

                    match self.active_tab {
                        SubtitleStudioTab::TimelineEditor => {
                            self.render_timeline_editor(ui, player);
                        }
                        SubtitleStudioTab::TranslationEngine => {
                            self.render_translation_engine(ui);
                        }
                        SubtitleStudioTab::TtsVoiceReader => {
                            self.render_tts_reader(ui);
                        }
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!("Total Subtitle Cues: {}", self.cues.len()));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Close").clicked() {
                                self.is_open = false;
                            }
                        });
                    });
                });
            });

        self.is_open = open;
    }

    fn render_timeline_editor(&mut self, ui: &mut egui::Ui, player: &crate::engine::Player) {
        // Toolbar controls
        ui.horizontal(|ui| {
            ui.label("🔍 Filter:");
            ui.add(egui::TextEdit::singleline(&mut self.search_query).desired_width(140.0));

            ui.add_space(8.0);
            if ui.button("➕ Insert Cue").clicked() {
                let current_time = player.stats().time_pos;
                let new_id = self.cues.len() + 1;


                self.cues.push(SubtitleCue {
                    id: new_id,
                    start_time: current_time,
                    end_time: current_time + 3.0,
                    text: "New subtitle line".to_string(),
                });
                self.cues.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap_or(std::cmp::Ordering::Equal));
            }

            if ui.button("🗑️ Delete Selected").clicked() {
                if let Some(idx) = self.selected_cue_idx {
                    if idx < self.cues.len() {
                        self.cues.remove(idx);
                        self.selected_cue_idx = None;
                    }
                }
            }

            ui.add_space(8.0);
            ui.label("Shift Time:");
            if ui.button("-500ms").clicked() {
                self.shift_all_cues(-0.5);
            }
            if ui.button("-100ms").clicked() {
                self.shift_all_cues(-0.1);
            }
            if ui.button("+100ms").clicked() {
                self.shift_all_cues(0.1);
            }
            if ui.button("+500ms").clicked() {
                self.shift_all_cues(0.5);
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("💾 Export SRT").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("SubRip Subtitle", &["srt"])
                        .set_file_name("subtitles.srt")
                        .save_file()
                    {
                        let srt_content = self.generate_srt_string();
                        let _ = fs::write(&path, srt_content);
                    }
                }
            });
        });

        ui.add_space(4.0);

        // Subtitle cues list
        ScrollArea::vertical()
            .max_height(340.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let query = self.search_query.to_lowercase();
                let mut seek_to_time = None;

                for (idx, cue) in self.cues.iter_mut().enumerate() {
                    if !query.is_empty() && !cue.text.to_lowercase().contains(&query) {
                        continue;
                    }

                    let is_selected = self.selected_cue_idx == Some(idx);
                    let bg_color = if is_selected {
                        Color32::from_rgb(45, 55, 75)
                    } else if idx % 2 == 0 {
                        Color32::from_rgb(25, 26, 32)
                    } else {
                        Color32::from_rgb(20, 21, 26)
                    };

                    egui::Frame::new()
                        .fill(bg_color)
                        .inner_margin(egui::Margin::symmetric(6, 4))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                // Jump button
                                if ui.button(format!("▶ {}", Self::format_timestamp(cue.start_time))).clicked() {
                                    seek_to_time = Some(cue.start_time);
                                    self.selected_cue_idx = Some(idx);
                                }

                                ui.label(format!("→ {}", Self::format_timestamp(cue.end_time)));
                                ui.label(RichText::new(format!("({:.2}s)", (cue.end_time - cue.start_time).max(0.0))).small().color(Color32::GRAY));

                                // Editable text field
                                ui.add(egui::TextEdit::singleline(&mut cue.text).desired_width(ui.available_width() - 10.0));
                            });
                        });

                    ui.add_space(2.0);
                }

                if let Some(t) = seek_to_time {
                    player.seek_absolute(t);
                }

            });
    }

    fn render_translation_engine(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(RichText::new("Multi-Engine AI Machine Translation").strong().color(Color32::from_rgb(120, 200, 255)));
            ui.label(
                RichText::new("Connect external neural translation engines to automatically translate foreign subtitle lines in real time.")
                    .small()
                    .color(Color32::from_rgb(160, 160, 170)),
            );
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Translation Engine:");
                egui::ComboBox::from_id_salt("trans_engine_combo")
                    .selected_text(&self.translation_provider)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.translation_provider, "DeepL API".to_string(), "DeepL Pro / Free API");
                        ui.selectable_value(&mut self.translation_provider, "Google Cloud Translation".to_string(), "Google Cloud Translation v3");
                        ui.selectable_value(&mut self.translation_provider, "Microsoft Bing Translator".to_string(), "Microsoft Bing Cognitive Services");
                        ui.selectable_value(&mut self.translation_provider, "LibreTranslate (Self-Hosted)".to_string(), "LibreTranslate (Local / Open-Source)");
                        ui.selectable_value(&mut self.translation_provider, "Baidu Translate".to_string(), "Baidu Translate API");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("API Secret Key:");
                ui.add(egui::TextEdit::singleline(&mut self.translation_api_key).password(true).desired_width(260.0));
            });

            ui.horizontal(|ui| {
                ui.label("Target Language:");
                egui::ComboBox::from_id_salt("trans_target_lang")
                    .selected_text(&self.target_language)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.target_language, "English (en)".to_string(), "English (en)");
                        ui.selectable_value(&mut self.target_language, "Spanish (es)".to_string(), "Spanish (es)");
                        ui.selectable_value(&mut self.target_language, "French (fr)".to_string(), "French (fr)");
                        ui.selectable_value(&mut self.target_language, "German (de)".to_string(), "German (de)");
                        ui.selectable_value(&mut self.target_language, "Japanese (ja)".to_string(), "Japanese (ja)");
                        ui.selectable_value(&mut self.target_language, "Korean (ko)".to_string(), "Korean (ko)");
                        ui.selectable_value(&mut self.target_language, "Chinese Simplified (zh-CN)".to_string(), "Chinese (zh-CN)");
                    });
            });

            ui.add_space(6.0);
            ui.checkbox(&mut self.auto_translate_dual, "Display translated text as secondary dual subtitle overlay");
        });
    }

    fn render_tts_reader(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(RichText::new("Microsoft SAPI5 / Windows OneCore TTS Voice Reader").strong().color(Color32::from_rgb(180, 240, 140)));
            ui.label(
                RichText::new("Reads on-screen subtitles aloud in real-time using Windows hardware speech synthesis.")
                    .small()
                    .color(Color32::from_rgb(160, 160, 170)),
            );
            ui.add_space(8.0);

            ui.checkbox(&mut self.tts_enabled, "🔊 Enable Real-Time Text-to-Speech Subtitle Reading");

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Speech Voice:");
                egui::ComboBox::from_id_salt("tts_voice_combo")
                    .selected_text(&self.tts_voice)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.tts_voice, "Microsoft David".to_string(), "Microsoft David (English US)");
                        ui.selectable_value(&mut self.tts_voice, "Microsoft Zira".to_string(), "Microsoft Zira (English US)");
                        ui.selectable_value(&mut self.tts_voice, "Microsoft Mark".to_string(), "Microsoft Mark (English US)");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Speech Rate:");
                ui.add(egui::Slider::new(&mut self.tts_rate, -10..=10).text("rate"));
            });

            ui.horizontal(|ui| {
                ui.label("Speech Volume:");
                ui.add(egui::Slider::new(&mut self.tts_volume, 0..=100).text("%"));
            });

            ui.add_space(8.0);
            if ui.button("📢 Test Voice Synthesizer").clicked() {
                self.speak_text("VortexPlayer speech synthesis engine operational.");
            }
        });
    }

    fn shift_all_cues(&mut self, offset_seconds: f64) {
        for cue in &mut self.cues {
            cue.start_time = (cue.start_time + offset_seconds).max(0.0);
            cue.end_time = (cue.end_time + offset_seconds).max(cue.start_time + 0.1);
        }
    }

    fn generate_srt_string(&self) -> String {
        let mut out = String::new();
        for (i, cue) in self.cues.iter().enumerate() {
            out.push_str(&format!("{}\n", i + 1));
            let start_srt = Self::format_timestamp(cue.start_time).replace('.', ",");
            let end_srt = Self::format_timestamp(cue.end_time).replace('.', ",");
            out.push_str(&format!("{} --> {}\n", start_srt, end_srt));
            out.push_str(&format!("{}\n\n", cue.text));
        }
        out
    }

    pub fn speak_text(&self, text: &str) {
        #[cfg(windows)]
        {
            let text_escaped = text.replace('\'', "''");
            let voice_name = self.tts_voice.clone();
            let rate = self.tts_rate;
            let volume = self.tts_volume;
            let script = format!(
                "Add-Type -AssemblyName System.speech; $synth = New-Object System.Speech.Synthesis.SpeechSynthesizer; try {{ $synth.SelectVoice('{}') }} catch {{}}; $synth.Rate = {}; $synth.Volume = {}; $synth.Speak('{}');",
                voice_name, rate, volume, text_escaped
            );
            std::thread::spawn(move || {
                let _ = std::process::Command::new("powershell")
                    .args(["-NoProfile", "-NonInteractive", "-Command", &script])
                    .creation_flags(0x08000000) // CREATE_NO_WINDOW
                    .output();
            });
        }
        #[cfg(target_os = "macos")]
        {
            let text_owned = text.to_string();
            std::thread::spawn(move || {
                let _ = std::process::Command::new("say").arg(&text_owned).output();
            });
        }
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            let text_owned = text.to_string();
            std::thread::spawn(move || {
                let _ = std::process::Command::new("espeak").arg(&text_owned).output();
            });
        }
    }
}
