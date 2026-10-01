pub mod associations;
pub mod gestures;
pub mod osd_config;

pub use gestures::{GestureBinding, GestureDirection, MouseGestureTracker};
pub use osd_config::{OsdConfig, OsdPosition};
pub mod bindings;
pub mod input_profiles;

pub use bindings::{GamepadTriggerButton, InputTrigger, MouseTriggerType, TriggerBinding};
pub use input_profiles::{InputProfileStore, InputProfileType, ProfileBindingSet};
pub mod hierarchy;
pub use hierarchy::{LayeredConfigState, ResolvedSetting, SettingSource};
pub mod commands;
pub mod keymap;
pub mod mouse;
pub mod per_file;

pub use commands::PlayerCommand;
pub use keymap::{KeyBinding, KeymapConfig};
pub use mouse::{MouseAction, MouseConfig};
pub use per_file::{PerItemMemoryStore, PerItemProfile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::engine::parametric_eq::ParametricEqConfig;
use crate::engine::shaders::ShaderConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    PotPlayerClassic, // Black & Gold
    OnyxDiamond,      // OLED Pitch Black & Emerald
    MidnightNavy,     // Deep Ocean Blue & Sapphire
    CyberpunkNeon,    // Dark Violet & Neon Magenta
    TitaniumSilver,   // Brushed Steel & Ice Blue
    EmeraldForest,    // Tactical Dark Green & Mint
    CrimsonRuby,      // Dark Burgundy & Ruby Flame
    NordicFrost,      // Minimal Dark Slate & Arctic White
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::PotPlayerClassic
    }
}

impl ThemeMode {
    pub const ALL: [ThemeMode; 8] = [
        ThemeMode::PotPlayerClassic,
        ThemeMode::OnyxDiamond,
        ThemeMode::MidnightNavy,
        ThemeMode::CyberpunkNeon,
        ThemeMode::TitaniumSilver,
        ThemeMode::EmeraldForest,
        ThemeMode::CrimsonRuby,
        ThemeMode::NordicFrost,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::PotPlayerClassic => "Vertex Classic (Black & Gold)",
            Self::OnyxDiamond => "Onyx Diamond (OLED Pure Black)",
            Self::MidnightNavy => "Midnight Sapphire (Deep Blue)",
            Self::CyberpunkNeon => "Cyberpunk Neon (Violet & Rose)",
            Self::TitaniumSilver => "Titanium Metal (Brushed Silver)",
            Self::EmeraldForest => "Emerald Forest (Tactical Green)",
            Self::CrimsonRuby => "Crimson Ruby (Dark Red)",
            Self::NordicFrost => "Nordic Frost (Clean Minimal Slate)",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::PotPlayerClassic => Self::OnyxDiamond,
            Self::OnyxDiamond => Self::MidnightNavy,
            Self::MidnightNavy => Self::CyberpunkNeon,
            Self::CyberpunkNeon => Self::TitaniumSilver,
            Self::TitaniumSilver => Self::EmeraldForest,
            Self::EmeraldForest => Self::CrimsonRuby,
            Self::CrimsonRuby => Self::NordicFrost,
            Self::NordicFrost => Self::PotPlayerClassic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoubleClickAction {
    ToggleFullscreen,
    PlayPause,
    MaximizeRestore,
}

impl Default for DoubleClickAction {
    fn default() -> Self {
        DoubleClickAction::PlayPause
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaylistEndAction {
    Stop,
    StopPlayback,
    RepeatPlaylist,
    ShowHomeScreen,
    CloseApplication,
    ClosePlayer,
    SleepPC,
    ShutdownPC,
}

impl Default for PlaylistEndAction {
    fn default() -> Self {
        PlaylistEndAction::ShowHomeScreen
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerFileConfig {
    pub audio_track_id: Option<i64>,
    pub subtitle_track_id: Option<i64>,
    pub audio_delay: Option<f64>,
    pub subtitle_delay: Option<f64>,
    pub aspect_ratio: Option<String>,
    pub resume_pos: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub theme_mode: ThemeMode,
    pub volume: f64,
    pub is_muted: bool,
    pub volume_boost_limit: f64,
    pub playback_speed: f64,
    pub auto_resume: bool,
    pub auto_load_next_episode: bool,
    pub remember_window_size: bool,
    pub always_on_top: bool,
    pub window_width: f32,
    pub window_height: f32,
    pub show_playlist: bool,
    pub show_control_panel: bool,
    pub double_click_action: DoubleClickAction,
    pub seek_step_short: f64,
    pub seek_step_medium: f64,
    pub seek_step_long: f64,
    pub seek_step_huge: f64,
    pub adaptive_seek: bool,
    pub playlist_end_action: PlaylistEndAction,
    pub subtitle_font_size: f32,
    pub subtitle_vertical_pos: f32,
    pub subtitle_color: String,
    pub subtitle_outline_color: String,
    pub subtitle_outline_width: f32,
    pub eq_enabled: bool,
    pub eq_preset: String,
    pub eq_bands: Vec<f64>,
    pub custom_eq_presets: std::collections::HashMap<String, Vec<f64>>,
    pub hardware_decoding: String,
    pub aspect_ratio: String,
    pub video_align_y: String,
    pub video_brightness: f64,
    pub video_contrast: f64,
    pub video_saturation: f64,
    pub video_hue: f64,
    pub video_gamma: f64,
    pub video_deinterlace: bool,
    pub video_sharpen: f64,
    pub video_rotation: i32,
    pub video_flip_horizontal: bool,
    pub video_flip_vertical: bool,
    pub video_deband: bool,
    pub hdr_tone_mapping: String,
    pub hdr_compute_peak: bool,
    pub hdr_target_colorspace_hint: bool,
    pub dither_depth: String,
    pub gamut_mapping_mode: String,
    pub resume_timestamps: HashMap<String, f64>,
    pub recent_files: Vec<String>,
    pub last_played_file: Option<String>,
    pub last_playlist: Vec<String>,
    pub last_playlist_index: Option<usize>,
    pub audio_normalize: bool,
    pub vocal_remover: bool,
    pub voice_enhance: bool,
    pub stereo_widen: f64,
    pub audio_delay: f64,
    pub show_recent_on_idle: bool,
    pub wasapi_exclusive: bool,
    pub audio_device: String,
    pub audio_channels: String,
    pub skip_intro_sec: f64,
    pub skip_outro_sec: f64,
    pub skip_intro_enabled: bool,
    pub secondary_subtitle_track: i64,

    // Advanced PotPlayer Power Additions
    pub lut_file: Option<String>,
    pub crop_left: u32,
    pub crop_right: u32,
    pub crop_top: u32,
    pub crop_bottom: u32,
    pub auto_crop_black_bars: bool,
    pub pitch_semitones: f64,
    pub resample_rate: u32,
    pub audio_bit_depth: String,
    pub resampler_quality: String,
    pub replaygain_mode: String,
    pub replaygain_preamp: f64,
    pub show_seekbar_thumbnail: bool,

    pub per_file_settings: HashMap<String, PerFileConfig>,
    pub parametric_eq: ParametricEqConfig,
    pub shaders: ShaderConfig,
    pub display_sync_enabled: bool,
    pub motion_interpolation: bool,
    pub interpolation_mode: String,
    pub library_folders: Vec<String>,
    pub playlist_detached: bool,
    pub playlist_pinned: bool,
    pub playlist_repeat_mode: String,
    pub playlist_shuffle: bool,
    #[serde(default)]
    pub restore_last_playlist: bool,

    // Frame Pacing & GPU Power Settings
    pub fps_interaction: u32,
    pub fps_playback: u32,
    pub fps_background: u32,
    pub zero_gpu_minimized: bool,

    // Advanced PotPlayer Power Settings & Engine Overrides
    pub custom_mpv_options: String,
    pub cache_demuxer_sec: f64,
    pub cache_demuxer_mb: u32,
    pub video_sync_mode: String,
    pub framedrop_mode: String,
    pub subtitle_sub_font: String,
    pub subtitle_render_to_video: bool,
    pub osd_duration_ms: u32,
    pub osd_font_size: f32,
    pub mouse_middle_click_action: String,
    pub mouse_wheel_action: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::PotPlayerClassic,
            fps_interaction: 60,
            fps_playback: 30,
            fps_background: 20,
            zero_gpu_minimized: true,
            volume: 80.0,
            is_muted: false,
            volume_boost_limit: 100.0,
            playback_speed: 1.0,
            auto_resume: true,
            auto_load_next_episode: true,
            remember_window_size: true,
            always_on_top: false,
            window_width: 1100.0,
            window_height: 680.0,
            show_playlist: false,
            show_control_panel: false,
            double_click_action: DoubleClickAction::PlayPause,
            seek_step_short: 5.0,
            seek_step_medium: 30.0,
            seek_step_long: 60.0,
            seek_step_huge: 300.0,
            adaptive_seek: false,
            playlist_end_action: PlaylistEndAction::ShowHomeScreen,
            subtitle_font_size: 28.0,
            subtitle_vertical_pos: 92.0,
            subtitle_color: "#FFFFFF".to_string(),
            subtitle_outline_color: "#000000".to_string(),
            subtitle_outline_width: 2.0,
            eq_enabled: false,
            eq_preset: "Flat".to_string(),
            eq_bands: vec![0.0; 18],
            custom_eq_presets: std::collections::HashMap::new(),
            hardware_decoding: "auto-safe".to_string(),
            aspect_ratio: "auto".to_string(),
            video_align_y: "center".to_string(),
            video_brightness: 100.0,
            video_contrast: 100.0,
            video_saturation: 100.0,
            video_hue: 0.0,
            video_gamma: 100.0,
            video_deinterlace: false,
            video_sharpen: 0.0,
            video_rotation: 0,
            video_flip_horizontal: false,
            video_flip_vertical: false,
            video_deband: false,
            hdr_tone_mapping: "auto".to_string(),
            hdr_compute_peak: true,
            hdr_target_colorspace_hint: true,
            dither_depth: "auto".to_string(),
            gamut_mapping_mode: "auto".to_string(),
            resume_timestamps: HashMap::new(),
            recent_files: Vec::new(),
            last_played_file: None,
            last_playlist: Vec::new(),
            last_playlist_index: None,
            audio_normalize: false,
            vocal_remover: false,
            voice_enhance: false,
            stereo_widen: 0.0,
            audio_delay: 0.0,
            show_recent_on_idle: true,
            wasapi_exclusive: false,
            audio_channels: "auto".to_string(),
            audio_device: "auto".to_string(),
            skip_intro_sec: 0.0,
            skip_outro_sec: 0.0,
            skip_intro_enabled: false,
            secondary_subtitle_track: 0,

            lut_file: None,
            crop_left: 0,
            crop_right: 0,
            crop_top: 0,
            crop_bottom: 0,
            auto_crop_black_bars: false,
            pitch_semitones: 0.0,
            resample_rate: 0,
            audio_bit_depth: "auto".to_string(),
            resampler_quality: "high".to_string(),
            replaygain_mode: "no".to_string(),
            replaygain_preamp: 0.0,
            show_seekbar_thumbnail: true,

            per_file_settings: HashMap::new(),
            parametric_eq: ParametricEqConfig::default(),
            shaders: ShaderConfig::default(),
            display_sync_enabled: false,
            motion_interpolation: false,
            interpolation_mode: "oversample".to_string(),
            library_folders: Vec::new(),
            playlist_detached: false,
            playlist_pinned: false,
            playlist_repeat_mode: "Off".to_string(),
            playlist_shuffle: false,
            restore_last_playlist: false,

            custom_mpv_options: String::new(),
            cache_demuxer_sec: 30.0,
            cache_demuxer_mb: 150,
            video_sync_mode: "audio".to_string(),
            framedrop_mode: "vo".to_string(),
            subtitle_sub_font: "Segoe UI".to_string(),
            subtitle_render_to_video: false,
            osd_duration_ms: 1500,
            osd_font_size: 20.0,
            mouse_middle_click_action: "Mute".to_string(),
            mouse_wheel_action: "Volume".to_string(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let dir = base.join("vortex-player");
        let _ = fs::create_dir_all(&dir);
        dir.join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        let fallback_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("VertexPlayer")
            .join("config.json");

        let active_path = if path.exists() {
            path
        } else if fallback_path.exists() {
            fallback_path
        } else {
            path
        };

        if active_path.exists() {
            if let Ok(data) = fs::read_to_string(&active_path) {
                if let Ok(mut cfg) = serde_json::from_str::<AppConfig>(&data) {
                    if cfg.eq_bands.len() != 18 {
                        cfg.eq_bands = vec![0.0; 18];
                    }
                    // Upgrade legacy 0.0 default color values to 100.0% PotPlayer neutral baseline
                    if cfg.video_brightness == 0.0 && cfg.video_contrast == 0.0 && cfg.video_saturation == 0.0 {
                        cfg.video_brightness = 100.0;
                        cfg.video_contrast = 100.0;
                        cfg.video_saturation = 100.0;
                    }
                    if cfg.video_gamma == 0.0 {
                        cfg.video_gamma = 100.0;
                    }
                    return cfg;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn add_recent_file(&mut self, path: &str) {
        self.recent_files.retain(|p| p != path);
        self.recent_files.insert(0, path.to_string());
        if self.recent_files.len() > 30 {
            self.recent_files.truncate(30);
        }
        let _ = self.save();
    }

    pub fn save_resume_time(&mut self, path: &str, time: f64) {
        let win_key = path.replace('/', "\\");
        let unix_key = path.replace('\\', "/");
        if time <= 2.0 {
            self.resume_timestamps.remove(&win_key);
            self.resume_timestamps.remove(&unix_key);
            self.resume_timestamps.remove(path);
        } else {
            self.resume_timestamps.insert(path.to_string(), time);
            self.resume_timestamps.insert(unix_key, time);
            self.resume_timestamps.insert(win_key, time);
        }
        let _ = self.save();
    }

    pub fn get_resume_time(&self, path: &str) -> Option<f64> {
        if !self.auto_resume {
            return None;
        }
        let win_key = path.replace('/', "\\");
        let unix_key = path.replace('\\', "/");
        self.resume_timestamps
            .get(path)
            .copied()
            .or_else(|| self.resume_timestamps.get(&unix_key).copied())
            .or_else(|| self.resume_timestamps.get(&win_key).copied())
    }

    pub fn save_per_file(&mut self, path: &str, pfc: PerFileConfig) {
        self.per_file_settings.insert(path.to_string(), pfc);
        let _ = self.save();
    }

    pub fn get_per_file(&self, path: &str) -> Option<&PerFileConfig> {
        self.per_file_settings.get(path)
    }
}
