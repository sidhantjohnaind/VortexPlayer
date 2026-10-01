#![allow(dead_code)]

use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct SubtitleEntry {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
}

pub struct SamiParser;

impl SamiParser {
    /// Case-insensitive ASCII search that strictly operates on char boundaries of haystack
    fn find_ascii_ci(haystack: &str, needle: &str) -> Option<usize> {
        let n_len = needle.len();
        if haystack.len() < n_len {
            return None;
        }
        haystack.char_indices().find_map(|(idx, _)| {
            if haystack.len() - idx >= n_len && haystack[idx..idx + n_len].eq_ignore_ascii_case(needle) {
                Some(idx)
            } else {
                None
            }
        })
    }

    /// Parse SAMI (.smi) content into SubtitleEntry items
    pub fn parse(content: &str) -> Vec<SubtitleEntry> {
        let mut entries = Vec::new();
        let mut current_sync_start = None;
        let mut current_text = String::new();

        let mut pos = 0;

        while pos < content.len() {
            let remaining = &content[pos..];
            let sync_idx = match Self::find_ascii_ci(remaining, "<sync") {
                Some(idx) => idx,
                None => break,
            };
            let actual_idx = pos + sync_idx;

            // Extract new sync start time
            let start_tag_slice = &content[actual_idx..];
            let mut new_sync_time = None;
            if let Some(gt_idx) = start_tag_slice.find('>') {
                let tag_inside = &start_tag_slice[..gt_idx];
                if let Some(start_val_idx) = Self::find_ascii_ci(tag_inside, "start=") {
                    let val_str = tag_inside[start_val_idx + 6..].trim_start_matches(|c: char| c == '"' || c == '\'' || c.is_ascii_whitespace());
                    let digits: String = val_str.chars().take_while(|c| c.is_ascii_digit()).collect();
                    if let Ok(ms) = digits.parse::<i64>() {
                        new_sync_time = Some(ms);
                    }
                }

                // If we had a previous pending sync, save it with end_ms = new_sync_time
                if let Some(start) = current_sync_start {
                    let clean_text = Self::clean_sami_html(&current_text);
                    if !clean_text.is_empty() && clean_text != "&nbsp;" {
                        let end = match new_sync_time {
                            Some(nxt) if nxt > start => nxt,
                            _ => start + 3000,
                        };
                        entries.push(SubtitleEntry {
                            start_ms: start,
                            end_ms: end,
                            text: clean_text,
                        });
                    }
                }

                current_sync_start = new_sync_time;
                pos = actual_idx + gt_idx + 1;
                if pos < content.len() {
                    let next_slice = &content[pos..];
                    let next_sync = Self::find_ascii_ci(next_slice, "<sync").unwrap_or(next_slice.len());
                    current_text = next_slice[..next_sync].to_string();
                    pos += next_sync;
                } else {
                    current_text.clear();
                    break;
                }
            } else {
                break;
            }
        }

        // Final entry
        if let Some(start) = current_sync_start {
            let clean_text = Self::clean_sami_html(&current_text);
            if !clean_text.is_empty() && clean_text != "&nbsp;" {
                entries.push(SubtitleEntry {
                    start_ms: start,
                    end_ms: start + 3000,
                    text: clean_text,
                });
            }
        }

        // Ensure non-overlapping end timestamps
        for i in 0..entries.len() {
            if i + 1 < entries.len() {
                let next_start = entries[i + 1].start_ms;
                if entries[i].end_ms > next_start {
                    entries[i].end_ms = next_start;
                }
            }
        }

        entries
    }

    fn clean_sami_html(text: &str) -> String {
        let mut clean = String::new();
        let mut in_tag = false;

        for c in text.chars() {
            if c == '<' {
                in_tag = true;
            } else if c == '>' {
                in_tag = false;
            } else if !in_tag {
                clean.push(c);
            }
        }

        clean
            .replace("&nbsp;", " ")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
            .replace("\r\n", " ")
            .replace('\n', " ")
            .trim()
            .to_string()
    }

    pub fn convert_to_srt(entries: &[SubtitleEntry]) -> String {
        let mut out = String::new();
        for (i, entry) in entries.iter().enumerate() {
            let start_str = Self::format_srt_time(entry.start_ms);
            let end_str = Self::format_srt_time(entry.end_ms);
            out.push_str(&format!("{}\r\n{} --> {}\r\n{}\r\n\r\n", i + 1, start_str, end_str, entry.text));
        }
        out
    }

    fn format_srt_time(ms: i64) -> String {
        let total_secs = ms / 1000;
        let millis = ms % 1000;
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}:{:02},{:03}", hours, mins, secs, millis)
    }

    pub fn load_sami_file_as_srt(path: &Path) -> Result<String, std::io::Error> {
        let content = fs::read_to_string(path)?;
        let entries = Self::parse(&content);
        Ok(Self::convert_to_srt(&entries))
    }
}
