//! linux.rs — Platform stubs and utilities for Linux / non-Windows systems

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalHotkeyEvent {
    PlayPause,
    Next,
    Previous,
    Stop,
    VolumeUp,
    VolumeDown,
    Mute,
    BringToFront,
}

pub struct GlobalHotkeyManager;

impl Default for GlobalHotkeyManager {
    fn default() -> Self {
        Self
    }
}

impl GlobalHotkeyManager {
    pub fn new() -> Self {
        Self
    }

    pub fn poll_events(&self) -> Vec<GlobalHotkeyEvent> {
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub enum SmtcEvent {
    Play,
    Pause,
    Toggle,
    Next,
    Prev,
    Stop,
    Seek(Duration),
}

#[derive(Default)]
pub struct WindowsSmtcAdapter;

impl WindowsSmtcAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn init(&mut self, _hwnd: isize) {}

    pub fn poll_events(&mut self) -> Vec<SmtcEvent> {
        Vec::new()
    }

    pub fn update_metadata(
        &mut self,
        _title: &str,
        _artist: Option<&str>,
        _album: Option<&str>,
        _cover: Option<&str>,
    ) {
    }

    pub fn update_playback(
        &mut self,
        _playing: bool,
        _position: Option<Duration>,
        _duration: Option<Duration>,
    ) {
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarProgressState {
    NoProgress,
    Indeterminate,
    Normal,
    Error,
    Paused,
}

#[derive(Default)]
pub struct WindowsTaskbarAdapter;

impl WindowsTaskbarAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn is_thumbbar_initialized(&self) -> bool {
        false
    }

    pub fn init_thumbbar(&mut self, _hwnd: isize) {}

    pub fn update_thumbbar(&mut self, _hwnd: isize, _is_paused: bool, _is_idle: bool) {}

    pub fn update_progress(
        &mut self,
        _hwnd: isize,
        _current_sec: f64,
        _total_sec: f64,
        _is_paused: bool,
        _is_idle: bool,
    ) {
    }

    pub fn clear_progress(&mut self, _hwnd: isize) {}

    pub fn set_thumbnail_clip(&mut self, _hwnd: isize, _clip_rect: Option<[i32; 4]>) {}

    pub fn poll_commands(&mut self) -> Vec<u16> {
        Vec::new()
    }
}

pub unsafe fn apply_border_suppression(_hwnd: isize) {}
pub unsafe fn apply_fullscreen_border_suppression(_hwnd: isize) {}
pub unsafe fn attach_subclass(_hwnd: isize) {}
pub fn build_button_icons() -> [isize; 8] {
    [0; 8]
}
pub fn install_startup_cbt_hook() {}
pub fn uninstall_startup_cbt_hook() {}
pub fn is_taskbar_autohide() -> bool {
    false
}
pub unsafe fn mark_fullscreen_window(_hwnd: isize, _fullscreen: bool) {}
pub fn set_native_fullscreen_state(_v: bool) {}
pub fn is_native_fullscreen() -> bool { false }
pub fn set_window_corner_style(_style: &str) {}
