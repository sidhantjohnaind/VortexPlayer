use eframe::egui::{self, Color32, CornerRadius, Margin, Stroke, Ui, Vec2};

/// Render a popup dialog as a sleek in-app floating window.
pub fn show_native_window(
    ctx: &egui::Context,
    key: &str,
    title: &str,
    size: Vec2,
    scroll: bool,
    open: &mut bool,
    mut contents: impl FnMut(&mut Ui),
) {
    if !*open {
        return;
    }

    let has_title_bar = !title.is_empty();

    let mut window = egui::Window::new(title)
        .id(egui::Id::new(key))
        .default_pos(ctx.content_rect().center())
        .pivot(egui::Align2::CENTER_CENTER)
        .default_size(size)
        .min_size(Vec2::new(280.0, 160.0))
        .resizable(true)
        .collapsible(false)
        .vscroll(scroll)
        .constrain(true)
        .title_bar(has_title_bar)
        .order(egui::Order::Foreground)
        .frame(
            egui::Frame::new()
                .fill(Color32::from_rgba_premultiplied(16, 18, 24, 250))
                .stroke(Stroke::new(1.0, Color32::from_rgb(45, 52, 70)))
                .corner_radius(CornerRadius::same(8))
                .inner_margin(Margin::same(12))
                .shadow(egui::Shadow {
                    offset: [0, 8],
                    blur: 24,
                    spread: 2,
                    color: Color32::from_black_alpha(200),
                }),
        );

    if has_title_bar {
        window = window.open(open);
    }

    window.show(ctx, |ui| {
        contents(ui);
    });
}
