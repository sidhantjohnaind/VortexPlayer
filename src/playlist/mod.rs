#![allow(dead_code)]
pub mod queue;
pub use queue::{PlayNextQueue, SequenceDetector};
pub mod smart;
pub use smart::{SmartPlaylist, SmartRule};

pub mod dpl;
pub mod m3u;
pub mod scanner;

pub use dpl::{DplFile, DplItem};
pub use m3u::{M3uFile, M3uItem};

use natord::compare;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlaylistItem {
    pub path: PathBuf,
    pub title: String,
    pub duration_secs: Option<f64>,
    pub file_size_bytes: Option<u64>,
}

impl PlaylistItem {
    pub fn from_path(path: PathBuf) -> Self {
        let title = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Self {
            path,
            title,
            duration_secs: None,
            file_size_bytes: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RepeatMode {
    #[default]
    Off,
    RepeatAll,
    RepeatTrack,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PlaylistTab {
    #[default]
    DefaultPlaylist,
    Albums,
    History,
    Favorites,
}

#[derive(Debug, Clone, Default)]
pub struct Playlist {
    pub items: Vec<PlaylistItem>,
    pub current_index: Option<usize>,
    pub repeat_mode: RepeatMode,
    pub is_shuffle: bool,
    pub shuffle_indices: Vec<usize>,
    pub shuffle_pos: usize,
    pub active_tab: PlaylistTab,
    pub search_query: String,
    pub favorites: Vec<PathBuf>,
    pub history: Vec<PathBuf>,
}

impl Playlist {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn current_item(&self) -> Option<&PlaylistItem> {
        self.current_index.and_then(|idx| self.items.get(idx))
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.current_index = None;
        self.shuffle_indices.clear();
        self.shuffle_pos = 0;
    }

    pub fn add_item(&mut self, path: PathBuf) -> usize {
        if let Some(existing_idx) = self.items.iter().position(|it| it.path == path) {
            return existing_idx;
        }
        let item = PlaylistItem::from_path(path);
        self.items.push(item);
        self.rebuild_shuffle();
        self.items.len() - 1
    }

    pub fn add_items(&mut self, paths: Vec<PathBuf>) {
        let mut existing: std::collections::HashSet<PathBuf> = self.items.iter().map(|it| it.path.clone()).collect();
        for path in paths {
            if existing.insert(path.clone()) {
                self.items.push(PlaylistItem::from_path(path));
            }
        }
        self.rebuild_shuffle();
    }

    pub fn remove_duplicates(&mut self) {
        let mut seen = std::collections::HashSet::new();
        let current_path = self.current_item().map(|it| it.path.clone());
        self.items.retain(|it| seen.insert(it.path.clone()));
        if let Some(path) = current_path {
            self.current_index = self.items.iter().position(|it| it.path == path);
        }
        self.rebuild_shuffle();
    }

    pub fn remove_missing_files(&mut self) {
        let current_path = self.current_item().map(|it| it.path.clone());
        self.items.retain(|it| {
            let p_str = it.path.to_string_lossy();
            if p_str.starts_with("http://") || p_str.starts_with("https://") || p_str.starts_with("rtsp://") {
                true
            } else {
                it.path.exists()
            }
        });
        if let Some(path) = current_path {
            self.current_index = self.items.iter().position(|it| it.path == path);
        }
        self.rebuild_shuffle();
    }

    pub fn load_dpl_file(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let dpl_items = DplFile::load_from_file(path)?;
        self.items.clear();
        for it in dpl_items {
            self.items.push(PlaylistItem {
                path: it.path,
                title: it.title,
                duration_secs: if it.duration_ms > 0 { Some(it.duration_ms as f64 / 1000.0) } else { None },
                file_size_bytes: None,
            });
        }
        self.current_index = if self.items.is_empty() { None } else { Some(0) };
        self.rebuild_shuffle();
        Ok(())
    }

    pub fn save_dpl_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let dpl_items: Vec<DplItem> = self.items.iter().map(|it| DplItem {
            path: it.path.clone(),
            title: it.title.clone(),
            duration_ms: it.duration_secs.map(|d| (d * 1000.0) as i64).unwrap_or(0),
            start_pos_ms: 0,
            played_time_ms: 0,
        }).collect();
        DplFile::save_to_file(path, &dpl_items)
    }

    pub fn load_m3u_file(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let m3u_items = M3uFile::load_from_file(path)?;
        self.items.clear();
        for it in m3u_items {
            self.items.push(PlaylistItem {
                path: it.path,
                title: it.title,
                duration_secs: if it.duration_secs > 0.0 { Some(it.duration_secs) } else { None },
                file_size_bytes: None,
            });
        }
        self.current_index = if self.items.is_empty() { None } else { Some(0) };
        self.rebuild_shuffle();
        Ok(())
    }

    pub fn save_m3u_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let m3u_items: Vec<M3uItem> = self.items.iter().map(|it| M3uItem {
            path: it.path.clone(),
            title: it.title.clone(),
            duration_secs: it.duration_secs.unwrap_or(0.0),
        }).collect();
        M3uFile::save_to_file(path, &m3u_items)
    }

    pub fn set_single_file_with_auto_detect(&mut self, path: PathBuf, auto_detect_neighbors: bool) -> usize {
        let neighboring_files = if auto_detect_neighbors {
            scanner::scan_neighboring_episodes(&path)
        } else {
            vec![path.clone()]
        };

        self.items.clear();
        let mut target_index = 0;

        for (idx, p) in neighboring_files.into_iter().enumerate() {
            if p == path {
                target_index = idx;
            }
            self.items.push(PlaylistItem::from_path(p));
        }

        self.current_index = Some(target_index);
        self.rebuild_shuffle();
        self.add_to_history(&path);
        target_index
    }

    pub fn play_index(&mut self, index: usize) -> Option<PathBuf> {
        if index < self.items.len() {
            self.current_index = Some(index);
            let path = self.items[index].path.clone();
            self.add_to_history(&path);
            Some(path)
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<PathBuf> {
        if self.items.is_empty() {
            return None;
        }

        if self.repeat_mode == RepeatMode::RepeatTrack {
            if let Some(idx) = self.current_index {
                return self.items.get(idx).map(|it| it.path.clone());
            }
        }

        if self.is_shuffle {
            if self.shuffle_indices.is_empty() {
                self.rebuild_shuffle();
            }
            if self.shuffle_indices.is_empty() {
                return None;
            }
            if self.shuffle_pos + 1 < self.shuffle_indices.len() {
                self.shuffle_pos += 1;
                let next_idx = self.shuffle_indices[self.shuffle_pos];
                return self.play_index(next_idx);
            } else if self.repeat_mode == RepeatMode::RepeatAll {
                self.rebuild_shuffle();
                self.shuffle_pos = 0;
                let next_idx = self.shuffle_indices[0];
                return self.play_index(next_idx);
            } else {
                return None;
            }
        }

        match self.repeat_mode {
            RepeatMode::RepeatAll => {
                let next_idx = match self.current_index {
                    Some(idx) => (idx + 1) % self.items.len(),
                    None => 0,
                };
                self.play_index(next_idx)
            }
            RepeatMode::Off => {
                if let Some(idx) = self.current_index {
                    if idx + 1 < self.items.len() {
                        return self.play_index(idx + 1);
                    }
                } else if !self.items.is_empty() {
                    return self.play_index(0);
                }
                None
            }
            RepeatMode::RepeatTrack => unreachable!(),
        }
    }

    pub fn previous(&mut self) -> Option<PathBuf> {
        if self.items.is_empty() {
            return None;
        }

        if self.repeat_mode == RepeatMode::RepeatTrack {
            if let Some(idx) = self.current_index {
                return self.items.get(idx).map(|it| it.path.clone());
            }
        }

        if self.is_shuffle {
            if self.shuffle_indices.is_empty() {
                self.rebuild_shuffle();
            }
            if self.shuffle_indices.is_empty() {
                return None;
            }
            if self.shuffle_pos > 0 {
                self.shuffle_pos -= 1;
            } else if self.repeat_mode == RepeatMode::RepeatAll {
                self.shuffle_pos = self.shuffle_indices.len().saturating_sub(1);
            }
            let prev_idx = self.shuffle_indices[self.shuffle_pos];
            return self.play_index(prev_idx);
        }

        match self.repeat_mode {
            RepeatMode::RepeatAll => {
                let prev_idx = match self.current_index {
                    Some(0) => self.items.len().saturating_sub(1),
                    Some(idx) => idx - 1,
                    None => 0,
                };
                self.play_index(prev_idx)
            }
            RepeatMode::Off => {
                if let Some(idx) = self.current_index {
                    if idx > 0 {
                        return self.play_index(idx - 1);
                    }
                }
                None
            }
            RepeatMode::RepeatTrack => unreachable!(),
        }
    }

    pub fn cycle_repeat_mode(&mut self) -> RepeatMode {
        self.repeat_mode = match self.repeat_mode {
            RepeatMode::Off => RepeatMode::RepeatAll,
            RepeatMode::RepeatAll => RepeatMode::RepeatTrack,
            RepeatMode::RepeatTrack => RepeatMode::Off,
        };
        self.repeat_mode
    }

    pub fn toggle_shuffle(&mut self) -> bool {
        self.is_shuffle = !self.is_shuffle;
        if self.is_shuffle {
            self.rebuild_shuffle();
        }
        self.is_shuffle
    }

    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.is_shuffle = shuffle;
        if self.is_shuffle {
            self.rebuild_shuffle();
        }
    }

    pub fn is_shuffle(&self) -> bool {
        self.is_shuffle
    }

    pub fn repeat_label(&self) -> &'static str {
        match self.repeat_mode {
            RepeatMode::Off => "Repeat: Off",
            RepeatMode::RepeatAll => "Repeat: All",
            RepeatMode::RepeatTrack => "Repeat: One",
        }
    }

    pub fn remove_at(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
            if let Some(cur) = self.current_index {
                if cur == index {
                    if self.items.is_empty() {
                        self.current_index = None;
                    } else if cur >= self.items.len() {
                        self.current_index = Some(self.items.len() - 1);
                    }
                } else if cur > index {
                    self.current_index = Some(cur - 1);
                }
            }
            self.rebuild_shuffle();
        }
    }

    pub fn move_up(&mut self, index: usize) {
        if index > 0 && index < self.items.len() {
            self.items.swap(index, index - 1);
            if self.current_index == Some(index) {
                self.current_index = Some(index - 1);
            } else if self.current_index == Some(index - 1) {
                self.current_index = Some(index);
            }
            self.rebuild_shuffle();
        }
    }

    pub fn move_down(&mut self, index: usize) {
        if index + 1 < self.items.len() {
            self.items.swap(index, index + 1);
            if self.current_index == Some(index) {
                self.current_index = Some(index + 1);
            } else if self.current_index == Some(index + 1) {
                self.current_index = Some(index);
            }
            self.rebuild_shuffle();
        }
    }

    pub fn sort_natural(&mut self) {
        let current_path = self.current_item().map(|it| it.path.clone());
        self.items.sort_by(|a, b| compare(&a.title, &b.title));
        if let Some(path) = current_path {
            self.current_index = self.items.iter().position(|it| it.path == path);
        }
        self.rebuild_shuffle();
    }

    pub fn sort_by_name(&mut self, ascending: bool) {
        let current_path = self.current_item().map(|it| it.path.clone());
        if ascending {
            self.items.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
        } else {
            self.items.sort_by(|a, b| b.title.to_lowercase().cmp(&a.title.to_lowercase()));
        }
        if let Some(path) = current_path {
            self.current_index = self.items.iter().position(|it| it.path == path);
        }
        self.rebuild_shuffle();
    }

    pub fn sort_by_size(&mut self, ascending: bool) {
        let current_path = self.current_item().map(|it| it.path.clone());
        if ascending {
            self.items.sort_by_key(|a| a.file_size_bytes.unwrap_or(0));
        } else {
            self.items.sort_by_key(|a| std::cmp::Reverse(a.file_size_bytes.unwrap_or(0)));
        }
        if let Some(path) = current_path {
            self.current_index = self.items.iter().position(|it| it.path == path);
        }
        self.rebuild_shuffle();
    }

    pub fn shuffle_items(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::rng();
        let current_path = self.current_item().map(|it| it.path.clone());
        self.items.shuffle(&mut rng);
        if let Some(path) = current_path {
            self.current_index = self.items.iter().position(|it| it.path == path);
        }
        self.rebuild_shuffle();
    }

    fn rebuild_shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::rng();
        self.shuffle_indices = (0..self.items.len()).collect();
        self.shuffle_indices.shuffle(&mut rng);
        self.shuffle_pos = 0;
    }

    pub fn add_to_history(&mut self, path: &Path) {
        let p_buf = path.to_path_buf();
        self.history.retain(|p| p != &p_buf);
        self.history.insert(0, p_buf);
        if self.history.len() > 100 {
            self.history.truncate(100);
        }
    }

    pub fn toggle_favorite(&mut self, path: &Path) -> bool {
        let p_buf = path.to_path_buf();
        if let Some(pos) = self.favorites.iter().position(|p| p == &p_buf) {
            self.favorites.remove(pos);
            false
        } else {
            self.favorites.push(p_buf);
            true
        }
    }

    pub fn is_favorite(&self, path: &Path) -> bool {
        self.favorites.iter().any(|p| p == path)
    }

    pub fn filtered_indices(&self) -> Vec<usize> {
        let query = self.search_query.trim().to_lowercase();
        if query.is_empty() {
            (0..self.items.len()).collect()
        } else {
            self.items
                .iter()
                .enumerate()
                .filter(|(_, it)| it.title.to_lowercase().contains(&query))
                .map(|(idx, _)| idx)
                .collect()
        }
    }

    /// Probe durations for any unprobed items in the playlist
    pub fn probe_all_durations(&mut self) {
        for item in &mut self.items {
            if item.duration_secs.is_none() || item.duration_secs == Some(0.0) {
                item.duration_secs = probe_media_duration(&item.path);
            }
        }
    }
}

/// Pure-Rust lightning-fast media header parser that extracts accurate durations
/// from audio & video file headers (FLAC, WAV, MP3, MP4/M4A, OGG) in microseconds.
pub fn probe_media_duration(path: &Path) -> Option<f64> {
    use std::fs::File;
    use std::io::Read;

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let mut file = File::open(path).ok()?;
    let file_size = file.metadata().ok().map(|m| m.len()).unwrap_or(0);

    // Read first 64KB for container header parsing
    let mut buf = vec![0u8; 65536];
    let n = file.read(&mut buf).ok()?;
    if n < 12 {
        return None;
    }
    let data = &buf[..n];

    match ext.as_str() {
        "flac" => probe_flac_duration(data),
        "wav" => probe_wav_duration(data),
        "mp3" => probe_mp3_duration(data, file_size),
        "mp4" | "m4a" | "mov" | "aac" => probe_mp4_duration(data),
        "ogg" | "opus" => probe_ogg_duration(path),
        _ => {
            if data.starts_with(b"fLaC") {
                probe_flac_duration(data)
            } else if data.starts_with(b"RIFF") && data.len() >= 12 && &data[8..12] == b"WAVE" {
                probe_wav_duration(data)
            } else if data.starts_with(b"ID3") || (data[0] == 0xFF && (data[1] & 0xE0) == 0xE0) {
                probe_mp3_duration(data, file_size)
            } else {
                probe_mp4_duration(data)
            }
        }
    }
}

fn probe_flac_duration(data: &[u8]) -> Option<f64> {
    if data.len() < 26 || &data[0..4] != b"fLaC" {
        return None;
    }
    let b = &data[18..26];
    let sr = ((b[0] as u32) << 12) | ((b[1] as u32) << 4) | ((b[2] as u32) >> 4);
    let total_samples = (((b[3] & 0x0F) as u64) << 32)
        | ((b[4] as u64) << 24)
        | ((b[5] as u64) << 16)
        | ((b[6] as u64) << 8)
        | (b[7] as u64);
    if sr > 0 && total_samples > 0 {
        Some(total_samples as f64 / sr as f64)
    } else {
        None
    }
}

fn probe_wav_duration(data: &[u8]) -> Option<f64> {
    if data.len() < 36 || &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return None;
    }
    let mut byte_rate = 0u32;
    let mut data_size = 0u32;
    let mut pos = 12;
    while pos + 8 <= data.len() {
        let chunk_id = &data[pos..pos + 4];
        let chunk_sz = u32::from_le_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]) as usize;
        if chunk_id == b"fmt " && pos + 8 + 16 <= data.len() {
            byte_rate = u32::from_le_bytes([data[pos + 16], data[pos + 17], data[pos + 18], data[pos + 19]]);
        } else if chunk_id == b"data" {
            data_size = chunk_sz as u32;
            break;
        }
        pos += 8 + chunk_sz;
        if chunk_sz % 2 != 0 {
            pos += 1;
        }
    }
    if byte_rate > 0 && data_size > 0 {
        Some(data_size as f64 / byte_rate as f64)
    } else {
        None
    }
}

fn probe_mp3_duration(data: &[u8], file_size: u64) -> Option<f64> {
    // 1. Check ID3v2 TLEN (tag length in ms)
    if data.len() >= 10 && &data[0..3] == b"ID3" {
        let tag_size = (((data[6] as usize) & 0x7F) << 21)
            | (((data[7] as usize) & 0x7F) << 14)
            | (((data[8] as usize) & 0x7F) << 7)
            | ((data[9] as usize) & 0x7F);
        let id3_end = (10 + tag_size).min(data.len());
        let id3_slice = &data[10..id3_end];
        if let Some(tlen_pos) = id3_slice.windows(4).position(|w| w == b"TLEN") {
            if tlen_pos + 10 <= id3_slice.len() {
                let frame_sz = u32::from_be_bytes([
                    id3_slice[tlen_pos + 4],
                    id3_slice[tlen_pos + 5],
                    id3_slice[tlen_pos + 6],
                    id3_slice[tlen_pos + 7],
                ]) as usize;
                if tlen_pos + 10 + frame_sz <= id3_slice.len() {
                    let text_bytes = &id3_slice[tlen_pos + 10..tlen_pos + 10 + frame_sz];
                    let text = String::from_utf8_lossy(if text_bytes.len() > 1 { &text_bytes[1..] } else { text_bytes });
                    if let Ok(ms) = text.trim().trim_matches('\0').parse::<f64>() {
                        if ms > 0.0 {
                            return Some(ms / 1000.0);
                        }
                    }
                }
            }
        }
    }

    // 2. Check Xing / Info header for total frames
    let xing_pos = data.windows(4).position(|w| w == b"Xing" || w == b"Info");
    if let Some(pos) = xing_pos {
        if pos + 16 <= data.len() {
            let flags = u32::from_be_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]);
            if (flags & 0x0001) != 0 && pos + 12 <= data.len() {
                let frames = u32::from_be_bytes([data[pos + 8], data[pos + 9], data[pos + 10], data[pos + 11]]) as f64;
                if frames > 0.0 {
                    return Some(frames * 1152.0 / 44100.0);
                }
            }
        }
    }

    // 3. Fallback estimate from file size
    if file_size > 10000 {
        Some((file_size as f64 * 8.0) / (256.0 * 1000.0))
    } else {
        None
    }
}

fn probe_mp4_duration(data: &[u8]) -> Option<f64> {
    if let Some(pos) = data.windows(4).position(|w| w == b"mvhd") {
        if pos + 24 <= data.len() {
            let version = data[pos + 4];
            if version == 0 {
                let timescale = u32::from_be_bytes([data[pos + 16], data[pos + 17], data[pos + 18], data[pos + 19]]) as f64;
                let duration = u32::from_be_bytes([data[pos + 20], data[pos + 21], data[pos + 22], data[pos + 23]]) as f64;
                if timescale > 0.0 && duration > 0.0 {
                    return Some(duration / timescale);
                }
            } else if version == 1 && pos + 36 <= data.len() {
                let timescale = u32::from_be_bytes([data[pos + 24], data[pos + 25], data[pos + 26], data[pos + 27]]) as f64;
                let duration = u64::from_be_bytes([
                    data[pos + 28], data[pos + 29], data[pos + 30], data[pos + 31],
                    data[pos + 32], data[pos + 33], data[pos + 34], data[pos + 35],
                ]) as f64;
                if timescale > 0.0 && duration > 0.0 {
                    return Some(duration / timescale);
                }
            }
        }
    }
    None
}

fn probe_ogg_duration(path: &Path) -> Option<f64> {
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};
    let mut file = File::open(path).ok()?;
    let len = file.metadata().ok()?.len();
    if len < 8192 {
        return None;
    }
    let seek_pos = len - 8192;
    file.seek(SeekFrom::Start(seek_pos)).ok()?;
    let mut buf = vec![0u8; 8192];
    let n = file.read(&mut buf).ok()?;
    let data = &buf[..n];
    let mut last_ogg = None;
    for (i, w) in data.windows(4).enumerate() {
        if w == b"OggS" {
            last_ogg = Some(i);
        }
    }
    if let Some(pos) = last_ogg {
        if pos + 14 <= data.len() {
            let granule = u64::from_le_bytes([
                data[pos + 6], data[pos + 7], data[pos + 8], data[pos + 9],
                data[pos + 10], data[pos + 11], data[pos + 12], data[pos + 13],
            ]);
            if granule > 0 && granule < 1_000_000_000 {
                return Some(granule as f64 / 48000.0);
            }
        }
    }
    None
}
