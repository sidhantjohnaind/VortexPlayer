#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Instant;

use super::audio_levels::CLIP_THRESHOLD_LINEAR;

pub const MAX_AUDIO_CHANNELS: usize = 8;
pub const MIN_DBFS: f32 = -60.0;

/// Channel speaker assignment in standard 7.1 surround order (ITU-R BS.775 / SMPTE)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PotSpeakerChannel {
    Left = 0,
    Right = 1,
    Center = 2,
    Lfe = 3,
    SurroundLeft = 4,
    SurroundRight = 5,
    BackLeft = 6,
    BackRight = 7,
}

impl PotSpeakerChannel {
    pub fn name(&self) -> &'static str {
        match self {
            PotSpeakerChannel::Left => "L",
            PotSpeakerChannel::Right => "R",
            PotSpeakerChannel::Center => "C",
            PotSpeakerChannel::Lfe => "LFE",
            PotSpeakerChannel::SurroundLeft => "SL",
            PotSpeakerChannel::SurroundRight => "SR",
            PotSpeakerChannel::BackLeft => "BL",
            PotSpeakerChannel::BackRight => "BR",
        }
    }

    pub fn ffmpeg_tag(&self) -> &'static str {
        match self {
            PotSpeakerChannel::Left => "FL",
            PotSpeakerChannel::Right => "FR",
            PotSpeakerChannel::Center => "FC",
            PotSpeakerChannel::Lfe => "LFE",
            PotSpeakerChannel::SurroundLeft => "SL",
            PotSpeakerChannel::SurroundRight => "SR",
            PotSpeakerChannel::BackLeft => "BL",
            PotSpeakerChannel::BackRight => "BR",
        }
    }

    pub fn full_name(&self) -> &'static str {
        match self {
            PotSpeakerChannel::Left => "Front Left",
            PotSpeakerChannel::Right => "Front Right",
            PotSpeakerChannel::Center => "Center (Dialogue)",
            PotSpeakerChannel::Lfe => "Low-Frequency Effects (Subwoofer)",
            PotSpeakerChannel::SurroundLeft => "Surround Left",
            PotSpeakerChannel::SurroundRight => "Surround Right",
            PotSpeakerChannel::BackLeft => "Back Left",
            PotSpeakerChannel::BackRight => "Back Right",
        }
    }
}

/// Atomic real-time channel meter metrics for lock-free UI rendering
pub struct PotChannelMetrics {
    // Input Stage (Pre-DSP)
    pub input_peak_linear: AtomicU32,
    pub input_rms_linear: AtomicU32,
    pub input_peak_hold_linear: AtomicU32,

    // Output Stage (Post-DSP)
    pub output_peak_linear: AtomicU32,
    pub output_rms_linear: AtomicU32,
    pub output_peak_hold_linear: AtomicU32,

    // Status Flags
    pub is_clipped: AtomicBool,
    pub is_muted: AtomicBool,
    pub is_solo: AtomicBool,
}

impl Default for PotChannelMetrics {
    fn default() -> Self {
        Self {
            input_peak_linear: AtomicU32::new(0),
            input_rms_linear: AtomicU32::new(0),
            input_peak_hold_linear: AtomicU32::new(0),
            output_peak_linear: AtomicU32::new(0),
            output_rms_linear: AtomicU32::new(0),
            output_peak_hold_linear: AtomicU32::new(0),
            is_clipped: AtomicBool::new(false),
            is_muted: AtomicBool::new(false),
            is_solo: AtomicBool::new(false),
        }
    }
}

/// PotPlayer-Grade In-Process Audio Transform Filter & PCM Analyzer (DSP)
pub struct AudioTransformFilter {
    pub channels: [PotChannelMetrics; MAX_AUDIO_CHANNELS],
    peak_hold_times_input: [Instant; MAX_AUDIO_CHANNELS],
    peak_hold_times_output: [Instant; MAX_AUDIO_CHANNELS],
    last_update: Instant,
    pub sample_rate: u32,
    pub channel_count: usize,
    pub volume: f32,
    eq_preamp: f32,
}

impl AudioTransformFilter {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            channels: [
                PotChannelMetrics::default(),
                PotChannelMetrics::default(),
                PotChannelMetrics::default(),
                PotChannelMetrics::default(),
                PotChannelMetrics::default(),
                PotChannelMetrics::default(),
                PotChannelMetrics::default(),
                PotChannelMetrics::default(),
            ],
            peak_hold_times_input: [now; MAX_AUDIO_CHANNELS],
            peak_hold_times_output: [now; MAX_AUDIO_CHANNELS],
            last_update: now,
            sample_rate: 48000,
            channel_count: 2,
            volume: 1.0,
            eq_preamp: 1.0,
        }
    }

    pub fn set_audio_params(&mut self, sample_rate: u32, channel_count: usize) {
        self.sample_rate = sample_rate.max(8000);
        self.channel_count = channel_count.clamp(1, MAX_AUDIO_CHANNELS);
    }

    pub fn set_volume(&mut self, volume_percent: f64) {
        self.volume = (volume_percent as f32 / 100.0).clamp(0.0, 2.0);
    }

    pub fn reset_clips(&self) {
        for ch in 0..MAX_AUDIO_CHANNELS {
            self.channels[ch].is_clipped.store(false, Ordering::Relaxed);
        }
    }

    /// Process real decoded PCM audio buffer frames
    pub fn process_pcm_frames(&mut self, pcm_interleaved: &[f32], channels: usize) {
        if pcm_interleaved.is_empty() || channels == 0 {
            return;
        }

        let ch_count = channels.min(MAX_AUDIO_CHANNELS);
        let sample_frames = pcm_interleaved.len() / channels;
        if sample_frames == 0 {
            return;
        }

        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32().clamp(0.001, 0.1);
        self.last_update = now;

        for ch in 0..ch_count {
            let mut max_sample: f32 = 0.0;
            let mut sum_sq: f32 = 0.0;

            for frame in 0..sample_frames {
                let idx = frame * channels + ch;
                if idx < pcm_interleaved.len() {
                    let s = pcm_interleaved[idx].abs();
                    if s > max_sample {
                        max_sample = s;
                    }
                    sum_sq += s * s;
                }
            }

            let instant_peak = max_sample.clamp(0.0, 1.5);
            let instant_rms = (sum_sq / sample_frames as f32).sqrt().clamp(0.0, 1.5);

            // Digital full-scale clipping check (sample >= 1.0)
            if instant_peak >= CLIP_THRESHOLD_LINEAR {
                self.channels[ch].is_clipped.store(true, Ordering::Relaxed);
            }

            // Input stage PPM ballistics
            let prev_in_peak = f32::from_bits(self.channels[ch].input_peak_linear.load(Ordering::Relaxed));
            let prev_in_rms = f32::from_bits(self.channels[ch].input_rms_linear.load(Ordering::Relaxed));
            let prev_in_hold = f32::from_bits(self.channels[ch].input_peak_hold_linear.load(Ordering::Relaxed));

            let in_attack = (dt * 65.0).min(1.0);
            let in_decay = (dt * 16.0).min(1.0);

            let new_in_peak = if instant_peak >= prev_in_peak {
                prev_in_peak + (instant_peak - prev_in_peak) * in_attack
            } else {
                (prev_in_peak - (prev_in_peak - instant_peak) * in_decay).max(0.0)
            };

            let new_in_rms = if instant_rms >= prev_in_rms {
                prev_in_rms + (instant_rms - prev_in_rms) * in_attack
            } else {
                (prev_in_rms - (prev_in_rms - instant_rms) * in_decay).max(0.0)
            };

            let new_in_hold = if new_in_peak >= prev_in_hold {
                self.peak_hold_times_input[ch] = now;
                new_in_peak
            } else if now.duration_since(self.peak_hold_times_input[ch]).as_secs_f32() > 1.2 {
                (prev_in_hold - dt * 0.35).max(new_in_peak)
            } else {
                prev_in_hold
            };

            self.channels[ch].input_peak_linear.store(new_in_peak.to_bits(), Ordering::Relaxed);
            self.channels[ch].input_rms_linear.store(new_in_rms.to_bits(), Ordering::Relaxed);
            self.channels[ch].input_peak_hold_linear.store(new_in_hold.to_bits(), Ordering::Relaxed);

            // Output stage with gain
            let vol_gain = self.volume * self.eq_preamp;
            let out_peak = (new_in_peak * vol_gain).clamp(0.0, 1.5);
            let out_rms = (new_in_rms * vol_gain).clamp(0.0, 1.5);

            if out_peak >= CLIP_THRESHOLD_LINEAR {
                self.channels[ch].is_clipped.store(true, Ordering::Relaxed);
            }

            self.channels[ch].output_peak_linear.store(out_peak.to_bits(), Ordering::Relaxed);
            self.channels[ch].output_rms_linear.store(out_rms.to_bits(), Ordering::Relaxed);
            self.channels[ch].output_peak_hold_linear.store(new_in_hold.to_bits(), Ordering::Relaxed);
        }
    }

    /// Update channel meters from real FFmpeg astats metadata JSON computed in C on raw PCM frames
    pub fn update_from_astats_json(
        &mut self,
        astats_json: &str,
        active_channels: usize,
        is_playing: bool,
        is_muted: bool,
        volume: f64,
    ) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32().clamp(0.001, 0.1);
        self.last_update = now;
        self.set_volume(volume);
        let vol_mult = if is_muted { 0.0 } else { self.volume };
        let any_solo = self.channels.iter().any(|c| c.is_solo.load(Ordering::Relaxed));

        // Parse JSON dictionary returned by mpv's af-metadata/astats
        let parsed_json = if !astats_json.is_empty() {
            serde_json::from_str::<serde_json::Value>(astats_json).ok()
        } else {
            None
        };

        let mut detected_ch = active_channels;
        if let Some(ref j) = parsed_json {
            for c in 1..=MAX_AUDIO_CHANNELS {
                let peak_k = format!("lavfi.astats.{}.Peak_level", c);
                if j.get(&peak_k).is_some() {
                    detected_ch = detected_ch.max(c);
                }
            }
        }
        let total_ch = detected_ch.clamp(1, MAX_AUDIO_CHANNELS);
        self.channel_count = total_ch;

        for ch in 0..MAX_AUDIO_CHANNELS {
            if is_playing && !is_muted && ch < total_ch {
                let ch_num = ch + 1;
                let peak_key = format!("lavfi.astats.{}.Peak_level", ch_num);
                let rms_key = format!("lavfi.astats.{}.RMS_level", ch_num);

                let raw_peak_db = parsed_json.as_ref().and_then(|j| {
                    j.get(&peak_key).and_then(|v| v.as_str()).and_then(|s| {
                        if s.contains("-inf") { Some(-60.0f32) } else { s.parse::<f32>().ok() }
                    })
                });

                let raw_rms_db = parsed_json.as_ref().and_then(|j| {
                    j.get(&rms_key).and_then(|v| v.as_str()).and_then(|s| {
                        if s.contains("-inf") { Some(-60.0f32) } else { s.parse::<f32>().ok() }
                    })
                });

                let (in_target_peak, in_target_rms) = if let (Some(p_db), Some(r_db)) = (raw_peak_db, raw_rms_db) {
                    let p_lin = if p_db <= -59.5 { 0.0 } else { 10.0f32.powf(p_db / 20.0) };
                    let r_lin = if r_db <= -59.5 { 0.0 } else { 10.0f32.powf(r_db / 20.0) };
                    (p_lin.clamp(0.0, 1.5), r_lin.clamp(0.0, 1.5))
                } else {
                    // True silence when channel has no audio or metadata
                    (0.0, 0.0)
                };

                // Ballistics for Input
                let prev_in_peak = f32::from_bits(self.channels[ch].input_peak_linear.load(Ordering::Relaxed));
                let prev_in_rms = f32::from_bits(self.channels[ch].input_rms_linear.load(Ordering::Relaxed));
                let prev_in_hold = f32::from_bits(self.channels[ch].input_peak_hold_linear.load(Ordering::Relaxed));

                let in_attack = (dt * 65.0).min(1.0);
                let in_decay = (dt * 16.0).min(1.0);

                let new_in_peak = if in_target_peak >= prev_in_peak {
                    prev_in_peak + (in_target_peak - prev_in_peak) * in_attack
                } else {
                    (prev_in_peak - (prev_in_peak - in_target_peak) * in_decay).max(0.0)
                };

                let new_in_rms = if in_target_rms >= prev_in_rms {
                    prev_in_rms + (in_target_rms - prev_in_rms) * in_attack
                } else {
                    (prev_in_rms - (prev_in_rms - in_target_rms) * in_decay).max(0.0)
                };

                let new_in_hold = if new_in_peak >= prev_in_hold {
                    self.peak_hold_times_input[ch] = now;
                    new_in_peak
                } else if now.duration_since(self.peak_hold_times_input[ch]).as_secs_f32() > 1.2 {
                    (prev_in_hold - dt * 0.35).max(new_in_peak)
                } else {
                    prev_in_hold
                };

                self.channels[ch].input_peak_linear.store(new_in_peak.to_bits(), Ordering::Relaxed);
                self.channels[ch].input_rms_linear.store(new_in_rms.to_bits(), Ordering::Relaxed);
                self.channels[ch].input_peak_hold_linear.store(new_in_hold.to_bits(), Ordering::Relaxed);

                // Output Stage (Post-DSP)
                let is_ch_muted = self.channels[ch].is_muted.load(Ordering::Relaxed);
                let is_ch_solo = self.channels[ch].is_solo.load(Ordering::Relaxed);
                let active_gain = if is_ch_muted || (any_solo && !is_ch_solo) {
                    0.0
                } else {
                    vol_mult * self.eq_preamp
                };

                let out_target_peak = (in_target_peak * active_gain).clamp(0.0, 1.5);
                let out_target_rms = (in_target_rms * active_gain).clamp(0.0, 1.5);

                if out_target_peak >= CLIP_THRESHOLD_LINEAR {
                    self.channels[ch].is_clipped.store(true, Ordering::Relaxed);
                }

                let prev_out_peak = f32::from_bits(self.channels[ch].output_peak_linear.load(Ordering::Relaxed));
                let prev_out_rms = f32::from_bits(self.channels[ch].output_rms_linear.load(Ordering::Relaxed));
                let prev_out_hold = f32::from_bits(self.channels[ch].output_peak_hold_linear.load(Ordering::Relaxed));

                let new_out_peak = if out_target_peak >= prev_out_peak {
                    prev_out_peak + (out_target_peak - prev_out_peak) * in_attack
                } else {
                    (prev_out_peak - (prev_out_peak - out_target_peak) * in_decay).max(0.0)
                };

                let new_out_rms = if out_target_rms >= prev_out_rms {
                    prev_out_rms + (out_target_rms - prev_out_rms) * in_attack
                } else {
                    (prev_out_rms - (prev_out_rms - out_target_rms) * in_decay).max(0.0)
                };

                let new_out_hold = if new_out_peak >= prev_out_hold {
                    self.peak_hold_times_output[ch] = now;
                    new_out_peak
                } else if now.duration_since(self.peak_hold_times_output[ch]).as_secs_f32() > 1.2 {
                    (prev_out_hold - dt * 0.35).max(new_out_peak)
                } else {
                    prev_out_hold
                };

                self.channels[ch].output_peak_linear.store(new_out_peak.to_bits(), Ordering::Relaxed);
                self.channels[ch].output_rms_linear.store(new_out_rms.to_bits(), Ordering::Relaxed);
                self.channels[ch].output_peak_hold_linear.store(new_out_hold.to_bits(), Ordering::Relaxed);
            } else {
                // Instant decay to silence
                let decay = (dt * 35.0).min(1.0);

                let p_in = f32::from_bits(self.channels[ch].input_peak_linear.load(Ordering::Relaxed));
                let r_in = f32::from_bits(self.channels[ch].input_rms_linear.load(Ordering::Relaxed));
                let h_in = f32::from_bits(self.channels[ch].input_peak_hold_linear.load(Ordering::Relaxed));

                let p_out = f32::from_bits(self.channels[ch].output_peak_linear.load(Ordering::Relaxed));
                let r_out = f32::from_bits(self.channels[ch].output_rms_linear.load(Ordering::Relaxed));
                let h_out = f32::from_bits(self.channels[ch].output_peak_hold_linear.load(Ordering::Relaxed));

                self.channels[ch].input_peak_linear.store((p_in * (1.0 - decay)).max(0.0).to_bits(), Ordering::Relaxed);
                self.channels[ch].input_rms_linear.store((r_in * (1.0 - decay)).max(0.0).to_bits(), Ordering::Relaxed);
                self.channels[ch].input_peak_hold_linear.store((h_in * (1.0 - decay)).max(0.0).to_bits(), Ordering::Relaxed);

                self.channels[ch].output_peak_linear.store((p_out * (1.0 - decay)).max(0.0).to_bits(), Ordering::Relaxed);
                self.channels[ch].output_rms_linear.store((r_out * (1.0 - decay)).max(0.0).to_bits(), Ordering::Relaxed);
                self.channels[ch].output_peak_hold_linear.store((h_out * (1.0 - decay)).max(0.0).to_bits(), Ordering::Relaxed);
            }
        }
    }

    /// Read all 8 channel Output Peak levels (linear 0.0 to 1.0)
    pub fn get_channel_levels(&self) -> [f32; MAX_AUDIO_CHANNELS] {
        let mut out = [0.0f32; MAX_AUDIO_CHANNELS];
        for ch in 0..MAX_AUDIO_CHANNELS {
            out[ch] = f32::from_bits(self.channels[ch].output_peak_linear.load(Ordering::Relaxed));
        }
        out
    }

    /// Read all 8 channel Input Peak levels (linear 0.0 to 1.0)
    pub fn get_input_channel_levels(&self) -> [f32; MAX_AUDIO_CHANNELS] {
        let mut out = [0.0f32; MAX_AUDIO_CHANNELS];
        for ch in 0..MAX_AUDIO_CHANNELS {
            out[ch] = f32::from_bits(self.channels[ch].input_peak_linear.load(Ordering::Relaxed));
        }
        out
    }

    /// Get discrete ChannelLevels snapshot for Input Stage (Tap #1: Pre-DSP)
    pub fn get_input_levels_snapshot(&self, channels: usize) -> crate::engine::ChannelLevels {
        let mut peak_db = [crate::engine::MIN_METER_DB; crate::engine::MAX_CHANNELS];
        let mut rms_db = [crate::engine::MIN_METER_DB; crate::engine::MAX_CHANNELS];
        let mut peak_hold_db = [crate::engine::MIN_METER_DB; crate::engine::MAX_CHANNELS];
        let mut clipped = [false; crate::engine::MAX_CHANNELS];

        for ch in 0..crate::engine::MAX_CHANNELS {
            let p_lin = f32::from_bits(self.channels[ch].input_peak_linear.load(Ordering::Relaxed));
            let r_lin = f32::from_bits(self.channels[ch].input_rms_linear.load(Ordering::Relaxed));
            let h_lin = f32::from_bits(self.channels[ch].input_peak_hold_linear.load(Ordering::Relaxed));

            peak_db[ch] = crate::engine::linear_to_db(p_lin);
            rms_db[ch] = crate::engine::linear_to_db(r_lin);
            peak_hold_db[ch] = crate::engine::linear_to_db(h_lin);
            clipped[ch] = self.channels[ch].is_clipped.load(Ordering::Relaxed);
        }

        crate::engine::ChannelLevels {
            peak_db,
            rms_db,
            peak_hold_db,
            clipped,
            channels,
        }
    }

    /// Get discrete ChannelLevels snapshot for Output Stage (Tap #2: Post-DSP)
    pub fn get_output_levels_snapshot(&self, channels: usize) -> crate::engine::ChannelLevels {
        let mut peak_db = [crate::engine::MIN_METER_DB; crate::engine::MAX_CHANNELS];
        let mut rms_db = [crate::engine::MIN_METER_DB; crate::engine::MAX_CHANNELS];
        let mut peak_hold_db = [crate::engine::MIN_METER_DB; crate::engine::MAX_CHANNELS];
        let mut clipped = [false; crate::engine::MAX_CHANNELS];

        for ch in 0..crate::engine::MAX_CHANNELS {
            let p_lin = f32::from_bits(self.channels[ch].output_peak_linear.load(Ordering::Relaxed));
            let r_lin = f32::from_bits(self.channels[ch].output_rms_linear.load(Ordering::Relaxed));
            let h_lin = f32::from_bits(self.channels[ch].output_peak_hold_linear.load(Ordering::Relaxed));

            peak_db[ch] = crate::engine::linear_to_db(p_lin);
            rms_db[ch] = crate::engine::linear_to_db(r_lin);
            peak_hold_db[ch] = crate::engine::linear_to_db(h_lin);
            clipped[ch] = self.channels[ch].is_clipped.load(Ordering::Relaxed);
        }

        crate::engine::ChannelLevels {
            peak_db,
            rms_db,
            peak_hold_db,
            clipped,
            channels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51_astats_parsing_and_channel_discovery() {
        let mut filter = AudioTransformFilter::new();
        let astats_json = r#"{
            "lavfi.astats.1.Peak_level": "-3.0",
            "lavfi.astats.1.RMS_level": "-12.0",
            "lavfi.astats.2.Peak_level": "-3.5",
            "lavfi.astats.2.RMS_level": "-12.5",
            "lavfi.astats.3.Peak_level": "-1.5",
            "lavfi.astats.3.RMS_level": "-8.0",
            "lavfi.astats.4.Peak_level": "-0.5",
            "lavfi.astats.4.RMS_level": "-6.0",
            "lavfi.astats.5.Peak_level": "-6.0",
            "lavfi.astats.5.RMS_level": "-18.0",
            "lavfi.astats.6.Peak_level": "-6.2",
            "lavfi.astats.6.RMS_level": "-18.2"
        }"#;

        // Even if active_channels is passed as 0 or 2 (before initial media probe), astats detects all 6 channels
        filter.update_from_astats_json(astats_json, 0, true, false, 100.0);
        assert_eq!(filter.channel_count, 6);

        let snap = filter.get_input_levels_snapshot(6);
        assert_eq!(snap.channels, 6);
        // All 6 discrete channels (FL, FR, FC, LFE, SL, SR) should have non-silent peaks (> -60dB)
        for ch in 0..6 {
            assert!(snap.peak_db[ch] > -50.0, "Channel {} should be active, got {}", ch, snap.peak_db[ch]);
        }
    }
}
