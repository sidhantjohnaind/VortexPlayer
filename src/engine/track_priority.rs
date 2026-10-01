#![allow(dead_code)]

use super::player::TrackInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackPriorityConfig {
    pub preferred_subtitle_languages: Vec<String>,
    pub preferred_audio_languages: Vec<String>,
    pub prefer_forced_subtitles: bool,
    pub ignore_sdh_subtitles: bool,
    pub ignore_commentary: bool,
    pub prefer_surround_audio: bool,
    pub preferred_codecs: Vec<String>,
}

impl Default for TrackPriorityConfig {
    fn default() -> Self {
        Self {
            preferred_subtitle_languages: vec![
                "hi".to_string(),
                "hin".to_string(),
                "hindi".to_string(),
                "en".to_string(),
                "eng".to_string(),
                "english".to_string(),
                "ja".to_string(),
                "jpn".to_string(),
            ],
            preferred_audio_languages: vec![
                "ja".to_string(),
                "jpn".to_string(),
                "japanese".to_string(),
                "hi".to_string(),
                "hin".to_string(),
                "hindi".to_string(),
                "en".to_string(),
                "eng".to_string(),
                "english".to_string(),
            ],
            prefer_forced_subtitles: false,
            ignore_sdh_subtitles: false,
            ignore_commentary: true,
            prefer_surround_audio: true,
            preferred_codecs: vec![
                "truehd".to_string(),
                "dts-hd".to_string(),
                "flac".to_string(),
                "eac3".to_string(),
                "ac3".to_string(),
                "aac".to_string(),
                "opus".to_string(),
            ],
        }
    }
}

impl TrackPriorityConfig {
    /// Evaluates all available subtitle tracks and returns the highest-scoring track ID
    pub fn select_best_subtitle_track(&self, sub_tracks: &[TrackInfo]) -> Option<i64> {
        if sub_tracks.is_empty() {
            return None;
        }

        let mut best_id = None;
        let mut highest_score = -1000;

        for track in sub_tracks {
            let mut score = 0;
            let title_lower = track.title.to_lowercase();
            let lang_lower = track.lang.to_lowercase();

            // Skip commentary if configured
            if self.ignore_commentary && (title_lower.contains("commentary") || title_lower.contains("director")) {
                score -= 500;
            }

            // Skip SDH if configured
            if self.ignore_sdh_subtitles && (title_lower.contains("sdh") || title_lower.contains("cc")) {
                score -= 100;
            }

            // Bonus for forced
            if self.prefer_forced_subtitles && (title_lower.contains("forced") || track.is_default) {
                score += 80;
            }

            // Match language rank
            let mut matched_lang = false;
            for (idx, pref_lang) in self.preferred_subtitle_languages.iter().enumerate() {
                let p = pref_lang.to_lowercase();
                if lang_lower == p || title_lower.contains(&p) {
                    score += 1000 - idx as i32 * 50;
                    matched_lang = true;
                    break;
                }
            }

            if !matched_lang && track.is_default {
                score += 50;
            }

            // Format preference (ASS/SSA > SRT > VTT)
            if track.codec.to_lowercase().contains("ass") || track.codec.to_lowercase().contains("ssa") {
                score += 40;
            } else if track.codec.to_lowercase().contains("subrip") || track.codec.to_lowercase().contains("srt") {
                score += 20;
            }

            if score > highest_score {
                highest_score = score;
                best_id = Some(track.id);
            }
        }

        best_id
    }

    /// Evaluates all available audio tracks and returns the highest-scoring track ID
    pub fn select_best_audio_track(&self, audio_tracks: &[TrackInfo]) -> Option<i64> {
        if audio_tracks.is_empty() {
            return None;
        }

        let mut best_id = None;
        let mut highest_score = -1000;

        for track in audio_tracks {
            let mut score = 0;
            let title_lower = track.title.to_lowercase();
            let lang_lower = track.lang.to_lowercase();
            let codec_lower = track.codec.to_lowercase();

            // Skip commentary
            if self.ignore_commentary && (title_lower.contains("commentary") || title_lower.contains("director")) {
                score -= 500;
            }

            // Match language rank
            let mut matched_lang = false;
            for (idx, pref_lang) in self.preferred_audio_languages.iter().enumerate() {
                let p = pref_lang.to_lowercase();
                if lang_lower == p || title_lower.contains(&p) {
                    score += 1000 - idx as i32 * 60;
                    matched_lang = true;
                    break;
                }
            }

            if !matched_lang && track.is_default {
                score += 50;
            }

            // Surround sound channels weighting (7.1 > 5.1 > 2.0)
            if self.prefer_surround_audio {
                if track.channels.contains("7.1") || track.channels.contains("8") || title_lower.contains("7.1") {
                    score += 150;
                } else if track.channels.contains("5.1") || track.channels.contains("6") || title_lower.contains("5.1") {
                    score += 100;
                }
            }

            // Codec ranking (TrueHD > DTS-HD > FLAC > E-AC3 > AAC)
            for (idx, pref_codec) in self.preferred_codecs.iter().enumerate() {
                if codec_lower.contains(&pref_codec.to_lowercase()) {
                    score += 80 - idx as i32 * 10;
                    break;
                }
            }

            if score > highest_score {
                highest_score = score;
                best_id = Some(track.id);
            }
        }

        best_id
    }
}
