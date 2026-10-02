use super::theme::VortexTheme;
use crate::config::hierarchy::{LayeredConfigState, SettingSource};
use eframe::egui::{self, Color32, Rect, RichText};

pub struct SettingsInspectorDialog {
    pub is_open: bool,
    pub state: LayeredConfigState,
}

impl Default for SettingsInspectorDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            state: LayeredConfigState::new(),
        }
    }
}

impl SettingsInspectorDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context) -> Option<Rect> {
        if !self.is_open {
            return None;
        }

        let mut open_flag = self.is_open;
        let resp = egui::Window::new("🔍 Settings Inspector & Layer Provenance")
            .open(&mut open_flag)
            .default_width(620.0)
            .default_height(420.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("EFFECTIVE SETTINGS & ORIGIN HIERARCHY").strong().color(VortexTheme::VORTEX_YELLOW));
                    ui.label(RichText::new("(Global → Profile → Folder → File)").color(Color32::from_rgb(140, 140, 160)));
                });
                ui.separator();

                egui::Grid::new("inspector_grid")
                    .striped(true)
                    .num_columns(4)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Setting Parameter").strong());
                        ui.label(RichText::new("Effective Value").strong());
                        ui.label(RichText::new("Active Layer").strong());
                        ui.label(RichText::new("Action").strong());
                        ui.end_row();

                        // Row 1: Subtitle Delay
                        ui.label("Subtitle Delay");
                        ui.label(format!("{:.3} s", self.state.subtitle_delay.value));
                        Self::render_badge(ui, self.state.subtitle_delay.source, &self.state.subtitle_delay.source_name);
                        if self.state.subtitle_delay.source != SettingSource::Global {
                            if ui.button("Reset").clicked() {
                                self.state.subtitle_delay.source = SettingSource::Global;
                                self.state.subtitle_delay.value = 0.0;
                            }
                        } else {
                            ui.label("—");
                        }
                        ui.end_row();

                        // Row 2: Aspect Ratio
                        ui.label("Aspect Ratio");
                        ui.label(&self.state.aspect_ratio.value);
                        Self::render_badge(ui, self.state.aspect_ratio.source, &self.state.aspect_ratio.source_name);
                        if self.state.aspect_ratio.source != SettingSource::Global {
                            if ui.button("Reset").clicked() {
                                self.state.aspect_ratio.source = SettingSource::Global;
                                self.state.aspect_ratio.value = "auto".to_string();
                            }
                        } else {
                            ui.label("—");
                        }
                        ui.end_row();

                        // Row 3: EQ Preset
                        ui.label("Equalizer Preset");
                        ui.label(&self.state.eq_preset.value);
                        Self::render_badge(ui, self.state.eq_preset.source, &self.state.eq_preset.source_name);
                        if self.state.eq_preset.source != SettingSource::Global {
                            if ui.button("Reset").clicked() {
                                self.state.eq_preset.source = SettingSource::Global;
                                self.state.eq_preset.value = "Flat".to_string();
                            }
                        } else {
                            ui.label("—");
                        }
                        ui.end_row();

                        // Row 4: Hardware Decoder
                        ui.label("Hardware Decoder");
                        ui.label(&self.state.hwdec_mode.value);
                        Self::render_badge(ui, self.state.hwdec_mode.source, &self.state.hwdec_mode.source_name);
                        ui.label("—");
                        ui.end_row();
                    });
            });
        self.is_open = open_flag;
        resp.map(|r| r.response.rect)
    }

    fn render_badge(ui: &mut egui::Ui, source: SettingSource, name: &str) {
        let (col, label) = match source {
            SettingSource::Global => (Color32::from_rgb(100, 100, 140), format!("Global ({})", name)),
            SettingSource::Profile => (Color32::from_rgb(50, 140, 240), format!("Profile: {}", name)),
            SettingSource::Folder => (Color32::from_rgb(220, 160, 40), format!("Folder: {}", name)),
            SettingSource::File => (Color32::from_rgb(220, 50, 80), format!("File Override")),
        };
        ui.colored_label(col, label);
    }
}
