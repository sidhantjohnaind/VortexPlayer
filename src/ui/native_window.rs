use eframe::egui::{self, Ui, Vec2, ViewportBuilder, ViewportId};

/// Render a popup of the player in its own native OS window.
///
/// The popups used to be drawn as overlays *inside* the player window. That
/// meant the title bar and the control bar of the player could cover them and
/// they could never be moved to a second monitor. Every popup is a real
/// window now: it gets its own taskbar entry, its own title bar and it can be
/// dragged anywhere.
///
/// `open` is cleared when the user closes the native window (via the window's
/// close button), so the caller can hide the popup again.
pub fn show_native_window(
    ctx: &egui::Context,
    key: &str,
    title: &str,
    size: Vec2,
    scroll: bool,
    open: &mut bool,
    contents: impl FnMut(&mut Ui),
) {
    if !*open {
        return;
    }

    let viewport_id = ViewportId::from_hash_of(key);
    let builder = ViewportBuilder::default()
        .with_title(title)
        .with_inner_size(size)
        .with_min_inner_size([280.0, 160.0])
        .with_resizable(true)
        .with_decorations(true);

    let mut close_requested = false;
    let mut contents = contents;
    ctx.show_viewport_immediate(viewport_id, builder, |ui, _class| {
        if ui.ctx().input(|i| i.viewport().close_requested()) {
            close_requested = true;
        }
        if scroll {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| contents(ui));
        } else {
            contents(ui);
        }
    });

    if close_requested {
        *open = false;
    }
}
