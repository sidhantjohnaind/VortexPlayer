//! seek_preview.rs — Ultra-Fast Responsive Seekbar Hover Video Frame Thumbnail Engine

#![allow(dead_code)]

use eframe::egui::{
    ColorImage, Context, TextureHandle, TextureOptions,
};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(Clone, Debug)]
struct ThumbnailRequest {
    file_path: PathBuf,
    timestamp: f64,
    slot: u64,
}

struct ThumbnailResponse {
    file_path: PathBuf,
    slot: u64,
    image: Option<ColorImage>,
}

pub struct SeekPreviewEngine {
    textures: HashMap<(PathBuf, u64), TextureHandle>,
    failed_keys: HashMap<(PathBuf, u64), Instant>,
    recent_keys: VecDeque<(PathBuf, u64)>,
    inflight: HashMap<(PathBuf, u64), Instant>,
    latest_req: Arc<Mutex<Option<ThumbnailRequest>>>,
    res_rx: Receiver<ThumbnailResponse>,
    last_rendered_tex: Option<TextureHandle>,
}

impl Default for SeekPreviewEngine {
    fn default() -> Self {
        let (res_tx, res_rx) = channel::<ThumbnailResponse>();
        let latest_req = Arc::new(Mutex::new(None::<ThumbnailRequest>));

        // Spawn 2 parallel high-speed thumbnail worker threads with LIFO latest request grabbing
        for thread_idx in 0..2 {
            let latest_req_clone = Arc::clone(&latest_req);
            let res_tx_clone = res_tx.clone();

            thread::Builder::new()
                .name(format!("vortex_seek_thumb_{}", thread_idx))
                .spawn(move || {
                    loop {
                        let req_opt = {
                            let mut lock = match latest_req_clone.lock() {
                                Ok(l) => l,
                                Err(_) => break,
                            };
                            lock.take()
                        };

                        let req = match req_opt {
                            Some(r) => r,
                            None => {
                                thread::sleep(std::time::Duration::from_millis(8));
                                continue;
                            }
                        };

                        if !req.file_path.exists() {
                            let _ = res_tx_clone.send(ThumbnailResponse {
                                file_path: req.file_path,
                                slot: req.slot,
                                image: None,
                            });
                            continue;
                        }

                        let time_arg = format!("{:.2}", req.timestamp.max(0.0));
                        let path_str = req.file_path.to_string_lossy().to_string();

                        let output = crate::platform::silent_command("ffmpeg")
                            .args([
                                "-ss",
                                &time_arg,
                                "-noaccurate_seek",
                                "-i",
                                &path_str,
                                "-an",
                                "-sn",
                                "-dn",
                                "-vframes",
                                "1",
                                "-s",
                                "192x108",
                                "-f",
                                "image2pipe",
                                "-vcodec",
                                "mjpeg",
                                "pipe:1",
                            ])
                            .stdout(Stdio::piped())
                            .stderr(Stdio::null())
                            .output();

                        let color_img = match output {
                            Ok(out) if out.status.success() && !out.stdout.is_empty() => {
                                if let Ok(dyn_img) = image::load_from_memory(&out.stdout) {
                                    let rgba = dyn_img.to_rgba8();
                                    let size = [rgba.width() as usize, rgba.height() as usize];
                                    let pixels = rgba.into_raw();
                                    Some(ColorImage::from_rgba_unmultiplied(size, &pixels))
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        };

                        let _ = res_tx_clone.send(ThumbnailResponse {
                            file_path: req.file_path,
                            slot: req.slot,
                            image: color_img,
                        });
                    }
                })
                .expect("Failed to spawn seek thumbnail worker thread");
        }

        Self {
            textures: HashMap::new(),
            failed_keys: HashMap::new(),
            recent_keys: VecDeque::new(),
            inflight: HashMap::new(),
            latest_req,
            res_rx,
            last_rendered_tex: None,
        }
    }
}

impl SeekPreviewEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Process received decoded thumbnails from background workers and register GPU textures
    pub fn pump(&mut self, ctx: &Context) {
        let mut got_any = false;
        loop {
            match self.res_rx.try_recv() {
                Ok(resp) => {
                    let key = (resp.file_path, resp.slot);
                    self.inflight.remove(&key);

                    if let Some(color_img) = resp.image {
                        let tex_id = format!("vortex_thumb_{}_{}", key.0.to_string_lossy(), key.1);
                        let tex = ctx.load_texture(
                            tex_id,
                            color_img,
                            TextureOptions::LINEAR,
                        );
                        self.recent_keys.push_back(key.clone());
                        self.textures.insert(key, tex);

                        // Evict oldest if exceeding 200 textures
                        while self.recent_keys.len() > 200 {
                            if let Some(old_key) = self.recent_keys.pop_front() {
                                self.textures.remove(&old_key);
                            }
                        }
                    } else {
                        self.failed_keys.insert(key, Instant::now());
                    }
                    got_any = true;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        if got_any {
            ctx.request_repaint();
        }

        // Clean up inflight requests older than 3 seconds
        let now = Instant::now();
        self.inflight.retain(|_, start| now.duration_since(*start).as_secs() < 3);
        self.failed_keys.retain(|_, start| now.duration_since(*start).as_secs() < 10);
    }

    /// Request or get cached texture for given file and timestamp
    pub fn get_or_request(
        &mut self,
        _ctx: &Context,
        file_path: &Path,
        time_secs: f64,
    ) -> Option<&TextureHandle> {
        let slot = (time_secs / 2.0).round().max(0.0) as u64;
        let key = (file_path.to_path_buf(), slot);

        // Exact match
        if self.textures.contains_key(&key) {
            return self.textures.get(&key);
        }

        // Request frame if not in flight or failed recently
        if !self.inflight.contains_key(&key) && !self.failed_keys.contains_key(&key) {
            self.inflight.insert(key.clone(), Instant::now());
            if let Ok(mut lock) = self.latest_req.lock() {
                *lock = Some(ThumbnailRequest {
                    file_path: file_path.to_path_buf(),
                    timestamp: time_secs,
                    slot,
                });
            }
        }

        // Return adjacent frame (slot ± 1 or ± 2) for smooth scrubbing preview without blank loading box
        for offset in &[1i64, -1, 2, -2, 3, -3] {
            let adj_slot = (slot as i64 + offset).max(0) as u64;
            let adj_key = (file_path.to_path_buf(), adj_slot);
            if self.textures.contains_key(&adj_key) {
                return self.textures.get(&adj_key);
            }
        }

        None
    }
}
