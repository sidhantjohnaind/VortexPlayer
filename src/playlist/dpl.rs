#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct DplItem {
    pub path: PathBuf,
    pub title: String,
    pub duration_ms: i64,
    pub start_pos_ms: i64,
    pub played_time_ms: i64,
}

pub struct DplFile;

impl DplFile {
    /// Parse a Daum Vortex Playlist (.dpl) content
    pub fn parse(content: &str) -> Vec<DplItem> {
        let mut items = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for line in lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with("DAUMPLAYLIST") || line.starts_with("playname=") || line.starts_with("playtime=") || line.starts_with("topindex=") || line.starts_with("saveplaypos=") {
                continue;
            }

            // Line format: 1*file*D:\Path\To\Video.mkv*Video Title*0*1450000*0
            if let Some(first_star) = line.find('*') {
                let rest = &line[first_star + 1..];
                let tokens: Vec<&str> = rest.split('*').collect();

                if tokens.len() >= 2 && tokens[0] == "file" {
                    let file_path = PathBuf::from(tokens[1]);
                    let title = if tokens.len() >= 3 && !tokens[2].is_empty() {
                        tokens[2].to_string()
                    } else {
                        file_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Track")
                            .to_string()
                    };

                    let duration_ms = if tokens.len() >= 6 {
                        tokens[4].parse::<i64>().unwrap_or(0)
                    } else {
                        0
                    };

                    let played_time_ms = if tokens.len() >= 7 {
                        tokens[5].parse::<i64>().unwrap_or(0)
                    } else {
                        0
                    };

                    items.push(DplItem {
                        path: file_path,
                        title,
                        duration_ms,
                        start_pos_ms: 0,
                        played_time_ms,
                    });
                }
            }
        }

        items
    }

    /// Serialize items into Daum Vortex Playlist (.dpl) format
    pub fn serialize(items: &[DplItem]) -> String {
        let mut out = String::from("DAUMPLAYLIST\r\n");
        if let Some(first) = items.first() {
            out.push_str(&format!("playname={}\r\n", first.title));
            out.push_str(&format!("playtime={}\r\n", first.duration_ms));
        }
        out.push_str("topindex=0\r\nsaveplaypos=0\r\n");

        for (i, item) in items.iter().enumerate() {
            out.push_str(&format!(
                "{}*file*{}*{}*0*{}*{}\r\n",
                i + 1,
                item.path.to_string_lossy(),
                item.title,
                item.duration_ms,
                item.played_time_ms
            ));
        }

        out
    }

    pub fn load_from_file(file_path: &Path) -> Result<Vec<DplItem>, std::io::Error> {
        let content = fs::read_to_string(file_path)?;
        Ok(Self::parse(&content))
    }

    pub fn save_to_file(file_path: &Path, items: &[DplItem]) -> Result<(), std::io::Error> {
        let content = Self::serialize(items);
        fs::write(file_path, content)
    }
}
