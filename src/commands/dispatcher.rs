#![allow(dead_code)]

use super::definitions::Command;
use crate::engine::Player;
use std::path::PathBuf;
use std::sync::Arc;

pub struct CommandDispatcher;

impl CommandDispatcher {
    pub fn dispatch(cmd: &Command, player: Option<&Arc<Player>>) {
        if let Some(p) = player {
            match cmd {
                Command::PlayPause => p.toggle_pause(),
                Command::Stop => p.stop(),
                Command::Seek { seconds } => p.seek_relative(*seconds),
                Command::SetSpeed { speed } => p.set_speed(*speed),
                Command::AdjustSpeed { delta } => {
                    let cur = p.stats().speed;
                    p.set_speed((cur + delta).clamp(0.1, 10.0));
                }
                Command::SetVolume { percent } => p.set_volume(*percent),
                Command::AdjustVolume { delta } => {
                    let cur = p.stats().volume;
                    p.set_volume((cur + delta).clamp(0.0, 200.0));
                }
                Command::ToggleMute => p.toggle_mute(),
                Command::FrameStep { forward } => {
                    if *forward {
                        p.frame_step();
                    } else {
                        p.frame_back_step();
                    }
                }
                Command::KeyframeStep { forward } => {
                    p.keyframe_step(*forward);
                }
                Command::ChapterStep { forward } => {
                    p.chapter_step(*forward);
                }
                Command::SubtitleDelay { delta_ms } => {
                    p.adjust_subtitle_delay(*delta_ms as f64 / 1000.0);
                }
                Command::SubtitleToggle => p.toggle_subtitles(),
                Command::SubtitleCycleTrack => p.cycle_subtitle_track(),
                Command::AudioCycleTrack => p.cycle_audio_track(),
                Command::SetAspectRatio { ratio } => p.set_aspect_ratio(ratio),
                Command::TakeScreenshot => {
                    let out = dirs::picture_dir().unwrap_or_else(|| PathBuf::from(".")).join("screenshot.png");
                    p.take_screenshot(&out);
                }
                _ => {}
            }
        }
    }
}
