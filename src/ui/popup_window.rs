//! popup_window.rs — tool dialogs as real top-level popup windows.
//!
//! `egui::Window` paints a floating panel *inside* the player window. That
//! works for small overlays, but a tool dialog (Equalizer, Subtitles, AI
//! studio, …) then competes with the video for space, cannot be moved to
//! another monitor and steals focus from the player.
//!
//! [`PopupWindow`] keeps the same builder API as `egui::Window`, but the
//! dialog body is rendered inside its own egui viewport, which eframe maps to
//! a real OS window: own title bar, own taskbar entry, resizable, and
//! movable between displays.

#![allow(dead_code)]

use eframe::egui::{self, Align2, Id, InnerResponse, Pos2, Rect, Response, Sense, Ui, Vec2, ViewportBuilder, ViewportId};
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Mutex;

/// What we remember about a popup window between frames.
#[derive(Clone, Copy)]
struct PopupState {
    /// Where the window was placed when it was created.
    position: Pos2,
    /// Size the content needed last time, so the window can fit it exactly.
    content_size: Option<Vec2>,
}

/// Per-window state, kept so that the window is only placed once and can be
/// resized to fit its content. Recomputing the position every frame would
/// yank the popup back to the middle of the player whenever the player window
/// is moved.
static POPUP_STATE: Mutex<Option<HashMap<ViewportId, PopupState>>> = Mutex::new(None);

/// A dialog rendered in its own top-level window.
///
/// Drop-in replacement for `egui::Window` for the subset of the builder API
/// the app uses.
pub struct PopupWindow<'a> {
    title: String,
    id_salt: Option<String>,
    open: Option<&'a mut bool>,
    resizable: bool,
    title_bar: bool,
    default_size: Vec2,
    min_size: Vec2,
    max_width: Option<f32>,
    max_height: Option<f32>,
    fixed_size: Option<[f32; 2]>,
    frame: Option<egui::Frame>,
    inner_margin: Option<i8>,
    vscroll: bool,
    area_id: Option<Id>,
    always_on_top: bool,
    anchor_offset: Vec2,
    default_pos: Option<Pos2>,
}

impl<'a> PopupWindow<'a> {
    /// Create a popup window with the given (native) window title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            id_salt: None,
            open: None,
            resizable: true,
            title_bar: true,
            default_size: Vec2::new(420.0, 520.0),
            min_size: Vec2::ZERO,
            max_width: None,
            max_height: None,
            fixed_size: None,
            frame: None,
            inner_margin: None,
            vscroll: true,
            area_id: None,
            always_on_top: false,
            anchor_offset: Vec2::ZERO,
            default_pos: None,
        }
    }

    /// Extra salt for the window identity, for dialogs whose title changes.
    pub fn with_id(mut self, salt: impl Into<String>) -> Self {
        self.id_salt = Some(salt.into());
        self
    }

    /// Bind the dialog to an open/closed flag.
    ///
    /// The flag is cleared when the user closes the native window, so the
    /// caller sees the same behaviour as with `egui::Window`.
    pub fn open(mut self, open: &'a mut bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Whether the window can be resized by the user.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Native windows have no collapse button; accepted for API compatibility.
    pub fn collapsible(self, _collapsible: bool) -> Self {
        self
    }

    /// Whether the window has a native title bar.
    pub fn title_bar(mut self, title_bar: bool) -> Self {
        self.title_bar = title_bar;
        self
    }

    /// Size of the window when it is created.
    pub fn default_size(mut self, size: impl Into<Vec2>) -> Self {
        self.default_size = size.into();
        self
    }

    /// Width of the window when it is created.
    pub fn default_width(mut self, width: f32) -> Self {
        self.default_size.x = width;
        self
    }

    /// Height of the window when it is created.
    pub fn default_height(mut self, height: f32) -> Self {
        self.default_size.y = height;
        self
    }

    /// Smallest size the user may shrink the window to.
    pub fn min_size(mut self, size: impl Into<Vec2>) -> Self {
        self.min_size = size.into();
        self
    }

    /// Smallest width the user may shrink the window to.
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_size.x = width;
        self
    }

    /// Largest size the window may grow to.
    pub fn max_size(mut self, size: impl Into<Vec2>) -> Self {
        let size = size.into();
        self.max_width = Some(size.x);
        self.max_height = Some(size.y);
        self
    }

    /// Largest width the window may grow to.
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Largest height the window may grow to.
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Accepted for API compatibility (the native window owns its content area).
    pub fn constrain(self, _constrain: bool) -> Self {
        self
    }

    /// Fixed window size (not resizable).
    pub fn fixed_size(mut self, size: impl Into<Vec2>) -> Self {
        let size = size.into();
        self.fixed_size = Some([size.x, size.y]);
        self.resizable = false;
        self
    }

    /// Frame painted behind the dialog content.
    pub fn frame(mut self, frame: egui::Frame) -> Self {
        self.frame = Some(frame);
        self
    }

    /// Margin between the window edge and the dialog content.
    pub fn inner_margin(mut self, margin: i8) -> Self {
        self.inner_margin = Some(margin);
        self
    }

    /// Whether the content scrolls when it does not fit.
    pub fn vscroll(mut self, vscroll: bool) -> Self {
        self.vscroll = vscroll;
        self
    }

    /// Extra identity for the dialog (used when the title is dynamic).
    pub fn id(mut self, id: Id) -> Self {
        self.area_id = Some(id);
        self
    }

    /// Offset from the anchored position.
    ///
    /// Dialogs that let the user drag their own header change this offset, and
    /// the popup window follows along.
    pub fn anchor(mut self, _anchor: Align2, offset: Vec2) -> Self {
        self.anchor_offset = offset;
        self
    }

    /// Preferred position of the window, relative to the player window.
    pub fn default_pos(mut self, pos: impl Into<Pos2>) -> Self {
        self.default_pos = Some(pos.into());
        self
    }

    /// Accepted for API compatibility (the native window owns its content area).
    pub fn content_rect(self, _rect: Rect) -> Self {
        self
    }

    /// Accepted for API compatibility (the native window owns its content area).
    pub fn center(self) -> Self {
        self
    }

    /// Accepted for API compatibility (the native window owns its content area).
    pub fn pivot(self, _pivot: Align2) -> Self {
        self
    }

    /// Keep the popup above other windows.
    pub fn always_on_top(mut self) -> Self {
        self.always_on_top = true;
        self
    }

    /// Show the dialog in its own top-level window.
    ///
    /// Returns the response of the popup area, mirroring `egui::Window::show`.
    pub fn show(self, ctx: &egui::Context, body: impl FnOnce(&mut Ui)) -> Option<InnerResponse<Option<()>>> {
        let PopupWindow {
            title,
            id_salt,
            open,
            resizable,
            title_bar,
            default_size,
            min_size,
            max_width,
            max_height,
            fixed_size,
            frame,
            inner_margin,
            vscroll,
            area_id,
            always_on_top,
            anchor_offset,
            default_pos,
        } = self;

        // A closed dialog must not keep a window around.
        if let Some(open) = open.as_deref()
            && !*open
        {
            return None;
        }

        let viewport_id = ViewportId::from_hash_of((
            "vortex_popup_window",
            &title,
            &id_salt,
            area_id,
        ));

        // Remember where the window goes and how big its content was.
        let state = popup_state(viewport_id, ctx, default_size, default_pos);
        let mut inner_size = match fixed_size {
            Some([w, h]) => Vec2::new(w, h),
            None => default_size,
        };
        if fixed_size.is_none()
            && let Some(fitted) = state.content_size
        {
            // Use the size the content was measured at. The size only ever
            // grows after the first fit: measuring a smaller content in a
            // smaller window would otherwise shrink the window every frame.
            inner_size = Vec2::new(
                fitted.x.clamp(min_size.x, screen_size(ctx).x * 0.95),
                fitted.y.clamp(min_size.y, screen_size(ctx).y * 0.92),
            );
        }

        let mut builder = ViewportBuilder::default()
            .with_title(title.clone())
            .with_inner_size(inner_size)
            .with_resizable(resizable)
            .with_decorations(title_bar);

        if min_size.x > 0.0 || min_size.y > 0.0 {
            builder = builder.with_min_inner_size(min_size);
        }
        if max_width.is_some() || max_height.is_some() {
            builder = builder.with_max_inner_size(Vec2::new(
                max_width.unwrap_or(10_000.0),
                max_height.unwrap_or(10_000.0),
            ));
        }
        if always_on_top {
            builder = builder.with_always_on_top();
        }
        builder = builder.with_position(state.position + anchor_offset);

        // The body is only good for one call, and the viewport callback may be
        // invoked more than once per frame, so hand it over exactly once.
        let body = Cell::new(Some(body));
        let response = Cell::new(None::<Response>);
        let content_size = Cell::new(state.content_size);
        let mut open = open;

        ctx.show_viewport_immediate(viewport_id, builder, |ui, _class| {
            // The popup lives in its own egui viewport, which does not inherit
            // the styling of the player window: apply the player's theme here.
            {
                let ctx = ui.ctx();
                crate::ui::theme::VortexTheme::apply(
                    ctx,
                    crate::ui::theme::VortexTheme::current_mode(),
                );
            }

            // The user closed the native window: mirror that into our state.
            if ui.ctx().input(|i| i.viewport().close_requested())
                && let Some(open) = open.as_deref_mut()
            {
                *open = false;
            }

            let rect = ui.max_rect();
            if let Some(body) = body.take() {
                let mut frame = frame.unwrap_or_else(|| {
                    // Same look as the in-window dialogs had.
                    let ctx = ui.ctx();
                    egui::Frame::window(&ctx.style_of(ctx.theme()))
                });
                if let Some(margin) = inner_margin {
                    frame = frame.inner_margin(egui::Margin::same(margin));
                }

                // Fill the whole window: otherwise the area around the content
                // stays unpainted and shows up as a black bar.
                ui.painter().rect_filled(rect, 0.0, frame.fill);

                // The frame's own margin is not part of the measured content.
                let margin = frame.inner_margin;

                if vscroll {
                    frame.show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("vortex_popup_scroll")
                            .show(ui, |ui| {
                                body(ui);
                                // Remember how big the content was, so the
                                // window can be sized to fit it exactly.
                                content_size.set(Some(content_rect(ui, margin)));
                            });
                    });
                } else {
                    frame.show(ui, |ui| {
                        body(ui);
                        content_size.set(Some(content_rect(ui, margin)));
                    });
                }
            }
            response.set(Some(ui.allocate_rect(rect, Sense::empty())));
        });

        // Remember the measured content size for the next frame.
        if let Some(measured) = content_size.get()
            && let Ok(mut guard) = POPUP_STATE.lock()
            && let Some(map) = guard.as_mut()
            && let Some(entry) = map.get_mut(&viewport_id)
            // Fit the window once. Measuring the content again in the resized
            // window would either shrink it forever (smaller content, smaller
            // window) or grow it forever, so the size is decided by the first
            // measurement and then left alone.
            && entry.content_size.is_none()
        {
            entry.content_size = Some(measured);
        }

        response.take().map(|response| InnerResponse::new(None, response))
    }
}

/// Size of the screen the popup may use.
fn screen_size(ctx: &egui::Context) -> Vec2 {
    ctx.input(|i| i.viewport().monitor_size)
        .unwrap_or_else(|| Vec2::new(1920.0, 1080.0))
}

/// Size the window needs for the given content, including the frame margin.
fn content_rect(ui: &Ui, margin: egui::Margin) -> Vec2 {
    let used = ui.min_rect().size();
    Vec2::new(
        used.x + margin.left as f32 + margin.right as f32,
        used.y + margin.top as f32 + margin.bottom as f32,
    )
}

/// State of a popup window, created on first use.
fn popup_state(
    viewport_id: ViewportId,
    ctx: &egui::Context,
    size: Vec2,
    default_pos: Option<Pos2>,
) -> PopupState {
    let mut guard = match POPUP_STATE.lock() {
        Ok(guard) => guard,
        Err(_) => {
            return PopupState {
                position: Pos2::ZERO,
                content_size: None,
            };
        }
    };
    let map = guard.get_or_insert_with(HashMap::new);
    if let Some(state) = map.get(&viewport_id) {
        return *state;
    }

    let parent = ctx
        .input(|i| i.viewport().inner_rect)
        .unwrap_or_else(|| ctx.viewport_rect());
    let position = match default_pos {
        Some(pos) => parent.min + pos.to_vec2(),
        None => Pos2::new(
            (parent.center().x - size.x / 2.0).max(parent.min.x),
            (parent.center().y - size.y / 2.0).max(parent.min.y),
        ),
    };
    let state = PopupState {
        position,
        content_size: None,
    };
    map.insert(viewport_id, state);
    state
}
