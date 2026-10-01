#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterItem {
    pub index: i64,
    pub title: String,
    pub time_pos: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ChapterManager {
    pub chapters: Vec<ChapterItem>,
    pub current_chapter_index: Option<i64>,
}

impl ChapterManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_chapters(&mut self, chapters: Vec<ChapterItem>) {
        self.chapters = chapters;
    }

    pub fn current_chapter(&self, time_pos: f64) -> Option<&ChapterItem> {
        let mut current = None;
        for ch in &self.chapters {
            if ch.time_pos <= time_pos + 0.5 {
                current = Some(ch);
            } else {
                break;
            }
        }
        current
    }

    pub fn next_chapter(&self, time_pos: f64) -> Option<f64> {
        for ch in &self.chapters {
            if ch.time_pos > time_pos + 1.0 {
                return Some(ch.time_pos);
            }
        }
        None
    }

    pub fn prev_chapter(&self, time_pos: f64) -> Option<f64> {
        let mut target = None;
        for ch in &self.chapters {
            if ch.time_pos < time_pos - 1.0 {
                target = Some(ch.time_pos);
            } else {
                break;
            }
        }
        target
    }
}
