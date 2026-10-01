#![allow(dead_code)]

use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct LyricLine {
    pub timestamp_sec: f64,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LyricTrack {
    pub lines: Vec<LyricLine>,
}

impl LyricTrack {
    pub fn from_lrc_file(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        Self::parse_lrc(&content)
    }

    pub fn parse_lrc(content: &str) -> Option<Self> {
        let mut lines = Vec::new();
        for raw_line in content.lines() {
            let trimmed = raw_line.trim();
            if trimmed.starts_with('[') {
                if let Some(close_idx) = trimmed.find(']') {
                    let time_part = &trimmed[1..close_idx];
                    let text = trimmed[close_idx + 1..].trim().to_string();
                    if let Some(sec) = Self::parse_timestamp(time_part) {
                        lines.push(LyricLine {
                            timestamp_sec: sec,
                            text,
                        });
                    }
                }
            }
        }
        if lines.is_empty() {
            None
        } else {
            lines.sort_by(|a, b| a.timestamp_sec.partial_cmp(&b.timestamp_sec).unwrap_or(std::cmp::Ordering::Equal));
            Some(Self { lines })
        }
    }

    fn parse_timestamp(s: &str) -> Option<f64> {
        // e.g. "01:23.45" or "01:23"
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 2 {
            let min: f64 = parts[0].parse().ok()?;
            let sec: f64 = parts[1].parse().ok()?;
            return Some(min * 60.0 + sec);
        }
        None
    }

    pub fn get_current_line(&self, current_time: f64) -> Option<(usize, &LyricLine)> {
        let mut best = None;
        for (i, line) in self.lines.iter().enumerate() {
            if line.timestamp_sec <= current_time {
                best = Some((i, line));
            } else {
                break;
            }
        }
        best
    }
}
