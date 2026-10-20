use super::theme::VortexTheme;
use eframe::egui::{self, Color32, RichText};
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileNavigatorSidebar {
    pub is_open: bool,
    pub current_dir: PathBuf,
    pub files: Vec<PathBuf>,
}

impl Default for FileNavigatorSidebar {
    fn default() -> Self {
        Self {
            is_open: false,
            current_dir: PathBuf::new(),
            files: Vec::new(),
        }
    }
}

impl FileNavigatorSidebar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_directory(&mut self, dir: &Path) {
        self.current_dir = dir.to_path_buf();
        self.refresh();
    }

    pub fn refresh(&mut self) {
        self.files.clear();
        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    self.files.push(p);
                }
            }
        }
        self.files.sort_by(|a, b| natord::compare(&a.to_string_lossy(), &b.to_string_lossy()));
    }

    pub fn render(&mut self, ctx: &egui::Context, file_to_open: &mut Option<PathBuf>) {
        if !self.is_open {
            return;
        }
        if self.current_dir.as_os_str().is_empty() {
            self.current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            self.refresh();
        }

        let mut open_flag = self.is_open;
        let viewport_id = egui::ViewportId::from_hash_of("vortex_file_navigator_window");
        let builder = egui::ViewportBuilder::default()
            .with_title("File Explorer Navigator")
            .with_inner_size([420.0, 560.0])
            .with_min_inner_size([320.0, 400.0])
            .with_resizable(true)
            .with_decorations(true);

        ctx.show_viewport_immediate(viewport_id, builder, |ctx, _class| {
            if ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                open_flag = false;
            }
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::new()
                        .fill(Color32::from_rgb(14, 16, 22))
                        .inner_margin(egui::Margin::same(12)),
                )
                .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("⬆ Parent").clicked() {
                        if let Some(parent) = self.current_dir.parent() {
                            self.current_dir = parent.to_path_buf();
                            self.refresh();
                        }
                    }
                    ui.label(RichText::new(self.current_dir.file_name().unwrap_or_default().to_string_lossy()).strong().color(VortexTheme::current_skin().accent_primary));
                });
                ui.separator();

                egui::ScrollArea::vertical().id_salt("file_navigator_scroll").show(ui, |ui| {
                    for f in &self.files {
                        let name = f.file_name().unwrap_or_default().to_string_lossy();
                        if ui.selectable_label(false, &*name).clicked() {
                            *file_to_open = Some(f.clone());
                        }
                    }
                });
            });
        });
        self.is_open = open_flag;
    }
}
