//! global_hotkeys.rs — Win32 System-Wide Global Hotkeys & Background Media Keys Manager

#![allow(dead_code)]

use std::sync::mpsc::{channel, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
#[cfg(windows)]
use std::thread;

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

pub struct GlobalHotkeyManager {
    rx: Receiver<GlobalHotkeyEvent>,
    running: Arc<AtomicBool>,
}

#[cfg(windows)]
impl GlobalHotkeyManager {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let running = Arc::new(AtomicBool::new(true));
        let thread_running = Arc::clone(&running);

        thread::spawn(move || {
            use windows_sys::Win32::UI::WindowsAndMessaging::*;
            use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;

            unsafe {
                // Hotkey ID constants (Media playback & optional Ctrl+Alt global combos)
                const HK_MEDIA_PLAY_PAUSE: i32 = 9001;
                const HK_MEDIA_NEXT: i32 = 9002;
                const HK_MEDIA_PREV: i32 = 9003;
                const HK_MEDIA_STOP: i32 = 9004;
                const HK_GLOBAL_PLAY: i32 = 9010;
                const HK_GLOBAL_NEXT: i32 = 9011;
                const HK_GLOBAL_PREV: i32 = 9012;
                const HK_GLOBAL_VOL_UP: i32 = 9013;
                const HK_GLOBAL_VOL_DOWN: i32 = 9014;
                const HK_GLOBAL_MUTE: i32 = 9015;
                const HK_GLOBAL_SHOW: i32 = 9016;

                let null_hwnd = std::ptr::null_mut();

                // Register standard hardware media transport keys (Leave hardware volume keys free for Windows System Volume)
                RegisterHotKey(null_hwnd, HK_MEDIA_PLAY_PAUSE, 0, VK_MEDIA_PLAY_PAUSE as u32);
                RegisterHotKey(null_hwnd, HK_MEDIA_NEXT, 0, VK_MEDIA_NEXT_TRACK as u32);
                RegisterHotKey(null_hwnd, HK_MEDIA_PREV, 0, VK_MEDIA_PREV_TRACK as u32);
                RegisterHotKey(null_hwnd, HK_MEDIA_STOP, 0, VK_MEDIA_STOP as u32);

                // Register global combos (Ctrl + Alt + Keys)
                let mod_ca = MOD_CONTROL | MOD_ALT;
                RegisterHotKey(null_hwnd, HK_GLOBAL_PLAY, mod_ca, VK_SPACE as u32);
                RegisterHotKey(null_hwnd, HK_GLOBAL_NEXT, mod_ca, VK_RIGHT as u32);
                RegisterHotKey(null_hwnd, HK_GLOBAL_PREV, mod_ca, VK_LEFT as u32);
                RegisterHotKey(null_hwnd, HK_GLOBAL_VOL_UP, mod_ca, VK_UP as u32);
                RegisterHotKey(null_hwnd, HK_GLOBAL_VOL_DOWN, mod_ca, VK_DOWN as u32);
                RegisterHotKey(null_hwnd, HK_GLOBAL_MUTE, mod_ca, 0x4D); // 'M'
                RegisterHotKey(null_hwnd, HK_GLOBAL_SHOW, mod_ca, 0x56); // 'V'

                let mut msg: MSG = std::mem::zeroed();
                while thread_running.load(Ordering::Relaxed) {
                    if PeekMessageW(&mut msg, null_hwnd, 0, 0, PM_REMOVE) != 0 {
                        if msg.message == WM_HOTKEY {
                            let hk_id = msg.wParam as i32;
                            let evt = match hk_id {
                                HK_MEDIA_PLAY_PAUSE | HK_GLOBAL_PLAY => Some(GlobalHotkeyEvent::PlayPause),
                                HK_MEDIA_NEXT | HK_GLOBAL_NEXT => Some(GlobalHotkeyEvent::Next),
                                HK_MEDIA_PREV | HK_GLOBAL_PREV => Some(GlobalHotkeyEvent::Previous),
                                HK_MEDIA_STOP => Some(GlobalHotkeyEvent::Stop),
                                HK_GLOBAL_VOL_UP => Some(GlobalHotkeyEvent::VolumeUp),
                                HK_GLOBAL_VOL_DOWN => Some(GlobalHotkeyEvent::VolumeDown),
                                HK_GLOBAL_MUTE => Some(GlobalHotkeyEvent::Mute),
                                HK_GLOBAL_SHOW => Some(GlobalHotkeyEvent::BringToFront),
                                _ => None,
                            };
                            if let Some(e) = evt {
                                let _ = tx.send(e);
                            }
                        }
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    } else {
                        thread::sleep(std::time::Duration::from_millis(15));
                    }
                }

                // Cleanup on exit
                for id in [
                    HK_MEDIA_PLAY_PAUSE, HK_MEDIA_NEXT, HK_MEDIA_PREV, HK_MEDIA_STOP,
                    HK_GLOBAL_PLAY, HK_GLOBAL_NEXT, HK_GLOBAL_PREV,
                    HK_GLOBAL_VOL_UP, HK_GLOBAL_VOL_DOWN, HK_GLOBAL_MUTE, HK_GLOBAL_SHOW,
                ] {
                    UnregisterHotKey(null_hwnd, id);
                }
            }
        });

        Self { rx, running }
    }

    pub fn poll_events(&self) -> Vec<GlobalHotkeyEvent> {
        let mut events = Vec::new();
        while let Ok(evt) = self.rx.try_recv() {
            events.push(evt);
        }
        events
    }
}

#[cfg(not(windows))]
impl GlobalHotkeyManager {
    pub fn new() -> Self {
        let (_tx, rx) = channel();
        Self { rx, running: Arc::new(AtomicBool::new(false)) }
    }

    pub fn poll_events(&self) -> Vec<GlobalHotkeyEvent> {
        Vec::new()
    }
}

impl Drop for GlobalHotkeyManager {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
