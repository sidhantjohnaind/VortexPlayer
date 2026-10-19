#![allow(dead_code)]

use std::collections::VecDeque;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct PlayNextQueue {
    pub queue: VecDeque<PathBuf>,
}

impl PlayNextQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_next(&mut self, path: PathBuf) {
        self.queue.push_back(path);
    }

    pub fn pop_next(&mut self) -> Option<PathBuf> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

pub struct SequenceDetector;

impl SequenceDetector {
    pub fn detect_next_part(current_path: &Path) -> Option<PathBuf> {
        let name = current_path.file_name()?.to_string_lossy();
        let parent = current_path.parent()?;

        // Detect part1 -> part2 or CD1 -> CD2, independent of casing.
        // Safe against multi-byte UTF-8 filenames by matching ASCII prefixes on char boundaries.
        // Avoid treating numbers with subsequent digits (such as "part10") as part1.
        for prefix in ["part", "cd"] {
            for (idx, _) in name.char_indices() {
                let tail = &name[idx..];
                let mut chars = tail.chars();
                let mut matches = true;
                for p_ch in prefix.chars() {
                    if chars.next().map(|c| c.to_ascii_lowercase()) != Some(p_ch) {
                        matches = false;
                        break;
                    }
                }
                if !matches {
                    continue;
                }
                if let Some(ch) = chars.next() {
                    if ch.is_ascii_digit() && ch != '0' {
                        if chars.next().is_some_and(|c| c.is_ascii_digit()) {
                            continue;
                        }
                        let current_digit = (ch as u8 - b'0') as u32;
                        let next_digit = (current_digit + 1).to_string();
                        let digit_idx = idx + prefix.len();
                        let mut next_name = name.to_string();
                        next_name.replace_range(digit_idx..digit_idx + 1, &next_digit);
                        let next_path = parent.join(next_name);
                        if next_path.exists() {
                            return Some(next_path);
                        }
                    }
                }
            }
        }
        None
    }
}
