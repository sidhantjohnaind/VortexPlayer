use super::theme::VortexTheme;
use crate::config::bindings::InputTrigger;
use crate::config::input_profiles::{InputProfileStore, InputProfileType};
use eframe::egui::{self, Color32, RichText};

pub struct InputEditorDialog {
    pub is_open: bool,
    pub search_filter: String,
    pub recording_index: Option<usize>,
}

impl Default for InputEditorDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            search_filter: String::new(),
            recording_index: None,
        }
    }
}

impl InputEditorDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, store: &mut InputProfileStore) {
        if !self.is_open {
            return;
        }

        let mut open_flag = self.is_open;
        egui::Window::new("⌨ Input Shortcuts & Gesture Binding Editor (Ctrl+K)")
            .open(&mut open_flag)
            .default_width(720.0)
            .default_height(500.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("ACTIVE PROFILE:").strong().color(VortexTheme::current_skin().accent_primary));
                    egui::ComboBox::from_id_salt("profile_select")
                        .selected_text(store.active_profile.display_name())
                        .show_ui(ui, |ui| {
                            for p in InputProfileType::ALL {
                                ui.selectable_value(&mut store.active_profile, p, p.display_name());
                            }
                        });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.text_edit_singleline(&mut self.search_filter);
                        ui.label("🔍 Search Action:");
                    });
                });
                ui.separator();

                if let Some(profile_set) = store.profiles.get_mut(&store.active_profile) {
                    egui::ScrollArea::vertical().id_salt("input_editor_scroll").show(ui, |ui| {
                        egui::Grid::new("bindings_table")
                            .striped(true)
                            .num_columns(4)
                            .spacing([20.0, 8.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("Action Description").strong());
                                ui.label(RichText::new("Input Trigger").strong());
                                ui.label(RichText::new("Command ID").strong());
                                ui.label(RichText::new("Edit").strong());
                                ui.end_row();

                                for (i, binding) in profile_set.bindings.iter_mut().enumerate() {
                                    let matches = self.search_filter.is_empty()
                                        || binding.description.to_lowercase().contains(&self.search_filter.to_lowercase())
                                        || binding.trigger.display_str().to_lowercase().contains(&self.search_filter.to_lowercase());

                                    if matches {
                                        ui.label(&binding.description);
                                        
                                        if self.recording_index == Some(i) {
                                            ui.label(RichText::new("🔴 Press key...").color(Color32::from_rgb(220, 50, 50)).strong());
                                        } else {
                                            ui.label(RichText::new(binding.trigger.display_str()).monospace().color(VortexTheme::current_skin().accent_primary));
                                        }

                                        ui.label(RichText::new(format!("{:?}", binding.command)).monospace().color(Color32::from_rgb(140, 140, 160)));

                                        ui.horizontal(|ui| {
                                            if self.recording_index == Some(i) {
                                                if ui.button("Cancel").clicked() {
                                                    self.recording_index = None;
                                                }
                                            } else {
                                                if ui.button("Rebind").clicked() {
                                                    self.recording_index = Some(i);
                                                }
                                            }
                                        });
                                        ui.end_row();
                                    }
                                }
                            });
                    });
                }
            });
        self.is_open = open_flag;
    }
}
