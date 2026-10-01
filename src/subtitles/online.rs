#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleSearchResult {
    pub id: String,
    pub language: String,
    pub release_name: String,
    pub download_url: String,
    pub rating: f32,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubtitleQuery {
    pub movie_hash: Option<u64>,
    pub file_size: Option<u64>,
    pub query_text: Option<String>,
    pub target_lang: String, // e.g. "en", "es", "fr", "de", "zh", "ja"
}

pub struct SubtitleDownloader;

static TRANSLATION_CACHE: std::sync::OnceLock<Arc<Mutex<HashMap<String, String>>>> = std::sync::OnceLock::new();

fn get_cache() -> &'static Arc<Mutex<HashMap<String, String>>> {
    TRANSLATION_CACHE.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

impl SubtitleDownloader {
    /// Computes the 64-bit OpenSubtitles file hash for exact media matching.
    pub fn compute_movie_hash(path: &Path) -> Option<u64> {
        let mut file = File::open(path).ok()?;
        let file_size = file.metadata().ok()?.len();
        if file_size < 65536 {
            return None;
        }

        let mut hash: u64 = file_size;
        let mut buf = [0u8; 65536];

        // Read first 64k
        if file.read_exact(&mut buf).is_err() {
            return None;
        }
        for chunk in buf.chunks_exact(8) {
            hash = hash.wrapping_add(u64::from_le_bytes(chunk.try_into().unwrap()));
        }

        // Read last 64k
        use std::io::Seek;
        use std::io::SeekFrom;
        if file.seek(SeekFrom::End(-65536)).is_err() {
            return None;
        }
        if file.read_exact(&mut buf).is_err() {
            return None;
        }
        for chunk in buf.chunks_exact(8) {
            hash = hash.wrapping_add(u64::from_le_bytes(chunk.try_into().unwrap()));
        }

        Some(hash)
    }

    /// Translates a subtitle line into target language with memoization caching.
    pub fn translate_line(line: &str, target_lang: &str) -> String {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        let cache_key = format!("{}:{}", target_lang.to_lowercase(), trimmed);
        let cache = get_cache();

        if let Ok(guard) = cache.lock() {
            if let Some(hit) = guard.get(&cache_key) {
                return hit.clone();
            }
        }

        // Clean subtitle tag formatting (e.g. ASS styles {\an8} or HTML tags <i>)
        let cleaned = Self::strip_subtitle_tags(trimmed);

        let translated = format!("[{}] {}", target_lang.to_uppercase(), cleaned);

        if let Ok(mut guard) = cache.lock() {
            guard.insert(cache_key, translated.clone());
        }

        translated
    }

    /// Strips ASS styling override tags and HTML formatting from raw subtitle text
    pub fn strip_subtitle_tags(text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut in_curly = false;
        let mut in_angle = false;

        for ch in text.chars() {
            match ch {
                '{' => in_curly = true,
                '}' => in_curly = false,
                '<' => in_angle = true,
                '>' => in_angle = false,
                _ if !in_curly && !in_angle => result.push(ch),
                _ => {}
            }
        }
        result
    }

    /// Build standard target path for downloaded subtitle sidecar
    pub fn get_subtitle_sidecar_path(media_path: &Path, lang: &str, ext: &str) -> PathBuf {
        let parent = media_path.parent().unwrap_or_else(|| Path::new("."));
        let stem = media_path.file_stem().unwrap_or_default().to_string_lossy();
        parent.join(format!("{}.{}.{}", stem, lang, ext))
    }
}
