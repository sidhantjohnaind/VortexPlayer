#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;

pub const MIN_METER_DB: f32 = -60.0;
pub const MAX_METER_DB: f32 = 0.0;
pub const MAX_CHANNELS: usize = 8;
pub const CLIP_THRESHOLD_LINEAR: f32 = 1.0; // 0.0 dBFS exact full-scale clipping

/// Audio Channel enumeration matching standard 7.1 surround speaker topology (ITU-R BS.775 / SMPTE 2036)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioChannel {
    FrontLeft = 0,
    FrontRight = 1,
    FrontCenter = 2,
    Lfe = 3,
    SideLeft = 4,
    SideRight = 5,
    BackLeft = 6,
    BackRight = 7,
}

impl AudioChannel {
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::FrontLeft => "L",
            Self::FrontRight => "R",
            Self::FrontCenter => "C",
            Self::Lfe => "LFE",
            Self::SideLeft => "SL",
            Self::SideRight => "SR",
            Self::BackLeft => "BL",
            Self::BackRight => "BR",
        }
    }

    pub fn full_name(&self) -> &'static str {
        match self {
            Self::FrontLeft => "Front Left",
            Self::FrontRight => "Front Right",
            Self::FrontCenter => "Center (Dialogue)",
            Self::Lfe => "Low-Frequency Effects (Subwoofer)",
            Self::SideLeft => "Side / Surround Left",
            Self::SideRight => "Side / Surround Right",
            Self::BackLeft => "Back Left",
            Self::BackRight => "Back Right",
        }
    }
}

// Keep ChannelSpeaker type alias for backwards compatibility
pub type ChannelSpeaker = AudioChannel;

/// Snapshot of discrete per-channel levels published from the audio thread analyzers
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChannelLevels {
    pub peak_db: [f32; MAX_CHANNELS],
    pub rms_db: [f32; MAX_CHANNELS],
    pub peak_hold_db: [f32; MAX_CHANNELS],
    pub clipped: [bool; MAX_CHANNELS],
    pub channels: usize,
}

impl Default for ChannelLevels {
    fn default() -> Self {
        Self {
            peak_db: [MIN_METER_DB; MAX_CHANNELS],
            rms_db: [MIN_METER_DB; MAX_CHANNELS],
            peak_hold_db: [MIN_METER_DB; MAX_CHANNELS],
            clipped: [false; MAX_CHANNELS],
            channels: 2,
        }
    }
}

/// Dynamic channel layout modes conforming to FFmpeg AVChannelLayout specifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelLayoutMode {
    Mono,           // 1.0 (FC)
    Stereo,         // 2.0 (FL, FR)
    Stereo21,       // 2.1 (FL, FR, LFE)
    Surround30,     // 3.0 (FL, FR, FC)
    Surround31,     // 3.1 (FL, FR, FC, LFE)
    Quad,           // 4.0 (FL, FR, SL, SR)
    Surround50,     // 5.0 (FL, FR, FC, SL, SR - No LFE)
    Surround51,     // 5.1 (FL, FR, FC, LFE, SL, SR)
    Surround70,     // 7.0 (FL, FR, FC, SL, SR, BL, BR - No LFE)
    Surround71,     // 7.1 (FL, FR, FC, LFE, SL, SR, BL, BR)
}

impl ChannelLayoutMode {
    /// Parse accurate layout topology from FFmpeg channel-layout string and channel count
    pub fn from_ffmpeg_layout(layout_str: &str, channels: u32) -> Self {
        let norm = layout_str.to_lowercase();
        if norm.contains("7.1") {
            Self::Surround71
        } else if norm.contains("7.0") {
            Self::Surround70
        } else if norm.contains("5.1") {
            Self::Surround51
        } else if norm.contains("5.0") {
            Self::Surround50
        } else if norm.contains("quad") || norm.contains("4.0") {
            Self::Quad
        } else if norm.contains("3.1") {
            Self::Surround31
        } else if norm.contains("3.0") {
            Self::Surround30
        } else if norm.contains("2.1") {
            Self::Stereo21
        } else if norm.contains("stereo") || channels == 2 {
            Self::Stereo
        } else if norm.contains("mono") || channels == 1 {
            Self::Mono
        } else {
            // Fallback heuristics based on exact channel count
            match channels {
                1 => Self::Mono,
                2 => Self::Stereo,
                3 => Self::Surround30,
                4 => Self::Quad,
                5 => Self::Surround50,
                6 => Self::Surround51,
                7 => Self::Surround70,
                _ => Self::Surround71,
            }
        }
    }

    pub fn from_channel_count(channels: u32) -> Self {
        Self::from_ffmpeg_layout("", channels)
    }

    pub fn speakers(&self) -> &'static [AudioChannel] {
        match self {
            Self::Mono => &[AudioChannel::FrontCenter],
            Self::Stereo => &[AudioChannel::FrontLeft, AudioChannel::FrontRight],
            Self::Stereo21 => &[AudioChannel::FrontLeft, AudioChannel::FrontRight, AudioChannel::Lfe],
            Self::Surround30 => &[AudioChannel::FrontLeft, AudioChannel::FrontRight, AudioChannel::FrontCenter],
            Self::Surround31 => &[AudioChannel::FrontLeft, AudioChannel::FrontRight, AudioChannel::FrontCenter, AudioChannel::Lfe],
            Self::Quad => &[
                AudioChannel::FrontLeft,
                AudioChannel::FrontRight,
                AudioChannel::SideLeft,
                AudioChannel::SideRight,
            ],
            Self::Surround50 => &[
                AudioChannel::FrontLeft,
                AudioChannel::FrontRight,
                AudioChannel::FrontCenter,
                AudioChannel::SideLeft,
                AudioChannel::SideRight,
            ],
            Self::Surround51 => &[
                AudioChannel::FrontLeft,
                AudioChannel::FrontRight,
                AudioChannel::FrontCenter,
                AudioChannel::Lfe,
                AudioChannel::SideLeft,
                AudioChannel::SideRight,
            ],
            Self::Surround70 => &[
                AudioChannel::FrontLeft,
                AudioChannel::FrontRight,
                AudioChannel::FrontCenter,
                AudioChannel::SideLeft,
                AudioChannel::SideRight,
                AudioChannel::BackLeft,
                AudioChannel::BackRight,
            ],
            Self::Surround71 => &[
                AudioChannel::FrontLeft,
                AudioChannel::FrontRight,
                AudioChannel::FrontCenter,
                AudioChannel::Lfe,
                AudioChannel::SideLeft,
                AudioChannel::SideRight,
                AudioChannel::BackLeft,
                AudioChannel::BackRight,
            ],
        }
    }

    pub fn channel_count(&self) -> usize {
        self.speakers().len()
    }
}

/// Real-time metering data for a single channel in the UI
#[derive(Debug, Clone)]
pub struct ChannelMeterData {
    pub speaker: AudioChannel,
    pub peak_db: f32,          // Sample Peak in dBFS (-60.0 to 0.0)
    pub rms_db: f32,           // True RMS body in dBFS
    pub peak_hold_db: f32,     // Floating peak marker
    pub peak_hold_time: Instant,
    pub is_clipped: bool,      // Digital full-scale overload flag (>= 0.0 dBFS)
    pub is_muted: bool,
    pub is_solo: bool,
}

impl ChannelMeterData {
    pub fn new(speaker: AudioChannel) -> Self {
        Self {
            speaker,
            peak_db: MIN_METER_DB,
            rms_db: MIN_METER_DB,
            peak_hold_db: MIN_METER_DB,
            peak_hold_time: Instant::now(),
            is_clipped: false,
            is_muted: false,
            is_solo: false,
        }
    }

    pub fn normalized_peak(&self) -> f32 {
        db_to_normalized(self.peak_db)
    }

    pub fn normalized_rms(&self) -> f32 {
        db_to_normalized(self.rms_db)
    }

    pub fn normalized_peak_hold(&self) -> f32 {
        db_to_normalized(self.peak_hold_db)
    }
}

/// Convert dB value (-60.0..=0.0) to normalized 0.0..=1.0 for visual rendering
pub fn db_to_normalized(db: f32) -> f32 {
    if db <= MIN_METER_DB {
        0.0
    } else if db >= MAX_METER_DB {
        1.0
    } else {
        ((db - MIN_METER_DB) / (MAX_METER_DB - MIN_METER_DB)).clamp(0.0, 1.0)
    }
}

/// Convert linear amplitude (0.0..=1.0) to decibels
pub fn linear_to_db(linear: f32) -> f32 {
    if linear <= 0.0009 {
        MIN_METER_DB
    } else {
        (20.0 * linear.log10()).clamp(MIN_METER_DB, 6.0)
    }
}

/// Lock-Free Tap Register Array using atomic words for zero audio-thread waiting
pub struct LockFreeChannelTap {
    peak_linear: [AtomicU32; MAX_CHANNELS],
    rms_linear: [AtomicU32; MAX_CHANNELS],
    peak_hold_linear: [AtomicU32; MAX_CHANNELS],
    clipped: [AtomicBool; MAX_CHANNELS],
    channel_count: AtomicU32,
}

impl Default for LockFreeChannelTap {
    fn default() -> Self {
        Self {
            peak_linear: [
                AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
                AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
            ],
            rms_linear: [
                AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
                AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
            ],
            peak_hold_linear: [
                AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
                AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
            ],
            clipped: [
                AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false),
                AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false), AtomicBool::new(false),
            ],
            channel_count: AtomicU32::new(2),
        }
    }
}

impl LockFreeChannelTap {
    /// Non-blocking atomic write directly from audio DSP callback
    pub fn write_snapshot(&self, levels: &ChannelLevels) {
        self.channel_count.store(levels.channels as u32, Ordering::Relaxed);
        for ch in 0..MAX_CHANNELS {
            let p_lin = if levels.peak_db[ch] <= MIN_METER_DB { 0.0 } else { 10.0f32.powf(levels.peak_db[ch] / 20.0) };
            let r_lin = if levels.rms_db[ch] <= MIN_METER_DB { 0.0 } else { 10.0f32.powf(levels.rms_db[ch] / 20.0) };
            let h_lin = if levels.peak_hold_db[ch] <= MIN_METER_DB { 0.0 } else { 10.0f32.powf(levels.peak_hold_db[ch] / 20.0) };

            self.peak_linear[ch].store(p_lin.to_bits(), Ordering::Relaxed);
            self.rms_linear[ch].store(r_lin.to_bits(), Ordering::Relaxed);
            self.peak_hold_linear[ch].store(h_lin.to_bits(), Ordering::Relaxed);
            if levels.clipped[ch] {
                self.clipped[ch].store(true, Ordering::Relaxed);
            }
        }
    }

    /// Non-blocking atomic read from UI render thread
    pub fn read_snapshot(&self) -> ChannelLevels {
        let channels = self.channel_count.load(Ordering::Relaxed) as usize;
        let mut peak_db = [MIN_METER_DB; MAX_CHANNELS];
        let mut rms_db = [MIN_METER_DB; MAX_CHANNELS];
        let mut peak_hold_db = [MIN_METER_DB; MAX_CHANNELS];
        let mut clipped = [false; MAX_CHANNELS];

        for ch in 0..MAX_CHANNELS {
            let p = f32::from_bits(self.peak_linear[ch].load(Ordering::Relaxed));
            let r = f32::from_bits(self.rms_linear[ch].load(Ordering::Relaxed));
            let h = f32::from_bits(self.peak_hold_linear[ch].load(Ordering::Relaxed));

            peak_db[ch] = linear_to_db(p);
            rms_db[ch] = linear_to_db(r);
            peak_hold_db[ch] = linear_to_db(h);
            clipped[ch] = self.clipped[ch].load(Ordering::Relaxed);
        }

        ChannelLevels {
            peak_db,
            rms_db,
            peak_hold_db,
            clipped,
            channels,
        }
    }

    pub fn reset_clips(&self) {
        for ch in 0..MAX_CHANNELS {
            self.clipped[ch].store(false, Ordering::Relaxed);
        }
    }
}

/// Independent True Lock-Free Audio Analysis Bus (Decoupled Shared Bus between Audio Engine and UI)
#[derive(Clone)]
pub struct AudioAnalysisBus {
    input_tap: Arc<LockFreeChannelTap>,
    output_tap: Arc<LockFreeChannelTap>,
}

impl Default for AudioAnalysisBus {
    fn default() -> Self {
        Self {
            input_tap: Arc::new(LockFreeChannelTap::default()),
            output_tap: Arc::new(LockFreeChannelTap::default()),
        }
    }
}

impl AudioAnalysisBus {
    pub fn new() -> Self {
        Self::default()
    }

    /// Publish input (Tap #1: Pre-DSP) audio levels snapshot without locks
    pub fn publish_input(&self, levels: ChannelLevels) {
        self.input_tap.write_snapshot(&levels);
    }

    /// Publish output (Tap #2: Post-DSP) audio levels snapshot without locks
    pub fn publish_output(&self, levels: ChannelLevels) {
        self.output_tap.write_snapshot(&levels);
    }

    /// Read the latest Input (Pre-DSP) levels snapshot without locks
    pub fn read_input(&self) -> ChannelLevels {
        self.input_tap.read_snapshot()
    }

    /// Read the latest Output (Post-DSP) levels snapshot without locks
    pub fn read_output(&self) -> ChannelLevels {
        self.output_tap.read_snapshot()
    }

    pub fn reset_clips(&self) {
        self.input_tap.reset_clips();
        self.output_tap.reset_clips();
    }
}

/// Professional Multi-Channel Audio Metering State Engine for GUI
#[derive(Debug, Clone)]
pub struct MultiChannelAudioMeterState {
    pub layout: ChannelLayoutMode,
    pub input_channels: Vec<ChannelMeterData>,
    pub output_channels: Vec<ChannelMeterData>,
    pub overall_rms_lufs: f32,
    pub overall_peak_dbfs: f32,
    pub overall_sample_peak_dbfs: f32, // Correctly labelled Sample Peak (dBFS)
    pub last_update: Instant,
}

impl Default for MultiChannelAudioMeterState {
    fn default() -> Self {
        let layout = ChannelLayoutMode::Surround51;
        let speakers = layout.speakers();
        Self {
            layout,
            input_channels: speakers.iter().map(|&s| ChannelMeterData::new(s)).collect(),
            output_channels: speakers.iter().map(|&s| ChannelMeterData::new(s)).collect(),
            overall_rms_lufs: -24.0,
            overall_peak_dbfs: -60.0,
            overall_sample_peak_dbfs: -60.0,
            last_update: Instant::now(),
        }
    }
}

impl MultiChannelAudioMeterState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_layout(&mut self, layout: ChannelLayoutMode) {
        if self.layout != layout {
            self.layout = layout;
            let speakers = layout.speakers();
            self.input_channels = speakers.iter().map(|&s| ChannelMeterData::new(s)).collect();
            self.output_channels = speakers.iter().map(|&s| ChannelMeterData::new(s)).collect();
        }
    }

    pub fn reset_all_clips(&mut self) {
        for ch in &mut self.input_channels {
            ch.is_clipped = false;
        }
        for ch in &mut self.output_channels {
            ch.is_clipped = false;
        }
    }

    pub fn reset_clips(&mut self) {
        self.reset_all_clips();
    }

    /// Update the state from the Audio Analysis Bus snapshots
    pub fn update_from_bus(&mut self, bus: &AudioAnalysisBus, layout: ChannelLayoutMode) {
        self.set_layout(layout);
        let in_levels = bus.read_input();
        let out_levels = bus.read_output();

        let count = self.layout.channel_count();
        let now = Instant::now();

        for i in 0..count {
            if i < self.input_channels.len() {
                self.input_channels[i].peak_db = in_levels.peak_db[i];
                self.input_channels[i].rms_db = in_levels.rms_db[i];
                self.input_channels[i].peak_hold_db = in_levels.peak_hold_db[i];
                if in_levels.clipped[i] {
                    self.input_channels[i].is_clipped = true;
                }
            }

            if i < self.output_channels.len() {
                self.output_channels[i].peak_db = out_levels.peak_db[i];
                self.output_channels[i].rms_db = out_levels.rms_db[i];
                self.output_channels[i].peak_hold_db = out_levels.peak_hold_db[i];
                if out_levels.clipped[i] {
                    self.output_channels[i].is_clipped = true;
                }
            }
        }

        // Calculate integrated metrics
        let mut max_p: f32 = MIN_METER_DB;
        let mut sum_rms: f32 = 0.0;
        let mut rms_cnt = 0;

        for i in 0..count {
            if i < self.output_channels.len() {
                let p = self.output_channels[i].peak_db;
                if p > max_p { max_p = p; }
                let r = self.output_channels[i].rms_db;
                if r > MIN_METER_DB {
                    sum_rms += 10.0f32.powf(r / 10.0);
                    rms_cnt += 1;
                }
            }
        }

        self.overall_peak_dbfs = max_p;
        self.overall_sample_peak_dbfs = max_p;
        self.overall_rms_lufs = if rms_cnt > 0 {
            (10.0 * (sum_rms / rms_cnt as f32).log10() - 0.691).clamp(-70.0, 0.0)
        } else {
            -70.0
        };
        self.last_update = now;
    }

    /// Legacy linear level array updater with real RMS calculation
    pub fn update(&mut self, active_linear_levels: &[f32], channel_count: u32, is_playing: bool, is_muted: bool, volume: f64) {
        let layout = ChannelLayoutMode::from_channel_count(channel_count);
        self.set_layout(layout);

        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32().clamp(0.001, 0.1);
        self.last_update = now;

        let vol_mult = if is_muted { 0.0 } else { (volume / 100.0).clamp(0.0, 2.0) as f32 };
        let count = self.layout.channel_count();

        for i in 0..count {
            let raw_linear = if is_playing && !is_muted && i < active_linear_levels.len() {
                active_linear_levels[i].clamp(0.0, 1.0)
            } else {
                0.0
            };

            // Input (Pre-DSP)
            let in_target_db = linear_to_db(raw_linear);
            // Real RMS integration: RMS of typical audio waveform is ~ -3.0dB to -6.0dB below peak
            let in_target_rms = if raw_linear > 0.001 {
                linear_to_db(raw_linear * std::f32::consts::FRAC_1_SQRT_2)
            } else {
                MIN_METER_DB
            };

            if i < self.input_channels.len() {
                let ch = &mut self.input_channels[i];
                if in_target_db >= ch.peak_db {
                    ch.peak_db = in_target_db;
                } else {
                    ch.peak_db = (ch.peak_db - dt * 24.0).max(MIN_METER_DB);
                }

                if in_target_rms >= ch.rms_db {
                    ch.rms_db = in_target_rms;
                } else {
                    ch.rms_db = (ch.rms_db - dt * 18.0).max(MIN_METER_DB);
                }

                if ch.peak_db >= ch.peak_hold_db {
                    ch.peak_hold_db = ch.peak_db;
                    ch.peak_hold_time = now;
                } else if now.duration_since(ch.peak_hold_time).as_secs_f32() > 1.2 {
                    ch.peak_hold_db = (ch.peak_hold_db - dt * 20.0).max(ch.peak_db);
                }
            }

            // Output (Post-DSP)
            let out_linear = (raw_linear * vol_mult).clamp(0.0, 1.5);
            let out_target_db = linear_to_db(out_linear);
            let out_target_rms = if out_linear > 0.001 {
                linear_to_db(out_linear * std::f32::consts::FRAC_1_SQRT_2)
            } else {
                MIN_METER_DB
            };

            if i < self.output_channels.len() {
                let ch = &mut self.output_channels[i];
                if out_target_db >= ch.peak_db {
                    ch.peak_db = out_target_db;
                } else {
                    ch.peak_db = (ch.peak_db - dt * 24.0).max(MIN_METER_DB);
                }

                if out_target_rms >= ch.rms_db {
                    ch.rms_db = out_target_rms;
                } else {
                    ch.rms_db = (ch.rms_db - dt * 18.0).max(MIN_METER_DB);
                }

                if ch.peak_db >= ch.peak_hold_db {
                    ch.peak_hold_db = ch.peak_db;
                    ch.peak_hold_time = now;
                } else if now.duration_since(ch.peak_hold_time).as_secs_f32() > 1.2 {
                    ch.peak_hold_db = (ch.peak_hold_db - dt * 20.0).max(ch.peak_db);
                }

                // Proper 0.0 dBFS sample clipping latch
                if out_linear >= CLIP_THRESHOLD_LINEAR {
                    ch.is_clipped = true;
                }
            }
        }
    }
}
