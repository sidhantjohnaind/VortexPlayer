#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct M3uItem {
    pub path: PathBuf,
    pub title: String,
    pub duration_secs: f64,
}

pub struct M3uFile;

impl M3uFile {
    /// Parse M3U / M3U8 content
    pub fn parse(content: &str, base_dir: Option<&Path>) -> Vec<M3uItem> {
        let mut items = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_title = String::new();
        let mut current_duration = 0.0;

        for line in lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with("#EXTM3U") {
                continue;
            }

            if line.starts_with("#EXTINF:") {
                let inf_body = &line[8..];
                if let Some(comma_pos) = inf_body.find(',') {
                    let dur_str = &inf_body[..comma_pos].trim();
                    current_duration = dur_str.parse::<f64>().unwrap_or(0.0);
                    current_title = inf_body[comma_pos + 1..].trim().to_string();
                } else {
                    current_title = inf_body.trim().to_string();
                }
            } else if !line.starts_with('#') {
                let path = if line.starts_with("http://") || line.starts_with("https://") || line.starts_with("rtsp://") {
                    PathBuf::from(line)
                } else if let Some(base) = base_dir {
                    let p = Path::new(line);
                    if p.is_absolute() {
                        p.to_path_buf()
                    } else {
                        base.join(p)
                    }
                } else {
                    PathBuf::from(line)
                };

                let title = if !current_title.is_empty() {
                    current_title.clone()
                } else {
                    path.file_name().and_then(|n| n.to_str()).unwrap_or("Track").to_string()
                };

                items.push(M3uItem {
                    path,
                    title,
                    duration_secs: current_duration,
                });

                current_title.clear();
                current_duration = 0.0;
            }
        }

        items
    }

    /// Serialize items into Extended M3U8 format
    pub fn serialize(items: &[M3uItem]) -> String {
        let mut out = String::from("#EXTM3U\r\n");
        for item in items {
            let dur = if item.duration_secs > 0.0 { item.duration_secs as i64 } else { -1 };
            out.push_str(&format!("#EXTINF:{},{}\r\n", dur, item.title));
            out.push_str(&format!("{}\r\n", item.path.to_string_lossy()));
        }
        out
    }

    pub fn load_from_file(file_path: &Path) -> Result<Vec<M3uItem>, std::io::Error> {
        let content = fs::read_to_string(file_path)?;
        let base_dir = file_path.parent();
        Ok(Self::parse(&content, base_dir))
    }

    pub fn save_to_file(file_path: &Path, items: &[M3uItem]) -> Result<(), std::io::Error> {
        let content = Self::serialize(items);
        fs::write(file_path, content)
    }
}
