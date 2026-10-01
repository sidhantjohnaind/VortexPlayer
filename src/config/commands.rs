#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerCommand {
    PlayPause,
    Stop,
    SeekShortFwd,      // default +5s
    SeekShortBack,     // default -5s
    SeekMediumFwd,     // default +30s
    SeekMediumBack,    // default -30s
    SeekLongFwd,       // default +60s
    SeekLongBack,      // default -60s
    SeekHugeFwd,       // default +300s
    SeekHugeBack,      // default -300s
    FrameStepFwd,
    FrameStepBack,
    NextChapter,
    PrevChapter,
    NextFile,
    PrevFile,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    SubtitleToggle,
    SubtitleCycle,
    SubtitleDelayPlus,
    SubtitleDelayMinus,
    SubtitleSizePlus,
    SubtitleSizeMinus,
    SubtitlePosUp,
    SubtitlePosDown,
    AudioCycleTrack,
    VideoCycleAspect,
    VideoRotateCw,
    VideoFlipH,
    VideoFullscreen,
    ToggleDiagnostics,
    TogglePlaylist,
    ToggleControlPanel,
    TogglePreferences,
    ToggleBookmarkStudio,
    ToggleSubtitleExplorer,
    ToggleMediaInfo,
    SpeedUp,
    SpeedDown,
    SpeedReset,
    RepeatSubtitleSentence,
    TakeScreenshot,
}

impl PlayerCommand {
    pub fn id_str(&self) -> &'static str {
        match self {
            PlayerCommand::PlayPause => "player.play_pause",
            PlayerCommand::Stop => "player.stop",
            PlayerCommand::SeekShortFwd => "player.seek_short_fwd",
            PlayerCommand::SeekShortBack => "player.seek_short_back",
            PlayerCommand::SeekMediumFwd => "player.seek_medium_fwd",
            PlayerCommand::SeekMediumBack => "player.seek_medium_back",
            PlayerCommand::SeekLongFwd => "player.seek_long_fwd",
            PlayerCommand::SeekLongBack => "player.seek_long_back",
            PlayerCommand::SeekHugeFwd => "player.seek_huge_fwd",
            PlayerCommand::SeekHugeBack => "player.seek_huge_back",
            PlayerCommand::FrameStepFwd => "player.frame_step_fwd",
            PlayerCommand::FrameStepBack => "player.frame_step_back",
            PlayerCommand::NextChapter => "player.next_chapter",
            PlayerCommand::PrevChapter => "player.prev_chapter",
            PlayerCommand::NextFile => "player.next_file",
            PlayerCommand::PrevFile => "player.prev_file",
            PlayerCommand::VolumeUp => "audio.volume_up",
            PlayerCommand::VolumeDown => "audio.volume_down",
            PlayerCommand::VolumeMute => "audio.volume_mute",
            PlayerCommand::SubtitleToggle => "subtitle.toggle",
            PlayerCommand::SubtitleCycle => "subtitle.cycle",
            PlayerCommand::SubtitleDelayPlus => "subtitle.delay_plus",
            PlayerCommand::SubtitleDelayMinus => "subtitle.delay_minus",
            PlayerCommand::SubtitleSizePlus => "subtitle.size_plus",
            PlayerCommand::SubtitleSizeMinus => "subtitle.size_minus",
            PlayerCommand::SubtitlePosUp => "subtitle.pos_up",
            PlayerCommand::SubtitlePosDown => "subtitle.pos_down",
            PlayerCommand::AudioCycleTrack => "audio.cycle_track",
            PlayerCommand::VideoCycleAspect => "video.cycle_aspect",
            PlayerCommand::VideoRotateCw => "video.rotate_cw",
            PlayerCommand::VideoFlipH => "video.flip_h",
            PlayerCommand::VideoFullscreen => "video.fullscreen",
            PlayerCommand::ToggleDiagnostics => "ui.toggle_diagnostics",
            PlayerCommand::TogglePlaylist => "ui.toggle_playlist",
            PlayerCommand::ToggleControlPanel => "ui.toggle_control_panel",
            PlayerCommand::TogglePreferences => "ui.toggle_preferences",
            PlayerCommand::ToggleBookmarkStudio => "ui.toggle_bookmark_studio",
            PlayerCommand::ToggleSubtitleExplorer => "ui.toggle_subtitle_explorer",
            PlayerCommand::ToggleMediaInfo => "ui.toggle_mediainfo",
            PlayerCommand::SpeedUp => "player.speed_up",
            PlayerCommand::SpeedDown => "player.speed_down",
            PlayerCommand::SpeedReset => "player.speed_reset",
            PlayerCommand::RepeatSubtitleSentence => "subtitle.repeat_sentence",
            PlayerCommand::TakeScreenshot => "capture.screenshot",
        }
    }
}
