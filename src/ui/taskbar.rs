#![allow(dead_code)]

pub struct TaskbarIntegration {
    adapter: crate::platform::WindowsTaskbarAdapter,
}

impl Default for TaskbarIntegration {
    fn default() -> Self {
        Self {
            adapter: crate::platform::WindowsTaskbarAdapter::new(),
        }
    }
}

impl TaskbarIntegration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_thumbbar_initialized(&self) -> bool {
        self.adapter.is_thumbbar_initialized()
    }

    pub fn init_thumbbar(&mut self, hwnd: isize) {
        self.adapter.init_thumbbar(hwnd);
    }

    pub fn update_thumbbar(&mut self, hwnd: isize, is_paused: bool, is_idle: bool) {
        self.adapter.update_thumbbar(hwnd, is_paused, is_idle);
    }

    pub fn update_progress(
        &mut self,
        hwnd: isize,
        current_sec: f64,
        total_sec: f64,
        is_paused: bool,
        is_idle: bool,
    ) {
        self.adapter
            .update_progress(hwnd, current_sec, total_sec, is_paused, is_idle);
    }

    pub fn clear_progress(&mut self, hwnd: isize) {
        self.adapter.clear_progress(hwnd);
    }

    pub fn set_thumbnail_clip(&mut self, hwnd: isize, clip_rect: Option<[i32; 4]>) {
        self.adapter.set_thumbnail_clip(hwnd, clip_rect);
    }

    pub fn poll_commands(&mut self) -> Vec<u16> {
        self.adapter.poll_commands()
    }
}
