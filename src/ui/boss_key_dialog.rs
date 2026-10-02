//! boss_key_dialog.rs — Boss Key & Instant Privacy Mute/Hide Studio (Vortex/KMPlayer style)

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossKeyAction {
    MinimizeToTray,
    MuteAndBlank,
    MinimizeWindow,
    ClosePlayer,
}

impl BossKeyAction {
    pub fn display_name(&self) -> &'static str {
        match self {
            BossKeyAction::MinimizeToTray => "Minimize to Notification Tray & Mute",
            BossKeyAction::MuteAndBlank => "Mute Audio & Show Black Screen",
            BossKeyAction::MinimizeWindow => "Minimize to Taskbar & Pause",
            BossKeyAction::ClosePlayer => "Emergency Close App Immediately",
        }
    }
}

pub struct BossKeyDialog {
    pub is_open: bool,
    pub is_triggered: bool,
    pub action: BossKeyAction,
    pub auto_pause_on_hide: bool,
    pub auto_mute_on_hide: bool,
    pub restore_unmute: bool,
    pub key_shortcut_desc: String,
    pub status_message: String,
}

impl Default for BossKeyDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            is_triggered: false,
            action: BossKeyAction::MinimizeToTray,
            auto_pause_on_hide: true,
            auto_mute_on_hide: true,
            restore_unmute: true,
            key_shortcut_desc: "Ctrl+Alt+B or ESC".to_string(),
            status_message: "Boss key armed. Press shortcut to instantly conceal player.".to_string(),
        }
    }
}

impl BossKeyDialog {
    pub fn new() -> Self { Self::default() }

    pub fn trigger_boss_key(&mut self, player: &crate::engine::Player, ctx: &egui::Context) {
        self.is_triggered = true;
        if self.auto_mute_on_hide {
            player.set_property_bool("mute", true);
        }
        if self.auto_pause_on_hide {
            player.set_property_bool("pause", true);
        }
        match self.action {
            BossKeyAction::MinimizeToTray | BossKeyAction::MinimizeWindow => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
            BossKeyAction::ClosePlayer => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            BossKeyAction::MuteAndBlank => {
                self.status_message = "Screen blanked and audio muted.".to_string();
            }
        }
    }

    pub fn render(&mut self, ctx: &egui::Context) {
        if !self.is_open { return; }
        let mut open = self.is_open;
        egui::Window::new("🛡️ Boss Key & Privacy Disguise")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size(Vec2::new(520.0, 340.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Boss Key Privacy & Panic Concealer").strong().size(15.0).color(Color32::from_rgb(255, 120, 120)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new("● ARMED").color(Color32::from_rgb(60, 220, 100)).strong());
                    });
                });
                ui.separator();

                ui.group(|ui| {
                    ui.label(RichText::new("Panic Action:").strong());
                    for act in [BossKeyAction::MinimizeToTray, BossKeyAction::MuteAndBlank, BossKeyAction::MinimizeWindow, BossKeyAction::ClosePlayer] {
                        if ui.selectable_label(self.action == act, act.display_name()).clicked() {
                            self.action = act;
                        }
                    }
                });

                ui.add_space(4.0);
                ui.group(|ui| {
                    ui.label(RichText::new("Privacy Protection Options").strong());
                    ui.checkbox(&mut self.auto_pause_on_hide, "Auto-Pause playback on panic");
                    ui.checkbox(&mut self.auto_mute_on_hide, "Instant Mute (Silence all audio)");
                    ui.checkbox(&mut self.restore_unmute, "Auto-Restore volume and unpause upon reopening");
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Default Trigger:").color(Color32::GRAY));
                    ui.label(RichText::new("Ctrl+Alt+K (Customizable)").strong().color(Color32::from_rgb(255, 200, 100)));
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
