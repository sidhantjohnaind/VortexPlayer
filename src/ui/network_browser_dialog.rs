use super::theme::VortexTheme;
use crate::network::{NetworkProtocol, NetworkServer};
use eframe::egui::{self, Align2, Color32, CornerRadius, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use std::path::PathBuf;

pub struct NetworkBrowserDialog {
    pub is_open: bool,
    pub server_name: String,
    pub protocol: NetworkProtocol,
    pub host: String,
    pub port: u16,
    pub share_path: String,
    pub username: String,
    pub password: String,
    pub direct_url: String,
    pub saved_servers: Vec<NetworkServer>,
}

impl Default for NetworkBrowserDialog {
    fn default() -> Self {
        Self {
            is_open: false,
            server_name: "Home NAS".to_string(),
            protocol: NetworkProtocol::Smb,
            host: "192.168.1.100".to_string(),
            port: 445,
            share_path: "movies/video.mkv".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            direct_url: "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_string(),
            saved_servers: vec![
                NetworkServer {
                    name: "Local SMB Share".to_string(),
                    protocol: NetworkProtocol::Smb,
                    host: "192.168.1.100".to_string(),
                    port: 445,
                    share_path: "Media/Movies/sample.mkv".to_string(),
                    username: None,
                    password: None,
                },
                NetworkServer {
                    name: "WebDAV Server".to_string(),
                    protocol: NetworkProtocol::WebDav,
                    host: "192.168.1.50".to_string(),
                    port: 8080,
                    share_path: "webdav/video.mp4".to_string(),
                    username: None,
                    password: None,
                },
            ],
        }
    }
}

impl NetworkBrowserDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, file_to_open: &mut Option<PathBuf>) {
        if !self.is_open {
            return;
        }

        let mut open_flag = self.is_open;
        egui::Window::new("🌐 Network Media Browser & Cloud Streaming (Ctrl+N)")
            .open(&mut open_flag)
            .default_width(560.0)
            .default_height(460.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("NETWORK STREAMING & STORAGE SERVERS").strong().color(VortexTheme::current_skin().accent_primary));
                });
                ui.separator();

                // Direct Stream URL section
                ui.label(RichText::new("Direct Stream URL (HTTP, HLS, DASH, RTSP):").strong());
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.direct_url).desired_width(400.0));
                    if ui.button(RichText::new("▶ Stream").strong().color(VortexTheme::current_skin().accent_primary)).clicked() {
                        *file_to_open = Some(PathBuf::from(&self.direct_url));
                        self.is_open = false;
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                
                // Saved Servers List
                if !self.saved_servers.is_empty() {
                    ui.label(RichText::new("Saved Server Profiles:").strong().color(VortexTheme::VORTEX_CYAN));
                    egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                        let mut to_load = None;
                        for (i, srv) in self.saved_servers.iter().enumerate() {
                            ui.horizontal(|ui| {
                                let label_text = format!("{} ({}://{})", srv.name, match srv.protocol {
                                    NetworkProtocol::Smb => "smb",
                                    NetworkProtocol::WebDav => "webdav",
                                    NetworkProtocol::Ftp => "ftp",
                                    NetworkProtocol::Dlna => "dlna",
                                    NetworkProtocol::UncShare => "unc",
                                }, srv.host);
                                if ui.selectable_label(false, label_text).clicked() {
                                    to_load = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = to_load {
                            let s = &self.saved_servers[idx];
                            self.server_name = s.name.clone();
                            self.protocol = s.protocol.clone();
                            self.host = s.host.clone();
                            self.port = s.port;
                            self.share_path = s.share_path.clone();
                            self.username = s.username.clone().unwrap_or_default();
                            self.password = s.password.clone().unwrap_or_default();
                        }
                    });
                    ui.add_space(6.0);
                    ui.separator();
                }

                ui.label(RichText::new("Server Connection Details:").strong().color(VortexTheme::current_skin().accent_primary));

                egui::Grid::new("net_grid").spacing(Vec2::new(12.0, 8.0)).show(ui, |ui| {
                    ui.label("Profile Name:");
                    ui.text_edit_singleline(&mut self.server_name);
                    ui.end_row();

                    ui.label("Protocol:");
                    egui::ComboBox::from_id_salt("net_proto")
                        .selected_text(match self.protocol {
                            NetworkProtocol::Smb => "SMB / Windows Share",
                            NetworkProtocol::WebDav => "WebDAV",
                            NetworkProtocol::Ftp => "FTP Server",
                            NetworkProtocol::Dlna => "DLNA / UPnP",
                            NetworkProtocol::UncShare => "UNC Share (\\\\server\\share)",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.protocol, NetworkProtocol::Smb, "SMB / Windows Share");
                            ui.selectable_value(&mut self.protocol, NetworkProtocol::WebDav, "WebDAV");
                            ui.selectable_value(&mut self.protocol, NetworkProtocol::Ftp, "FTP Server");
                            ui.selectable_value(&mut self.protocol, NetworkProtocol::Dlna, "DLNA / UPnP");
                            ui.selectable_value(&mut self.protocol, NetworkProtocol::UncShare, "UNC Share");
                        });
                    ui.end_row();

                    ui.label("Host / IP:");
                    ui.text_edit_singleline(&mut self.host);
                    ui.end_row();

                    ui.label("Port:");
                    ui.add(egui::DragValue::new(&mut self.port).range(1..=65535));
                    ui.end_row();

                    ui.label("Path / Share / File:");
                    ui.text_edit_singleline(&mut self.share_path);
                    ui.end_row();

                    ui.label("Username (Optional):");
                    ui.text_edit_singleline(&mut self.username);
                    ui.end_row();

                    ui.label("Password (Optional):");
                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                    ui.end_row();
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("💾 Save Profile").clicked() {
                        let srv = NetworkServer {
                            name: self.server_name.clone(),
                            protocol: self.protocol.clone(),
                            host: self.host.clone(),
                            port: self.port,
                            share_path: self.share_path.clone(),
                            username: if self.username.is_empty() { None } else { Some(self.username.clone()) },
                            password: if self.password.is_empty() { None } else { Some(self.password.clone()) },
                        };
                        self.saved_servers.push(srv);
                    }

                    if ui.button(RichText::new("⚡ Connect & Play").strong().color(Color32::WHITE)).clicked() {
                        let srv = NetworkServer {
                            name: self.server_name.clone(),
                            protocol: self.protocol.clone(),
                            host: self.host.clone(),
                            port: self.port,
                            share_path: self.share_path.clone(),
                            username: if self.username.is_empty() { None } else { Some(self.username.clone()) },
                            password: if self.password.is_empty() { None } else { Some(self.password.clone()) },
                        };
                        let url = srv.build_url();
                        *file_to_open = Some(PathBuf::from(url));
                        self.is_open = false;
                    }
                });
            });
        self.is_open = open_flag;
    }
}
