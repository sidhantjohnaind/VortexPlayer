#![allow(dead_code)]

use std::path::Path;

pub struct ArchiveEngine;

impl ArchiveEngine {
    pub const ARCHIVE_EXTENSIONS: [&'static str; 3] = ["zip", "rar", "7z"];

    pub fn is_archive(path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            Self::ARCHIVE_EXTENSIONS.contains(&ext.to_lowercase().as_str())
        } else {
            false
        }
    }

    pub fn build_archive_url(path: &Path, internal_file: &str) -> String {
        format!("archive://{}|{}", path.to_string_lossy(), internal_file)
    }
}
