#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct LyricLine {
    pub timestamp_sec: f64,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LyricTrack {
    pub lines: Vec<LyricLine>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub offset_sec: f64,
    pub is_synced: bool,
}

impl LyricTrack {
    /// Loads and parses an external `.lrc` file with robust encoding detection (UTF-8, UTF-16, Latin-1)
    pub fn from_lrc_file(path: &Path) -> Option<Self> {
        let bytes = fs::read(path).ok()?;
        if bytes.is_empty() {
            return None;
        }

        // 1. Try UTF-8
        if let Ok(s) = std::str::from_utf8(&bytes) {
            return Self::parse_lrc(s);
        }

        // 2. Try UTF-16 LE / BE with BOM
        if bytes.len() >= 2 {
            if bytes[0] == 0xFF && bytes[1] == 0xFE {
                let u16_chars: Vec<u16> = bytes[2..]
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect();
                if let Ok(s) = String::from_utf16(&u16_chars) {
                    return Self::parse_lrc(&s);
                }
            } else if bytes[0] == 0xFE && bytes[1] == 0xFF {
                let u16_chars: Vec<u16> = bytes[2..]
                    .chunks_exact(2)
                    .map(|c| u16::from_be_bytes([c[0], c[1]]))
                    .collect();
                if let Ok(s) = String::from_utf16(&u16_chars) {
                    return Self::parse_lrc(&s);
                }
            }
        }

        // 3. Fallback: Lossy UTF-8
        let lossy = String::from_utf8_lossy(&bytes);
        Self::parse_lrc(&lossy)
    }

    /// Robust Universal LRC Parser:
    /// - Supports standard newlines (`\r\n`, `\n`, `\r`)
    /// - Supports slash-separated timestamps (`[0:00.54] Line 1 / [0:05.80] Line 2`) common in embedded tags and MediaInfo
    /// - Supports multiple timestamps per line (`[00:01.00][00:05.00] Chorus`)
    /// - Handles timestamps with 1-digit minute (`0:00.54`), 2-digit minute (`00:00.54`), 3-digit ms, and hh:mm:ss
    /// - Parses `[offset:+/-ms]` and metadata tags (`[ti:...]`, `[ar:...]`, `[al:...]`)
    /// - Gracefully supports unsynced plain lyrics
    pub fn parse_lrc(content: &str) -> Option<Self> {
        let mut lines = Vec::new();
        let mut title = None;
        let mut artist = None;
        let mut album = None;
        let mut offset_ms = 0.0f64;
        let mut unsynced_lines = Vec::new();

        // Normalize segments: split on newlines, but also handle slash-delimited timed entries
        let mut raw_segments = Vec::new();
        for raw_line in content.lines() {
            let trimmed = raw_line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.contains(" / [") || (trimmed.starts_with('[') && trimmed.contains(" / ")) {
                for part in trimmed.split(" / ") {
                    let p = part.trim();
                    if !p.is_empty() {
                        raw_segments.push(p);
                    }
                }
            } else {
                raw_segments.push(trimmed);
            }
        }

        for segment in raw_segments {
            let seg = segment.trim();
            if seg.is_empty() {
                continue;
            }

            // Extract metadata tags e.g. [ti:Title], [ar:Artist], [al:Album], [offset:500]
            if seg.starts_with('[') {
                if let Some(close_pos) = seg.find(']') {
                    let tag_content = &seg[1..close_pos];
                    if let Some(colon_pos) = tag_content.find(':') {
                        let key = tag_content[..colon_pos].trim().to_ascii_lowercase();
                        let val = tag_content[colon_pos + 1..].trim();
                        match key.as_str() {
                            "ti" | "title" => {
                                if !val.is_empty() {
                                    title = Some(val.to_string());
                                }
                            }
                            "ar" | "artist" => {
                                if !val.is_empty() {
                                    artist = Some(val.to_string());
                                }
                            }
                            "al" | "album" => {
                                if !val.is_empty() {
                                    album = Some(val.to_string());
                                }
                            }
                            "offset" => {
                                if let Ok(ms) = val.parse::<f64>() {
                                    offset_ms = ms;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            // Extract all timestamps from segment: e.g. [00:12.50][00:25.00]Some lyrics
            let mut timestamps = Vec::new();
            let mut search_idx = 0;
            let mut text_start = 0;

            while let Some(open_idx) = seg[search_idx..].find('[') {
                let actual_open = search_idx + open_idx;
                if let Some(close_idx) = seg[actual_open..].find(']') {
                    let actual_close = actual_open + close_idx;
                    let tag_inner = &seg[actual_open + 1..actual_close];
                    if let Some(sec) = Self::parse_timestamp(tag_inner) {
                        timestamps.push(sec);
                        text_start = actual_close + 1;
                        search_idx = actual_close + 1;
                    } else {
                        search_idx = actual_close + 1;
                    }
                } else {
                    break;
                }
            }

            if !timestamps.is_empty() {
                let lyric_text = seg[text_start..].trim().trim_start_matches('/').trim().to_string();
                for sec in timestamps {
                    lines.push(LyricLine {
                        timestamp_sec: sec,
                        text: lyric_text.clone(),
                    });
                }
            } else if !seg.starts_with('[') && !seg.is_empty() {
                // Potential unsynced plain lyric line
                unsynced_lines.push(seg.to_string());
            }
        }

        if lines.is_empty() {
            // Unsynced lyrics fallback if non-empty text exists
            if !unsynced_lines.is_empty() {
                let dummy_lines: Vec<LyricLine> = unsynced_lines
                    .into_iter()
                    .enumerate()
                    .map(|(i, text)| LyricLine {
                        timestamp_sec: i as f64 * 4.0, // estimate 4s per line
                        text,
                    })
                    .collect();
                return Some(Self {
                    lines: dummy_lines,
                    title,
                    artist,
                    album,
                    offset_sec: 0.0,
                    is_synced: false,
                });
            }
            return None;
        }

        // Apply global offset if present
        if offset_ms != 0.0 {
            let offset_sec = offset_ms / 1000.0;
            for line in &mut lines {
                line.timestamp_sec = (line.timestamp_sec + offset_sec).max(0.0);
            }
        }

        // Sort chronologically
        lines.sort_by(|a, b| {
            a.timestamp_sec
                .partial_cmp(&b.timestamp_sec)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Some(Self {
            lines,
            title,
            artist,
            album,
            offset_sec: offset_ms / 1000.0,
            is_synced: true,
        })
    }

    /// Flexible timestamp parser:
    /// Supports `mm:ss.xx`, `m:ss.xx`, `hh:mm:ss.xx`, `mm:ss`, with `.` or `,` decimal separator
    pub fn parse_timestamp(s: &str) -> Option<f64> {
        let s = s.trim();
        let parts: Vec<&str> = s.split(':').collect();
        match parts.len() {
            2 => {
                let min: f64 = parts[0].trim().parse().ok()?;
                let sec_str = parts[1].trim().replace(',', ".");
                let sec: f64 = sec_str.parse().ok()?;
                Some(min * 60.0 + sec)
            }
            3 => {
                let hr: f64 = parts[0].trim().parse().ok()?;
                let min: f64 = parts[1].trim().parse().ok()?;
                let sec_str = parts[2].trim().replace(',', ".");
                let sec: f64 = sec_str.parse().ok()?;
                Some(hr * 3600.0 + min * 60.0 + sec)
            }
            _ => None,
        }
    }

    /// Finds current active lyric line at `current_time`
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

/// Discovers an external `.lrc` file matching the media file
pub fn find_external_lrc(media_path: &Path) -> Option<PathBuf> {
    // 1. Exact stem + .lrc or .LRC
    let lrc1 = media_path.with_extension("lrc");
    if lrc1.is_file() {
        return Some(lrc1);
    }
    let lrc2 = media_path.with_extension("LRC");
    if lrc2.is_file() {
        return Some(lrc2);
    }

    // 2. Directory scan matching file stem (case-insensitive)
    let parent = media_path.parent()?;
    let stem = media_path.file_stem()?.to_string_lossy().to_lowercase();
    if let Ok(entries) = fs::read_dir(parent) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                    if ext.eq_ignore_ascii_case("lrc") {
                        if let Some(ent_stem) = p.file_stem().and_then(|s| s.to_str()) {
                            if ent_stem.to_lowercase() == stem {
                                return Some(p);
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Pure-Rust instant extractor for FLAC Vorbis comment metadata block (block_type 4)
pub fn extract_from_flac(path: &Path) -> Option<LyricTrack> {
    use std::io::Read;
    let mut file = fs::File::open(path).ok()?;
    // Read the first 4MB (where FLAC metadata headers reside)
    let mut buf = vec![0u8; 4 * 1024 * 1024];
    let bytes_read = file.read(&mut buf).unwrap_or(0);
    if bytes_read < 8 || &buf[0..4] != b"fLaC" {
        return None;
    }
    let data = &buf[..bytes_read];
    let mut offset = 4usize;

    while offset + 4 <= data.len() {
        let b0 = data[offset];
        let is_last = (b0 & 0x80) != 0;
        let block_type = b0 & 0x7F;
        let length = ((data[offset + 1] as usize) << 16)
            | ((data[offset + 2] as usize) << 8)
            | (data[offset + 3] as usize);
        offset += 4;
        if offset + length > data.len() {
            break;
        }

        let block_slice = &data[offset..offset + length];
        if block_type == 4 && block_slice.len() >= 8 {
            let vendor_len = u32::from_le_bytes([
                block_slice[0],
                block_slice[1],
                block_slice[2],
                block_slice[3],
            ]) as usize;
            let mut cur = 4usize.saturating_add(vendor_len);
            if cur + 4 <= block_slice.len() {
                let comment_count = u32::from_le_bytes([
                    block_slice[cur],
                    block_slice[cur + 1],
                    block_slice[cur + 2],
                    block_slice[cur + 3],
                ]) as usize;
                cur += 4;
                for _ in 0..comment_count {
                    if cur + 4 > block_slice.len() {
                        break;
                    }
                    let c_len = u32::from_le_bytes([
                        block_slice[cur],
                        block_slice[cur + 1],
                        block_slice[cur + 2],
                        block_slice[cur + 3],
                    ]) as usize;
                    cur += 4;
                    if cur + c_len > block_slice.len() {
                        break;
                    }
                    let comment_bytes = &block_slice[cur..cur + c_len];
                    cur += c_len;
                    if let Ok(comment_str) = std::str::from_utf8(comment_bytes) {
                        if let Some(eq_idx) = comment_str.find('=') {
                            let key = &comment_str[..eq_idx];
                            if key.eq_ignore_ascii_case("LYRICS")
                                || key.eq_ignore_ascii_case("UNSYNCEDLYRICS")
                                || key.eq_ignore_ascii_case("SYNCEDLYRICS")
                                || key.eq_ignore_ascii_case("DESCRIPTION")
                            {
                                let val = &comment_str[eq_idx + 1..];
                                if let Some(track) = LyricTrack::parse_lrc(val) {
                                    return Some(track);
                                }
                            }
                        }
                    }
                }
            }
        }

        if is_last {
            break;
        }
        offset += length;
    }

    None
}

/// Helper to decode ID3v2 text frames based on encoding byte
fn decode_id3_text(bytes: &[u8], enc: u8) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    match enc {
        0 => {
            // ISO-8859-1 / Latin-1
            bytes.iter().map(|&b| b as char).collect()
        }
        1 => {
            // UTF-16 with BOM
            if bytes.len() >= 2 {
                if bytes[0] == 0xFF && bytes[1] == 0xFE {
                    let u: Vec<u16> = bytes[2..]
                        .chunks_exact(2)
                        .map(|c| u16::from_le_bytes([c[0], c[1]]))
                        .collect();
                    String::from_utf16_lossy(&u)
                } else if bytes[0] == 0xFE && bytes[1] == 0xFF {
                    let u: Vec<u16> = bytes[2..]
                        .chunks_exact(2)
                        .map(|c| u16::from_be_bytes([c[0], c[1]]))
                        .collect();
                    String::from_utf16_lossy(&u)
                } else {
                    let u: Vec<u16> = bytes
                        .chunks_exact(2)
                        .map(|c| u16::from_le_bytes([c[0], c[1]]))
                        .collect();
                    String::from_utf16_lossy(&u)
                }
            } else {
                String::new()
            }
        }
        2 => {
            // UTF-16BE
            let u: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16_lossy(&u)
        }
        3 => {
            // UTF-8
            String::from_utf8_lossy(bytes).to_string()
        }
        _ => String::from_utf8_lossy(bytes).to_string(),
    }
}

/// Pure-Rust instant extractor for MP3 ID3v2 USLT / SYLT / TXXX frames
pub fn extract_from_mp3_id3(path: &Path) -> Option<LyricTrack> {
    use std::io::Read;
    let mut file = fs::File::open(path).ok()?;
    let mut buf = vec![0u8; 2 * 1024 * 1024];
    let bytes_read = file.read(&mut buf).unwrap_or(0);
    if bytes_read < 10 || &buf[0..3] != b"ID3" {
        return None;
    }
    let data = &buf[..bytes_read];
    let tag_size = ((data[6] as usize & 0x7F) << 21)
        | ((data[7] as usize & 0x7F) << 14)
        | ((data[8] as usize & 0x7F) << 7)
        | (data[9] as usize & 0x7F);
    let max_len = (10 + tag_size).min(data.len());
    let mut cur = 10usize;

    while cur + 10 <= max_len {
        let frame_id = &data[cur..cur + 4];
        if frame_id[0] == 0 {
            break;
        }
        let sz1 = ((data[cur + 4] as usize) << 24)
            | ((data[cur + 5] as usize) << 16)
            | ((data[cur + 6] as usize) << 8)
            | (data[cur + 7] as usize);
        let sz2 = ((data[cur + 4] as usize & 0x7F) << 21)
            | ((data[cur + 5] as usize & 0x7F) << 14)
            | ((data[cur + 6] as usize & 0x7F) << 7)
            | (data[cur + 7] as usize & 0x7F);
        let frame_len = if cur + 10 + sz1 <= max_len { sz1 } else { sz2 };
        cur += 10;
        if cur + frame_len > max_len {
            break;
        }
        let payload = &data[cur..cur + frame_len];
        cur += frame_len;

        if frame_id == b"USLT" && payload.len() > 4 {
            let enc = payload[0];
            let rest = &payload[4..]; // skip 3-byte lang code
            // Find null descriptor terminator
            let desc_end = if enc == 1 || enc == 2 {
                rest.windows(2).position(|w| w == [0, 0]).map(|p| p + 2).unwrap_or(0)
            } else {
                rest.iter().position(|&b| b == 0).map(|p| p + 1).unwrap_or(0)
            };
            let lyrics_bytes = if desc_end < rest.len() { &rest[desc_end..] } else { rest };
            let lyrics_text = decode_id3_text(lyrics_bytes, enc);
            if let Some(track) = LyricTrack::parse_lrc(&lyrics_text) {
                return Some(track);
            }
        } else if frame_id == b"TXXX" && payload.len() > 1 {
            let enc = payload[0];
            let text = decode_id3_text(&payload[1..], enc);
            if text.to_ascii_uppercase().starts_with("LYRICS\0") || text.to_ascii_uppercase().starts_with("LYRICS=") {
                let val = text.splitn(2, |c| c == '\0' || c == '=').nth(1).unwrap_or(&text);
                if let Some(track) = LyricTrack::parse_lrc(val) {
                    return Some(track);
                }
            }
        }
    }

    None
}

/// Pure-Rust instant extractor for MP4 / M4A `©lyr` atom
pub fn extract_from_mp4(path: &Path) -> Option<LyricTrack> {
    use std::io::Read;
    let mut file = fs::File::open(path).ok()?;
    let mut buf = vec![0u8; 2 * 1024 * 1024];
    let bytes_read = file.read(&mut buf).unwrap_or(0);
    if bytes_read < 16 {
        return None;
    }
    let data = &buf[..bytes_read];

    let lyr_keys: [&[u8]; 2] = [b"\xa9lyr", b"\xc2\xa9lyr"];
    for key in lyr_keys {
        if let Some(pos) = data.windows(key.len()).position(|w| w == key) {
            // Found ©lyr atom, look for 'data' subatom
            let slice = &data[pos..];
            if let Some(data_pos) = slice.windows(4).position(|w| w == b"data") {
                let payload_start = data_pos + 12; // skip 'data' (4), type (4), locale (4)
                if payload_start < slice.len() {
                    let payload = &slice[payload_start..];
                    let end = payload.iter().position(|&b| b == 0).unwrap_or(payload.len().min(32768));
                    let s = String::from_utf8_lossy(&payload[..end]);
                    if let Some(track) = LyricTrack::parse_lrc(&s) {
                        return Some(track);
                    }
                }
            }
        }
    }

    None
}

/// Parses lyrics from MediaInfo CLI stdout
pub fn extract_from_mediainfo_text(text: &str) -> Option<LyricTrack> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed
            .strip_prefix("Lyrics : ")
            .or_else(|| trimmed.strip_prefix("Lyrics/Text : "))
            .or_else(|| trimmed.strip_prefix("UNSYNCEDLYRICS : "))
            .or_else(|| trimmed.strip_prefix("Lyrics           : "))
        {
            if let Some(track) = LyricTrack::parse_lrc(rest.trim()) {
                return Some(track);
            }
        }
    }
    None
}

/// Universal multi-tiered loader:
/// 1. External `.lrc` file in song folder
/// 2. MPV metadata property if provided
/// 3. Embedded metadata in file (FLAC Vorbis comment, MP3 ID3, MP4)
/// 4. MediaInfo output text
pub fn load_lyrics_for_media(
    file_path: &str,
    mpv_lyrics: Option<&str>,
    mediainfo_text: Option<&str>,
) -> Option<LyricTrack> {
    if file_path.is_empty() {
        return None;
    }
    let path = Path::new(file_path);

    // 1. External .lrc file (highest user precedence)
    if let Some(lrc_path) = find_external_lrc(path) {
        if let Some(track) = LyricTrack::from_lrc_file(&lrc_path) {
            return Some(track);
        }
    }

    // 2. MPV metadata property if provided
    if let Some(mpv_lrc) = mpv_lyrics {
        if !mpv_lrc.trim().is_empty() {
            if let Some(track) = LyricTrack::parse_lrc(mpv_lrc) {
                return Some(track);
            }
        }
    }

    // 3. Pure-Rust embedded metadata extractors
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        match ext_lower.as_str() {
            "flac" | "ogg" | "opus" => {
                if let Some(track) = extract_from_flac(path) {
                    return Some(track);
                }
            }
            "mp3" | "wav" | "aiff" => {
                if let Some(track) = extract_from_mp3_id3(path) {
                    return Some(track);
                }
            }
            "m4a" | "mp4" => {
                if let Some(track) = extract_from_mp4(path) {
                    return Some(track);
                }
            }
            _ => {}
        }
    }

    // 4. MediaInfo output text (from cached or background scan)
    if let Some(mi_text) = mediainfo_text {
        if let Some(track) = extract_from_mediainfo_text(mi_text) {
            return Some(track);
        }
    }

    None
}
