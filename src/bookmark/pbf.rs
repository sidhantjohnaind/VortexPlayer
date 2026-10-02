use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BookmarkItem {
    pub time_pos: f64, // in seconds
    pub title: String,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AbLoopSegment {
    pub point_a: Option<f64>,
    pub point_b: Option<f64>,
    pub is_active: bool,
    pub loop_count: usize,
    pub current_iteration: usize,
    pub repeat_delay_secs: f64,
}

pub struct PbfFile;

impl PbfFile {
    /// Convert a video file path to its corresponding .pbf path
    pub fn pbf_path_for_video(video_path: &Path) -> PathBuf {
        let mut pbf_path = video_path.to_path_buf();
        pbf_path.set_extension("pbf");
        pbf_path
    }

    /// Parse a Vortex .pbf bookmark file
    pub fn parse(content: &str) -> Vec<BookmarkItem> {
        let mut bookmarks = Vec::new();
        let mut in_bookmark_section = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.eq_ignore_ascii_case("[Bookmark]") {
                in_bookmark_section = true;
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                in_bookmark_section = false;
                continue;
            }

            if in_bookmark_section {
                // Line format: 0=123456*Title*0 or 0=123456*Title
                if let Some((idx_str, data)) = trimmed.split_once('=') {
                    let index = idx_str.trim().parse::<usize>().unwrap_or(bookmarks.len());
                    let parts: Vec<&str> = data.split('*').collect();
                    if let Some(ms_str) = parts.first() {
                        if let Ok(ms) = ms_str.trim().parse::<f64>() {
                            let time_pos = ms / 1000.0;
                            let title = if parts.len() > 1 && !parts[1].trim().is_empty() {
                                parts[1].trim().to_string()
                            } else {
                                format!("Bookmark at {:.1}s", time_pos)
                            };
                            bookmarks.push(BookmarkItem {
                                time_pos,
                                title,
                                index,
                            });
                        }
                    }
                }
            }
        }

        bookmarks.sort_by(|a, b| a.time_pos.partial_cmp(&b.time_pos).unwrap_or(std::cmp::Ordering::Equal));
        // Re-index sequentially
        for (i, b) in bookmarks.iter_mut().enumerate() {
            b.index = i;
        }

        bookmarks
    }

    /// Load bookmarks from a .pbf file on disk
    pub fn load_for_video(video_path: &Path) -> Option<Vec<BookmarkItem>> {
        let pbf_path = Self::pbf_path_for_video(video_path);
        if pbf_path.exists() {
            if let Ok(content) = fs::read_to_string(&pbf_path) {
                let items = Self::parse(&content);
                if !items.is_empty() {
                    return Some(items);
                }
            }
        }
        None
    }

    /// Serialize bookmarks into Vortex .pbf format
    pub fn serialize(bookmarks: &[BookmarkItem]) -> String {
        let mut out = String::from("[Bookmark]\n");
        let mut sorted = bookmarks.to_vec();
        sorted.sort_by(|a, b| a.time_pos.partial_cmp(&b.time_pos).unwrap_or(std::cmp::Ordering::Equal));

        for (idx, bm) in sorted.iter().enumerate() {
            let ms = (bm.time_pos * 1000.0).round() as u64;
            out.push_str(&format!("{}={}*{}*0\n", idx, ms, bm.title));
        }

        out
    }

    /// Save bookmarks to a .pbf file on disk alongside the video
    pub fn save_for_video(video_path: &Path, bookmarks: &[BookmarkItem]) -> Result<PathBuf, std::io::Error> {
        let pbf_path = Self::pbf_path_for_video(video_path);
        if bookmarks.is_empty() {
            if pbf_path.exists() {
                let _ = fs::remove_file(&pbf_path);
            }
            return Ok(pbf_path);
        }
        let content = Self::serialize(bookmarks);
        fs::write(&pbf_path, content)?;
        Ok(pbf_path)
    }
}
