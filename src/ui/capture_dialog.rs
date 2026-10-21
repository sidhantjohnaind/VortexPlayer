use super::theme::VortexTheme;
use crate::capture::{BurstCaptureEngine, CaptureConfig, CaptureFormat};
use crate::engine::Player;
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};

pub struct CaptureDialog {
    pub active_tab: String,
    pub is_recording: bool,
}

impl Default for CaptureDialog {
    fn default() -> Self {
        Self {
            active_tab: "Burst Snapshot".to_string(),
            is_recording: false,
        }
    }
}

impl CaptureDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        config: &mut CaptureConfig,
        burst: &mut BurstCaptureEngine,
        player: &Player,
    ) -> Option<Rect> {
        if !*is_open {
            return None;
        }

        let resp = egui::Window::new("Capture & Recording Studio (Ctrl+S)")
            .open(is_open)
            .resizable(false)
            .default_width(520.0)
            .default_height(380.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let tabs = ["Burst Snapshot", "Contact Sheet", "Video & Stream Recording"];
                    for t in tabs {
                        let is_sel = self.active_tab == t;
                        let color = if is_sel { VortexTheme::current_skin().accent_primary } else { VortexTheme::current_skin().text_secondary };
                        let bg = if is_sel { VortexTheme::current_skin().bg_btn_active } else { Color32::TRANSPARENT };
                        if ui.add(egui::Button::new(RichText::new(t).strong().color(color)).fill(bg)).clicked() {
                            self.active_tab = t.to_string();
                        }
                    }
                });

                ui.separator();

                match self.active_tab.as_str() {
                    "Burst Snapshot" => {
                        ui.label(RichText::new("Continuous Burst Capture").strong().color(VortexTheme::current_skin().accent_primary));
                        ui.horizontal(|ui| {
                            ui.label("Format:");
                            for fmt in CaptureFormat::ALL {
                                ui.radio_value(&mut config.format, fmt, format!("{:?}", fmt));
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Capture Interval (Seconds):");
                            ui.add(egui::Slider::new(&mut config.burst_interval_secs, 0.5..=10.0).suffix("s"));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Save Directory:");
                            ui.label(RichText::new(config.output_dir.to_string_lossy()).monospace().size(10.5));
                            if ui.button("Browse...").clicked() {
                                if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                                    config.output_dir = dir;
                                }
                            }
                        });

                        ui.separator();
                        let btn_text = if burst.is_running { "⏹ Stop Burst Capture" } else { "▶ Start Burst Capture" };
                        let btn_col = if burst.is_running { Color32::from_rgb(200, 40, 40) } else { VortexTheme::current_skin().accent_primary };
                        if ui.add(egui::Button::new(RichText::new(btn_text).color(btn_col).strong()).min_size(Vec2::new(180.0, 28.0))).clicked() {
                            burst.toggle();
                        }
                        if burst.is_running {
                            ui.label(RichText::new(format!("Burst Active — Captured {} frames", burst.frame_counter)).color(VortexTheme::VORTEX_LIME_OSD));
                        }
                    }
                    "Contact Sheet" => {
                        ui.label(RichText::new("Thumbnail Sheet / Contact Sheet").strong().color(VortexTheme::current_skin().accent_primary));
                        ui.label("Generates a structured grid of preview frames across the video duration.");
                        ui.horizontal(|ui| {
                            ui.label("Grid Columns:");
                            ui.add(egui::DragValue::new(&mut config.contact_sheet_cols).range(2..=8));
                            ui.label("Rows:");
                            ui.add(egui::DragValue::new(&mut config.contact_sheet_rows).range(2..=8));
                        });

                        if ui.button("📸 Generate 4×4 Contact Sheet").clicked() {
                            // Trigger contact sheet generator
                        }
                    }
                    "Video & Stream Recording" => {
                        ui.label(RichText::new("Direct Playback & Stream Recorder").strong().color(VortexTheme::current_skin().accent_primary));
                        ui.horizontal(|ui| {
                            ui.label("Encoding Engine:");
                            for codec in &["copy (lossless direct stream)", "hevc_nvenc (NVIDIA 10-bit)", "h264_qsv (Intel)"] {
                                ui.selectable_value(&mut config.recording_codec, codec.to_string(), *codec);
                            }
                        });

                        ui.separator();
                        let rec_text = if self.is_recording { "⏹ Stop Recording" } else { "⏺ Start Video Recording" };
                        let rec_col = if self.is_recording { Color32::from_rgb(200, 40, 40) } else { Color32::from_rgb(40, 180, 80) };
                        if ui.add(egui::Button::new(RichText::new(rec_text).color(rec_col).strong()).min_size(Vec2::new(200.0, 30.0))).clicked() {
                            self.is_recording = !self.is_recording;
                            if self.is_recording {
                                let out_path = config.output_dir.join(format!("Record_{}.mp4", chrono::Utc::now().format("%Y%m%d_%H%M%S")));
                                player.command(&["stream-record", &out_path.to_string_lossy()]);
                            } else {
                                player.command(&["stream-record", ""]);
                            }
                        }
                    }
                    _ => {}
                }
            });
        resp.map(|r| r.response.rect)
    }
}
