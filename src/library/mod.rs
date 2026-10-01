#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub path: PathBuf,
    pub title: String,
    pub poster_path: Option<PathBuf>,
    pub is_tv_show: bool,
    pub season: Option<u32>,
    pub episode: Option<u32>,
    pub duration_secs: f64,
    pub last_played_pos: f64,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaLibrary {
    pub watch_folders: Vec<PathBuf>,
    pub items: Vec<MediaItem>,
}

impl MediaLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scan_folder(&mut self, folder: &Path) {
        if !folder.exists() || !folder.is_dir() {
            return;
        }

        let media_exts = ["mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "ts", "m2ts", "vob"];
        let poster_names = ["poster.jpg", "cover.jpg", "folder.jpg", "poster.png", "cover.png", "folder.png"];

        // Look for folder-level poster
        let mut folder_poster = None;
        for p_name in poster_names {
            let p_path = folder.join(p_name);
            if p_path.exists() {
                folder_poster = Some(p_path);
                break;
            }
        }

        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    self.scan_folder(&path);
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if media_exts.contains(&ext.to_lowercase().as_str()) {
                        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown").to_string();
                        
                        // Parse Season & Episode (e.g. S01E02)
                        let (is_tv, season, ep) = Self::parse_season_episode(&file_stem);

                        // Look for per-file poster (e.g. Movie.jpg)
                        let mut item_poster = folder_poster.clone();
                        for p_ext in &["jpg", "png", "webp"] {
                            let sidecar = path.with_extension(p_ext);
                            if sidecar.exists() {
                                item_poster = Some(sidecar);
                                break;
                            }
                        }

                        // Avoid duplicate paths
                        if !self.items.iter().any(|i| i.path == path) {
                            self.items.push(MediaItem {
                                path,
                                title: file_stem,
                                poster_path: item_poster,
                                is_tv_show: is_tv,
                                season,
                                episode: ep,
                                duration_secs: 0.0,
                                last_played_pos: 0.0,
                                is_favorite: false,
                            });
                        }
                    }
                }
            }
        }
    }

    fn parse_season_episode(name: &str) -> (bool, Option<u32>, Option<u32>) {
        let name_upper = name.to_uppercase();
        if let Some(s_idx) = name_upper.find('S') {
            if let Some(e_idx) = name_upper[s_idx..].find('E') {
                let actual_e = s_idx + e_idx;
                let s_str = &name_upper[s_idx + 1..actual_e];
                let mut e_end = actual_e + 1;
                while e_end < name_upper.len() && name_upper.as_bytes()[e_end].is_ascii_digit() {
                    e_end += 1;
                }
                let e_str = &name_upper[actual_e + 1..e_end];

                if let (Ok(s), Ok(e)) = (s_str.parse::<u32>(), e_str.parse::<u32>()) {
                    return (true, Some(s), Some(e));
                }
            }
        }
        (false, None, None)
    }
}
