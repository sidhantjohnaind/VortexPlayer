#![allow(unused_imports)]

pub mod music_view;
pub use music_view::MusicBackgroundView;
pub mod audio_meter;
pub use audio_meter::*;
pub mod osd_events;
pub mod timeline_markers;

pub use osd_events::{ActiveOsdNotification, OsdEvent};
pub use timeline_markers::{MarkerType, TimelineMarker, TimelineMarkerRenderer};
pub mod input_editor;
pub use input_editor::InputEditorDialog;
pub mod bookmark_overlay;
pub mod bookmark_studio;
pub mod capture_dialog;
pub mod control_panel;
pub mod controls;
pub mod file_navigator;
pub mod icons;
pub mod jump_time_dialog;
pub use jump_time_dialog::JumpTimeDialog;
pub mod library_view;
pub mod lyrics_overlay;
pub mod mediainfo_dialog;
pub mod menu;
pub mod network_browser_dialog;
pub mod osd;
pub mod parametric_eq_dialog;
pub mod pip_mode;
pub mod playlist_panel;
pub mod preferences_dialog;
pub mod settings_inspector;
pub mod split_compare;
pub mod stream_url_dialog;
pub mod subtitle_explorer;
pub mod telemetry_hud;
pub mod theme;
pub mod titlebar;
pub mod toast;
pub mod visualizer;
pub mod win32_menu;
pub mod gtk_menu;
pub mod seek_preview;
pub use seek_preview::*;

pub use bookmark_overlay::BookmarkOverlay;
pub use bookmark_studio::BookmarkStudioDialog;
pub use capture_dialog::CaptureDialog;
pub use control_panel::ControlPanel;
pub use controls::ControlBar;
pub use file_navigator::FileNavigatorSidebar;
pub use icons::Icons;
pub use library_view::LibraryView;
pub use lyrics_overlay::LyricsOverlay;
pub use mediainfo_dialog::MediaInfoDialog;
pub use menu::VortexMenu;
pub use network_browser_dialog::NetworkBrowserDialog;
pub use osd::OsdEngine;
pub use parametric_eq_dialog::ParametricEqDialog;
pub use pip_mode::PipModeView;
pub use playlist_panel::PlaylistPanel;
pub use preferences_dialog::PreferencesDialog;
pub use settings_inspector::SettingsInspectorDialog;
pub use split_compare::SplitCompareView;
pub use stream_url_dialog::StreamUrlDialog;
pub use subtitle_explorer::SubtitleExplorerDialog;
pub use telemetry_hud::TelemetryHudView;
pub use theme::{ThemeMode, VortexTheme};
pub use titlebar::TitleBar;
pub use toast::ToastManager;
pub use visualizer::{VisualizerMode, VisualizerSuite};

pub mod notification_nav;
pub use notification_nav::{NotificationNavBar, NotificationNavActions};

pub mod record_dialog;
pub mod contact_sheet_dialog;
pub use record_dialog::StreamRecordDialog;
pub use contact_sheet_dialog::ContactSheetDialog;

pub mod gif_maker_dialog;
pub mod device_capture_dialog;
pub mod subtitle_lookup_dialog;
pub use gif_maker_dialog::GifMakerDialog;
pub use device_capture_dialog::DeviceCaptureDialog;
pub use subtitle_lookup_dialog::SubtitleLookupDialog;

pub mod taskbar;
pub use taskbar::TaskbarIntegration;

pub mod shader_studio_dialog;
pub mod broadcast_dialog;
pub mod chain_editor;
pub mod channel_matrix_dialog;
pub mod auto_skip_dialog;

pub use shader_studio_dialog::ShaderStudioDialog;
pub use broadcast_dialog::BroadcastDialog;
pub use chain_editor::ChainEditorDialog;
pub use channel_matrix_dialog::{ChannelMatrixConfig, ChannelMatrixDialog};
pub use auto_skip_dialog::AutoSkipDialog;

pub mod stereo_3d_dialog;
pub mod karaoke_studio_dialog;
pub mod subtitle_studio_dialog;
pub mod screen_capture_dialog;
pub mod iptv_guide_dialog;
pub mod logo_watermark_dialog;

pub mod cd_ripper_dialog;
pub mod disc_navigation_dialog;
pub mod bda_tuner_dialog;
pub mod bdj_dialog;
pub mod vst_winamp_dialog;
pub mod detached_windows;
pub mod discord_rpc_dialog;
pub mod web_remote_dialog;
pub mod scrobbler_dialog;
pub mod media_server_dialog;
pub mod ambilight_dialog;
pub mod cast_renderer_dialog;
pub mod transcoder_dialog;
pub mod zoom_magnifier_dialog;
pub mod radio_directory_dialog;
pub mod video_puzzle_dialog;
pub mod vr_360_studio_dialog;
pub mod damaged_file_repair_dialog;
pub mod binaural_crossfeed_dialog;
pub mod video_wall_matrix_dialog;
pub mod chapter_marker_dialog;

pub use stereo_3d_dialog::{Stereo3dDialog, Stereo3dPreset};
pub use karaoke_studio_dialog::KaraokeStudioDialog;
pub use subtitle_studio_dialog::{SubtitleCue, SubtitleStudioDialog};
pub use screen_capture_dialog::{CaptureSourceType, ScreenCaptureDialog};
pub use iptv_guide_dialog::IptvGuideDialog;
pub use logo_watermark_dialog::{LogoWatermarkDialog, WatermarkPosition};
pub use cd_ripper_dialog::{CdRipperDialog, CdTrack, RipFormat};
pub use disc_navigation_dialog::{DiscNavigationDialog, DiscTitleInfo};
pub use bda_tuner_dialog::{BdaTunerDialog, TunerStandard, TvChannel};
pub use bdj_dialog::{BdjDialog, BlurayRegion};
pub use vst_winamp_dialog::{AudioPluginEntry, PluginType, VstWinampDialog};
pub use detached_windows::DetachedWindowsController;
pub use discord_rpc_dialog::{DiscordRpcDialog, RpcPrivacyMode};
pub use web_remote_dialog::WebRemoteDialog;
pub use scrobbler_dialog::ScrobblerDialog;
pub use media_server_dialog::{MediaServerDialog, ServerType};
pub use ambilight_dialog::{AmbilightDialog, LightingDevice};
pub use cast_renderer_dialog::{CastDeviceType, CastRendererDevice, CastRendererDialog};
pub use transcoder_dialog::{OutputContainer, TranscoderDialog};
pub use zoom_magnifier_dialog::ZoomMagnifierDialog;
pub use radio_directory_dialog::{RadioDirectoryDialog, RadioStation};
pub use video_puzzle_dialog::VideoPuzzleDialog;
pub use vr_360_studio_dialog::{Vr360StudioDialog, VrProjectionMode};
pub use damaged_file_repair_dialog::DamagedFileRepairDialog;
pub use binaural_crossfeed_dialog::{BinauralCrossfeedDialog, CrossfeedPreset};
pub use video_wall_matrix_dialog::VideoWallMatrixDialog;
pub use chapter_marker_dialog::{ChapterEntry, ChapterMarkerDialog};

pub mod audio_compressor_dialog;
pub mod video_crop_dialog;
pub mod goto_frame_dialog;
pub mod playback_history_dialog;
pub mod media_tag_editor_dialog;
pub mod deinterlace_dialog;

pub use audio_compressor_dialog::{AudioCompressorDialog, CompressorPreset};
pub use video_crop_dialog::{CropPreset, VideoCropDialog};
pub use goto_frame_dialog::GotoFrameDialog;
pub use playback_history_dialog::{HistoryEntry, PlaybackHistoryDialog};
pub use media_tag_editor_dialog::MediaTagEditorDialog;
pub use deinterlace_dialog::{DeinterlaceDialog, DeinterlaceMode};

pub mod ab_repeat_dialog;
pub mod boss_key_dialog;
pub mod motion_interpolation_dialog;
pub mod color_lut_dialog;
pub mod subtitle_translator_dialog;

pub use ab_repeat_dialog::AbRepeatDialog;
pub use boss_key_dialog::{BossKeyAction, BossKeyDialog};
pub use motion_interpolation_dialog::{InterpolationAlgorithm, MotionInterpolationDialog};
pub use color_lut_dialog::{ColorLutDialog, LutPreset};
pub use subtitle_translator_dialog::{SubtitleTranslatorDialog, VocabWord};

pub mod ai_subtitle_dialog;
pub mod ai_upscaling_dialog;
pub mod ai_audio_dialog;
pub mod loudness_radar_dialog;
pub mod frame_dumper_dialog;

pub use ai_subtitle_dialog::{AiSubtitleDialog, AiTranslateEngine, SubtitleSegment, WhisperModelSize};
pub use ai_upscaling_dialog::{AiUpscaleMethod, AiUpscalingDialog};
pub use ai_audio_dialog::{AiAudioDialog, AiAudioFilterMode};
pub use loudness_radar_dialog::{LoudnessRadarDialog, LoudnessStandard};
pub use frame_dumper_dialog::{FrameDumpInterval, FrameDumperDialog};

pub mod color_blindness_dialog;
pub mod cinema_matte_dialog;
pub mod motion_vector_inspector_dialog;
pub mod rich_bookmark_notes_dialog;
pub mod pitch_formant_dialog;

pub use color_blindness_dialog::{ColorBlindnessDialog, VisionDeficiencyMode};
pub use cinema_matte_dialog::{CinemaMatteDialog, CinemaMattePreset};
pub use motion_vector_inspector_dialog::{MotionVectorInspectorDialog, VectorVisualizationMode};
pub use rich_bookmark_notes_dialog::{RichBookmarkNotesDialog, RichNoteEntry};
pub use pitch_formant_dialog::{PitchFormantDialog, PitchKeyPreset};

pub mod sleep_timer_dialog;
pub use sleep_timer_dialog::SleepTimerDialog;

pub mod update_dialog;
pub use update_dialog::UpdateDialog;

pub mod surround_eq_dialog;
pub use surround_eq_dialog::{SurroundEqDialog, SurroundTab};

