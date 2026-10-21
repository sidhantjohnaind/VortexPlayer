use super::theme::VortexTheme;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, Margin, Rect, RichText, ScrollArea, Stroke,
    Vec2, Window,
};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub const CURRENT_VERSION: &str = "v1.1.0";

#[derive(Debug, Clone)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub html_url: String,
    #[allow(dead_code)]
    pub published_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate(String),
    Available(String),
    Error(String),
}

pub struct UpdateDialog {
    pub status: UpdateStatus,
    pub release: Option<ReleaseInfo>,
    rx: Option<Receiver<Result<ReleaseInfo, String>>>,
}

impl Default for UpdateDialog {
    fn default() -> Self {
        Self {
            status: UpdateStatus::Idle,
            release: None,
            rx: None,
        }
    }
}

impl UpdateDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check_for_updates(&mut self) {
        self.status = UpdateStatus::Checking;
        self.release = None;

        let (tx, rx): (Sender<Result<ReleaseInfo, String>>, Receiver<Result<ReleaseInfo, String>>) = channel();
        self.rx = Some(rx);

        thread::spawn(move || {
            let res = Self::fetch_latest_release();
            let _ = tx.send(res);
        });
    }

    fn fetch_latest_release() -> Result<ReleaseInfo, String> {
        let url = "https://api.github.com/repos/sidhantjohnaind/VortexPlayer/releases/latest";

        #[cfg(windows)]
        {
            // Use PowerShell Invoke-RestMethod for native TLS and zero extra heavy deps
            let cmd_str = format!(
                "$ProgressPreference='SilentlyContinue'; [Net.ServicePointManager]::SecurityProtocol=[Net.SecurityProtocolType]::Tls12; $r = Invoke-RestMethod -Uri '{}' -Headers @{{'User-Agent'='VortexPlayer-Updater'}}; $r | ConvertTo-Json -Depth 4",
                url
            );
            let output = std::process::Command::new("powershell")
                .args(["-NoProfile", "-NonInteractive", "-Command", &cmd_str])
                .output()
                .map_err(|e| format!("Network request failed: {}", e))?;

            if !output.status.success() {
                return Err("Failed to query GitHub Releases API (rate limit or offline).".to_string());
            }

            let text = String::from_utf8_lossy(&output.stdout);
            let json: serde_json::Value = serde_json::from_str(&text)
                .map_err(|e| format!("Invalid JSON response: {}", e))?;

            let tag = json["tag_name"].as_str().unwrap_or("").to_string();
            let name = json["name"].as_str().unwrap_or("").to_string();
            let body = json["body"].as_str().unwrap_or("").to_string();
            let html_url = json["html_url"].as_str().unwrap_or("https://github.com/sidhantjohnaind/VortexPlayer/releases").to_string();
            let published_at = json["published_at"].as_str().unwrap_or("").to_string();

            if tag.is_empty() {
                return Err("No release tag found in GitHub response.".to_string());
            }

            Ok(ReleaseInfo {
                tag_name: tag,
                name,
                body,
                html_url,
                published_at,
            })
        }

        #[cfg(not(windows))]
        {
            let output = std::process::Command::new("curl")
                .args(["-s", "-H", "User-Agent: VortexPlayer-Updater", url])
                .output()
                .map_err(|e| format!("curl command failed: {}", e))?;

            let text = String::from_utf8_lossy(&output.stdout);
            let json: serde_json::Value = serde_json::from_str(&text)
                .map_err(|e| format!("JSON parsing failed: {}", e))?;

            let tag = json["tag_name"].as_str().unwrap_or("").to_string();
            let name = json["name"].as_str().unwrap_or("").to_string();
            let body = json["body"].as_str().unwrap_or("").to_string();
            let html_url = json["html_url"].as_str().unwrap_or("https://github.com/sidhantjohnaind/VortexPlayer/releases").to_string();
            let published_at = json["published_at"].as_str().unwrap_or("").to_string();

            if tag.is_empty() {
                return Err("No release tag found.".to_string());
            }

            Ok(ReleaseInfo {
                tag_name: tag,
                name,
                body,
                html_url,
                published_at,
            })
        }
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
    ) -> Option<Rect> {
        if !*is_open {
            return None;
        }

        // Poll background thread
        if let Some(ref rx) = self.rx {
            if let Ok(res) = rx.try_recv() {
                match res {
                    Ok(info) => {
                        let is_newer = info.tag_name != CURRENT_VERSION;
                        if is_newer {
                            self.status = UpdateStatus::Available(info.tag_name.clone());
                        } else {
                            self.status = UpdateStatus::UpToDate(info.tag_name.clone());
                        }
                        self.release = Some(info);
                    }
                    Err(err) => {
                        self.status = UpdateStatus::Error(err);
                    }
                }
                self.rx = None;
            }
        }

        let accent = VortexTheme::current_skin().accent_primary;
        let mut do_close = false;

        let resp = Window::new("✨ Check for Updates")
            .open(is_open)
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_size(Vec2::new(440.0, 340.0))
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(18, 20, 26))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(45, 52, 68)))
                    .corner_radius(CornerRadius::same(8))
                    .inner_margin(Margin::same(16)),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("VortexPlayer Software Update")
                                .font(egui::FontId::proportional(15.0))
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("Installed: {}", CURRENT_VERSION))
                                    .font(egui::FontId::monospace(11.0))
                                    .color(Color32::from_rgb(150, 160, 180)),
                            );
                        });
                    });

                    ui.add_space(8.0);

                    match &self.status {
                        UpdateStatus::Idle => {
                            ui.label("Click 'Check Now' to inspect GitHub for new releases.");
                            ui.add_space(12.0);
                            if ui.button(RichText::new("Check Now").color(Color32::WHITE)).clicked() {
                                self.check_for_updates();
                            }
                        }
                        UpdateStatus::Checking => {
                            ui.horizontal(|ui| {
                                ui.spinner();
                                ui.label(
                                    RichText::new("Connecting to GitHub Releases API...")
                                        .color(Color32::from_rgb(180, 190, 210)),
                                );
                            });
                        }
                        UpdateStatus::UpToDate(tag) => {
                            ui.group(|ui| {
                                ui.set_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("✓").strong().color(Color32::from_rgb(80, 220, 120)));
                                    ui.label(
                                        RichText::new(format!("VortexPlayer is up to date ({})!", tag))
                                            .strong()
                                            .color(Color32::WHITE),
                                    );
                                });
                                ui.label(
                                    RichText::new("You are currently running the newest release with all features.")
                                        .color(Color32::from_rgb(160, 170, 190))
                                        .font(egui::FontId::proportional(11.5)),
                                );
                            });
                        }
                        UpdateStatus::Available(tag) => {
                            ui.group(|ui| {
                                ui.set_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("🎉").color(accent));
                                    ui.label(
                                        RichText::new(format!("New Version Available: {}", tag))
                                            .strong()
                                            .color(Color32::WHITE),
                                    );
                                });

                                if let Some(ref rel) = self.release {
                                    ui.label(RichText::new(&rel.name).strong().color(accent));
                                    ui.add_space(4.0);
                                    ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                                        ui.label(RichText::new(&rel.body).color(Color32::from_rgb(190, 200, 220)).font(egui::FontId::proportional(11.0)));
                                    });
                                }
                            });
                        }
                        UpdateStatus::Error(err) => {
                            ui.group(|ui| {
                                ui.set_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("⚠").color(Color32::from_rgb(255, 120, 120)));
                                    ui.label(RichText::new("Update Check Failed").strong().color(Color32::WHITE));
                                });
                                ui.label(RichText::new(err).color(Color32::from_rgb(255, 160, 160)).font(egui::FontId::proportional(11.0)));
                            });
                        }
                    }

                    ui.add_space(14.0);

                    // Action buttons
                    ui.horizontal(|ui| {
                        if let UpdateStatus::Available(_) = self.status {
                            if let Some(ref rel) = self.release {
                                let download_btn = egui::Button::new(
                                    RichText::new("Download Update (GitHub)").color(Color32::WHITE).strong(),
                                )
                                .fill(accent);
                                if ui.add(download_btn).clicked() {
                                    crate::ui::subtitle_lookup_dialog::SubtitleLookupDialog::open_url(&rel.html_url);
                                }
                            }
                        }

                        if self.status != UpdateStatus::Checking {
                            if ui.button("Check Again").clicked() {
                                self.check_for_updates();
                            }
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Close").clicked() {
                                do_close = true;
                            }
                        });
                    });
                });
            });

        if do_close {
            *is_open = false;
        }

        resp.map(|r| r.response.rect)
    }
}
