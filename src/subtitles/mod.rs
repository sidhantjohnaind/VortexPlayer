#![allow(unused_imports)]

pub mod controller;
pub mod online;
pub mod sami;

pub use controller::{SubtitleController, SubtitleLayerState};
pub use online::{SubtitleDownloader, SubtitleQuery, SubtitleSearchResult};
pub use sami::{SamiParser, SubtitleEntry};

use std::path::{Path, PathBuf};

/// Helper to parse timestamp in SRT or WebVTT format (e.g. "00:01:23,456" or "00:01:23.456" or "01:23.456")
pub fn parse_timestamp_srt_vtt(ts: &str) -> Option<f64> {
    let clean = ts.trim().replace(',', ".");
    let parts: Vec<&str> = clean.split(':').collect();
    match parts.len() {
        3 => {
            let h: f64 = parts[0].trim().parse().ok()?;
            let m: f64 = parts[1].trim().parse().ok()?;
            let s: f64 = parts[2].trim().parse().ok()?;
            Some(h * 3600.0 + m * 60.0 + s)
        }
        2 => {
            let m: f64 = parts[0].trim().parse().ok()?;
            let s: f64 = parts[1].trim().parse().ok()?;
            Some(m * 60.0 + s)
        }
        _ => None,
    }
}

/// Helper to parse timestamp in ASS/SSA format (e.g. "0:01:23.45")
pub fn parse_timestamp_ass(ts: &str) -> Option<f64> {
    let clean = ts.trim();
    let parts: Vec<&str> = clean.split(':').collect();
    if parts.len() == 3 {
        let h: f64 = parts[0].trim().parse().ok()?;
        let m: f64 = parts[1].trim().parse().ok()?;
        let s: f64 = parts[2].trim().parse().ok()?;
        Some(h * 3600.0 + m * 60.0 + s)
    } else {
        None
    }
}

/// Parse raw subtitle text from an SRT, WebVTT, ASS, or SSA file into sorted (start, end, dialogue) cues
pub fn parse_subtitles_from_content(content: &str) -> Vec<(f64, f64, String)> {
    let mut lines = Vec::new();
    let is_ass = content.contains("[Events]") || content.lines().any(|l| l.trim_start().starts_with("Dialogue:"));

    if is_ass {
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("Dialogue:") {
                // Fields separated by commas:
                // Format: Marked/Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
                let parts: Vec<&str> = rest.splitn(10, ',').collect();
                if parts.len() >= 10 {
                    let start = parse_timestamp_ass(parts[1]).unwrap_or(0.0);
                    let end = parse_timestamp_ass(parts[2]).unwrap_or(start + 3.0);
                    let raw_text = parts[9];
                    let clean = SubtitleDownloader::strip_subtitle_tags(raw_text)
                        .replace(r"\N", " ")
                        .replace(r"\n", " ")
                        .replace('\n', " ")
                        .trim()
                        .to_string();
                    if !clean.is_empty() {
                        lines.push((start, end, clean));
                    }
                }
            }
        }
    } else {
        // SRT / WebVTT
        let mut cur_start = 0.0;
        let mut cur_end = 0.0;
        let mut cur_text = Vec::new();
        let mut in_cue = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.contains("-->") {
                if in_cue && !cur_text.is_empty() {
                    let joined = cur_text.join(" ");
                    let clean = SubtitleDownloader::strip_subtitle_tags(&joined)
                        .replace(r"\N", " ")
                        .replace(r"\n", " ")
                        .replace('\n', " ")
                        .trim()
                        .to_string();
                    if !clean.is_empty() {
                        lines.push((cur_start, cur_end, clean));
                    }
                    cur_text.clear();
                }

                let arrow_parts: Vec<&str> = trimmed.split("-->").collect();
                if arrow_parts.len() == 2 {
                    let start_str = arrow_parts[0].trim();
                    let end_str = arrow_parts[1].trim().split_whitespace().next().unwrap_or("");
                    if let (Some(s), Some(e)) = (parse_timestamp_srt_vtt(start_str), parse_timestamp_srt_vtt(end_str)) {
                        cur_start = s;
                        cur_end = e;
                        in_cue = true;
                    }
                }
            } else if in_cue {
                if trimmed.is_empty() {
                    if !cur_text.is_empty() {
                        let joined = cur_text.join(" ");
                        let clean = SubtitleDownloader::strip_subtitle_tags(&joined)
                            .replace(r"\N", " ")
                            .replace(r"\n", " ")
                            .replace('\n', " ")
                            .trim()
                            .to_string();
                        if !clean.is_empty() {
                            lines.push((cur_start, cur_end, clean));
                        }
                        cur_text.clear();
                    }
                    in_cue = false;
                } else if !trimmed.chars().all(|c| c.is_ascii_digit()) {
                    cur_text.push(trimmed.to_string());
                }
            }
        }

        if in_cue && !cur_text.is_empty() {
            let joined = cur_text.join(" ");
            let clean = SubtitleDownloader::strip_subtitle_tags(&joined)
                .replace(r"\N", " ")
                .replace(r"\n", " ")
                .replace('\n', " ")
                .trim()
                .to_string();
            if !clean.is_empty() {
                lines.push((cur_start, cur_end, clean));
            }
        }
    }

    lines.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    lines
}

/// Load and parse subtitles from an on-disk file (.srt, .vtt, .ass, .ssa, .smi)
pub fn load_subtitles_from_file(path: &Path) -> Result<Vec<(f64, f64, String)>, std::io::Error> {
    if path.to_string_lossy().ends_with(".smi") {
        if let Ok(srt) = SamiParser::load_sami_file_as_srt(path) {
            return Ok(parse_subtitles_from_content(&srt));
        }
    }

    let bytes = std::fs::read(path)?;
    let content = match String::from_utf8(bytes.clone()) {
        Ok(s) => s,
        Err(_) => String::from_utf8_lossy(&bytes).to_string(),
    };
    Ok(parse_subtitles_from_content(&content))
}

/// Auto-discover and load matching subtitle sidecars for a given media file in the same directory
pub fn find_and_load_matching_subtitles(media_path: &Path) -> Option<(PathBuf, Vec<(f64, f64, String)>)> {
    let parent = media_path.parent()?;
    let stem = media_path.file_stem()?.to_string_lossy().to_lowercase();

    // Priority candidates
    let sub_exts = ["srt", "ass", "ssa", "vtt", "smi"];
    let lang_tags = ["", ".en", ".eng", ".default", ".forced", ".ja", ".jpn", ".zh", ".es"];

    for ext in &sub_exts {
        for lang in &lang_tags {
            let candidate_name = format!("{}{}.{}", stem, lang, ext);
            let candidate_path = parent.join(&candidate_name);
            if candidate_path.is_file() {
                if let Ok(lines) = load_subtitles_from_file(&candidate_path) {
                    if !lines.is_empty() {
                        return Some((candidate_path, lines));
                    }
                }
            }
        }
    }

    // Fallback: check all subtitle files in the directory
    if let Ok(entries) = std::fs::read_dir(parent) {
        for entry in entries.flatten() {
            let p = entry.path();
            if let Some(ext) = p.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
                if sub_exts.contains(&ext.as_str()) {
                    let fname = p.file_stem().unwrap_or_default().to_string_lossy().to_lowercase();
                    if fname.starts_with(&stem) || stem.starts_with(&fname) {
                        if let Ok(lines) = load_subtitles_from_file(&p) {
                            if !lines.is_empty() {
                                return Some((p, lines));
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_srt() {
        let sample = "1\n00:00:10,500 --> 00:00:13,000\nHello, world!\n\n2\n00:00:15,000 --> 00:00:18,500\nSecond dialogue line\n";
        let lines = parse_subtitles_from_content(sample);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].0, 10.5);
        assert_eq!(lines[0].1, 13.0);
        assert_eq!(lines[0].2, "Hello, world!");
        assert_eq!(lines[1].0, 15.0);
        assert_eq!(lines[1].2, "Second dialogue line");
    }

    #[test]
    fn test_parse_ass() {
        let sample = "[Events]\nFormat: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\nDialogue: 0,0:01:23.50,0:01:26.00,Default,,0,0,0,,{\\an8}Welcome to anime!\n";
        let lines = parse_subtitles_from_content(sample);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].0, 83.5);
        assert_eq!(lines[0].1, 86.0);
        assert_eq!(lines[0].2, "Welcome to anime!");
    }
}
