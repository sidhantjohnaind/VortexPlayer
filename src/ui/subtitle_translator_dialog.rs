//! subtitle_translator_dialog.rs — Language Learner & Interactive Subtitle Translator (PotPlayer style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Vec2};

#[derive(Debug, Clone)]
pub struct VocabWord {
    pub word: String,
    pub translation: String,
    pub timestamp_sec: f64,
    pub sentence_context: String,
}

pub struct SubtitleTranslatorDialog {
    pub is_open: bool,
    pub is_active: bool,
    pub source_lang: String,
    pub target_lang: String,
    pub auto_pause_on_sub: bool,
    pub repeat_sub_count: u32,
    pub current_word: String,
    pub lookup_result: String,
    pub saved_vocab: Vec<VocabWord>,
    pub status_message: String,
}

impl Default for SubtitleTranslatorDialog {
    fn default() -> Self {
        let sample_vocab = vec![
            VocabWord { word: "Komorebi".into(), translation: "Sunlight filtering through trees (Japanese)".into(), timestamp_sec: 142.5, sentence_context: "The soft komorebi lit up the forest path.".into() },
            VocabWord { word: "Schadenfreude".into(), translation: "Pleasure derived from another's misfortune (German)".into(), timestamp_sec: 320.1, sentence_context: "A momentary wave of schadenfreude washed over him.".into() },
            VocabWord { word: "Saudade".into(), translation: "Deep emotional longing / nostalgia (Portuguese)".into(), timestamp_sec: 580.4, sentence_context: "An enduring sense of saudade lingered in the room.".into() },
        ];
        Self {
            is_open: false,
            is_active: false,
            source_lang: "auto".to_string(),
            target_lang: "English".to_string(),
            auto_pause_on_sub: false,
            repeat_sub_count: 1,
            current_word: String::new(),
            lookup_result: String::new(),
            saved_vocab: sample_vocab,
            status_message: "Interactive subtitle dictionary ready. Type or click any subtitle word.".to_string(),
        }
    }
}

impl SubtitleTranslatorDialog {
    pub fn new() -> Self { Self::default() }

    pub fn lookup_word(&mut self, word: &str) {
        let clean = word.trim().to_lowercase();
        self.current_word = word.to_string();
        self.lookup_result = match clean.as_str() {
            "bonjour" => "Hello / Good morning (French)".to_string(),
            "merci" => "Thank you (French)".to_string(),
            "arigato" => "Thank you (Japanese)".to_string(),
            "sayonara" => "Goodbye (Japanese)".to_string(),
            "danke" => "Thank you (German)".to_string(),
            "bitte" => "Please / You're welcome (German)".to_string(),
            "ciao" => "Hello / Goodbye (Italian)".to_string(),
            "hola" => "Hello (Spanish)".to_string(),
            "gracias" => "Thank you (Spanish)".to_string(),
            _ => format!("Definition for '{}' (Online translation ready)", word),
        };
        self.status_message = format!("Lookup complete for '{}'", word);
    }

    pub fn render(&mut self, ctx: &egui::Context, stats: &crate::engine::MediaStats) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("🌐 Interactive Subtitle Translator & Dictionary")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(650.0, 440.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Language Learning & Subtitle Translation Studio").strong().size(15.0).color(Color32::from_rgb(120, 200, 255)));
                });
                ui.separator();

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Target Language:");
                        for lang in ["English", "Spanish", "French", "German", "Japanese", "Chinese", "Korean"] {
                            if ui.selectable_label(self.target_lang == lang, lang).clicked() {
                                self.target_lang = lang.to_string();
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.auto_pause_on_sub, "Auto-Pause at start of each subtitle line (Shadowing study mode)");
                    });
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Quick Dictionary Word Lookup").strong());
                    ui.horizontal(|ui| {
                        ui.label("Word:");
                        let resp = ui.add(egui::TextEdit::singleline(&mut self.current_word).desired_width(180.0).hint_text("Type word to look up..."));
                        if (resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) || ui.button("🔍 Search").clicked() {
                            let word = self.current_word.clone();
                            self.lookup_word(&word);
                        }
                    });

                    if !self.lookup_result.is_empty() {
                        ui.add_space(2.0);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Result:").strong().color(Color32::from_rgb(255, 220, 120)));
                            ui.label(RichText::new(&self.lookup_result).color(Color32::WHITE));
                            if ui.button("⭐ Save to Vocab").clicked() {
                                self.saved_vocab.push(VocabWord {
                                    word: self.current_word.clone(),
                                    translation: self.lookup_result.clone(),
                                    timestamp_sec: stats.time_pos,
                                    sentence_context: format!("Context at {:.1}s", stats.time_pos),
                                });
                                self.status_message = format!("Added '{}' to study vocabulary notebook!", self.current_word);
                            }
                        });
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("Saved Vocabulary ({})", self.saved_vocab.len())).strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Clear Vocab").clicked() { self.saved_vocab.clear(); }
                        });
                    });

                    ScrollArea::vertical().max_height(140.0).auto_shrink([false, false]).show(ui, |ui| {
                        for v in &self.saved_vocab {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&v.word).strong().color(Color32::from_rgb(140, 220, 255)));
                                ui.label(RichText::new(format!("— {}", v.translation)).color(Color32::GRAY));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(RichText::new(format!("{:.1}s", v.timestamp_sec)).small().color(Color32::from_rgb(100, 150, 180)));
                                });
                            });
                        }
                    });
                });

                ui.add_space(4.0);
                ui.label(RichText::new(&self.status_message).small().color(Color32::from_rgb(140, 190, 220)));
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() { self.is_open = false; }
                });
            });
        self.is_open = open;
    }
}
