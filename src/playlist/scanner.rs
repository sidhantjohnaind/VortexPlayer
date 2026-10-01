use natord::compare;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub const SUPPORTED_VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "ts", "m2ts", "mts", "vob",
    "ogv", "3gp", "3g2", "asf", "rm", "rmvb", "f4v", "divx", "mpg", "mpeg", "m1v", "m2v",
    "mpv", "dv",
];

pub const SUPPORTED_AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "aac", "wav", "m4a", "ogg", "opus", "wma", "alac", "aiff", "ape", "ac3", "dts",
];

pub const SUPPORTED_SUBTITLE_EXTENSIONS: &[&str] = &[
    "srt", "ass", "ssa", "vtt", "sub", "idx", "sup", "smi", "rt", "lrc",
];

pub fn is_media_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        SUPPORTED_VIDEO_EXTENSIONS.contains(&ext_lower.as_str())
            || SUPPORTED_AUDIO_EXTENSIONS.contains(&ext_lower.as_str())
    } else {
        false
    }
}

pub fn is_subtitle_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        SUPPORTED_SUBTITLE_EXTENSIONS.contains(&ext_lower.as_str())
    } else {
        false
    }
}

/// Find all neighboring media files in the same directory as the given file, sorted naturally
pub fn scan_neighboring_episodes(file_path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Some(parent_dir) = file_path.parent() {
        if let Ok(entries) = fs::read_dir(parent_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && is_media_file(&path) {
                    files.push(path);
                }
            }
        }
    }

    // Natural alphanumeric sort
    files.sort_by(|a, b| {
        let name_a = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let name_b = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
        compare(name_a, name_b)
    });

    files
}

/// Recursively scan a folder for media files
pub fn scan_folder_recursive(folder_path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(folder_path)
        .follow_links(false)
        .max_depth(10)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && is_media_file(path) {
            files.push(path.to_path_buf());
        }
    }

    files.sort_by(|a, b| {
        let str_a = a.to_string_lossy();
        let str_b = b.to_string_lossy();
        compare(&str_a, &str_b)
    });

    files
}

/// Check for subtitle files with matching or similar base name in same directory
pub fn find_associated_subtitles(video_path: &Path) -> Vec<PathBuf> {
    let mut subs = Vec::new();
    if let Some(parent) = video_path.parent() {
        let stem = video_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if let Ok(entries) = fs::read_dir(parent) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && is_subtitle_file(&path) {
                    let sub_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    if sub_stem.starts_with(stem) {
                        subs.push(path);
                    }
                }
            }
        }
    }
    subs
}

/// Detect split/multi-part sibling files (e.g. CD1/CD2, Part1/Part2, _1/_2)
pub fn detect_multipart_siblings(file_path: &Path) -> Vec<PathBuf> {
    let mut siblings = Vec::new();
    let file_stem = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

    // Check for common multi-part markers
    let markers = ["cd1", "part1", "pt1", "disc1", "disk1", "_1", "-1", ".1"];
    let has_part1 = markers.iter().any(|&m| file_stem.contains(m));

    if has_part1 {
        let all_eps = scan_neighboring_episodes(file_path);
        for ep in all_eps {
            let ep_stem = ep.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
            // Check if it shares the common prefix
            let prefix_len = file_stem.len().saturating_sub(4);
            if prefix_len > 0 && ep_stem.starts_with(&file_stem[..prefix_len]) {
                siblings.push(ep);
            }
        }
    }

    siblings
}
