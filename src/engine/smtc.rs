#![allow(dead_code)]

use souvlaki::{
    MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig,
};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum SmtcCommand {
    Play,
    Pause,
    Toggle,
    Next,
    Prev,
    Stop,
    Seek(Duration),
}

pub struct SmtcEngine {
    controls: Option<MediaControls>,
    rx: Receiver<SmtcCommand>,
    tx: Sender<SmtcCommand>,
    last_title: String,
    last_artist: Option<String>,
    last_album: Option<String>,
    last_cover: Option<String>,
    last_state: Option<bool>,
    pub is_initialized: bool,
}

impl Default for SmtcEngine {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            controls: None,
            rx,
            tx,
            last_title: String::new(),
            last_artist: None,
            last_album: None,
            last_cover: None,
            last_state: None,
            is_initialized: false,
        }
    }
}

impl SmtcEngine {
    pub fn new() -> Self {
        Self::default()
    }

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
            dbus_name: "vortex_player",
            display_name: "VortexPlayer",
            hwnd: hwnd_ptr,
        };

        if let Ok(mut controls) = MediaControls::new(config) {
            let tx = self.tx.clone();
            let _ = controls.attach(move |event: MediaControlEvent| {
                let cmd = match event {
                    MediaControlEvent::Play => SmtcCommand::Play,
                    MediaControlEvent::Pause => SmtcCommand::Pause,
                    MediaControlEvent::Toggle => SmtcCommand::Toggle,
                    MediaControlEvent::Next => SmtcCommand::Next,
                    MediaControlEvent::Previous => SmtcCommand::Prev,
                    MediaControlEvent::Stop => SmtcCommand::Stop,
                    MediaControlEvent::SetPosition(pos) => SmtcCommand::Seek(pos.0),
                    _ => return,
                };
                let _ = tx.send(cmd);
            });
            self.controls = Some(controls);
            self.is_initialized = true;
        }
    }

    pub fn poll_events(&self) -> Vec<SmtcCommand> {
        let mut events = Vec::new();
        while let Ok(cmd) = self.rx.try_recv() {
            events.push(cmd);
        }
        events
    }

    pub fn update_metadata(
        &mut self,
        title: &str,
        artist: Option<&str>,
        album: Option<&str>,
        cover_url: Option<&str>,
        duration_sec: f64,
        time_pos_sec: f64,
        is_paused: bool,
        is_idle: bool,
    ) {
        let Some(ref mut controls) = self.controls else {
            return;
        };

        if is_idle {
            if !self.last_title.is_empty() {
                let _ = controls.set_playback(MediaPlayback::Stopped);
                self.last_title.clear();
                self.last_artist = None;
                self.last_album = None;
                self.last_cover = None;
                self.last_state = None;
            }
            return;
        }

        let effective_title = if !title.is_empty() {
            title
        } else {
            "VortexPlayer"
        };

        let cover_str = cover_url.map(|s| s.to_string());
        let artist_str = artist.map(|s| s.to_string());
        let album_str = album.map(|s| s.to_string());

        let meta_changed = self.last_title != effective_title
            || self.last_artist != artist_str
            || self.last_album != album_str
            || self.last_cover != cover_str;

        if meta_changed {
            self.last_title = effective_title.to_string();
            self.last_artist = artist_str;
            self.last_album = album_str;
            self.last_cover = cover_str;
            let dur = if duration_sec > 0.0 {
                Some(Duration::from_secs_f64(duration_sec))
            } else {
                None
            };
            let _ = controls.set_metadata(MediaMetadata {
                title: Some(effective_title),
                artist,
                album,
                cover_url,
                duration: dur,
            });
            let pos = MediaPosition(Duration::from_secs_f64(time_pos_sec.max(0.0)));
            let _ = controls.set_playback(if is_paused {
                MediaPlayback::Paused { progress: Some(pos) }
            } else {
                MediaPlayback::Playing { progress: Some(pos) }
            });
            self.last_state = Some(is_paused);
        } else if self.last_state != Some(is_paused) {
            self.last_state = Some(is_paused);
            let pos = MediaPosition(Duration::from_secs_f64(time_pos_sec.max(0.0)));
            if is_paused {
                let _ = controls.set_playback(MediaPlayback::Paused {
                    progress: Some(pos),
                });
            } else {
                let _ = controls.set_playback(MediaPlayback::Playing {
                    progress: Some(pos),
                });
            }
        }
    }
}
