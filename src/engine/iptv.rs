#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IptvChannel {
    pub name: String,
    pub url: String,
    pub logo: Option<String>,
    pub group: String,
}

pub struct IptvManager {
    pub channels: Vec<IptvChannel>,
}

impl Default for IptvManager {
    fn default() -> Self {
        Self {
            channels: Vec::new(),
        }
    }
}

impl IptvManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_m3u(&mut self, path: &Path) -> Result<usize, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        self.parse_m3u(&content);
        Ok(self.channels.len())
    }

    pub fn parse_m3u(&mut self, content: &str) {
        self.channels.clear();
        let mut current_name = String::new();
        let mut current_logo = None;
        let mut current_group = "General".to_string();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("#EXTINF:") {
                if let Some(comma_pos) = trimmed.rfind(',') {
                    current_name = trimmed[comma_pos + 1..].trim().to_string();
                }
                if let Some(logo_pos) = trimmed.find("tvg-logo=\"") {
                    let rem = &trimmed[logo_pos + 10..];
                    if let Some(end_quote) = rem.find('\"') {
                        current_logo = Some(rem[..end_quote].to_string());
                    }
                }
                if let Some(grp_pos) = trimmed.find("group-title=\"") {
                    let rem = &trimmed[grp_pos + 13..];
                    if let Some(end_quote) = rem.find('\"') {
                        current_group = rem[..end_quote].to_string();
                    }
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with('#') {
                self.channels.push(IptvChannel {
                    name: if current_name.is_empty() { "Stream Channel".to_string() } else { current_name.clone() },
                    url: trimmed.to_string(),
                    logo: current_logo.clone(),
                    group: current_group.clone(),
                });
                current_name.clear();
                current_logo = None;
                current_group = "General".to_string();
            }
        }
    }
}
