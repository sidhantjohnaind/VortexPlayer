pub mod pbf;

pub use pbf::{AbLoopSegment, BookmarkItem, PbfFile};
use std::path::{Path, PathBuf};

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkipInterval {
    pub start: f64,
    pub end: f64,
    pub label: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct BookmarkManager {
    pub current_video_path: Option<PathBuf>,
    pub bookmarks: Vec<BookmarkItem>,
    pub ab_loop: AbLoopSegment,
    pub auto_skip_bookmarks: bool,
    pub skip_intervals: Vec<SkipInterval>,
    pub last_skipped_time: f64,
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self {
            current_video_path: None,
            bookmarks: Vec::new(),
            ab_loop: AbLoopSegment::default(),
            auto_skip_bookmarks: true,
            skip_intervals: Vec::new(),
            last_skipped_time: -100.0,
        }
    }
}

impl BookmarkManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_video(&mut self, video_path: Option<&Path>) {
        // Save current video bookmarks before switching
        if let Some(ref current_path) = self.current_video_path {
            let _ = PbfFile::save_for_video(current_path, &self.bookmarks);
        }

        self.current_video_path = video_path.map(|p| p.to_path_buf());
        self.ab_loop = AbLoopSegment::default();
        self.bookmarks.clear();
        self.skip_intervals.clear();
        self.last_skipped_time = -100.0;

        if let Some(path) = video_path {
            if let Some(loaded) = PbfFile::load_for_video(path) {
                self.bookmarks = loaded;
                self.load_skip_intervals_from_bookmarks();
            }
        }
    }

    pub fn load_skip_intervals_from_bookmarks(&mut self) {
        self.skip_intervals.clear();
        let mut starts: Vec<(String, f64, bool)> = Vec::new();
        for b in &self.bookmarks {
            let t = b.title.trim();
            if t.starts_with("[SKIP:") || t.starts_with("[SKIP(") {
                let enabled = !t.starts_with("[SKIP(DISABLED)");
                if t.ends_with("_END") {
                    let label = t.trim_start_matches("[SKIP:")
                        .trim_start_matches("[SKIP(DISABLED):")
                        .trim_end_matches("_END")
                        .trim_end_matches(']')
                        .trim()
                        .to_string();
                    if let Some(pos) = starts.iter().rposition(|(l, _, _)| l == &label) {
                        let (_, start_time, en) = starts.remove(pos);
                        if b.time_pos > start_time {
                            self.skip_intervals.push(SkipInterval {
                                start: start_time,
                                end: b.time_pos,
                                label,
                                enabled: en && enabled,
                            });
                        }
                    }
                } else {
                    let label = t.trim_start_matches("[SKIP:")
                        .trim_start_matches("[SKIP(DISABLED):")
                        .trim_end_matches(']')
                        .trim()
                        .to_string();
                    starts.push((label, b.time_pos, enabled));
                }
            }
        }
        self.skip_intervals.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap_or(std::cmp::Ordering::Equal));
    }

    pub fn sync_skip_intervals_to_bookmarks(&mut self) {
        self.bookmarks.retain(|b| !b.title.starts_with("[SKIP"));
        for interval in &self.skip_intervals {
            let prefix = if interval.enabled { "[SKIP:" } else { "[SKIP(DISABLED):" };
            self.bookmarks.push(BookmarkItem {
                time_pos: interval.start,
                title: format!("{} {}]", prefix, interval.label),
                index: 0,
            });
            self.bookmarks.push(BookmarkItem {
                time_pos: interval.end,
                title: format!("{} {}]_END", prefix, interval.label),
                index: 0,
            });
        }
        self.bookmarks.sort_by(|a, b| a.time_pos.partial_cmp(&b.time_pos).unwrap_or(std::cmp::Ordering::Equal));
        for (i, b) in self.bookmarks.iter_mut().enumerate() {
            b.index = i;
        }
        if let Some(ref path) = self.current_video_path {
            let _ = PbfFile::save_for_video(path, &self.bookmarks);
        }
    }

    pub fn add_bookmark(&mut self, time_pos: f64, custom_title: Option<String>) {
        let mins = (time_pos / 60.0).floor() as u32;
        let secs = (time_pos % 60.0).floor() as u32;
        let ms = ((time_pos % 1.0) * 1000.0).floor() as u32;
        let default_title = format!("{:02}:{:02}.{:03}", mins, secs, ms);
        let title = custom_title.unwrap_or(default_title);

        let item = BookmarkItem {
            time_pos,
            title,
            index: self.bookmarks.len(),
        };

        self.bookmarks.push(item);
        self.bookmarks.sort_by(|a, b| a.time_pos.partial_cmp(&b.time_pos).unwrap_or(std::cmp::Ordering::Equal));
        for (i, b) in self.bookmarks.iter_mut().enumerate() {
            b.index = i;
        }

        if let Some(ref path) = self.current_video_path {
            let _ = PbfFile::save_for_video(path, &self.bookmarks);
        }
    }

    pub fn add_skip_interval(&mut self, start: f64, end: f64, label: String) {
        if end > start {
            self.skip_intervals.push(SkipInterval { start, end, label, enabled: true });
            self.skip_intervals.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap_or(std::cmp::Ordering::Equal));
            self.sync_skip_intervals_to_bookmarks();
        }
    }

    pub fn remove_skip_interval(&mut self, index: usize) {
        if index < self.skip_intervals.len() {
            self.skip_intervals.remove(index);
            self.sync_skip_intervals_to_bookmarks();
        }
    }

    pub fn clear_skip_intervals(&mut self) {
        self.skip_intervals.clear();
        self.sync_skip_intervals_to_bookmarks();
    }

    pub fn toggle_auto_skip(&mut self) -> bool {
        self.auto_skip_bookmarks = !self.auto_skip_bookmarks;
        self.auto_skip_bookmarks
    }

    pub fn should_skip(&mut self, current_time: f64) -> Option<(f64, String)> {
        if !self.auto_skip_bookmarks {
            return None;
        }
        // Avoid multiple rapid triggers on the same timestamp
        if (current_time - self.last_skipped_time).abs() < 1.5 {
            return None;
        }

        // 1. Check explicitly marked skip intervals (e.g. Intro/Outro)
        for interval in &self.skip_intervals {
            if interval.enabled && current_time >= interval.start && current_time < interval.end - 0.4 {
                self.last_skipped_time = interval.end;
                return Some((interval.end, interval.label.clone()));
            }
        }

        // 2. Check bookmarks tagged with "[SKIP]" in their title
        for bm in &self.bookmarks {
            let upper = bm.title.to_uppercase();
            if upper.contains("[SKIP]") || upper.contains("SKIP:") {
                if current_time >= bm.time_pos - 0.25 && current_time < bm.time_pos + 0.75 {
                    if let Some(next_pos) = self.next_bookmark(bm.time_pos + 0.5) {
                        if next_pos > bm.time_pos {
                            self.last_skipped_time = next_pos;
                            return Some((next_pos, bm.title.clone()));
                        }
                    }
                }
            }
        }

        None
    }

    pub fn remove_bookmark(&mut self, index: usize) {
        if index < self.bookmarks.len() {
            self.bookmarks.remove(index);
            for (i, b) in self.bookmarks.iter_mut().enumerate() {
                b.index = i;
            }
            if let Some(ref path) = self.current_video_path {
                let _ = PbfFile::save_for_video(path, &self.bookmarks);
            }
        }
    }

    pub fn clear_bookmarks(&mut self) {
        self.bookmarks.clear();
        self.skip_intervals.clear();
        if let Some(ref path) = self.current_video_path {
            let _ = PbfFile::save_for_video(path, &self.bookmarks);
        }
    }

    pub fn next_bookmark(&self, current_time: f64) -> Option<f64> {
        if self.bookmarks.is_empty() {
            return None;
        }
        for bm in &self.bookmarks {
            if bm.time_pos > current_time + 0.5 {
                return Some(bm.time_pos);
            }
        }
        // Wrap around to first
        self.bookmarks.first().map(|b| b.time_pos)
    }

    pub fn prev_bookmark(&self, current_time: f64) -> Option<f64> {
        if self.bookmarks.is_empty() {
            return None;
        }
        for bm in self.bookmarks.iter().rev() {
            if bm.time_pos < current_time - 0.5 {
                return Some(bm.time_pos);
            }
        }
        // Wrap around to last
        self.bookmarks.last().map(|b| b.time_pos)
    }

    // A-B Loop Controls
    pub fn set_loop_a(&mut self, time: f64) -> String {
        self.ab_loop.point_a = Some(time);
        if let Some(b) = self.ab_loop.point_b {
            if b <= time {
                self.ab_loop.point_b = None;
                self.ab_loop.is_active = false;
            } else {
                self.ab_loop.is_active = true;
            }
        }
        format_time(time)
    }

    pub fn set_loop_b(&mut self, time: f64) -> Result<String, &'static str> {
        if let Some(a) = self.ab_loop.point_a {
            if time <= a {
                return Err("Point B must be after Point A");
            }
            self.ab_loop.point_b = Some(time);
            self.ab_loop.is_active = true;
            Ok(format_time(time))
        } else {
            Err("Point A must be set first")
        }
    }

    pub fn toggle_ab_loop(&mut self) -> bool {
        if self.ab_loop.is_active {
            self.ab_loop.is_active = false;
        } else if self.ab_loop.point_a.is_some() && self.ab_loop.point_b.is_some() {
            self.ab_loop.is_active = true;
        }
        self.ab_loop.is_active
    }

    pub fn clear_ab_loop(&mut self) {
        self.ab_loop = AbLoopSegment::default();
    }
}

pub fn format_time(seconds: f64) -> String {
    let s = seconds.max(0.0);
    let hrs = (s / 3600.0).floor() as u64;
    let mins = ((s % 3600.0) / 60.0).floor() as u64;
    let secs = (s % 60.0).floor() as u64;

    if hrs > 0 {
        format!("{:02}:{:02}:{:02}", hrs, mins, secs)
    } else {
        format!("{:02}:{:02}", mins, secs)
    }
}

pub fn format_time_hms(seconds: f64) -> String {
    let s = seconds.max(0.0);
    let hrs = (s / 3600.0).floor() as u64;
    let mins = ((s % 3600.0) / 60.0).floor() as u64;
    let secs = (s % 60.0).floor() as u64;
    format!("{:02}:{:02}:{:02}", hrs, mins, secs)
}

pub fn parse_time_hms(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return Some(0.0);
    }
    let parts: Vec<&str> = s.split(':').map(|p| p.trim()).collect();
    match parts.len() {
        1 => parts[0].parse::<f64>().ok(),
        2 => {
            let m = parts[0].parse::<f64>().ok()?;
            let s = parts[1].parse::<f64>().ok()?;
            Some(m * 60.0 + s)
        }
        3 => {
            let h = parts[0].parse::<f64>().ok()?;
            let m = parts[1].parse::<f64>().ok()?;
            let s = parts[2].parse::<f64>().ok()?;
            Some(h * 3600.0 + m * 60.0 + s)
        }
        _ => None,
    }
}

