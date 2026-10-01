#![allow(dead_code)]

use crate::commands::{Command, CommandDispatcher};
use souvlaki::{
    MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig,
};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

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

/// Windows System Media Transport Controls (SMTC) Adapter
pub struct WindowsSmtcAdapter {
    controls: Option<MediaControls>,
    rx: Receiver<SmtcEvent>,
    tx: Sender<SmtcEvent>,
    last_title: String,
    last_playback_state: Option<bool>,
    pub is_initialized: bool,
}

impl Default for WindowsSmtcAdapter {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            controls: None,
            rx,
            tx,
            last_title: String::new(),
            last_playback_state: None,
            is_initialized: false,
        }
    }
}

impl WindowsSmtcAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize the SMTC controls attached to the main window HWND
    pub fn init(&mut self, _hwnd: isize) {
        if self.is_initialized {
            return;
        }

        #[cfg(target_os = "windows")]
        let hwnd_ptr = if _hwnd != 0 {
            Some(_hwnd as *mut std::ffi::c_void)
        } else {
            None
        };
        #[cfg(not(target_os = "windows"))]
        let hwnd_ptr = None;

        let config = PlatformConfig {
            dbus_name: "vertex_player",
            display_name: "VertexPlayer",
            hwnd: hwnd_ptr,
        };

        if let Ok(mut controls) = MediaControls::new(config) {
            let tx = self.tx.clone();
            let _ = controls.attach(move |event: MediaControlEvent| {
                let smtc_event = match event {
                    MediaControlEvent::Play => SmtcEvent::Play,
                    MediaControlEvent::Pause => SmtcEvent::Pause,
                    MediaControlEvent::Toggle => SmtcEvent::Toggle,
                    MediaControlEvent::Next => SmtcEvent::Next,
                    MediaControlEvent::Previous => SmtcEvent::Prev,
                    MediaControlEvent::Stop => SmtcEvent::Stop,
                    MediaControlEvent::SetPosition(pos) => SmtcEvent::Seek(pos.0),
                    _ => return,
                };
                let _ = tx.send(smtc_event);
            });
            self.controls = Some(controls);
            self.is_initialized = true;
        }
    }

    /// Dispatches all incoming SMTC events into the central CommandDispatcher
    pub fn dispatch_pending_events(&self, player: Option<&std::sync::Arc<crate::engine::Player>>) {
        while let Ok(event) = self.rx.try_recv() {
            let cmd = match event {
                SmtcEvent::Play | SmtcEvent::Pause | SmtcEvent::Toggle => Command::PlayPause,
                SmtcEvent::Stop => Command::Stop,
                SmtcEvent::Next => Command::Seek { seconds: 10.0 },
                SmtcEvent::Prev => Command::Seek { seconds: -10.0 },
                SmtcEvent::Seek(dur) => Command::Seek {
                    seconds: dur.as_secs_f64(),
                },
            };
            CommandDispatcher::dispatch(&cmd, player);
        }
    }

    /// Update media metadata in Windows Quick Settings / Lock Screen
    pub fn update_metadata(
        &mut self,
        title: &str,
        artist: Option<&str>,
        album: Option<&str>,
        cover_url: Option<&str>,
        duration: Option<Duration>,
    ) {
        if let Some(ref mut controls) = self.controls {
            if self.last_title != title {
                self.last_title = title.to_string();
                let _ = controls.set_metadata(MediaMetadata {
                    title: Some(title),
                    artist,
                    album,
                    cover_url,
                    duration,
                });
            }
        }
    }

    /// Update playback state and position
    pub fn update_playback_state(&mut self, is_playing: bool, position_sec: f64) {
        if let Some(ref mut controls) = self.controls {
            let progress = Some(MediaPosition(Duration::from_secs_f64(position_sec.max(0.0))));
            let playback = if is_playing {
                MediaPlayback::Playing { progress }
            } else {
                MediaPlayback::Paused { progress }
            };
            let _ = controls.set_playback(playback);
            self.last_playback_state = Some(is_playing);
        }
    }
}
