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

        // Detect part1 -> part2 or CD1 -> CD2, independent of casing. Avoid
        // treating names such as "part10" as "part1".
        for marker in ["part1", "cd1"] {
            let lower_name = name.to_lowercase();
            let Some(start) = lower_name.find(marker) else { continue };
            let end = start + marker.len();
            if lower_name[end..].chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
                continue;
            }

            let mut next_name = name.to_string();
            // Replace only the sequence digit so the original casing is kept
            // (e.g. "Part1" becomes "Part2").
            next_name.replace_range(end - 1..end, "2");
            let next_path = parent.join(next_name);
            if next_path.exists() {
                return Some(next_path);
            }
        }
        None
    }
}
