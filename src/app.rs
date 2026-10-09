//! app.rs — VertexPlayer Main Application Struct & UI Orchestration.

use crate::bookmark::{format_time, BookmarkManager};
use crate::config::{AppConfig, DoubleClickAction, PlaylistEndAction};
use crate::engine::{MediaStats, Player};
use crate::playlist::scanner::{self, is_media_file, is_subtitle_file};
use crate::playlist::Playlist;
use crate::subtitles::SamiParser;
use crate::ui::*;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Margin, Pos2, Rect, RichText, Sense, Stroke, Vec2,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub struct VortexApp {
    smtc: crate::engine::SmtcEngine,
    shader_studio: ShaderStudioDialog,
    broadcast_dialog: BroadcastDialog,
    is_motion_interpolated: bool,

    taskbar: TaskbarIntegration,
    is_vr_360: bool,
    is_seamless_stitch: bool,
    gif_maker: GifMakerDialog,
    device_capture: DeviceCaptureDialog,
    subtitle_lookup: SubtitleLookupDialog,
    is_passthrough: bool,

    record_dialog: StreamRecordDialog,
    contact_sheet_dialog: ContactSheetDialog,
    current_3d_mode: String,
    secondary_sub_track: i64,
    vr_yaw: f64,
    vr_pitch: f64,
    vr_zoom: f64,

    pub stereo_3d_dialog: Stereo3dDialog,
    pub karaoke_studio: KaraokeStudioDialog,
    pub subtitle_studio: SubtitleStudioDialog,
    pub screen_capture: ScreenCaptureDialog,
    pub iptv_guide: IptvGuideDialog,
    pub logo_watermark: LogoWatermarkDialog,
    pub disc_nav: DiscNavigationDialog,
    pub cd_ripper: CdRipperDialog,
    pub bda_tuner: BdaTunerDialog,
    pub bdj_dialog: BdjDialog,
    pub vst_winamp: VstWinampDialog,
    pub detached_windows: DetachedWindowsController,
    pub discord_rpc: DiscordRpcDialog,
    pub web_remote: WebRemoteDialog,
    pub scrobbler_dialog: ScrobblerDialog,
    pub media_server: MediaServerDialog,
    pub ambilight_dialog: AmbilightDialog,
    pub cast_renderer: CastRendererDialog,
    pub transcoder_dialog: TranscoderDialog,
    pub zoom_magnifier: ZoomMagnifierDialog,
    pub radio_directory: RadioDirectoryDialog,
    pub video_puzzle: VideoPuzzleDialog,
    pub vr_360_studio: Vr360StudioDialog,
    pub damaged_file_repair: DamagedFileRepairDialog,
    pub binaural_crossfeed: BinauralCrossfeedDialog,
    pub video_wall_matrix: VideoWallMatrixDialog,
    pub chapter_marker_dialog: ChapterMarkerDialog,
    pub audio_compressor: AudioCompressorDialog,
    pub video_crop: VideoCropDialog,
    pub goto_frame: GotoFrameDialog,
    pub playback_history: PlaybackHistoryDialog,
    pub media_tag_editor: MediaTagEditorDialog,
    pub deinterlace_dialog: DeinterlaceDialog,
    pub ab_repeat_dialog: AbRepeatDialog,
    pub boss_key_dialog: BossKeyDialog,
    pub motion_interpolation: MotionInterpolationDialog,
    pub color_lut_dialog: ColorLutDialog,
    pub subtitle_translator: SubtitleTranslatorDialog,
    pub ai_subtitle: AiSubtitleDialog,
    pub ai_upscaling: AiUpscalingDialog,
    pub ai_audio: AiAudioDialog,
    pub loudness_radar: LoudnessRadarDialog,
    pub frame_dumper: FrameDumperDialog,
    pub color_blindness: ColorBlindnessDialog,
    pub cinema_matte: CinemaMatteDialog,
    pub motion_vector_inspector: MotionVectorInspectorDialog,
    pub rich_bookmark_notes: RichBookmarkNotesDialog,
    pub pitch_formant: PitchFormantDialog,

    player: Result<Arc<Player>, String>,








    pub playlist: Arc<Mutex<Playlist>>,

    config: AppConfig,
    bookmark_mgr: BookmarkManager,
    osd: OsdEngine,
    toast: ToastManager,
    control_panel: ControlPanel,
    preferences_dialog: PreferencesDialog,
    peq_dialog: ParametricEqDialog,
    library_view: LibraryView,
    library: crate::library::MediaLibrary,
    display_sync: crate::engine::DisplaySyncManager,
    ipc_server: crate::engine::IpcServer,
    capture_dialog: CaptureDialog,
    capture_config: crate::capture::CaptureConfig,
    burst_engine: crate::capture::BurstCaptureEngine,
    visualizer: VisualizerSuite,
    lyrics_overlay: LyricsOverlay,
    mediainfo_dialog: MediaInfoDialog,
    show_mediainfo_dialog: bool,
    network_browser: NetworkBrowserDialog,
    split_compare: SplitCompareView,
    gamepad: crate::engine::GamepadEngine,
    bookmark_studio: BookmarkStudioDialog,
    subtitle_explorer: SubtitleExplorerDialog,
    settings_inspector: SettingsInspectorDialog,
    input_editor: InputEditorDialog,
    input_profiles: crate::config::InputProfileStore,
    gesture_tracker: crate::config::MouseGestureTracker,
    osd_config: crate::config::OsdConfig,
    file_navigator: FileNavigatorSidebar,
    video_transform: crate::engine::VideoTransformState,
    subtitle_controller: crate::subtitles::SubtitleController,
    show_telemetry_hud: bool,
    play_next_queue: crate::playlist::PlayNextQueue,
    sleep_timer: crate::engine::SleepTimerEngine,
    pip_mode: PipModeView,
    music_view: MusicBackgroundView,
    crash_sentinel: crate::engine::CrashRecoverySentinel,
    event_bus: crate::engine::ScriptEventBus,
    stream_url_dialog: StreamUrlDialog,
    channel_matrix_dialog: ChannelMatrixDialog,
    channel_matrix_config: ChannelMatrixConfig,
    chain_editor: ChainEditorDialog,
    video_chain: crate::engine::VideoFilterChain,
    audio_chain: crate::engine::AudioDspChain,
    track_priority: crate::engine::TrackPriorityConfig,
    auto_skip_dialog: AutoSkipDialog,
    show_playlist: bool,
    show_control_panel: bool,
    show_preferences: bool,
    show_peq_dialog: bool,
    show_jump_time_dialog: bool,
    jump_time_dialog: JumpTimeDialog,
    pub show_sleep_timer_dialog: bool,
    pub sleep_timer_dialog: SleepTimerDialog,
    pub show_update_dialog: bool,
    pub update_dialog: UpdateDialog,
    pub show_surround_eq_dialog: bool,
    pub surround_eq_dialog: SurroundEqDialog,
    show_library_view: bool,
    show_bookmark_overlay: bool,
    show_capture_dialog: bool,
    show_stream_url_dialog: bool,
    show_main_menu: bool,
    show_audio_channels_popup: bool,
    audio_channels_popup_pos: Option<Pos2>,
    show_speed_popup: bool,
    speed_popup_pos: Option<Pos2>,
    menu_anchor_pos: Option<(i32, i32)>,
    menu_position: Option<Pos2>,
    show_remaining_time: bool,
    show_about_dialog: bool,
    is_fullscreen: bool,
    pre_fullscreen_rect: Option<egui::Rect>,
    #[cfg(windows)]
    pre_fullscreen_win32_rect: Option<windows_sys::Win32::Foundation::RECT>,
    pre_fullscreen_maximized: bool,
    last_windowed_rect: Option<egui::Rect>,
    restore_frames_pending: u8,
    attached_hwnd: bool,
    parent_hwnd: isize,
    child_hwnd: isize,
    last_saved_time_pos: f64,
    prev_was_playing: bool,
    pending_resume: Option<(String, f64)>,
    pending_audio_track: Option<i64>,
    pending_subtitle_track: Option<i64>,
    last_mouse_activity: std::time::Instant,
    last_maximize_time: std::time::Instant,
    last_fullscreen_change: std::time::Instant,
    last_mouse_pos: Option<Pos2>,
    is_top_hovered: bool,
    is_bottom_hovered: bool,
    pub pending_initial_file: Option<PathBuf>,
    pub global_hotkeys: crate::platform::GlobalHotkeyManager,
    folder_scan_receiver: Option<std::sync::mpsc::Receiver<Vec<PathBuf>>>,
    neighbor_scan_receiver: Option<std::sync::mpsc::Receiver<Vec<PathBuf>>>,
    last_video_rect: Rect,
    active_popup_rects: Vec<Rect>,
    menu_opened_frame: u64,
    frame_counter: u64,
    last_viewport_size: Option<Vec2>,
    is_resizing_window: bool,
    seek_preview: crate::ui::SeekPreviewEngine,
    queue_repaint_active: Arc<AtomicBool>,
    pub drawer_tab: crate::ui::playlist_panel::DrawerTab,
    pub drawer_browser_dir: PathBuf,
    pub drawer_browser_search: String,
    pub drawer_sub_search: String,
    intro_segment_key: Option<String>,
    intro_btn_shown_at: Option<std::time::Instant>,
    intro_last_time_pos: f64,
    last_skipped_chapter_index: Option<i64>,
    has_initial_centered: bool,
}



#[cfg(windows)]
static PARENT_HWND: std::sync::atomic::AtomicIsize = std::sync::atomic::AtomicIsize::new(0);

/// Get the window's local client rectangle in logical points, guaranteed to start at (0, 0)
fn get_window_client_rect(ctx: &egui::Context) -> Rect {
    ctx.input(|i| {
        if let Some(r) = i.raw.screen_rect {
            Rect::from_min_size(Pos2::ZERO, r.size())
        } else if let Some(r) = i.viewport().inner_rect {
            Rect::from_min_size(Pos2::ZERO, r.size())
        } else {
            Rect::from_min_size(Pos2::ZERO, Vec2::new(1920.0, 1080.0))
        }
    })
}

// Fullscreen controls are overlays, not layout panels. Keeping them out of
// egui's panel layout prevents revealing either edge bar from changing the
// video/music canvas size or aspect ratio.
fn show_top_control_surface(
    _ui: &mut egui::Ui,
    ctx: &egui::Context,
    _fullscreen: bool,
    screen_w: f32,
    _screen_h: f32,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    // Always use Foreground Area so panels paint ON TOP of the mpv video render
    // (mpv_render_context_render writes directly to fbo=0, overwriting normal Background layers)
    egui::Area::new(egui::Id::new("vortex_top_controls"))
        .fixed_pos(Pos2::ZERO)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            ui.allocate_ui(Vec2::new(screen_w, 32.0), |ui| {
                ui.set_width(screen_w);
                egui::Frame::new()
                    .fill(Color32::from_rgb(18, 18, 20))
                    .stroke(Stroke::NONE)
                    .corner_radius(CornerRadius::ZERO)
                    .inner_margin(Margin::ZERO)
                    .show(ui, |ui| {
                        ui.set_width(screen_w);
                        add_contents(ui);
                    });
            });
        });
}

fn show_bottom_control_surface(
    _ui: &mut egui::Ui,
    ctx: &egui::Context,
    _fullscreen: bool,
    screen_w: f32,
    screen_h: f32,
    bar_height: f32,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    // Always use Foreground Area so panels paint ON TOP of the mpv video render
    let top_y = (screen_h - bar_height).max(0.0);
    egui::Area::new(egui::Id::new("vortex_bottom_controls"))
        .fixed_pos(Pos2::new(0.0, top_y))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            ui.allocate_ui(Vec2::new(screen_w, bar_height), |ui| {
                ui.set_width(screen_w);
                egui::Frame::new()
                    .fill(Color32::from_rgb(20, 21, 26))
                    .stroke(Stroke::NONE)
                    .inner_margin(Margin::ZERO)
                    .show(ui, |ui| {
                        ui.set_width(screen_w);
                        add_contents(ui);
                    });
            });
        });
}

impl VortexApp {
    pub fn new(cc: &eframe::CreationContext<'_>, initial_file: Option<PathBuf>) -> Self {
        crate::log_step("6a. entering VortexApp::new");
        let config = AppConfig::load();
        VortexTheme::apply(&cc.egui_ctx, config.theme_mode);
        crate::platform::set_window_corner_style(&config.window_corner_style);

        let playlist = Arc::new(Mutex::new(Playlist::new()));
        let bookmark_mgr = BookmarkManager::new();
        let player = match Player::new().map(Arc::new) {
            Ok(p) => {
                crate::log_step("6b. Player::new succeeded");
                p.set_volume(config.volume);
                p.set_audio_device(&config.audio_device);
                p.set_hwdec(&config.hardware_decoding);
                p.set_audio_channels(&config.audio_channels);
                p.set_audio_normalize(config.audio_normalize);
                #[cfg(windows)]
                p.set_wasapi_exclusive(config.wasapi_exclusive);
                p.set_aspect_ratio(&config.aspect_ratio);
                p.set_video_align_y(&config.video_align_y);
                p.apply_all_subtitle_settings(&config);
                p.set_hdr_tone_mapping(&config.hdr_tone_mapping);
                p.set_hdr_colorspace_hint(config.hdr_target_colorspace_hint);
                p.set_gamut_mapping(&config.gamut_mapping_mode);
                p.set_speed(config.playback_speed);
                p.set_equalizer(config.eq_enabled, &config.eq_bands);
                let _ = p.ensure_render_context(&cc.egui_ctx);
                Ok(p)
            }
            Err(e) => {
                let err_msg = format!(
                    "VortexPlayer could not initialize the MPV media playback engine.\n\nError: {}\n\nPlease ensure libmpv-2.dll is present in the application folder.",
                    e
                );
                crate::show_error_box("VortexPlayer - Playback Engine Error", &err_msg);
                eprintln!("⚠️ VortexPlayer Player initialization error: {}", e);
                Err(e)
            }
        };

        let track_priority = crate::engine::TrackPriorityConfig::from_config(&config);

        let mut app = Self {
            smtc: crate::engine::SmtcEngine::new(),
            shader_studio: ShaderStudioDialog::new(),
            broadcast_dialog: BroadcastDialog::new(),
            is_motion_interpolated: false,

            taskbar: TaskbarIntegration::new(),
            is_vr_360: false,
            is_seamless_stitch: true,
            gif_maker: GifMakerDialog::new(),
            device_capture: DeviceCaptureDialog::new(),
            subtitle_lookup: SubtitleLookupDialog::new(),
            is_passthrough: false,

            record_dialog: StreamRecordDialog::new(),
            contact_sheet_dialog: ContactSheetDialog::new(),
            current_3d_mode: "off".to_string(),
            secondary_sub_track: 0,
            vr_yaw: 0.0,
            vr_pitch: 0.0,
            vr_zoom: 0.0,

            stereo_3d_dialog: Stereo3dDialog::new(),
            karaoke_studio: KaraokeStudioDialog::new(),
            subtitle_studio: SubtitleStudioDialog::new(),
            screen_capture: ScreenCaptureDialog::new(),
            iptv_guide: IptvGuideDialog::new(),
            logo_watermark: LogoWatermarkDialog::new(),
            disc_nav: DiscNavigationDialog::new(),
            cd_ripper: CdRipperDialog::new(),
            bda_tuner: BdaTunerDialog::new(),
            bdj_dialog: BdjDialog::new(),
            vst_winamp: VstWinampDialog::new(),
            detached_windows: DetachedWindowsController::new(),
            discord_rpc: DiscordRpcDialog::new(),
            web_remote: WebRemoteDialog::new(),
            scrobbler_dialog: ScrobblerDialog::new(),
            media_server: MediaServerDialog::new(),
            ambilight_dialog: AmbilightDialog::new(),
            cast_renderer: CastRendererDialog::new(),
            transcoder_dialog: TranscoderDialog::new(),
            zoom_magnifier: ZoomMagnifierDialog::new(),
            radio_directory: RadioDirectoryDialog::new(),
            video_puzzle: VideoPuzzleDialog::new(),
            vr_360_studio: Vr360StudioDialog::new(),
            damaged_file_repair: DamagedFileRepairDialog::new(),
            binaural_crossfeed: BinauralCrossfeedDialog::new(),
            video_wall_matrix: VideoWallMatrixDialog::new(),
            chapter_marker_dialog: ChapterMarkerDialog::new(),
            audio_compressor: AudioCompressorDialog::new(),
            video_crop: VideoCropDialog::new(),
            goto_frame: GotoFrameDialog::new(),
            playback_history: PlaybackHistoryDialog::new(),
            media_tag_editor: MediaTagEditorDialog::new(),
            deinterlace_dialog: DeinterlaceDialog::new(),
            ab_repeat_dialog: AbRepeatDialog::new(),
            boss_key_dialog: BossKeyDialog::new(),
            motion_interpolation: MotionInterpolationDialog::new(),
            color_lut_dialog: ColorLutDialog::new(),
            subtitle_translator: SubtitleTranslatorDialog::new(),
            ai_subtitle: AiSubtitleDialog::new(),
            ai_upscaling: AiUpscalingDialog::new(),
            ai_audio: AiAudioDialog::new(),
            loudness_radar: LoudnessRadarDialog::new(),
            frame_dumper: FrameDumperDialog::new(),
            color_blindness: ColorBlindnessDialog::new(),
            cinema_matte: CinemaMatteDialog::new(),
            motion_vector_inspector: MotionVectorInspectorDialog::new(),
            rich_bookmark_notes: RichBookmarkNotesDialog::new(),
            pitch_formant: PitchFormantDialog::new(),

            player,








            playlist,

            config,
            bookmark_mgr,
            osd: OsdEngine::new(),
            toast: ToastManager::new(),
            control_panel: ControlPanel::new(),
            preferences_dialog: PreferencesDialog::new(),
            peq_dialog: ParametricEqDialog::new(),
            library_view: LibraryView::new(),
            library: crate::library::MediaLibrary::new(),
            display_sync: crate::engine::DisplaySyncManager::new(),
            ipc_server: crate::engine::IpcServer::start(),
            show_peq_dialog: false,
            show_jump_time_dialog: false,
            jump_time_dialog: JumpTimeDialog::new(),
            show_sleep_timer_dialog: false,
            sleep_timer_dialog: SleepTimerDialog::new(),
            show_update_dialog: false,
            update_dialog: UpdateDialog::new(),
            show_surround_eq_dialog: false,
            surround_eq_dialog: SurroundEqDialog::new(),
            show_library_view: false,
            capture_dialog: CaptureDialog::new(),
            capture_config: crate::capture::CaptureConfig::default(),
            burst_engine: crate::capture::BurstCaptureEngine::new(),
            visualizer: VisualizerSuite::new(),
            lyrics_overlay: LyricsOverlay::new(),
            mediainfo_dialog: MediaInfoDialog::new(),
            show_mediainfo_dialog: false,
            network_browser: NetworkBrowserDialog::new(),
            split_compare: SplitCompareView::new(),
            gamepad: crate::engine::GamepadEngine::new(),
            bookmark_studio: BookmarkStudioDialog::new(),
            subtitle_explorer: SubtitleExplorerDialog::new(),
            settings_inspector: SettingsInspectorDialog::new(),
            input_editor: InputEditorDialog::new(),
            input_profiles: crate::config::InputProfileStore::default(),
            gesture_tracker: crate::config::MouseGestureTracker::default(),
            osd_config: crate::config::OsdConfig::default(),
            file_navigator: FileNavigatorSidebar::new(),
            video_transform: crate::engine::VideoTransformState::new(),
            subtitle_controller: crate::subtitles::SubtitleController::new(),
            show_telemetry_hud: false,
            play_next_queue: crate::playlist::PlayNextQueue::new(),
            sleep_timer: crate::engine::SleepTimerEngine::new(),
            pip_mode: PipModeView::new(),
            music_view: MusicBackgroundView::new(),
            crash_sentinel: crate::engine::CrashRecoverySentinel::new(),
            event_bus: crate::engine::ScriptEventBus::new(),
            stream_url_dialog: StreamUrlDialog::new(),
            channel_matrix_dialog: ChannelMatrixDialog::new(),
            channel_matrix_config: ChannelMatrixConfig::default(),
            chain_editor: ChainEditorDialog::new(),
            video_chain: crate::engine::VideoFilterChain::default(),
            audio_chain: crate::engine::AudioDspChain::default(),
            track_priority,
            auto_skip_dialog: AutoSkipDialog::new(),
            show_playlist: false,
            show_control_panel: false,
            show_preferences: false,
            show_bookmark_overlay: false,
            show_capture_dialog: false,
            show_stream_url_dialog: false,
            show_main_menu: false,
            show_audio_channels_popup: false,
            audio_channels_popup_pos: None,
            show_speed_popup: false,
            speed_popup_pos: None,
            menu_anchor_pos: None,
            menu_position: None,
            show_remaining_time: false,
            show_about_dialog: false,
            is_fullscreen: false,
            pre_fullscreen_rect: None,
            #[cfg(windows)]
            pre_fullscreen_win32_rect: None,
            pre_fullscreen_maximized: false,
            last_windowed_rect: None,
            restore_frames_pending: 0,
            attached_hwnd: false,
            parent_hwnd: 0,
            child_hwnd: 0,
            last_saved_time_pos: 0.0,
            prev_was_playing: false,
            pending_resume: None,
            pending_audio_track: None,
            pending_subtitle_track: None,
            last_video_rect: Rect::ZERO,
            active_popup_rects: Vec::new(),
            last_mouse_activity: std::time::Instant::now(),
            last_maximize_time: std::time::Instant::now() - std::time::Duration::from_secs(10),
            last_fullscreen_change: std::time::Instant::now() - std::time::Duration::from_secs(10),
            last_mouse_pos: None,
            is_top_hovered: false,
            is_bottom_hovered: false,
            pending_initial_file: None,
            global_hotkeys: crate::platform::GlobalHotkeyManager::new(),
            folder_scan_receiver: None,
            neighbor_scan_receiver: None,
            menu_opened_frame: 0,
            frame_counter: 0,
            last_viewport_size: None,
            is_resizing_window: false,
            seek_preview: crate::ui::SeekPreviewEngine::new(),
            drawer_tab: crate::ui::playlist_panel::DrawerTab::Playlist,
            drawer_browser_dir: dirs::video_dir().unwrap_or_else(|| PathBuf::from("C:\\")),
            drawer_browser_search: String::new(),
            drawer_sub_search: String::new(),
            intro_segment_key: None,
            intro_btn_shown_at: None,
            intro_last_time_pos: 0.0,
            last_skipped_chapter_index: None,
            has_initial_centered: false,
            queue_repaint_active: {
                let active = Arc::new(AtomicBool::new(false));
                let repaint_ctx = cc.egui_ctx.clone();
                let repaint_active = Arc::clone(&active);
                std::thread::spawn(move || {
                    loop {
                        if repaint_active.load(Ordering::Relaxed) {
                            repaint_ctx.request_repaint();
                            std::thread::sleep(std::time::Duration::from_millis(250));
                        } else {
                            std::thread::sleep(std::time::Duration::from_secs(1));
                        }
                    }
                });
                active
            },
        };

        crate::ui::theme::VortexTheme::apply(&cc.egui_ctx, app.config.theme_mode);

        // Synchronize recent_files list and playback_history store on startup
        if app.config.recent_files.is_empty() && !app.playback_history.entries.is_empty() {
            app.config.recent_files = app.playback_history.entries.iter().map(|e| e.file_path.clone()).collect();
            let _ = app.config.save();
        } else if !app.config.recent_files.is_empty() && app.playback_history.entries.is_empty() {
            for f in &app.config.recent_files {
                let rt = app.config.get_resume_time(f).unwrap_or(0.0);
                app.playback_history.add_entry(f, rt, 0.0);
            }
        }

        if let Some(init_f) = initial_file {
            // Opened with a specific file (CLI / "Open With" / file association):
            // NEVER load previous playlist, start clean with only the opened file.
            app.pending_initial_file = Some(init_f);
        } else if app.config.restore_last_playlist {
            // Direct launch without a file, and restore_last_playlist is enabled:
            // 1. Instant non-blocking batch restoration of previous playlist on startup
            let paths: Vec<std::path::PathBuf> = app.config.last_playlist.iter().map(std::path::PathBuf::from).collect();
            if !paths.is_empty() {
                let mut pl = app.playlist.lock().unwrap();
                pl.add_items(paths);
                pl.probe_all_durations();
            }

            // 2. Restore active playlist item index
            if let Some(idx) = app.config.last_playlist_index {
                let mut pl = app.playlist.lock().unwrap();
                if idx < pl.items.len() {
                    pl.current_index = Some(idx);
                }
            }

            // 3. Queue auto-resume of last played file if configured
            if app.config.auto_resume {
                if let Some(ref last_file) = app.config.last_played_file {
                    let last_path = std::path::PathBuf::from(last_file);
                    app.pending_initial_file = Some(last_path);
                }
            }
        }

        // Restore repeat mode and shuffle settings
        {
            let mut pl = app.playlist.lock().unwrap();
            let r_mode = match app.config.playlist_repeat_mode.as_str() {
                "RepeatAll" => crate::playlist::RepeatMode::RepeatAll,
                "RepeatTrack" => crate::playlist::RepeatMode::RepeatTrack,
                _ => crate::playlist::RepeatMode::Off,
            };
            pl.repeat_mode = r_mode;
            pl.set_shuffle(app.config.playlist_shuffle);
        }

        if let Ok(ref player) = app.player {
            player.set_video_brightness(app.config.video_brightness - 100.0);
            player.set_video_contrast(app.config.video_contrast - 100.0);
            player.set_video_saturation(app.config.video_saturation - 100.0);
            player.set_video_hue(app.config.video_hue);
            if app.config.eq_enabled {
                player.set_equalizer(true, &app.config.eq_bands);
            }
        }

        // 4. Autonomous Background Track Advancement (Even when window is minimized)
        if let Ok(ref player) = app.player {
            let bg_player = Arc::clone(player);
            let bg_pl = Arc::clone(&app.playlist);
            let bg_ctx = cc.egui_ctx.clone();

            player.set_on_eof(move || {
                let next_opt = {
                    if let Ok(mut pl) = bg_pl.lock() {
                        pl.next()
                    } else {
                        None
                    }
                };
                if let Some(next_path) = next_opt {
                    let path_str = next_path.to_string_lossy().to_string();
                    let is_audio = if let Some(ext) = next_path.extension().and_then(|e| e.to_str()) {
                        matches!(
                            ext.to_lowercase().as_str(),
                            "flac" | "mp3" | "wav" | "m4a" | "aac" | "ogg" | "opus" | "wma" | "alac" | "aiff" | "ape" | "ac3" | "dts"
                        )
                    } else {
                        false
                    };

                    if is_audio {
                        bg_player.set_property_string("vid", "no");
                        bg_player.set_property_string("audio-display", "no");
                        bg_player.set_property_string("video", "no");
                    } else {
                        bg_player.set_property_string("vid", "auto");
                        bg_player.set_property_string("audio-display", "no");
                        bg_player.set_property_string("video", "auto");
                        bg_player.set_property_string("vo", "libmpv");
                    }

                    bg_player.load_file(&path_str);
                    bg_player.play();
                    bg_ctx.request_repaint();
                }
            });
        }

        crate::log_step("6c. VortexApp::new returning app");
        app
    }


    #[inline]
    pub fn playlist_lock(&self) -> std::sync::MutexGuard<'_, crate::playlist::Playlist> {
        self.playlist.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn save_current_playback_state(&mut self) {
        self.config.last_playlist = self.playlist.lock().unwrap().items.iter().map(|i| i.path.to_string_lossy().to_string()).collect();
        self.config.last_playlist_index = self.playlist.lock().unwrap().current_index;
        self.config.playlist_repeat_mode = match self.playlist.lock().unwrap().repeat_mode {
            crate::playlist::RepeatMode::RepeatAll => "RepeatAll".to_string(),
            crate::playlist::RepeatMode::RepeatTrack => "RepeatTrack".to_string(),
            _ => "Off".to_string(),
        };
        self.config.playlist_shuffle = self.playlist.lock().unwrap().is_shuffle();
        if let Ok(ref player) = self.player {
            let stats = player.stats();
            if !stats.file_path.is_empty() {
                let save_pos = if stats.duration > 0.0 && stats.time_pos >= stats.duration - 8.0 {
                    0.0
                } else if stats.time_pos > 1.0 {
                    stats.time_pos
                } else {
                    0.0
                };
                self.config.save_resume_time(&stats.file_path, save_pos);
                self.playback_history.update_position(&stats.file_path, stats.time_pos, stats.duration);
                self.config.last_played_file = Some(stats.file_path.clone());

                let pfc = crate::config::PerFileConfig {
                    audio_track_id: if stats.selected_audio_track > 0 { Some(stats.selected_audio_track) } else { None },
                    subtitle_track_id: if stats.selected_subtitle_track > 0 { Some(stats.selected_subtitle_track) } else { None },
                    audio_delay: Some(stats.audio_delay),
                    subtitle_delay: Some(stats.subtitle_delay),
                    aspect_ratio: Some(stats.aspect_ratio.clone()),
                    resume_pos: Some(save_pos),
                };
                self.config.save_per_file(&stats.file_path, pfc);
            }
        }
        let _ = self.config.save();
    }

    pub fn open_media_file(&mut self, path: PathBuf) {
        crate::log_step(&format!("20. open_media_file entered for {:?}", path));
        self.save_current_playback_state();
        crate::log_step("21. save_current_playback_state done");
        if let Ok(ref player) = self.player {
            let path_str = path.to_string_lossy().to_string();
            
            let is_audio = if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                matches!(
                    ext.to_lowercase().as_str(),
                    "flac" | "mp3" | "wav" | "m4a" | "aac" | "ogg" | "opus" | "wma" | "alac" | "aiff" | "ape" | "ac3" | "dts"
                )
            } else {
                false
            };

            // Audio songs always start from 0:00 (videos only auto-resume if > 5.0s)
            let resume_pos = if self.config.auto_resume && !is_audio {
                self.config.get_resume_time(&path_str).filter(|&t| t > 5.0)
            } else {
                None
            };


            if is_audio {
                player.set_property_string("vid", "no");
                player.set_property_string("audio-display", "no");
                player.set_property_string("video", "no");
            } else {
                player.set_property_string("vid", "auto");
                player.set_property_string("audio-display", "no");
                player.set_property_string("video", "auto");
                player.set_property_string("vo", "libmpv");
            }

            player.set_property_string("alang", &self.config.audio_languages);
            player.set_property_string("slang", &self.config.subtitle_languages);

            if let Some(pos) = resume_pos {
                player.set_property_string("start", &format!("{:.2}", pos));
                player.load_file(&path_str);
                player.set_property_string("start", "none");
                player.play();
                self.pending_resume = Some((path_str.clone(), pos));
                let msg = format!("Resumed at {}", format_time(pos));
                self.osd.show(msg, 2000);
            } else {
                crate::log_step(&format!("23. calling player.load_file for {}", path_str));
                player.load_file(&path_str);
                crate::log_step("24. calling player.play");
                player.play();
                self.pending_resume = None;
            }
            crate::log_step("25. load_file and play returned");
            self.last_saved_time_pos = resume_pos.unwrap_or(0.0);


            let idx = self.playlist.lock().unwrap().add_item(path.clone());
            self.playlist.lock().unwrap().current_index = Some(idx);
            self.prev_was_playing = true;
            self.config.last_played_file = Some(path_str.clone());
            self.config.last_playlist = self.playlist.lock().unwrap().items.iter().map(|i| i.path.to_string_lossy().to_string()).collect();
            self.config.add_recent_file(&path_str);
            self.playback_history.add_entry(&path_str, resume_pos.unwrap_or(0.0), 0.0);

            // Apply configured audio device, channel layout & normalization
            player.set_audio_device(&self.config.audio_device);
            player.set_audio_channels(&self.config.audio_channels);
            player.set_audio_normalize(self.config.audio_normalize);
            player.set_aspect_ratio(&self.config.aspect_ratio);
            let align = if self.is_fullscreen { "center" } else { &self.config.video_align_y };
            player.set_video_align_y(align);
            player.set_video_brightness(self.config.video_brightness - 100.0);
            player.set_video_contrast(self.config.video_contrast - 100.0);
            player.set_video_saturation(self.config.video_saturation - 100.0);
            player.set_video_hue(self.config.video_hue);
            if self.config.eq_enabled {
                player.set_equalizer(true, &self.config.eq_bands);
            }
            player.apply_all_subtitle_settings(&self.config);

            // Restore last used soundtrack (audio track) and subtitle track for this file
            if let Some(pfc) = self.config.get_per_file(&path_str).cloned() {
                if let Some(aid) = pfc.audio_track_id {
                    player.set_audio_track(aid);
                    self.pending_audio_track = Some(aid);
                }
                if let Some(sid) = pfc.subtitle_track_id {
                    player.set_subtitle_track(sid);
                    self.pending_subtitle_track = Some(sid);
                }
                if let Some(adelay) = pfc.audio_delay {
                    player.set_property_string("audio-delay", &adelay.to_string());
                }
                if let Some(sdelay) = pfc.subtitle_delay {
                    player.set_property_string("sub-delay", &sdelay.to_string());
                }
            } else {
                // Apply intelligent Track Priority engine rules
                self.track_priority = crate::engine::TrackPriorityConfig::from_config(&self.config);
                let stats = player.stats();
                if let Some(best_aid) = self.track_priority.select_best_audio_track(&stats.audio_tracks) {
                    player.set_audio_track(best_aid);
                    self.pending_audio_track = Some(best_aid);
                }
                if let Some(best_sid) = self.track_priority.select_best_subtitle_track(&stats.subtitle_tracks) {
                    player.set_subtitle_track(best_sid);
                    self.pending_subtitle_track = Some(best_sid);
                }
            }

            let _ = self.config.save();

            let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            self.osd.show(format!("Playing: {}", filename), 2000);

            if self.config.auto_load_next_episode && self.playlist.lock().unwrap().len() <= 1 {
                let scan_path = path.clone();
                let (tx, rx) = std::sync::mpsc::channel();
                self.neighbor_scan_receiver = Some(rx);
                std::thread::spawn(move || {
                    let folder_files = scanner::scan_neighboring_episodes(&scan_path);
                    let _ = tx.send(folder_files);
                });
            }


            self.bookmark_mgr.set_video(Some(&path));

            let subs = scanner::find_associated_subtitles(&path);
            for s in subs {
                if s.to_string_lossy().ends_with(".smi") {
                    if let Ok(srt) = SamiParser::load_sami_file_as_srt(&s) {
                        let temp_srt = std::env::temp_dir().join("vortex_auto_smi.srt");
                        let _ = std::fs::write(&temp_srt, srt);
                        player.load_external_subtitle(&temp_srt.to_string_lossy());
                    }
                } else {
                    player.load_external_subtitle(&s.to_string_lossy());
                }
            }
        }
    }

    pub fn open_folder(&mut self, folder: PathBuf) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.folder_scan_receiver = Some(rx);
        self.osd.show("Scanning folder...".to_string(), 1500);

        std::thread::spawn(move || {
            let media_files = scanner::scan_folder_recursive(&folder);
            let _ = tx.send(media_files);
        });
    }

    pub fn handle_shortcuts(&mut self, ctx: &egui::Context, stats: &mut MediaStats) {
        let mut open_file_path: Option<PathBuf> = None;
        let mut open_folder_path: Option<PathBuf> = None;
        let mut toggle_fullscreen_requested = false;
        let mut exit_fullscreen_requested = false;
        let mut toggle_pip_requested = false;
        let mut window_resize_preset: Option<f32> = None;
        let mut center_window_requested = false;
        let mut do_skip_intro_key = false;
        let mut toggle_playlist_unified_requested = false;
        let mut do_close_file = false;

        let wants_kb = ctx.egui_wants_keyboard_input();

        // ── Sleep Timer Expiration Check ─────────────────────────────────────
        if self.sleep_timer.is_expired() {
            let action = self.sleep_timer.completion_action;
            self.sleep_timer.cancel();
            if let Ok(ref player) = self.player {
                player.pause();
            }
            self.osd.show(format!("🌙 Sleep Timer Triggered: {}", action.label()), 3000);
            self.sleep_timer.execute_system_action();
        }

        if let Ok(ref player) = self.player {
            let input = ctx.input(|i| i.clone());

            // ── Escape: Close Dialogs / Popups / Exit Fullscreen ───────────────
            if input.key_pressed(egui::Key::Escape) {
                let had_dialog = self.has_open_dialog()
                    || self.show_main_menu
                    || self.show_speed_popup
                    || self.show_audio_channels_popup;

                self.show_main_menu = false;
                self.show_preferences = false;
                self.show_control_panel = false;
                self.show_bookmark_overlay = false;
                self.show_mediainfo_dialog = false;
                self.show_capture_dialog = false;
                self.show_stream_url_dialog = false;
                self.show_about_dialog = false;
                self.show_peq_dialog = false;
                self.show_sleep_timer_dialog = false;
                self.show_update_dialog = false;
                self.show_surround_eq_dialog = false;
                self.show_jump_time_dialog = false;
                self.show_library_view = false;
                self.show_speed_popup = false;
                self.show_audio_channels_popup = false;
                self.bookmark_studio.is_open = false;
                self.subtitle_explorer.is_open = false;
                self.settings_inspector.is_open = false;
                self.input_editor.is_open = false;
                self.file_navigator.is_open = false;
                self.network_browser.is_open = false;
                self.gif_maker.is_open = false;
                self.device_capture.is_open = false;
                self.subtitle_lookup.is_open = false;
                self.record_dialog.is_open = false;
                self.contact_sheet_dialog.is_open = false;
                self.broadcast_dialog.is_open = false;
                self.transcoder_dialog.is_open = false;
                self.boss_key_dialog.is_open = false;
                self.stereo_3d_dialog.is_open = false;
                self.karaoke_studio.is_open = false;
                self.subtitle_studio.is_open = false;
                self.screen_capture.is_open = false;
                self.iptv_guide.is_open = false;
                self.logo_watermark.is_open = false;
                self.disc_nav.is_open = false;
                self.cd_ripper.is_open = false;
                self.bda_tuner.is_open = false;
                self.bdj_dialog.is_open = false;
                self.vst_winamp.is_open = false;
                self.detached_windows.is_open = false;
                self.discord_rpc.is_open = false;
                self.web_remote.is_open = false;
                self.scrobbler_dialog.is_open = false;
                self.media_server.is_open = false;
                self.ambilight_dialog.is_open = false;
                self.cast_renderer.is_open = false;
                self.zoom_magnifier.is_open = false;
                self.radio_directory.is_open = false;
                self.video_puzzle.is_open = false;
                self.vr_360_studio.is_open = false;
                self.damaged_file_repair.is_open = false;

                if self.is_fullscreen && !had_dialog {
                    exit_fullscreen_requested = true;
                }
            }

            // ── Function Keys & Telemetry HUD ─────────────────────────────────
            if input.key_pressed(egui::Key::F1) && !input.modifiers.ctrl {
                self.show_about_dialog = !self.show_about_dialog;
            }
            if input.modifiers.ctrl && input.key_pressed(egui::Key::F1) {
                self.show_mediainfo_dialog = !self.show_mediainfo_dialog;
                if self.show_mediainfo_dialog {
                    self.osd.show_media_info = false;
                    if input.modifiers.shift {
                        self.mediainfo_dialog.selected_tab = 1;
                    }
                }
            }
            if input.key_pressed(egui::Key::F5) {
                self.show_preferences = !self.show_preferences;
            }
            if input.key_pressed(egui::Key::F6) || input.key_pressed(egui::Key::F7) {
                self.show_control_panel = !self.show_control_panel;
            }
            if input.key_pressed(egui::Key::F8) {
                toggle_playlist_unified_requested = true;
            }
            if input.key_pressed(egui::Key::F9) {
                self.show_peq_dialog = !self.show_peq_dialog;
            }
            if input.key_pressed(egui::Key::F10) {
                self.settings_inspector.is_open = !self.settings_inspector.is_open;
            }
            if input.key_pressed(egui::Key::F11) {
                toggle_fullscreen_requested = true;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::Enter) {
                if !self.is_fullscreen {
                    toggle_fullscreen_requested = true;
                }
                if let Ok(ref player) = self.player {
                    let next_ar = player.cycle_aspect_ratio();
                    self.osd.show(format!("Aspect Ratio: {}", next_ar.to_uppercase()), 1200);
                }
            }
            if input.key_pressed(egui::Key::F12) {
                self.show_library_view = !self.show_library_view;
            }
            if input.key_pressed(egui::Key::Tab) && !self.show_mediainfo_dialog {
                self.osd.toggle_media_info();
            }

            // ── File & Stream Open Dialogs ────────────────────────────────────
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::O) && !input.modifiers.alt {
                if let Some(file) = rfd::FileDialog::new()
                    .add_filter("Media & Playlists", &["mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "mp3", "flac", "wav", "dpl", "m3u", "m3u8"])
                    .pick_file()
                {
                    open_file_path = Some(file);
                }
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::F) && !input.modifiers.shift && !input.modifiers.alt {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    open_folder_path = Some(folder);
                }
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::U) && !input.modifiers.shift && !input.modifiers.alt {
                self.show_stream_url_dialog = !self.show_stream_url_dialog;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.modifiers.shift && input.key_pressed(egui::Key::U) {
                self.show_update_dialog = true;
                self.update_dialog.check_for_updates();
            }

            // ── Ctrl / Cmd Combinations ───────────────────────────────────────
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::P) && !input.modifiers.shift && !input.modifiers.alt {
                toggle_playlist_unified_requested = true;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::K) && !input.modifiers.shift && !input.modifiers.alt {
                self.input_editor.is_open = !self.input_editor.is_open;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::M) && !input.modifiers.shift && !input.modifiers.alt {
                self.channel_matrix_dialog.is_open = !self.channel_matrix_dialog.is_open;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::T) && !input.modifiers.shift && !input.modifiers.alt {
                self.show_telemetry_hud = !self.show_telemetry_hud;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::R) && !input.modifiers.shift && !input.modifiers.alt {
                let mode = self.playlist.lock().unwrap().cycle_repeat_mode();
                let msg = match mode {
                    crate::playlist::RepeatMode::RepeatAll => "Repeat: All Tracks",
                    crate::playlist::RepeatMode::RepeatTrack => "Repeat: Current Track",
                    crate::playlist::RepeatMode::Off => "Repeat: Off",
                };
                self.osd.show(msg.to_string(), 1500);
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::S) && !input.modifiers.shift && !input.modifiers.alt {
                self.show_capture_dialog = !self.show_capture_dialog;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::E) && !input.modifiers.shift && !input.modifiers.alt {
                if let Some(pic_dir) = dirs::picture_dir() {
                    let path = pic_dir.join(format!("VertexPlayer_{}.jpg", chrono::Utc::now().format("%Y%m%d_%H%M%S")));
                    player.take_screenshot(&path);
                    self.toast.success("Screenshot saved to Pictures");
                }
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::G) && !input.modifiers.shift && !input.modifiers.alt {
                self.gif_maker.is_open = !self.gif_maker.is_open;
            }
            let is_close_file_key = ((input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::W) && !input.modifiers.shift && !input.modifiers.alt)
                || input.key_pressed(egui::Key::F4);
            if is_close_file_key && !self.show_main_menu {
                do_close_file = true;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::B) && !input.modifiers.shift && !input.modifiers.alt {
                self.broadcast_dialog.is_open = !self.broadcast_dialog.is_open;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::H) && !input.modifiers.shift && !input.modifiers.alt {
                player.toggle_subtitles();
                self.osd.show("Toggle Subtitles".to_string(), 1200);
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::I) && !input.modifiers.shift && !input.modifiers.alt {
                self.iptv_guide.is_open = !self.iptv_guide.is_open;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::D) && !input.modifiers.shift && !input.modifiers.alt {
                self.disc_nav.is_open = !self.disc_nav.is_open;
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::F6) && !input.modifiers.shift && !input.modifiers.alt {
                let next_ar = player.cycle_aspect_ratio();
                self.osd.show(format!("Aspect Ratio: {}", next_ar.to_uppercase()), 1200);
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::OpenBracket) && !input.modifiers.shift && !input.modifiers.alt {
                let time_str = self.bookmark_mgr.set_loop_a(stats.time_pos);
                self.osd.show(format!("[A-B Repeat] Set A: {}", time_str), 1500);
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::CloseBracket) && !input.modifiers.shift && !input.modifiers.alt {
                match self.bookmark_mgr.set_loop_b(stats.time_pos) {
                    Ok(time_str) => self.osd.show(format!("[A-B Repeat] Set B: {}", time_str), 1500),
                    Err(e) => self.osd.show(format!("[A-B Repeat] Error: {}", e), 1500),
                }
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::Backslash) && !input.modifiers.shift && !input.modifiers.alt {
                self.bookmark_mgr.clear_ab_loop();
                self.osd.show("[A-B Repeat] Cleared".to_string(), 1200);
            }
            if (input.modifiers.ctrl || input.modifiers.command) && input.key_pressed(egui::Key::L) && !input.modifiers.shift && !input.modifiers.alt {
                if self.bookmark_mgr.ab_loop.point_a.is_none() {
                    let time_str = self.bookmark_mgr.set_loop_a(stats.time_pos);
                    self.osd.show(format!("[A-B Loop] Set Point A: {}", time_str), 1500);
                } else if self.bookmark_mgr.ab_loop.point_b.is_none() {
                    match self.bookmark_mgr.set_loop_b(stats.time_pos) {
                        Ok(time_str) => self.osd.show(format!("[A-B Loop] Set Point B: {}", time_str), 1500),
                        Err(e) => self.osd.show(format!("[A-B Loop] Error: {}", e), 1500),
                    }
                } else {
                    self.bookmark_mgr.clear_ab_loop();
                    self.osd.show("[A-B Loop] Cleared".to_string(), 1200);
                }
            }

            // ── Ctrl+Shift Combinations ───────────────────────────────────────
            if input.modifiers.ctrl && input.modifiers.shift && !input.modifiers.alt {
                if input.key_pressed(egui::Key::Z) {
                    self.show_sleep_timer_dialog = !self.show_sleep_timer_dialog;
                }
                if input.key_pressed(egui::Key::V) {
                    self.visualizer.toggle();
                    let state = if self.visualizer.is_visible { "ON" } else { "OFF" };
                    self.osd.show(format!("Visualizer Suite: {}", state), 1200);
                }
                if input.key_pressed(egui::Key::Num7) {
                    self.show_surround_eq_dialog = !self.show_surround_eq_dialog;
                }
                if input.key_pressed(egui::Key::F) {
                    self.chain_editor.is_open = !self.chain_editor.is_open;
                }
                if input.key_pressed(egui::Key::B) {
                    self.bookmark_studio.is_open = !self.bookmark_studio.is_open;
                }
                if input.key_pressed(egui::Key::S) {
                    self.subtitle_explorer.is_open = !self.subtitle_explorer.is_open;
                }
                if input.key_pressed(egui::Key::E) {
                    if let Some(ref path) = self.bookmark_mgr.current_video_path {
                        match crate::bookmark::PbfFile::save_for_video(path, &self.bookmark_mgr.bookmarks) {
                            Ok(pbf_path) => {
                                let msg = format!("Exported PBF: {}", pbf_path.file_name().unwrap_or_default().to_string_lossy());
                                self.toast.success(msg);
                            }
                            Err(e) => {
                                self.toast.error_toast(format!("PBF Export failed: {}", e));
                            }
                        }
                    }
                }
                if input.key_pressed(egui::Key::L) {
                    self.ab_repeat_dialog.is_open = !self.ab_repeat_dialog.is_open;
                }
                if input.key_pressed(egui::Key::A) {
                    self.auto_skip_dialog.is_open = !self.auto_skip_dialog.is_open;
                }
                if input.key_pressed(egui::Key::D) {
                    self.cd_ripper.is_open = !self.cd_ripper.is_open;
                }
                if input.key_pressed(egui::Key::T) {
                    self.color_lut_dialog.is_open = !self.color_lut_dialog.is_open;
                }
                if input.key_pressed(egui::Key::M) {
                    self.motion_interpolation.is_open = !self.motion_interpolation.is_open;
                }
                if input.key_pressed(egui::Key::N) {
                    self.audio_compressor.is_open = !self.audio_compressor.is_open;
                }
                if input.key_pressed(egui::Key::C) {
                    self.video_crop.is_open = !self.video_crop.is_open;
                }
                if input.key_pressed(egui::Key::G) {
                    self.goto_frame.is_open = !self.goto_frame.is_open;
                }
                if input.key_pressed(egui::Key::I) {
                    self.deinterlace_dialog.is_open = !self.deinterlace_dialog.is_open;
                }
                if input.key_pressed(egui::Key::Delete) {
                    let (path, next_path) = {
                        let mut pl = self.playlist.lock().unwrap_or_else(|e| e.into_inner());
                        if let Some(idx) = pl.current_index {
                            if idx < pl.items.len() {
                                let path = pl.items[idx].path.clone();
                                pl.items.remove(idx);
                                let next = pl.next();
                                (Some(path), next)
                            } else {
                                (None, None)
                            }
                        } else {
                            (None, None)
                        }
                    };
                    if let Some(path) = path {
                        let _ = std::fs::remove_file(&path);
                        self.toast.warning(format!("Deleted from disk: {}", path.file_name().unwrap_or_default().to_string_lossy()));
                        if let Some(next) = next_path {
                            open_file_path = Some(next);
                        } else {
                            player.stop();
                        }
                    }
                }
            }

            // ── Alt Combinations ──────────────────────────────────────────────
            if input.modifiers.alt && !input.modifiers.ctrl && !input.modifiers.shift {
                if input.key_pressed(egui::Key::Enter) {
                    toggle_fullscreen_requested = true;
                }
                if input.key_pressed(egui::Key::Num1) {
                    window_resize_preset = Some(0.5);
                }
                if input.key_pressed(egui::Key::Num2) {
                    window_resize_preset = Some(1.0);
                }
                if input.key_pressed(egui::Key::Num3) {
                    window_resize_preset = Some(1.5);
                }
                if input.key_pressed(egui::Key::Num4) {
                    window_resize_preset = Some(2.0);
                }
                if input.key_pressed(egui::Key::Num5) || input.key_pressed(egui::Key::Num0) {
                    center_window_requested = true;
                }
                if input.key_pressed(egui::Key::PageUp) {
                    self.config.subtitle_font_size = (self.config.subtitle_font_size + 2.0).clamp(10.0, 80.0);
                    player.set_subtitle_font_size(self.config.subtitle_font_size);
                    self.osd.show(format!("Subtitle Size: {:.0}pt", self.config.subtitle_font_size), 1200);
                    let _ = self.config.save();
                }
                if input.key_pressed(egui::Key::PageDown) {
                    self.config.subtitle_font_size = (self.config.subtitle_font_size - 2.0).clamp(10.0, 80.0);
                    player.set_subtitle_font_size(self.config.subtitle_font_size);
                    self.osd.show(format!("Subtitle Size: {:.0}pt", self.config.subtitle_font_size), 1200);
                    let _ = self.config.save();
                }
                if input.key_pressed(egui::Key::ArrowUp) {
                    self.config.subtitle_vertical_pos = (self.config.subtitle_vertical_pos - 2.0).clamp(0.0, 115.0);
                    player.set_subtitle_pos(self.config.subtitle_vertical_pos);
                    self.osd.show(format!("Subtitle Position: {:.0}%", self.config.subtitle_vertical_pos), 1200);
                    let _ = self.config.save();
                }
                if input.key_pressed(egui::Key::ArrowDown) {
                    self.config.subtitle_vertical_pos = (self.config.subtitle_vertical_pos + 2.0).clamp(0.0, 115.0);
                    player.set_subtitle_pos(self.config.subtitle_vertical_pos);
                    self.osd.show(format!("Subtitle Position: {:.0}%", self.config.subtitle_vertical_pos), 1200);
                    let _ = self.config.save();
                }
                if input.key_pressed(egui::Key::Home) {
                    self.config.subtitle_font_size = 28.0;
                    self.config.subtitle_vertical_pos = 102.0;
                    player.set_subtitle_font_size(28.0);
                    player.set_subtitle_pos(102.0);
                    self.osd.show("Subtitle Size & Position: Reset".to_string(), 1200);
                    let _ = self.config.save();
                }
                if input.key_pressed(egui::Key::C) {
                    self.record_dialog.is_open = !self.record_dialog.is_open;
                }
                if input.key_pressed(egui::Key::P) {
                    toggle_pip_requested = true;
                }
                if input.key_pressed(egui::Key::A) {
                    player.cycle_audio_track();
                    self.osd.show("Cycled Audio Stream".to_string(), 1500);
                }
                if input.key_pressed(egui::Key::H) {
                    self.config.video_flip_horizontal = !self.config.video_flip_horizontal;
                    let mut fx = crate::engine::VideoEffectsConfig::default();
                    fx.flip_horizontal = self.config.video_flip_horizontal;
                    fx.flip_vertical = self.config.video_flip_vertical;
                    fx.deband = self.config.video_deband;
                    fx.lut_file = self.config.lut_file.clone();
                    player.set_video_filter(&fx.build_video_filter_string());
                    let state = if self.config.video_flip_horizontal { "ON" } else { "OFF" };
                    self.osd.show(format!("Horizontal Flip: {}", state), 1200);
                }
                if input.key_pressed(egui::Key::V) {
                    self.config.video_flip_vertical = !self.config.video_flip_vertical;
                    let mut fx = crate::engine::VideoEffectsConfig::default();
                    fx.flip_horizontal = self.config.video_flip_horizontal;
                    fx.flip_vertical = self.config.video_flip_vertical;
                    fx.deband = self.config.video_deband;
                    fx.lut_file = self.config.lut_file.clone();
                    player.set_video_filter(&fx.build_video_filter_string());
                    let state = if self.config.video_flip_vertical { "ON" } else { "OFF" };
                    self.osd.show(format!("Vertical Flip: {}", state), 1200);
                }
                if input.key_pressed(egui::Key::L) {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("3D LUT File (*.cube)", &["cube"])
                        .pick_file()
                    {
                        self.config.lut_file = Some(path.to_string_lossy().to_string());
                        let mut fx = crate::engine::VideoEffectsConfig::default();
                        fx.lut_file = self.config.lut_file.clone();
                        fx.deband = self.config.video_deband;
                        fx.flip_horizontal = self.config.video_flip_horizontal;
                        fx.flip_vertical = self.config.video_flip_vertical;
                        player.set_video_filter(&fx.build_video_filter_string());
                        self.osd.show("🎨 3D LUT: Applied".to_string(), 1500);
                    }
                }
            }

            // ── Ctrl+Alt Combinations ─────────────────────────────────────────
            if input.modifiers.ctrl && input.modifiers.alt {
                if input.key_pressed(egui::Key::B) {
                    // Boss key
                    // Keep playback, audio, and the queue unchanged while the
                    // window is hidden. MPV's mute flag is global and would
                    // otherwise carry into every later playlist item.
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                    self.toast.info("Boss Key: Minimized");
                }
                if input.key_pressed(egui::Key::Num1) {
                    self.vr_360_studio.is_open = !self.vr_360_studio.is_open;
                }
                if input.key_pressed(egui::Key::Num2) {
                    self.video_wall_matrix.is_open = !self.video_wall_matrix.is_open;
                }
                if input.key_pressed(egui::Key::Num3) {
                    self.stereo_3d_dialog.is_open = !self.stereo_3d_dialog.is_open;
                }
                if input.key_pressed(egui::Key::Num4) {
                    self.ai_upscaling.is_open = !self.ai_upscaling.is_open;
                }
                if input.key_pressed(egui::Key::Num5) {
                    self.frame_dumper.is_open = !self.frame_dumper.is_open;
                }
                if input.key_pressed(egui::Key::Num6) {
                    self.ai_audio.is_open = !self.ai_audio.is_open;
                }
                if input.key_pressed(egui::Key::Num7) {
                    self.loudness_radar.is_open = !self.loudness_radar.is_open;
                }
                if input.key_pressed(egui::Key::Num8) {
                    self.rich_bookmark_notes.is_open = !self.rich_bookmark_notes.is_open;
                }
                if input.key_pressed(egui::Key::Num9) {
                    self.color_blindness.is_open = !self.color_blindness.is_open;
                }
                if input.key_pressed(egui::Key::Num0) {
                    self.cinema_matte.is_open = !self.cinema_matte.is_open;
                }
                if input.key_pressed(egui::Key::P) {
                    self.pitch_formant.is_open = !self.pitch_formant.is_open;
                }
                if input.key_pressed(egui::Key::J) {
                    self.chapter_marker_dialog.is_open = !self.chapter_marker_dialog.is_open;
                }
                if input.key_pressed(egui::Key::Y) {
                    self.playback_history.is_open = !self.playback_history.is_open;
                }
                if input.key_pressed(egui::Key::E) {
                    self.media_tag_editor.is_open = !self.media_tag_editor.is_open;
                }
                if input.key_pressed(egui::Key::T) {
                    self.bda_tuner.is_open = !self.bda_tuner.is_open;
                }
                if input.key_pressed(egui::Key::V) {
                    self.vst_winamp.is_open = !self.vst_winamp.is_open;
                }
                if input.key_pressed(egui::Key::D) {
                    self.detached_windows.is_open = !self.detached_windows.is_open;
                }
                if input.key_pressed(egui::Key::R) {
                    self.discord_rpc.is_open = !self.discord_rpc.is_open;
                }
                if input.key_pressed(egui::Key::W) {
                    self.web_remote.is_open = !self.web_remote.is_open;
                }
                if input.key_pressed(egui::Key::S) {
                    self.scrobbler_dialog.is_open = !self.scrobbler_dialog.is_open;
                }
                if input.key_pressed(egui::Key::L) {
                    self.ambilight_dialog.is_open = !self.ambilight_dialog.is_open;
                }
                if input.key_pressed(egui::Key::C) {
                    self.cast_renderer.is_open = !self.cast_renderer.is_open;
                }
                if input.key_pressed(egui::Key::M) {
                    self.zoom_magnifier.is_open = !self.zoom_magnifier.is_open;
                }
                if input.key_pressed(egui::Key::I) {
                    self.radio_directory.is_open = !self.radio_directory.is_open;
                }
                if input.key_pressed(egui::Key::Z) {
                    self.video_puzzle.is_open = !self.video_puzzle.is_open;
                }
                if input.key_pressed(egui::Key::F) {
                    self.damaged_file_repair.is_open = !self.damaged_file_repair.is_open;
                }
                if input.key_pressed(egui::Key::H) {
                    self.binaural_crossfeed.is_open = !self.binaural_crossfeed.is_open;
                }
                if input.key_pressed(egui::Key::K) {
                    self.boss_key_dialog.trigger_boss_key(player, ctx);
                    self.boss_key_dialog.is_open = !self.boss_key_dialog.is_open;
                }
            }

            // ── Shift Navigation & Audio/Video Adjustments ─────────────────────
            if input.modifiers.shift && !input.modifiers.ctrl && !input.modifiers.alt {
                if input.key_pressed(egui::Key::ArrowLeft) {
                    player.seek_relative(-60.0);
                    self.osd.show_seek(stats.time_pos - 60.0, stats.duration, Some(-60.0));
                }
                if input.key_pressed(egui::Key::ArrowRight) {
                    player.seek_relative(60.0);
                    self.osd.show_seek(stats.time_pos + 60.0, stats.duration, Some(60.0));
                }
                if input.key_pressed(egui::Key::D) {
                    self.config.video_deband = !self.config.video_deband;
                    let mut fx = crate::engine::VideoEffectsConfig::default();
                    fx.deband = self.config.video_deband;
                    fx.flip_horizontal = self.config.video_flip_horizontal;
                    fx.flip_vertical = self.config.video_flip_vertical;
                    fx.lut_file = self.config.lut_file.clone();
                    player.set_video_filter(&fx.build_video_filter_string());
                    let state = if self.config.video_deband { "ON" } else { "OFF" };
                    self.osd.show(format!("Deband Filter: {}", state), 1200);
                }
                if input.key_pressed(egui::Key::C) {
                    self.config.auto_crop_black_bars = !self.config.auto_crop_black_bars;
                    let mut fx = crate::engine::VideoEffectsConfig::default();
                    fx.auto_crop_black_bars = self.config.auto_crop_black_bars;
                    fx.deband = self.config.video_deband;
                    fx.flip_horizontal = self.config.video_flip_horizontal;
                    fx.flip_vertical = self.config.video_flip_vertical;
                    fx.lut_file = self.config.lut_file.clone();
                    player.set_video_filter(&fx.build_video_filter_string());
                    let state = if self.config.auto_crop_black_bars { "ON" } else { "OFF" };
                    self.osd.show(format!("Auto-Crop Black Bars: {}", state), 1200);
                }
                if input.key_pressed(egui::Key::OpenBracket) {
                    player.adjust_audio_delay(-0.05);
                    let ms = (stats.audio_delay * 1000.0).round() as i64;
                    self.osd.show_audio_delay(ms);
                }
                if input.key_pressed(egui::Key::CloseBracket) {
                    player.adjust_audio_delay(0.05);
                    let ms = (stats.audio_delay * 1000.0).round() as i64;
                    self.osd.show_audio_delay(ms);
                }
                if input.key_pressed(egui::Key::Backslash) {
                    player.set_audio_delay(0.0);
                    self.osd.show_audio_delay(0);
                }
                if input.key_pressed(egui::Key::P) && !wants_kb {
                    self.bookmark_mgr.add_bookmark(stats.time_pos, Some(format!("[SKIP] Marker at {}", format_time(stats.time_pos))));
                    self.osd.show(format!("Skip Bookmark Added at {}", format_time(stats.time_pos)), 1800);
                }
                if (input.key_pressed(egui::Key::H) || input.key_pressed(egui::Key::B)) && !wants_kb {
                    if let Some(target) = self.bookmark_mgr.prev_bookmark(stats.time_pos) {
                        player.seek_absolute(target);
                        self.osd.show(format!("Jump to Prev Bookmark: {}", format_time(target)), 1200);
                    }
                }
                if input.key_pressed(egui::Key::Delete) && !wants_kb {
                    let (path, next_path) = {
                        let mut pl = self.playlist.lock().unwrap_or_else(|e| e.into_inner());
                        if let Some(idx) = pl.current_index {
                            if idx < pl.items.len() {
                                let path = pl.items[idx].path.clone();
                                pl.items.remove(idx);
                                let next = pl.next();
                                (Some(path), next)
                            } else {
                                (None, None)
                            }
                        } else {
                            (None, None)
                        }
                    };
                    if let Some(path) = path {
                        let _ = std::fs::remove_file(&path);
                        self.toast.warning(format!("Deleted from disk: {}", path.file_name().unwrap_or_default().to_string_lossy()));
                        if let Some(next) = next_path {
                            open_file_path = Some(next);
                        } else {
                            player.stop();
                        }
                    }
                }
            }

            // ── Guarded Playback / Navigation Shortcuts (Ignored when typing in text fields) ──
            if !wants_kb && !input.modifiers.ctrl && !input.modifiers.alt && !input.modifiers.command {
                // Space: Play / Pause with unified OSD
                if input.key_pressed(egui::Key::Space) {
                    if stats.is_idle {
                        if let Some(item) = self.playlist.lock().unwrap().current_item() {
                            open_file_path = Some(item.path.clone());
                        } else if let Some(recent) = self.config.recent_files.first() {
                            let p = PathBuf::from(recent);
                            if p.exists() {
                                open_file_path = Some(p);
                            }
                        }
                    } else {
                        player.toggle_pause();
                        let is_paused = !stats.is_paused;
                        self.osd.show_play_pause(is_paused);
                    }
                }

                // Enter / Return: Toggle Fullscreen (Keep Aspect Ratio)
                if input.key_pressed(egui::Key::Enter) {
                    toggle_fullscreen_requested = true;
                }

                // Navigation: Left / Right (Seek)
                if !input.modifiers.shift {
                    if input.key_pressed(egui::Key::ArrowLeft) {
                        let step = if self.config.adaptive_seek { (stats.duration * 0.01).clamp(2.0, 10.0) } else { self.config.seek_step_short };
                        player.seek_relative(-step);
                        self.osd.show_seek(stats.time_pos - step, stats.duration, Some(-step));
                    }
                    if input.key_pressed(egui::Key::ArrowRight) {
                        let step = if self.config.adaptive_seek { (stats.duration * 0.01).clamp(2.0, 10.0) } else { self.config.seek_step_short };
                        player.seek_relative(step);
                        self.osd.show_seek(stats.time_pos + step, stats.duration, Some(step));
                    }
                }

                // Volume: Up / Down
                if input.key_pressed(egui::Key::ArrowUp) {
                    let next_vol = (stats.volume + 5.0).clamp(0.0, self.config.volume_boost_limit);
                    player.set_volume(next_vol);
                    self.config.volume = next_vol;
                    self.osd.show_volume(next_vol, stats.is_muted);
                }
                if input.key_pressed(egui::Key::ArrowDown) {
                    let next_vol = (stats.volume - 5.0).clamp(0.0, self.config.volume_boost_limit);
                    player.set_volume(next_vol);
                    self.config.volume = next_vol;
                    self.osd.show_volume(next_vol, stats.is_muted);
                }

                // M: Mute
                if input.key_pressed(egui::Key::M) && !input.modifiers.shift {
                    player.toggle_mute();
                    self.osd.show_volume(stats.volume, !stats.is_muted);
                }

                // Speed Control: Z (1.0x), X (-0.1x), C (+0.1x)
                if !input.modifiers.shift {
                    if input.key_pressed(egui::Key::Z) {
                        player.reset_speed();
                        self.osd.show("Speed: 1.00x (Normal)".to_string(), 1200);
                    }
                    if input.key_pressed(egui::Key::X) {
                        player.adjust_speed(-0.1);
                        self.osd.show(format!("Speed: {:.2}x", stats.speed), 1200);
                    }
                    if input.key_pressed(egui::Key::C) {
                        player.adjust_speed(0.1);
                        self.osd.show(format!("Speed: {:.2}x", stats.speed), 1200);
                    }
                }

                // Subtitle Sync: [ (Faster -0.05s), ] (Slower +0.05s), \ (Reset 0.0s)
                if !input.modifiers.shift {
                    if input.key_pressed(egui::Key::OpenBracket) {
                        let next = player.adjust_subtitle_delay(-0.05);
                        self.osd.show_sub_delay(next);
                    }
                    if input.key_pressed(egui::Key::CloseBracket) {
                        let next = player.adjust_subtitle_delay(0.05);
                        self.osd.show_sub_delay(next);
                    }
                    if input.key_pressed(egui::Key::Backslash) {
                        player.set_subtitle_delay(0.0);
                        self.osd.show_sub_delay(0.0);
                    }
                }

                // Q: Reset Video Color adjustments (Brightness, Contrast, Saturation, Hue) to 100%
                if input.key_pressed(egui::Key::Q) && !input.modifiers.shift {
                    self.config.video_brightness = 100.0;
                    self.config.video_contrast = 100.0;
                    self.config.video_saturation = 100.0;
                    self.config.video_hue = 0.0;
                    player.reset_video_colors();
                    let _ = self.config.save();
                    self.osd.show("Video Colors: Reset to 100% Default".to_string(), 1500);
                }

                // P: Add Bookmark
                if input.key_pressed(egui::Key::P) && !input.modifiers.shift {
                    self.bookmark_mgr.add_bookmark(stats.time_pos, None);
                    self.osd.show(format!("Bookmark Added at {}", format_time(stats.time_pos)), 1500);
                }

                // L: Toggle Bookmark Overlay
                if input.key_pressed(egui::Key::L) && !input.modifiers.shift {
                    self.show_bookmark_overlay = !self.show_bookmark_overlay;
                }

                // G: Jump Time Dialog
                if input.key_pressed(egui::Key::G) && !input.modifiers.shift {
                    self.show_jump_time_dialog = !self.show_jump_time_dialog;
                }

                // T: Skin / Theme Cycling
                if input.key_pressed(egui::Key::T) && !input.modifiers.shift {
                    self.config.theme_mode = self.config.theme_mode.next();
                    let _ = self.config.save();
                    VortexTheme::apply(ctx, self.config.theme_mode);
                    self.osd.show(format!("Skin: {}", self.config.theme_mode.display_name()), 1500);
                }

                // Chapter Navigation: H or ] (Next Chapter), Shift+H or [ (Previous Chapter)
                if (input.key_pressed(egui::Key::H) || input.key_pressed(egui::Key::CloseBracket)) && !input.modifiers.shift && !input.modifiers.command {
                    player.next_chapter();
                    self.osd.show("Next Chapter ⏭".to_string(), 1200);
                }
                if (input.key_pressed(egui::Key::OpenBracket) || (input.key_pressed(egui::Key::H) && input.modifiers.shift)) && !input.modifiers.command {
                    player.prev_chapter();
                    self.osd.show("Previous Chapter ⏮".to_string(), 1200);
                }

                // Bookmark Jump: B (Next Bookmark)
                if input.key_pressed(egui::Key::B) && !input.modifiers.shift && !input.modifiers.command {
                    if let Some(target) = self.bookmark_mgr.next_bookmark(stats.time_pos) {
                        player.seek_absolute(target);
                        self.osd.show(format!("Jump to Next Bookmark: {}", format_time(target)), 1200);
                    }
                }

                // S / Tab: Instant Skip Intro / Anime OP
                if input.key_pressed(egui::Key::S) && !input.modifiers.shift && !input.modifiers.command && !input.modifiers.alt {
                    do_skip_intro_key = true;
                }

                // Shift+S: Playlist Shuffle
                if input.key_pressed(egui::Key::S) && input.modifiers.shift && !input.modifiers.command {
                    self.playlist.lock().unwrap().shuffle_items();
                    self.osd.show("🔀 Playlist Shuffled".to_string(), 1200);
                }

                // Ctrl+Alt+S: Toggle Auto-Skip Engine
                if input.key_pressed(egui::Key::S) && input.modifiers.alt && input.modifiers.command {
                    self.config.skip_intro_enabled = !self.config.skip_intro_enabled;
                    let _ = self.config.save();
                    let st = if self.config.skip_intro_enabled { "ON" } else { "OFF" };
                    self.osd.show(format!("Auto-Skip Intro: {}", st), 1500);
                }

                // Frame Step (when paused): F (Forward), D (Backward)
                if stats.is_paused && !input.modifiers.shift {
                    if input.key_pressed(egui::Key::F) {
                        player.step_frame_forward();
                        self.osd.show(format!("Frame Step +1 ({})", format_time(stats.time_pos)), 1000);
                    }
                    if input.key_pressed(egui::Key::D) {
                        player.step_frame_backward();
                        self.osd.show(format!("Frame Step -1 ({})", format_time(stats.time_pos)), 1000);
                    }
                }

                // Subtitle Sync / Delay Shortcuts:
                // > / . : Subtitle 0.5s Faster (-0.5s) [Shift+> / . for fine 0.1s]
                // < / , : Subtitle 0.5s Slower (+0.5s) [Shift+< / , for fine 0.1s]
                // /     : Reset Subtitle Sync (0.0s)
                if input.key_pressed(egui::Key::Period) {
                    let delta = if input.modifiers.shift { -0.1 } else { -0.5 };
                    let next = player.adjust_subtitle_delay(delta);
                    self.osd.show_sub_delay(next);
                    ctx.request_repaint();
                }
                if input.key_pressed(egui::Key::Comma) {
                    let delta = if input.modifiers.shift { 0.1 } else { 0.5 };
                    let next = player.adjust_subtitle_delay(delta);
                    self.osd.show_sub_delay(next);
                    ctx.request_repaint();
                }
                if input.key_pressed(egui::Key::Slash) && !input.modifiers.ctrl && !input.modifiers.alt {
                    player.set_subtitle_delay(0.0);
                    self.osd.show_sub_delay(0.0);
                    ctx.request_repaint();
                }

                // Audio Delay Shortcuts:
                // Shift+D : Audio Sync -50ms
                // Shift+F : Audio Sync +50ms
                if input.modifiers.shift && !input.modifiers.ctrl && !input.modifiers.alt {
                    if input.key_pressed(egui::Key::D) {
                        player.adjust_audio_delay(-0.05);
                        let next = stats.audio_delay;
                        let ms = (next * 1000.0).round() as i64;
                        self.osd.show_audio_delay(ms);
                    }
                    if input.key_pressed(egui::Key::F) {
                        player.adjust_audio_delay(0.05);
                        let next = stats.audio_delay;
                        let ms = (next * 1000.0).round() as i64;
                        self.osd.show_audio_delay(ms);
                    }
                }

                // PageUp / PageDown / Home / End: Playlist Track Navigation
                if input.key_pressed(egui::Key::PageUp) {
                    if let Some(prev) = self.playlist_lock().previous() {
                        open_file_path = Some(prev);
                    }
                }
                if input.key_pressed(egui::Key::PageDown) {
                    if let Some(next) = self.playlist_lock().next() {
                        open_file_path = Some(next);
                    }
                }
                if input.key_pressed(egui::Key::Home) {
                    let mut pl = self.playlist_lock();
                    if !pl.items.is_empty() {
                        pl.current_index = Some(0);
                        open_file_path = Some(pl.items[0].path.clone());
                    }
                }
                if input.key_pressed(egui::Key::End) {
                    let mut pl = self.playlist_lock();
                    if !pl.items.is_empty() {
                        let last_idx = pl.items.len() - 1;
                        pl.current_index = Some(last_idx);
                        open_file_path = Some(pl.items[last_idx].path.clone());
                    }
                }

                // Delete: Remove currently playing track from playlist only (keeps file on disk)
                if input.key_pressed(egui::Key::Delete) && !input.modifiers.shift {
                    let (title, next_path) = {
                        let mut pl = self.playlist.lock().unwrap_or_else(|e| e.into_inner());
                        if let Some(idx) = pl.current_index {
                            if idx < pl.items.len() {
                                let item = pl.items.remove(idx);
                                let next = pl.next();
                                (Some(item.title), next)
                            } else {
                                (None, None)
                            }
                        } else {
                            (None, None)
                        }
                    };
                    if let Some(title) = title {
                        self.osd.show(format!("Removed from playlist: {}", title), 1500);
                        if let Some(next_path) = next_path {
                            open_file_path = Some(next_path);
                        } else {
                            player.stop();
                        }
                    }
                }

                // Percentage Jump: 0..9
                if !input.modifiers.shift {
                    let pct_keys = [
                        (egui::Key::Num0, 0),
                        (egui::Key::Num1, 10),
                        (egui::Key::Num2, 20),
                        (egui::Key::Num3, 30),
                        (egui::Key::Num4, 40),
                        (egui::Key::Num5, 50),
                        (egui::Key::Num6, 60),
                        (egui::Key::Num7, 70),
                        (egui::Key::Num8, 80),
                        (egui::Key::Num9, 90),
                    ];
                    for (k, pct) in pct_keys {
                        if input.key_pressed(k) {
                            if stats.duration > 0.0 {
                                let target = (pct as f64 / 100.0) * stats.duration;
                                player.seek_absolute(target);
                                self.osd.show_jump_percent(pct, target);
                            }
                            break;
                        }
                    }
                }
            }
        }


        if toggle_fullscreen_requested {
            self.toggle_fullscreen(ctx);
        }
        if exit_fullscreen_requested {
            self.set_fullscreen(ctx, false);
        }
        if toggle_pip_requested {
            self.toggle_pip(ctx);
        }
        if toggle_playlist_unified_requested {
            self.toggle_playlist_unified(ctx);
        }
        if let Some(factor) = window_resize_preset {
            if let Ok(ref player) = self.player {
                let stats = player.stats();
                self.resize_window_to_preset(ctx, factor, &stats);
            }
        }
        if center_window_requested {
            self.center_window_with_monitor_aspect_ratio(ctx);
        }

        if let Some(path) = open_file_path {
            self.open_media_file(path);
        }
        if let Some(folder) = open_folder_path {
            self.open_folder(folder);
        }
        if do_skip_intro_key {
            let stats = self.player.as_ref().ok().map(|p| p.stats()).unwrap_or_default();
            self.skip_intro_or_op(&stats);
        }
        if do_close_file {
            self.close_current_media(stats, ctx);
        }
    }

    pub fn handle_drag_and_drop(&mut self, ctx: &egui::Context) {
        let dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
        for file in dropped_files {
            let path = file.path().to_path_buf();
            if path.as_os_str().is_empty() {
                continue;
            }
            if path.is_dir() {
                self.open_folder(path);
            } else if is_subtitle_file(&path) {
                if let Ok(ref player) = self.player {
                    if path.to_string_lossy().ends_with(".smi") {
                        if let Ok(srt) = SamiParser::load_sami_file_as_srt(&path) {
                            let temp_srt = std::env::temp_dir().join("vortex_drag_smi.srt");
                            let _ = std::fs::write(&temp_srt, srt);
                            player.load_external_subtitle(&temp_srt.to_string_lossy());
                        }
                    } else {
                        player.load_external_subtitle(&path.to_string_lossy());
                    }
                    self.osd.show(format!("Loaded subtitle: {}", path.file_name().unwrap_or_default().to_string_lossy()), 2000);
                }
            } else if is_media_file(&path) {
                self.open_media_file(path);
            }
        }
    }

    pub fn check_ab_loop_and_resume(&mut self) {
        if let Ok(ref player) = self.player {
            let stats = player.stats();

            let is_song_mode = MusicBackgroundView::is_song(&stats);

            // Deferred seek if mpv did not jump to resume position on start (videos only)
            if !is_song_mode {
                if let Some((ref pending_path, pending_pos)) = self.pending_resume {
                    let norm_cur = stats.file_path.replace('/', "\\");
                    let norm_pen = pending_path.replace('/', "\\");
                    if norm_cur == norm_pen && stats.duration > 0.0 {
                        if (stats.time_pos - pending_pos).abs() > 2.5 && stats.time_pos < pending_pos {
                            player.seek_absolute(pending_pos);
                        }
                        self.pending_resume = None;
                    }
                }
            } else {
                self.pending_resume = None;
            }

            // A-B Loop Repeat Enforcement
            if self.bookmark_mgr.ab_loop.is_active {
                if let (Some(a), Some(b)) = (self.bookmark_mgr.ab_loop.point_a, self.bookmark_mgr.ab_loop.point_b) {
                    if stats.time_pos >= b {
                        player.seek_absolute(a);
                    }
                }
            }

            // Automatic Bookmark & Skip Interval Skipping
            if let Some((target_time, label)) = self.bookmark_mgr.should_skip(stats.time_pos) {
                player.seek_absolute(target_time);
                self.osd.show(format!("[Auto-Skip] Skipped: {} ⏭", label), 2000);
            }

            // Anime OP/ED & Intro/Outro Auto-Skip Engine
            if self.config.skip_intro_enabled && stats.duration > 0.0 {
                // If playback sought backwards by > 2.0s, reset skip throttle so re-entered skip zones can fire again
                if stats.time_pos < self.intro_last_time_pos - 2.0 {
                    self.bookmark_mgr.last_skipped_time = -100.0;
                    self.last_skipped_chapter_index = None;
                }

                // 1. Chapter-based auto-skip using saved chapter title keywords
                if self.config.skip_chapters_enabled && !stats.chapters.is_empty() {
                    let mut found_skip = None;
                    for (i, c) in stats.chapters.iter().enumerate() {
                        let c_start = c.time_pos;
                        let c_end = if let Some(next_c) = stats.chapters.get(i + 1) {
                            next_c.time_pos
                        } else if stats.duration > c_start {
                            stats.duration
                        } else {
                            c_start + 90.0
                        };

                        if c_end <= c_start {
                            continue;
                        }

                        if self.config.matches_skip_chapter(&c.title) {
                            let in_chapter = stats.time_pos >= c_start && stats.time_pos < c_end;
                            if in_chapter {
                                let crossed_into = self.intro_last_time_pos < c_start && stats.time_pos >= c_start;
                                let near_start = stats.time_pos < (c_start + 2.5).min(c_end - 0.2);
                                let not_yet_skipped = self.last_skipped_chapter_index != Some(c.index);

                                if (crossed_into || near_start) && not_yet_skipped {
                                    found_skip = Some((i, c.clone(), c_end));
                                    break;
                                }
                            }
                        }
                    }

                    if let Some((idx, ch, _)) = found_skip {
                        // Find next chapter that is NOT marked for skipping
                        let mut target_ch = None;
                        for next_c in stats.chapters.iter().skip(idx + 1) {
                            if !self.config.matches_skip_chapter(&next_c.title) {
                                target_ch = Some(next_c);
                                break;
                            }
                        }

                        let (target_pos, label) = if let Some(target) = target_ch {
                            (target.time_pos, target.title.clone())
                        } else {
                            // Last chapter or all remaining chapters are skippable (e.g. ED + Credits)
                            ((stats.duration - 0.5).max(0.0), "End of File".to_string())
                        };

                        if (self.bookmark_mgr.last_skipped_time - target_pos).abs() > 1.5 {
                            self.last_skipped_chapter_index = Some(ch.index);
                            self.bookmark_mgr.last_skipped_time = target_pos;
                            player.seek_absolute(target_pos);
                            let msg = if label == "End of File" {
                                format!("[Auto-Skip] Skipped {}: reached end of file ⏭", ch.title)
                            } else {
                                format!("[Auto-Skip] Skipped {}: jumped to {} ⏭", ch.title, label)
                            };
                            self.osd.show(msg, 2000);
                        }
                    } else {
                        // Reset tracked skip chapter once playback has moved outside it
                        if let Some(skipped_idx) = self.last_skipped_chapter_index {
                            if let Some(ch) = stats.chapters.iter().find(|c| c.index == skipped_idx) {
                                let c_end = stats.chapters.iter().find(|n| n.index == ch.index + 1)
                                    .map(|n| n.time_pos)
                                    .unwrap_or(stats.duration);
                                if stats.time_pos < ch.time_pos - 1.0 || stats.time_pos >= c_end {
                                    self.last_skipped_chapter_index = None;
                                }
                            } else {
                                self.last_skipped_chapter_index = None;
                            }
                        }
                    }
                }

                // 2. Fixed intro duration skip at start (skip_intro_at_start flag)
                if self.config.skip_intro_at_start && self.config.skip_intro_sec > 0.0 && stats.time_pos < 1.0 {
                    if (self.bookmark_mgr.last_skipped_time - self.config.skip_intro_sec).abs() > 2.0 {
                        self.bookmark_mgr.last_skipped_time = self.config.skip_intro_sec;
                        player.seek_absolute(self.config.skip_intro_sec);
                        let msg = format!("[Auto-Skip] Skipped Intro ({:.0}s) ⏭", self.config.skip_intro_sec);
                        self.osd.show(msg, 2000);
                    }
                }

                // 3. Fixed ending/outro duration skip at end (skip_ending_at_end flag)
                if self.config.skip_ending_at_end && self.config.skip_outro_sec > 0.0 && stats.duration > self.config.skip_outro_sec + 5.0 {
                    let outro_start = stats.duration - self.config.skip_outro_sec;
                    let crossed_outro = self.intro_last_time_pos < outro_start && stats.time_pos >= outro_start;
                    let near_outro_start = stats.time_pos >= outro_start && stats.time_pos < outro_start + 2.5;
                    if crossed_outro || near_outro_start {
                        if (self.bookmark_mgr.last_skipped_time - stats.duration).abs() > 2.0 {
                            self.bookmark_mgr.last_skipped_time = stats.duration;
                            player.seek_absolute((stats.duration - 0.5).max(0.0));
                            let msg = format!("[Auto-Skip] Skipped Outro ({:.0}s) ⏭", self.config.skip_outro_sec);
                            self.osd.show(msg, 2000);
                        }
                    }
                }

                // 4. Config-saved skip intervals (from Skip Setup table)
                for item in &self.config.skip_intervals {
                    if item.enabled && item.interval_type == "Skip" {
                        let end = item.start + item.length;
                        let crossed = self.intro_last_time_pos < item.start && stats.time_pos >= item.start;
                        let near = stats.time_pos >= item.start && stats.time_pos < (item.start + 2.5).min(end - 0.2);
                        if crossed || near {
                            if (self.bookmark_mgr.last_skipped_time - end).abs() > 2.0 {
                                self.bookmark_mgr.last_skipped_time = end;
                                player.seek_absolute(end);
                                let msg = format!("[Auto-Skip] Skipped interval ({} → {}) ⏭",
                                    crate::bookmark::format_time(item.start),
                                    crate::bookmark::format_time(end));
                                self.osd.show(msg, 2000);
                            }
                        }
                    }
                }
            }

            // Continuously track & save seek timestamp when position changes by > 1.5s (videos only)
            if !is_song_mode && (stats.time_pos - self.last_saved_time_pos).abs() > 1.5 && !stats.file_path.is_empty() && stats.time_pos > 1.0 {
                self.last_saved_time_pos = stats.time_pos;
                let save_pos = if stats.duration > 0.0 && stats.time_pos >= stats.duration - 8.0 {
                    0.0
                } else {
                    stats.time_pos
                };
                self.config.save_resume_time(&stats.file_path, save_pos);
                self.playback_history.update_position(&stats.file_path, stats.time_pos, stats.duration);
                let _ = self.config.save();
            }

        }
    }

    pub fn check_auto_advance(&mut self) -> Option<PathBuf> {
        if let Ok(ref player) = self.player {
            if player.is_manually_stopped() {
                self.prev_was_playing = false;
                return None;
            }
            let stats = player.stats();
            let is_now_playing = !stats.is_idle && !stats.file_path.is_empty() && stats.duration > 0.0 && !stats.eof_reached;

            if is_now_playing {
                self.prev_was_playing = true;
            } else if self.prev_was_playing && (stats.is_idle || stats.eof_reached || (stats.duration > 1.0 && stats.time_pos >= stats.duration - 0.35)) {
                self.prev_was_playing = false;

                // Move to next track in playlist (handles RepeatTrack, RepeatAll, Shuffle, and sequential progression)
                if let Some(next) = self.playlist.lock().unwrap().next() {
                    return Some(next);
                }

                // Check if sleep timer was set for end of playlist
                if self.sleep_timer.active && self.sleep_timer.trigger_on_playlist_end {
                    let action = self.sleep_timer.completion_action;
                    self.sleep_timer.cancel();
                    player.pause();
                    self.osd.show(format!("🌙 Sleep Timer Triggered (Playlist End): {}", action.label()), 3000);
                    self.sleep_timer.execute_system_action();
                }

                // If playlist has reached its end with RepeatMode::Off, apply configured end action
                match self.config.playlist_end_action {
                    PlaylistEndAction::RepeatPlaylist => {
                        if !self.playlist.lock().unwrap().items.is_empty() {
                            return self.playlist.lock().unwrap().play_index(0);
                        }
                    }
                    PlaylistEndAction::StopPlayback | PlaylistEndAction::Stop => {}
                    PlaylistEndAction::CloseApplication | PlaylistEndAction::ClosePlayer => {
                        std::process::exit(0);
                    }
                    PlaylistEndAction::SleepPC => {
                        #[cfg(windows)]
                        {
                            let _ = crate::platform::silent_command("rundll32.exe")
                                .args(["powrprof.dll,SetSuspendState", "0,1,0"])
                                .spawn();
                        }
                    }
                    PlaylistEndAction::ShutdownPC => {
                        #[cfg(windows)]
                        {
                            let _ = crate::platform::silent_command("shutdown")
                                .args(["/s", "/t", "60", "/c", "VertexPlayer: Playlist completed. System will shut down in 60 seconds."])
                                .spawn();
                        }
                    }
                    PlaylistEndAction::ShowHomeScreen => {}
                }
            } else if !self.prev_was_playing && (stats.is_idle || stats.eof_reached) {
                // If playback became idle while minimized, advance if there is a next pending item
                let mut pl = self.playlist.lock().unwrap();
                if !pl.is_empty() {
                    if let Some(cur_idx) = pl.current_index {
                        if cur_idx + 1 < pl.items.len() && stats.file_path.is_empty() {
                            self.prev_was_playing = false;
                            return pl.next();
                        }
                    }
                }
            }
        }
        None
    }
}

impl VortexApp {
    pub fn has_open_dialog(&self) -> bool {
        self.show_library_view
            || self.show_preferences
            || self.show_control_panel
            || self.show_mediainfo_dialog
            || self.show_about_dialog
            || self.show_stream_url_dialog
            || self.show_bookmark_overlay
            || self.show_peq_dialog
            || self.show_sleep_timer_dialog
            || self.show_update_dialog
            || self.show_surround_eq_dialog
            || self.show_capture_dialog
            || self.show_jump_time_dialog
            || self.bookmark_studio.is_open
            || self.subtitle_explorer.is_open
            || self.settings_inspector.is_open
            || self.input_editor.is_open
            || self.file_navigator.is_open
            || self.network_browser.is_open
            || self.gif_maker.is_open
            || self.device_capture.is_open
            || self.subtitle_lookup.is_open
            || self.record_dialog.is_open
            || self.contact_sheet_dialog.is_open
            || self.broadcast_dialog.is_open
            || self.stereo_3d_dialog.is_open
            || self.karaoke_studio.is_open
            || self.subtitle_studio.is_open
            || self.screen_capture.is_open
            || self.iptv_guide.is_open
            || self.logo_watermark.is_open
            || self.disc_nav.is_open
            || self.cd_ripper.is_open
            || self.bda_tuner.is_open
            || self.bdj_dialog.is_open
            || self.vst_winamp.is_open
            || self.detached_windows.is_open
            || self.discord_rpc.is_open
            || self.web_remote.is_open
            || self.scrobbler_dialog.is_open
            || self.media_server.is_open
            || self.ambilight_dialog.is_open
            || self.cast_renderer.is_open
            || self.transcoder_dialog.is_open
            || self.zoom_magnifier.is_open
            || self.radio_directory.is_open
            || self.video_puzzle.is_open
            || self.vr_360_studio.is_open
            || self.damaged_file_repair.is_open
            || self.boss_key_dialog.is_open
            || self.playback_history.is_open
            || self.media_tag_editor.is_open
            || self.auto_skip_dialog.is_open
    }

    #[allow(dead_code)]
    pub fn has_modal_dialog(&self) -> bool {
        self.show_preferences
            || self.show_mediainfo_dialog
            || self.show_about_dialog
            || self.show_stream_url_dialog
            || self.show_jump_time_dialog
            || self.show_capture_dialog
            || self.show_peq_dialog
            || self.chain_editor.is_open
            || self.channel_matrix_dialog.is_open
            || self.auto_skip_dialog.is_open
            || self.media_tag_editor.is_open
            || self.playback_history.is_open
            || self.damaged_file_repair.is_open
            || self.boss_key_dialog.is_open
            || self.broadcast_dialog.is_open
    }
}

#[cfg(windows)]
fn is_window_maximized_or_workarea(hwnd: isize, ctx: &egui::Context) -> bool {
    if ctx.input(|i| i.viewport().maximized.unwrap_or(false)) {
        return true;
    }
    if hwnd == 0 {
        return false;
    }
    use windows_sys::Win32::Foundation::RECT;
    use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST};
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowRect, IsZoomed};

    unsafe {
        if IsZoomed(hwnd as _) != 0 {
            return true;
        }
        let mut w_rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        if GetWindowRect(hwnd as _, &mut w_rect) != 0 {
            let hmon = MonitorFromWindow(hwnd as _, MONITOR_DEFAULTTONEAREST);
            if !hmon.is_null() {
                let mut mi: MONITORINFO = std::mem::zeroed();
                mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
                if GetMonitorInfoW(hmon, &mut mi) != 0 {
                    let w_width = w_rect.right - w_rect.left;
                    let w_height = w_rect.bottom - w_rect.top;
                    let work_width = mi.rcWork.right - mi.rcWork.left;
                    let work_height = mi.rcWork.bottom - mi.rcWork.top;
                    // Require BOTH origin and size to align with monitor work area within 12px
                    let pos_matches = (w_rect.left - mi.rcWork.left).abs() <= 12 && (w_rect.top - mi.rcWork.top).abs() <= 12;
                    let size_matches = (w_width - work_width).abs() <= 12 && (w_height - work_height).abs() <= 12;
                    if pos_matches && size_matches {
                        return true;
                    }
                }
            }
        }
        false
    }
}
#[cfg(not(windows))]
fn is_window_maximized_or_workarea(_hwnd: isize, ctx: &egui::Context) -> bool {
    ctx.input(|i| i.viewport().maximized.unwrap_or(false))
}

impl VortexApp {
    /// Center the window on the active monitor with the exact same aspect ratio as the monitor,
    /// scaled to comfortably fit without overflowing the work area in windowed mode.
    pub fn center_window_with_monitor_aspect_ratio(&mut self, ctx: &egui::Context) {
        if self.is_fullscreen {
            self.set_fullscreen(ctx, false);
        }

        #[cfg(windows)]
        if self.parent_hwnd != 0 {
            use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST};
            use windows_sys::Win32::UI::WindowsAndMessaging::{SetWindowPos, SWP_NOZORDER, SWP_FRAMECHANGED, SWP_SHOWWINDOW};

            let mut mi: MONITORINFO = unsafe { std::mem::zeroed() };
            mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            let hmon = unsafe { MonitorFromWindow(self.parent_hwnd as _, MONITOR_DEFAULTTONEAREST) };
            if !hmon.is_null() && unsafe { GetMonitorInfoW(hmon, &mut mi) } != 0 {
                let mon_w = (mi.rcMonitor.right - mi.rcMonitor.left).max(1) as f32;
                let mon_h = (mi.rcMonitor.bottom - mi.rcMonitor.top).max(1) as f32;
                let mon_ar = mon_w / mon_h;

                let work_w = (mi.rcWork.right - mi.rcWork.left).max(400) as f32;
                let work_h = (mi.rcWork.bottom - mi.rcWork.top).max(300) as f32;

                // Scale to 75% of work area while preserving the exact monitor aspect ratio
                let target_scale = 0.75;
                let mut win_w = (work_w * target_scale).round();
                let mut win_h = (win_w / mon_ar).round();

                if win_h > work_h * target_scale {
                    win_h = (work_h * target_scale).round();
                    win_w = (win_h * mon_ar).round();
                }

                // Clamped so it strictly never overflows the monitor work area
                win_w = win_w.min(work_w - 40.0).max(480.0);
                win_h = win_h.min(work_h - 40.0).max(320.0);

                let left = mi.rcWork.left + ((work_w - win_w) / 2.0).round() as i32;
                let top = mi.rcWork.top + ((work_h - win_h) / 2.0).round() as i32;

                unsafe {
                    SetWindowPos(
                        self.parent_hwnd as _,
                        0 as _,
                        left,
                        top,
                        win_w as i32,
                        win_h as i32,
                        SWP_NOZORDER | SWP_FRAMECHANGED | SWP_SHOWWINDOW,
                    );
                }

                let ppp = ctx.pixels_per_point().max(0.1);
                let log_w = win_w / ppp;
                let log_h = win_h / ppp;
                let log_left = left as f32 / ppp;
                let log_top = top as f32 / ppp;

                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(log_w, log_h)));
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(Pos2::new(log_left, log_top)));

                self.last_windowed_rect = Some(Rect::from_min_size(
                    Pos2::new(log_left, log_top),
                    Vec2::new(log_w, log_h),
                ));
                self.config.window_width = log_w;
                self.config.window_height = log_h;
                let _ = self.config.save();
                self.osd.show(format!("Window Centered ({:.0}×{:.0}, {:.2}:1)", win_w, win_h, mon_ar), 1500);
            }
        }
    }

    pub fn resize_window_to_preset(&mut self, ctx: &egui::Context, factor: f32, stats: &MediaStats) {
        if self.is_fullscreen {
            self.set_fullscreen(ctx, false);
        }
        let vw = if stats.video_width > 0 { stats.video_width as f32 } else { 1280.0 };
        let vh = if stats.video_height > 0 { stats.video_height as f32 } else { 720.0 };
        let is_docked_pl = self.show_playlist && !self.config.playlist_detached;
        let extra_w = if is_docked_pl { 330.0 } else { 0.0 };
        let target_w = (vw * factor + extra_w).max(480.0);
        let target_h = (vh * factor).max(270.0);

        #[cfg(windows)]
        if self.parent_hwnd != 0 {
            use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST};
            use windows_sys::Win32::UI::WindowsAndMessaging::{SetWindowPos, SWP_NOZORDER, SWP_FRAMECHANGED, SWP_SHOWWINDOW};

            let mut mi: MONITORINFO = unsafe { std::mem::zeroed() };
            mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            let hmon = unsafe { MonitorFromWindow(self.parent_hwnd as _, MONITOR_DEFAULTTONEAREST) };
            if !hmon.is_null() && unsafe { GetMonitorInfoW(hmon, &mut mi) } != 0 {
                let work_w = (mi.rcWork.right - mi.rcWork.left).max(400) as f32;
                let work_h = (mi.rcWork.bottom - mi.rcWork.top).max(300) as f32;

                let ppp = ctx.pixels_per_point().max(0.1);
                let mut pw = (target_w * ppp).round();
                let mut ph = (target_h * ppp).round();

                // Clamp to monitor work area so it NEVER overflows the monitor!
                if pw > work_w - 40.0 || ph > work_h - 40.0 {
                    let scale = ((work_w - 40.0) / pw).min((work_h - 40.0) / ph);
                    pw = (pw * scale).round();
                    ph = (ph * scale).round();
                }

                // Center in monitor work area
                let left = mi.rcWork.left + ((work_w - pw) / 2.0).round() as i32;
                let top = mi.rcWork.top + ((work_h - ph) / 2.0).round() as i32;

                unsafe {
                    SetWindowPos(self.parent_hwnd as _, 0 as _, left, top, pw as i32, ph as i32, SWP_NOZORDER | SWP_FRAMECHANGED | SWP_SHOWWINDOW);
                }

                let log_w = pw / ppp;
                let log_h = ph / ppp;
                let log_left = left as f32 / ppp;
                let log_top = top as f32 / ppp;

                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(log_w, log_h)));
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(Pos2::new(log_left, log_top)));

                self.last_windowed_rect = Some(Rect::from_min_size(
                    Pos2::new(log_left, log_top),
                    Vec2::new(log_w, log_h),
                ));
                self.config.window_width = log_w;
                self.config.window_height = log_h;
                let _ = self.config.save();
            }
        }
        #[cfg(not(windows))]
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(target_w, target_h)));

        let pct = (factor * 100.0).round() as i32;
        self.osd.show(format!("Window Size: {}% ({:.0}×{:.0})", pct, target_w - extra_w, target_h), 1500);
    }

    /// Toggle the playlist panel, and if in docked (non-detached) mode and the window is not
    /// maximized/fullscreen, automatically expand/contract the window width by 330px so the video
    /// area is preserved at the same size — Vortex / MPC-HC style.
    pub fn toggle_playlist_unified(&mut self, ctx: &egui::Context) {
        let opening = !self.show_playlist;
        self.show_playlist = opening;

        // Only resize when docked and windowed (not fullscreen, not maximized)
        ctx.request_repaint();
    }


    pub fn toggle_maximize_window(&mut self, ctx: &egui::Context) {
        self.last_maximize_time = std::time::Instant::now();
        if self.is_fullscreen {
            self.set_fullscreen(ctx, false);
            return;
        }
        let is_max = is_window_maximized_or_workarea(self.parent_hwnd, ctx);
        if is_max {
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
            #[cfg(windows)]
            if self.parent_hwnd != 0 {
                use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_RESTORE};
                unsafe {
                    ShowWindow(self.parent_hwnd as _, SW_RESTORE);
                    crate::platform::apply_border_suppression(self.parent_hwnd as _);
                }
            }
        } else {
            #[cfg(windows)]
            if self.parent_hwnd != 0 {
                use windows_sys::Win32::Foundation::RECT;
                use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST};
                use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowRect, SetWindowPos, ShowWindow, SW_MAXIMIZE, SWP_FRAMECHANGED, SWP_NOZORDER, SWP_SHOWWINDOW};

                let mut cur_rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                if unsafe { GetWindowRect(self.parent_hwnd as _, &mut cur_rc) } != 0 && (cur_rc.right - cur_rc.left) >= 200 && (cur_rc.bottom - cur_rc.top) >= 100 {
                    let ppp = ctx.pixels_per_point().max(0.1);
                    self.last_windowed_rect = Some(Rect::from_min_size(
                        Pos2::new(cur_rc.left as f32 / ppp, cur_rc.top as f32 / ppp),
                        Vec2::new((cur_rc.right - cur_rc.left) as f32 / ppp, (cur_rc.bottom - cur_rc.top) as f32 / ppp),
                    ));
                }

                let h_mon = unsafe { MonitorFromWindow(self.parent_hwnd as _, MONITOR_DEFAULTTONEAREST) };
                let mut mi: MONITORINFO = unsafe { std::mem::zeroed() };
                mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
                let has_mon = unsafe { GetMonitorInfoW(h_mon, &mut mi) } != 0;

                if crate::platform::is_taskbar_autohide() && has_mon {
                    let full_w = mi.rcMonitor.right - mi.rcMonitor.left;
                    let full_h = mi.rcMonitor.bottom - mi.rcMonitor.top;
                    unsafe {
                        SetWindowPos(
                            self.parent_hwnd as _,
                            0 as _,
                            mi.rcMonitor.left,
                            mi.rcMonitor.top,
                            full_w,
                            full_h - 1,
                            SWP_NOZORDER | SWP_SHOWWINDOW | SWP_FRAMECHANGED,
                        );
                    }
                } else {
                    unsafe {
                        ShowWindow(self.parent_hwnd as _, SW_MAXIMIZE);
                    }
                }
                unsafe {
                    crate::platform::apply_border_suppression(self.parent_hwnd as _);
                }
            }
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
        }
        ctx.request_repaint();
    }

    pub fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        let next = !self.is_fullscreen;
        self.set_fullscreen(ctx, next);
    }

    pub fn set_fullscreen(&mut self, ctx: &egui::Context, fullscreen: bool) {
        if self.is_fullscreen != fullscreen {
            if fullscreen {
                if self.pip_mode.is_pip {
                    self.toggle_pip(ctx);
                }
                // Save current window state before entering fullscreen
                #[cfg(windows)]
                let is_max = if self.parent_hwnd != 0 {
                    unsafe { windows_sys::Win32::UI::WindowsAndMessaging::IsZoomed(self.parent_hwnd as _) != 0 }
                } else {
                    is_window_maximized_or_workarea(self.parent_hwnd, ctx)
                };
                #[cfg(not(windows))]
                let is_max = is_window_maximized_or_workarea(self.parent_hwnd, ctx);

                self.pre_fullscreen_maximized = is_max;

                #[cfg(windows)]
                if self.parent_hwnd != 0 && !is_max {
                    use windows_sys::Win32::Foundation::RECT;
                    use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowRect;
                    let mut rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                    if unsafe { GetWindowRect(self.parent_hwnd as _, &mut rc) } != 0 && (rc.right - rc.left) >= 200 && (rc.bottom - rc.top) >= 100 {
                        self.pre_fullscreen_win32_rect = Some(rc);
                        let ppp = ctx.pixels_per_point().max(0.1);
                        let r = Rect::from_min_size(
                            Pos2::new(rc.left as f32 / ppp, rc.top as f32 / ppp),
                            Vec2::new((rc.right - rc.left) as f32 / ppp, (rc.bottom - rc.top) as f32 / ppp),
                        );
                        self.pre_fullscreen_rect = Some(r);
                        self.last_windowed_rect = Some(r);
                    }
                }
                #[cfg(not(windows))]
                if !is_max {
                    if let Some(r) = ctx.input(|i| i.viewport().outer_rect.or(i.viewport().inner_rect).or(i.raw.screen_rect)) {
                        self.pre_fullscreen_rect = Some(r);
                        self.last_windowed_rect = Some(r);
                    }
                }

                self.is_fullscreen = true;
                self.is_top_hovered = false;
                self.is_bottom_hovered = false;
                self.last_fullscreen_change = std::time::Instant::now();
                self.last_mouse_activity = std::time::Instant::now() - std::time::Duration::from_secs(10);
                #[cfg(not(windows))]
                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
                #[cfg(windows)]
                if self.parent_hwnd != 0 {
                    reset_child_geom_cache();
                    set_native_fullscreen(self.parent_hwnd, self.child_hwnd, true, false, self.pre_fullscreen_win32_rect, self.config.always_on_top);
                } else {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
                }
            } else {
                // Leaving fullscreen
                self.is_fullscreen = false;
                self.is_top_hovered = false;
                self.is_bottom_hovered = false;
                self.last_fullscreen_change = std::time::Instant::now();
                #[cfg(not(windows))]
                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
                #[cfg(windows)]
                if self.parent_hwnd != 0 {
                    reset_child_geom_cache();
                    set_native_fullscreen(self.parent_hwnd, self.child_hwnd, false, self.pre_fullscreen_maximized, self.pre_fullscreen_win32_rect, self.config.always_on_top);
                } else {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
                }
                self.restore_frames_pending = 0;
                #[cfg(not(windows))]
                {
                    if self.pre_fullscreen_maximized {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
                    } else if let Some(target) = self.pre_fullscreen_rect.or(self.last_windowed_rect) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(target.min.into()));
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(target.size()));
                    }
                }
                #[cfg(windows)]
                if self.parent_hwnd == 0 {
                    if self.pre_fullscreen_maximized {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
                    } else if let Some(target) = self.pre_fullscreen_rect.or(self.last_windowed_rect) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(target.min.into()));
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(target.size()));
                    }
                }
            }

            if let Ok(ref player) = self.player {
                let align = if self.is_fullscreen { "center" } else { &self.config.video_align_y };
                player.set_video_align_y(align);
            }
            ctx.request_repaint();
            self.osd.show(if self.is_fullscreen { "Fullscreen".to_string() } else { "Windowed".to_string() }, 1200);
        }
    }

    pub fn toggle_pip(&mut self, ctx: &egui::Context) {
        if self.is_fullscreen {
            self.set_fullscreen(ctx, false);
        }
        self.pip_mode.is_pip = !self.pip_mode.is_pip;
        let is_pip = self.pip_mode.is_pip;
        #[cfg(windows)]
        if self.parent_hwnd != 0 {
            set_native_pip(self.parent_hwnd, is_pip);
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
            if is_pip { egui::WindowLevel::AlwaysOnTop } else { egui::WindowLevel::Normal }
        ));
        self.osd.show(if is_pip { "Picture-in-Picture: ON".to_string() } else { "Picture-in-Picture: OFF".to_string() }, 1200);
    }

    pub fn close_current_media(&mut self, stats: &mut MediaStats, ctx: &egui::Context) {
        if let Ok(ref player) = self.player {
            if !stats.file_path.is_empty() && stats.time_pos > 1.0 {
                let save_pos = if stats.duration > 0.0 && stats.time_pos >= stats.duration - 8.0 { 0.0 } else { stats.time_pos };
                self.config.save_resume_time(&stats.file_path, save_pos);
                let _ = self.config.save();
            }
            self.prev_was_playing = false;
            player.close_media();
            self.bookmark_mgr.set_video(None);
            self.playlist.lock().unwrap().current_index = None;
            self.taskbar.clear_progress(self.parent_hwnd);
            self.osd.show("✕ Closed Media (Home Screen)".to_string(), 1200);

            // Immediately reset local stats so current frame renders home screen & hides child video window
            stats.is_idle = true;
            stats.file_path.clear();
            stats.title.clear();
            stats.artist.clear();
            stats.album.clear();
            stats.duration = 0.0;
            stats.time_pos = 0.0;
            stats.percent_pos = 0.0;
            stats.video_width = 0;
            stats.video_height = 0;
            stats.chapters.clear();
            self.intro_segment_key = None;
            self.intro_btn_shown_at = None;
            self.intro_last_time_pos = 0.0;
            self.last_skipped_chapter_index = None;

            ctx.request_repaint();
        }
    }

    pub fn skip_intro_or_op(&mut self, stats: &MediaStats) {
        self.intro_btn_shown_at = None;
        if let Ok(ref player) = self.player {
            // 1. If currently inside a chapter, skip forward to next non-skip chapter or end of video!
            if !stats.chapters.is_empty() {
                let current_ch_idx = stats.chapters.iter().position(|c| {
                    if let Some(next_c) = stats.chapters.iter().find(|n| n.index == c.index + 1) {
                        stats.time_pos >= c.time_pos && stats.time_pos < next_c.time_pos
                    } else {
                        stats.time_pos >= c.time_pos
                    }
                });

                if let Some(idx) = current_ch_idx {
                    let cur = &stats.chapters[idx];
                    // Look ahead for the next chapter that is NOT a skip chapter
                    let mut target_ch = None;
                    for n in stats.chapters.iter().skip(idx + 1) {
                        if !self.config.matches_skip_chapter(&n.title) {
                            target_ch = Some(n);
                            break;
                        }
                    }

                    if let Some(next_c) = target_ch {
                        player.seek_absolute(next_c.time_pos);
                        let msg = format!("Skipped to: {} ⏭", next_c.title);
                        self.osd.show(msg, 1500);
                        return;
                    } else {
                        // Current chapter is the last chapter or all subsequent are skip chapters
                        let target_pos = (stats.duration - 0.5).max(0.0);
                        if stats.time_pos < target_pos - 1.0 {
                            player.seek_absolute(target_pos);
                            let msg = format!("Skipped {}: jumped to end ⏭", cur.title);
                            self.osd.show(msg, 1500);
                            return;
                        }
                    }
                }
            }

            // 2. If configured intro duration
            if self.config.skip_intro_sec > 0.0 && stats.time_pos < self.config.skip_intro_sec {
                player.seek_absolute(self.config.skip_intro_sec);
                let msg = format!("Skipped Intro ({:.0}s) ⏭", self.config.skip_intro_sec);
                self.osd.show(msg, 1500);
                return;
            }

            // 3. Standard Anime OP duration (85s)
            player.seek_relative(85.0);
            self.osd.show("Skipped 85s (Anime OP) ⏭".to_string(), 1500);
        }
    }

    /// Render invisible 8-way resize handles along all window edges and corners & display pixel dimensions OSD (Vortex style)
    fn handle_window_resizing(&mut self, ctx: &egui::Context, stats: &MediaStats) {
        if self.restore_frames_pending > 0 {
            self.restore_frames_pending -= 1;
            if self.pre_fullscreen_maximized {
                ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
            } else if let Some(target) = self.pre_fullscreen_rect {
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(target.min.into()));
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(target.size()));
            }
            ctx.request_repaint();
        }

        #[cfg(windows)]
        let is_max = if self.parent_hwnd != 0 {
            unsafe { windows_sys::Win32::UI::WindowsAndMessaging::IsZoomed(self.parent_hwnd as _) != 0 }
        } else {
            is_window_maximized_or_workarea(self.parent_hwnd, ctx)
        };
        #[cfg(not(windows))]
        let is_max = is_window_maximized_or_workarea(self.parent_hwnd, ctx);

        if !self.is_fullscreen && !self.pip_mode.is_pip && !is_max && self.restore_frames_pending == 0 {
            #[cfg(windows)]
            let cur_rect = if self.parent_hwnd != 0 {
                use windows_sys::Win32::Foundation::RECT;
                use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowRect;
                let mut rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                if unsafe { GetWindowRect(self.parent_hwnd as _, &mut rc) } != 0 && (rc.right - rc.left) >= 200 && (rc.bottom - rc.top) >= 100 {
                    self.pre_fullscreen_win32_rect = Some(rc);
                    let ppp = ctx.pixels_per_point();
                    Some(Rect::from_min_size(
                        Pos2::new(rc.left as f32 / ppp, rc.top as f32 / ppp),
                        Vec2::new((rc.right - rc.left) as f32 / ppp, (rc.bottom - rc.top) as f32 / ppp),
                    ))
                } else {
                    None
                }
            } else {
                None
            };
            #[cfg(not(windows))]
            let cur_rect: Option<Rect> = None;

            let cur_rect = cur_rect.or_else(|| {
                ctx.input(|i| {
                    i.viewport().outer_rect
                        .or(i.viewport().inner_rect)
                        .or(i.raw.screen_rect)
                })
            });

            if let Some(r) = cur_rect {
                if r.width() >= 200.0 && r.height() >= 100.0 {
                    self.last_windowed_rect = Some(r);
                    self.config.window_width = r.width();
                    self.config.window_height = r.height();
                }
            }
        }

        let screen = get_window_client_rect(ctx);
        let ppp = ctx.pixels_per_point();

        #[cfg(windows)]
        let (phys_w, phys_h) = if self.parent_hwnd != 0 {
            use windows_sys::Win32::Foundation::RECT;
            use windows_sys::Win32::UI::WindowsAndMessaging::GetClientRect;
            let mut rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
            unsafe { GetClientRect(self.parent_hwnd as _, &mut rc) };
            let w = (rc.right - rc.left).max(1) as u32;
            let h = (rc.bottom - rc.top).max(1) as u32;
            (w, h)
        } else {
            (((screen.width() * ppp).round() as u32).max(1), ((screen.height() * ppp).round() as u32).max(1))
        };
        #[cfg(not(windows))]
        let (phys_w, phys_h) = {
            (((screen.width() * ppp).round() as u32).max(1), ((screen.height() * ppp).round() as u32).max(1))
        };

        let cur_size = Vec2::new(phys_w as f32, phys_h as f32);

        if let Some(last_size) = self.last_viewport_size {
            if (cur_size.x - last_size.x).abs() >= 1.0 || (cur_size.y - last_size.y).abs() >= 1.0 {
                if !self.is_fullscreen && !ctx.input(|i| i.viewport().maximized.unwrap_or(false)) {
                    self.osd.show_resize(phys_w, phys_h, stats.video_width, stats.video_height);
                    ctx.request_repaint();
                }
                self.last_viewport_size = Some(cur_size);
            }
        } else {
            self.last_viewport_size = Some(cur_size);
        }

        #[cfg(windows)]
        if self.parent_hwnd != 0 {
            // Windows native WM_NCHITTEST in taskbar.rs handles all borders and corners natively with 0 latency
            return;
        }

        let is_max = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        if is_max {
            return;
        }

        let egui_size = ctx.input(|i| {
            i.viewport().inner_rect
                .or(i.raw.screen_rect)
                .map(|r| r.size())
        });
        let width = if let Some(sz) = egui_size { screen.width().min(sz.x) } else { screen.width() };
        let height = if let Some(sz) = egui_size { screen.height().min(sz.y) } else { screen.height() };

        let border_size = 6.0;
        let border_size_top = 3.0; // Minimal 3px top sliver on Linux so titlebar dragging and buttons are never blocked
        let corner_size = 10.0;
        let corner_size_top = 4.0;

        use egui::viewport::ResizeDirection;

        let handles: &[(Rect, ResizeDirection, egui::CursorIcon)] = &[
            // 4 Corners
            (Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(corner_size, corner_size_top)), ResizeDirection::NorthWest, egui::CursorIcon::ResizeNorthWest),
            (Rect::from_min_size(Pos2::new(width - corner_size, 0.0), Vec2::new(corner_size, corner_size_top)), ResizeDirection::NorthEast, egui::CursorIcon::ResizeNorthEast),
            (Rect::from_min_size(Pos2::new(0.0, height - corner_size), Vec2::new(corner_size, corner_size)), ResizeDirection::SouthWest, egui::CursorIcon::ResizeSouthWest),
            (Rect::from_min_size(Pos2::new(width - corner_size, height - corner_size), Vec2::new(corner_size, corner_size)), ResizeDirection::SouthEast, egui::CursorIcon::ResizeSouthEast),
            // 4 Edges
            (Rect::from_min_size(Pos2::new(corner_size, 0.0), Vec2::new(width - 2.0 * corner_size, border_size_top)), ResizeDirection::North, egui::CursorIcon::ResizeNorth),
            (Rect::from_min_size(Pos2::new(corner_size, height - border_size), Vec2::new(width - 2.0 * corner_size, border_size)), ResizeDirection::South, egui::CursorIcon::ResizeSouth),
            (Rect::from_min_size(Pos2::new(0.0, corner_size_top), Vec2::new(border_size, height - corner_size_top - corner_size)), ResizeDirection::West, egui::CursorIcon::ResizeWest),
            (Rect::from_min_size(Pos2::new(width - border_size, corner_size_top), Vec2::new(border_size, height - corner_size_top - corner_size)), ResizeDirection::East, egui::CursorIcon::ResizeEast),
        ];

        if let Some(pos) = ctx.pointer_hover_pos().or_else(|| ctx.pointer_latest_pos()) {
            for (rect, dir, cursor) in handles {
                if rect.contains(pos) {
                    ctx.set_cursor_icon(*cursor);
                    if ctx.input(|inp| inp.pointer.button_pressed(egui::PointerButton::Primary)) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(*dir));
                    }
                    break;
                }
            }
        }
    }

    pub fn handle_ipc_commands(&mut self, ctx: &egui::Context, file_to_open: &mut Option<PathBuf>, stats: &MediaStats) {
        while let Some(cmd) = self.ipc_server.poll_command() {
            match cmd {
                crate::engine::IpcCommand::Play => {
                    if let Ok(ref p) = self.player {
                        p.play();
                        self.osd.show("▶ Play".to_string(), 1200);
                    }
                }
                crate::engine::IpcCommand::Pause => {
                    if let Ok(ref p) = self.player {
                        p.pause();
                        self.osd.show("⏸ Pause".to_string(), 1200);
                    }
                }
                crate::engine::IpcCommand::TogglePause => {
                    if let Ok(ref p) = self.player {
                        p.toggle_pause();
                        self.osd.show_play_pause(!stats.is_paused);
                    }
                }
                crate::engine::IpcCommand::Stop => {
                    if let Ok(ref p) = self.player {
                        p.stop();
                        self.osd.show("⏹ Stop".to_string(), 1200);
                    }
                }
                crate::engine::IpcCommand::SeekRelative(offset) => {
                    if let Ok(ref p) = self.player {
                        p.seek_relative(offset);
                        self.osd.show_seek(stats.time_pos + offset, stats.duration, Some(offset));
                    }
                }
                crate::engine::IpcCommand::SeekAbsolute(pos) => {
                    if let Ok(ref p) = self.player {
                        p.seek_absolute(pos);
                        self.osd.show_seek(pos, stats.duration, None);
                    }
                }
                crate::engine::IpcCommand::SeekPercent(pct) => {
                    if let Ok(ref p) = self.player {
                        p.seek_percent(pct);
                        let dur = stats.duration;
                        let target_time = dur * (pct / 100.0);
                        self.osd.show_seek(target_time, dur, None);
                    }
                }
                crate::engine::IpcCommand::SetVolume(vol) => {
                    if let Ok(ref p) = self.player {
                        let clamped = vol.clamp(0.0, self.config.volume_boost_limit);
                        p.set_volume(clamped);
                        self.config.volume = clamped;
                        self.osd.show_volume(clamped, false);
                    }
                }
                crate::engine::IpcCommand::ToggleMute => {
                    if let Ok(ref p) = self.player {
                        p.toggle_mute();
                        self.osd.show_volume(stats.volume, stats.is_muted);
                    }
                }
                crate::engine::IpcCommand::ToggleFullscreen => {
                    self.toggle_fullscreen(ctx);
                }
                crate::engine::IpcCommand::SetSpeed(speed) => {
                    if let Ok(ref p) = self.player {
                        let clamped = speed.clamp(0.1, 4.0);
                        p.set_speed(clamped);
                        self.osd.show(format!("Speed: {:.2}x", clamped), 1200);
                    }
                }
                crate::engine::IpcCommand::OpenFile(path_str) => {
                    let pb = PathBuf::from(path_str);
                    self.playlist.lock().unwrap().clear();
                    *file_to_open = Some(pb);
                }
                crate::engine::IpcCommand::OpenUrl(url_str) => {
                    if let Ok(ref p) = self.player {
                        p.load_file(&url_str);
                        p.play();
                        self.toast.info(format!("Stream: {}", url_str));
                    }
                }
                crate::engine::IpcCommand::NextTrack => {
                    if let Some(next_p) = self.playlist.lock().unwrap().next() {
                        *file_to_open = Some(next_p);
                    }
                }
                crate::engine::IpcCommand::PrevTrack => {
                    if let Some(prev_p) = self.playlist.lock().unwrap().previous() {
                        *file_to_open = Some(prev_p);
                    }
                }
                crate::engine::IpcCommand::CycleAudio => {
                    if let Ok(ref p) = self.player {
                        p.cycle_audio_track();
                        self.osd.show("Audio Track Cycled".to_string(), 1200);
                    }
                }
                crate::engine::IpcCommand::CycleSubtitle => {
                    if let Ok(ref p) = self.player {
                        p.cycle_subtitle_track();
                        self.osd.show("Subtitle Track Cycled".to_string(), 1200);
                    }
                }
                crate::engine::IpcCommand::TakeScreenshot => {
                    if let Ok(ref p) = self.player {
                        let pic_dir = dirs::picture_dir().unwrap_or_else(|| PathBuf::from("."));
                        let filename = format!("Vortex_{}.png", chrono::Local::now().format("%Y%m%d_%H%M%S"));
                        let out_path = pic_dir.join(filename);
                        p.take_screenshot(&out_path);
                        self.toast.success("Snapshot saved");
                    }
                }
                crate::engine::IpcCommand::StartRecording(path_str) => {
                    if let Ok(ref p) = self.player {
                        p.start_stream_recording(&path_str);
                        self.osd.show("🔴 Recording Started".to_string(), 1500);
                    }
                }
                crate::engine::IpcCommand::StopRecording => {
                    if let Ok(ref p) = self.player {
                        p.stop_stream_recording();
                        self.toast.info("Recording saved");
                    }
                }
            }
        }

        if self.frame_counter % 10 == 0 && self.player.is_ok() {
            let audio_name = stats.audio_tracks.iter().find(|t| t.is_selected).map(|t| if t.title.is_empty() { format!("Track {}", t.id) } else { t.title.clone() }).unwrap_or_else(|| "Default".to_string());
            let sub_name = stats.subtitle_tracks.iter().find(|t| t.is_selected).map(|t| if t.title.is_empty() { format!("Track {}", t.id) } else { t.title.clone() }).unwrap_or_else(|| "Off".to_string());
            crate::engine::web_remote::update_shared_status(crate::engine::web_remote::WebPlayerStatus {
                status: "ok".to_string(),
                title: if stats.title.is_empty() { "VertexPlayer Ready".to_string() } else { stats.title.clone() },
                duration: stats.duration,
                time_pos: stats.time_pos,
                percent_pos: stats.percent_pos,
                is_paused: stats.is_paused,
                volume: stats.volume,
                is_muted: stats.is_muted,
                speed: stats.speed,
                audio_track: audio_name,
                subtitle_track: sub_name,
            });
        }
    }
}

impl eframe::App for VortexApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        if let Ok(ref player) = self.player {
            let stats = player.stats();
            let is_song = MusicBackgroundView::is_song(&stats);
            if !stats.is_idle && !stats.file_path.is_empty() && !is_song {
                return [0.0, 0.0, 0.0, 1.0];
            }
        }
        [18.0 / 255.0, 19.0 / 255.0, 24.0 / 255.0, 1.0]
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        static FRAME_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let f_cnt = FRAME_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if f_cnt < 5 {
            crate::log_step(&format!("VortexApp::ui frame {} entered", f_cnt + 1));
        }
        let ctx = ui.ctx().clone();

        // Setup single window handles and SMTC on initial render
        if !self.attached_hwnd {
            crate::log_step("10a. !self.attached_hwnd entered");
            if let Ok(ref player) = self.player {
                crate::log_step("10b. player is Ok");
                let _ = player.ensure_render_context(&ctx);
                if let Ok(handle) = frame.window_handle() {
                    crate::log_step("10c. got window_handle");
                    match handle.as_raw() {
                        RawWindowHandle::Win32(win32_handle) => {
                            let parent_hwnd = win32_handle.hwnd.get();
                            crate::log_step(&format!("10d. parent_hwnd = {}", parent_hwnd));
                            self.parent_hwnd = parent_hwnd;
                            self.child_hwnd = 0;
                            #[cfg(windows)]
                            PARENT_HWND.store(parent_hwnd, std::sync::atomic::Ordering::Relaxed);
                            enable_native_borderless_resizing(parent_hwnd);
                            crate::log_step("10e. enable_native_borderless_resizing done");

                            self.smtc.init(parent_hwnd);
                            crate::log_step("10h. smtc.init done");
                            self.taskbar.init_thumbbar(parent_hwnd);
                            self.attached_hwnd = true;
                            if !self.has_initial_centered && !self.is_fullscreen {
                                self.center_window_with_monitor_aspect_ratio(&ctx);
                                self.has_initial_centered = true;
                            }
                        }
                        RawWindowHandle::Xlib(xlib_handle) => {
                            let parent_wid = xlib_handle.window as isize;
                            self.parent_hwnd = parent_wid;
                            self.child_hwnd = 0;
                            self.attached_hwnd = true;
                        }
                        RawWindowHandle::Xcb(xcb_handle) => {
                            let parent_wid = xcb_handle.window.get() as isize;
                            self.parent_hwnd = parent_wid;
                            self.child_hwnd = 0;
                            self.attached_hwnd = true;
                        }
                        RawWindowHandle::Wayland(wayland_handle) => {
                            let wid = wayland_handle.surface.as_ptr() as isize;
                            self.parent_hwnd = wid;
                            self.child_hwnd = 0;
                            self.attached_hwnd = true;
                        }
                        RawWindowHandle::AppKit(appkit_handle) => {
                            let wid = appkit_handle.ns_view.as_ptr() as isize;
                            self.parent_hwnd = wid;
                            self.child_hwnd = 0;
                            self.attached_hwnd = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        self.frame_counter = self.frame_counter.wrapping_add(1);
        self.active_popup_rects.clear();
        ctx.data_mut(|d| d.insert_temp(egui::Id::new("all_active_submenu_rects"), Vec::<Rect>::new()));

        // Single frame stats already computed above

        let mut file_to_open: Option<PathBuf> = self.check_auto_advance().or_else(|| self.pending_initial_file.take());

        // Process completed asynchronous background folder scanning
        if let Some(ref rx) = self.folder_scan_receiver {
            ctx.request_repaint();
            match rx.try_recv() {
                Ok(media_files) => {
                    if !media_files.is_empty() {
                        let count = media_files.len();
                        let first = media_files[0].clone();
                        {
                            let mut pl = self.playlist.lock().unwrap();
                            pl.clear();
                            pl.add_items(media_files);
                            pl.current_index = Some(0);
                            self.config.last_playlist = pl.items.iter().map(|i| i.path.to_string_lossy().to_string()).collect();
                            self.config.last_playlist_index = Some(0);
                            let _ = self.config.save();
                        }
                        file_to_open = Some(first);
                        let msg = format!("Loaded {} files from folder", count);
                        self.osd.show(msg, 2500);
                    } else {
                        self.osd.show("No supported media files found in folder".to_string(), 2500);
                    }
                    self.folder_scan_receiver = None;
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.folder_scan_receiver = None;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
            }
        }

        // Process completed asynchronous background neighboring episodes scanning
        if let Some(ref rx) = self.neighbor_scan_receiver {
            ctx.request_repaint();
            match rx.try_recv() {
                Ok(media_files) => {
                    if !media_files.is_empty() {
                        let mut pl = self.playlist.lock().unwrap();
                        let current_path = pl.current_item().map(|it| it.path.clone());
                        for f in media_files {
                            if !pl.items.iter().any(|i| i.path == f) {
                                pl.items.push(crate::playlist::PlaylistItem::from_path(f));
                            }
                        }
                        pl.sort_natural();
                        if let Some(ref cp) = current_path {
                            pl.current_index = pl.items.iter().position(|i| &i.path == cp);
                        }
                        self.config.last_playlist = pl.items.iter().map(|i| i.path.to_string_lossy().to_string()).collect();
                        self.config.last_playlist_index = pl.current_index;
                        let _ = self.config.save();
                    }
                    self.neighbor_scan_receiver = None;
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.neighbor_scan_receiver = None;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
            }
        }

        let mut stats = match self.player {
            Ok(ref p) => p.stats(),
            Err(_) => MediaStats::default(),
        };

        self.handle_ipc_commands(&ctx, &mut file_to_open, &stats);

        // Process OS-wide Global System Hotkeys (Background Multimedia Keys)
        for hk_evt in self.global_hotkeys.poll_events() {
            if let Ok(ref player) = self.player {
                match hk_evt {
                    crate::platform::GlobalHotkeyEvent::PlayPause => {
                        player.toggle_pause();
                        self.osd.show_play_pause(!stats.is_paused);
                    }
                    crate::platform::GlobalHotkeyEvent::Next => {
                        if let Some(next_f) = self.playlist.lock().unwrap().next() {
                            file_to_open = Some(next_f);
                            self.osd.show("Next Track".to_string(), 1200);
                        }
                    }
                    crate::platform::GlobalHotkeyEvent::Previous => {
                        if let Some(prev_f) = self.playlist.lock().unwrap().previous() {
                            file_to_open = Some(prev_f);
                            self.osd.show("Previous Track".to_string(), 1200);
                        }
                    }
                    crate::platform::GlobalHotkeyEvent::Stop => {
                        player.stop();
                        self.osd.show("Stopped".to_string(), 1200);
                    }
                    crate::platform::GlobalHotkeyEvent::VolumeUp => {
                        let cur_vol = stats.volume;
                        let next_vol = (cur_vol + 5.0).clamp(0.0, self.config.volume_boost_limit);
                        player.set_volume(next_vol);
                        self.config.volume = next_vol;
                        self.osd.show_volume(next_vol, false);
                    }
                    crate::platform::GlobalHotkeyEvent::VolumeDown => {
                        let cur_vol = stats.volume;
                        let next_vol = (cur_vol - 5.0).clamp(0.0, self.config.volume_boost_limit);
                        player.set_volume(next_vol);
                        self.config.volume = next_vol;
                        self.osd.show_volume(next_vol, false);
                    }
                    crate::platform::GlobalHotkeyEvent::Mute => {
                        player.toggle_mute();
                        self.osd.show_volume(stats.volume, !stats.is_muted);
                    }
                    crate::platform::GlobalHotkeyEvent::BringToFront => {
                        #[cfg(windows)]
                        if self.parent_hwnd != 0 {
                            unsafe {
                                windows_sys::Win32::UI::WindowsAndMessaging::ShowWindow(self.parent_hwnd as _, windows_sys::Win32::UI::WindowsAndMessaging::SW_RESTORE);
                                windows_sys::Win32::UI::WindowsAndMessaging::SetForegroundWindow(self.parent_hwnd as _);
                            }
                        }
                    }
                }
            }
        }

        // Process seekbar hover preview frame textures, shortcuts, drag-and-drop, window resizing, A-B loop
        self.seek_preview.pump(&ctx);
        self.handle_shortcuts(&ctx, &mut stats);
        self.handle_drag_and_drop(&ctx);
        self.handle_window_resizing(&ctx, &stats);
        self.check_ab_loop_and_resume();

        // ── Windows Taskbar Navigation Hover Thumbnail Controls (7 Buttons) ──
        #[cfg(windows)]
        if self.parent_hwnd != 0 {
            if !self.taskbar.is_thumbbar_initialized() {
                self.taskbar.init_thumbbar(self.parent_hwnd);
            }
            let thumb_cmds = self.taskbar.poll_commands();
            for cmd in thumb_cmds {
                match cmd {
                    crate::platform::windows::taskbar::THUMB_CMD_PREV => {
                        if let Some(prev) = self.playlist.lock().unwrap().previous() {
                            file_to_open = Some(prev);
                        } else if let Ok(ref player) = self.player {
                            player.prev_chapter();
                            self.osd.show("⏮ Previous Track / Chapter".to_string(), 1200);
                        }
                    }
                    crate::platform::windows::taskbar::THUMB_CMD_REWIND => {
                        if let Ok(ref player) = self.player {
                            player.seek_relative(-10.0);
                            self.osd.show("⏪ Rewind 10s".to_string(), 1000);
                        }
                    }
                    crate::platform::windows::taskbar::THUMB_CMD_STOP => {
                        self.close_current_media(&mut stats, &ctx);
                    }
                    crate::platform::windows::taskbar::THUMB_CMD_PLAY_PAUSE => {
                        if stats.is_idle {
                            if let Some(item) = self.playlist.lock().unwrap().current_item() {
                                file_to_open = Some(item.path.clone());
                            } else if let Some(recent) = self.config.recent_files.first() {
                                let p = PathBuf::from(recent);
                                if p.exists() {
                                    file_to_open = Some(p);
                                }
                            }
                        } else if let Ok(ref player) = self.player {
                            player.toggle_pause();
                            self.osd.show_play_pause(!stats.is_paused);
                        }
                    }
                    crate::platform::windows::taskbar::THUMB_CMD_FORWARD => {
                        if let Ok(ref player) = self.player {
                            player.seek_relative(10.0);
                            self.osd.show("⏩ Forward 10s".to_string(), 1000);
                        }
                    }
                    crate::platform::windows::taskbar::THUMB_CMD_NEXT => {
                        if let Some(next) = self.playlist.lock().unwrap().next() {
                            file_to_open = Some(next);
                        } else if let Ok(ref player) = self.player {
                            player.next_chapter();
                            self.osd.show("⏭ Next Track / Chapter".to_string(), 1200);
                        }
                    }
                    crate::platform::windows::taskbar::THUMB_CMD_FULLSCREEN => {
                        self.toggle_fullscreen(&ctx);
                    }
                    _ => {}
                }
            }
        }

        let queue_has_next = self.playlist.lock().unwrap().items.len() > 1;
        self.queue_repaint_active.store(
            queue_has_next && !stats.is_idle && !stats.is_paused,
            Ordering::Relaxed,
        );

        let mut folder_to_open: Option<PathBuf> = None;
        let mut toggle_pip_requested = false;

        // Codec and audio/video badge string
        let is_song_mode = MusicBackgroundView::is_song(&stats);
        let _resolution_tag = if is_song_mode {
            let raw_codec = if !stats.audio_codec.is_empty() {
                stats.audio_codec.split(|c: char| c.is_whitespace() || c == '(' || c == '/').next().unwrap_or("FLAC")
            } else {
                std::path::Path::new(&stats.file_path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("AUDIO")
            };
            Some(raw_codec.to_uppercase())
        } else if stats.video_width > 0 {
            let hdr_part = if stats.is_hdr { format!(" · {}", stats.hdr_format) } else { "".to_string() };
            let bit_depth_part = if stats.pixel_format.contains("10") || stats.pixel_format.contains("p010") {
                " 10-bit"
            } else if stats.pixel_format.contains("12") {
                " 12-bit"
            } else {
                ""
            };
            Some(format!("{}x{} {}{}{}", stats.video_width, stats.video_height, stats.video_codec.to_uppercase(), bit_depth_part, hdr_part))
        } else {
            None
        };

        let sr = get_window_client_rect(&ctx);
        let screen_w = sr.width();
        let screen_h = sr.height();
        #[cfg(windows)]
        let mouse_pos = {
            let egui_pos = ctx.input(|i| i.pointer.hover_pos().or_else(|| i.pointer.latest_pos()));
            if self.parent_hwnd != 0 {
                use windows_sys::Win32::Foundation::POINT;
                use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
                use windows_sys::Win32::UI::WindowsAndMessaging::{GetAncestor, GetCursorPos, GetForegroundWindow, GetWindowThreadProcessId, GA_ROOT};
                let fg = unsafe { GetForegroundWindow() };
                let mut fg_pid = 0;
                if !fg.is_null() {
                    unsafe { GetWindowThreadProcessId(fg, &mut fg_pid) };
                }
                let is_fg = fg == (self.parent_hwnd as _)
                    || (self.child_hwnd != 0 && fg == (self.child_hwnd as _))
                    || (!fg.is_null() && unsafe { GetAncestor(fg, GA_ROOT) } == (self.parent_hwnd as _))
                    || (fg_pid != 0 && fg_pid == std::process::id());

                let mut pt = POINT { x: 0, y: 0 };
                if (self.is_fullscreen || is_fg) && unsafe { GetCursorPos(&mut pt) != 0 && ScreenToClient(self.parent_hwnd as _, &mut pt) != 0 } {
                    let ppp = ctx.pixels_per_point().max(0.1);
                    let client_pt = Pos2::new(pt.x as f32 / ppp, pt.y as f32 / ppp);
                    if client_pt.x >= -5.0 && client_pt.x <= screen_w + 5.0 && client_pt.y >= -5.0 && client_pt.y <= screen_h + 5.0 {
                        Some(client_pt)
                    } else {
                        None
                    }
                } else {
                    egui_pos
                }
            } else {
                egui_pos
            }
        };
        #[cfg(not(windows))]
        let mouse_pos = ctx.input(|i| i.pointer.hover_pos().or_else(|| i.pointer.latest_pos()));

        let mouse_moved = match (mouse_pos, self.last_mouse_pos) {
            (Some(p1), Some(p2)) => (p1 - p2).length() > 1.0,
            (Some(_), None) => true,
            _ => false,
        };
        if mouse_moved {
            if self.last_fullscreen_change.elapsed().as_millis() > 600 {
                self.last_mouse_activity = std::time::Instant::now();
            }
            self.last_mouse_pos = mouse_pos;
            ctx.request_repaint();
        }

        let is_fullscreen_active = self.is_fullscreen;
        let is_song_playback = MusicBackgroundView::is_song(&stats);
        let bar_h = if is_song_playback {
            ControlBar::HEIGHT_MUSIC
        } else if is_fullscreen_active {
            ControlBar::HEIGHT_FULLSCREEN
        } else {
            ControlBar::HEIGHT_COMPACT
        };

        let (show_top_menu, show_bottom_menu) = if self.pip_mode.is_pip {
            (false, false)
        } else if !is_fullscreen_active {
            self.is_top_hovered = false;
            self.is_bottom_hovered = false;
            (true, true)
        } else {
            let pointer_down = {
                let egui_down = ctx.input(|i| i.pointer.any_down());
                #[cfg(windows)]
                {
                    if egui_down {
                        use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON, VK_MBUTTON};
                        unsafe {
                            (GetAsyncKeyState(VK_LBUTTON as i32) as i16) < 0
                                || (GetAsyncKeyState(VK_RBUTTON as i32) as i16) < 0
                                || (GetAsyncKeyState(VK_MBUTTON as i32) as i16) < 0
                        }
                    } else {
                        false
                    }
                }
                #[cfg(not(windows))]
                egui_down
            };
            let popups_open = self.show_speed_popup || self.show_audio_channels_popup;

            // Top titlebar: 32px height.
            // Hysteresis: trigger when within 38px of top; stay open while within 48px
            let top_threshold = if self.is_top_hovered { 48.0 } else { 38.0 };
            let in_top_hover = mouse_pos.map_or(false, |p| p.y <= top_threshold);

            // Bottom control bar: 52px (or 120px in music mode).
            // Hysteresis: trigger when within bar_h + 10px of bottom; stay open while within bar_h + 20px
            let bottom_threshold = if self.is_bottom_hovered { 
                screen_h - (bar_h + 20.0) 
            } else { 
                screen_h - (bar_h + 10.0) 
            };
            let in_bottom_hover = mouse_pos.map_or(false, |p| p.y >= bottom_threshold);

            // Hide after 2.5s if mouse sits completely stationary, even inside the hover zone
            let idle_secs = self.last_mouse_activity.elapsed().as_secs_f32();
            let is_idle = idle_secs > 2.5;

            // Top bar shows when hovered near top, titlebar menu button is open, or dragging
            // Does NOT show when opening the right-click context menu in fullscreen!
            let show_top = (self.show_main_menu && self.menu_anchor_pos.is_some()) 
                || (in_top_hover && !is_idle)
                || (pointer_down && self.is_top_hovered);

            // Bottom bar shows when hovered near bottom, popups open, or dragging
            // Hides IMMEDIATELY when mouse moves away!
            let show_bottom = popups_open 
                || (in_bottom_hover && !is_idle)
                || (pointer_down && self.is_bottom_hovered);

            if show_top != self.is_top_hovered || show_bottom != self.is_bottom_hovered {
                ctx.request_repaint();
            }

            // In fullscreen, hide cursor when idle over the video with no bars, dialogs, or menus open
            if is_fullscreen_active && is_idle && !show_top && !show_bottom && !self.has_open_dialog() && !self.show_main_menu && !popups_open {
                ctx.set_cursor_icon(egui::CursorIcon::None);
            }

            // Always request periodic repaint in fullscreen to keep polling cursor position and handle autohide
            ctx.request_repaint_after(std::time::Duration::from_millis(50));

            self.is_top_hovered = show_top;
            self.is_bottom_hovered = show_bottom;

            (show_top, show_bottom)
        };


        let mut titlebar_tooltip: Option<(Pos2, String)> = None;

        // =========================================================================
        // 1. TOP PANEL: DOCKED TITLEBAR (Exact height 32.0, autohidden in fullscreen)
        // =========================================================================
        if show_top_menu {
            show_top_control_surface(ui, &ctx, is_fullscreen_active, screen_w, screen_h, |ui| {
                    let prev_menu = self.show_main_menu;
                    let clean_title = if !stats.file_path.is_empty() {
                        std::path::Path::new(&stats.file_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| if stats.title.is_empty() { "VortexPlayer".to_string() } else { stats.title.clone() })
                    } else if !stats.title.is_empty() && stats.title != "VertoxPlayer" && stats.title != "VortexPlayer" && stats.title != "VertexPlayer" && stats.title != "Ready" {
                        stats.title.clone()
                    } else {
                        "VortexPlayer".to_string()
                    };

                    let formatted_title = {
                        let pl = self.playlist.lock().unwrap();
                        if pl.items.len() > 1 {
                            if let Some(idx) = pl.current_index {
                                format!("[{}/{}] {}", idx + 1, pl.items.len(), clean_title)
                            } else {
                                clean_title
                            }
                        } else {
                            clean_title
                        }
                    };

                    let clean_codec_badge = {
                        let raw = if !stats.video_codec.is_empty() {
                            stats.video_codec.as_str()
                        } else if !stats.audio_codec.is_empty() {
                            stats.audio_codec.as_str()
                        } else {
                            ""
                        };
                        if raw.is_empty() {
                            None
                        } else {
                            let first = raw.split(|c: char| c == '(' || c == '/' || c.is_whitespace()).next().unwrap_or(raw);
                            Some(first.to_uppercase())
                        }
                    };

                    let is_max_state = is_window_maximized_or_workarea(self.parent_hwnd, &ctx);
                    let tb_resp = TitleBar::render(
                        ui,
                        &formatted_title,
                        clean_codec_badge.as_deref(),
                        &mut self.config.always_on_top,
                        &mut self.show_main_menu,
                        &mut self.menu_anchor_pos,
                        &mut self.menu_position,
                        is_fullscreen_active,
                        is_max_state,
                        self.config.theme_mode,
                    );
                    titlebar_tooltip = tb_resp.tooltip;
                    if !prev_menu && self.show_main_menu {
                        self.menu_opened_frame = self.frame_counter;
                    }

                    if tb_resp.double_clicked || tb_resp.toggle_maximize {
                        self.toggle_maximize_window(&ctx);
                    } else if tb_resp.drag_started && self.last_maximize_time.elapsed().as_millis() > 350 {
                        if self.is_fullscreen {
                            // Moving/dragging using top bar in fullscreen:
                            // Exit fullscreen and transition to windowed UI mode, smoothly dragging window!
                            self.set_fullscreen(&ctx, false);
                            self.pre_fullscreen_maximized = false;

                            #[cfg(windows)]
                            if self.parent_hwnd != 0 {
                                use windows_sys::Win32::Foundation::{POINT, RECT};
                                use windows_sys::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
                                use windows_sys::Win32::UI::WindowsAndMessaging::{
                                    GetCursorPos, GetWindowRect, PostMessageW, SetWindowPos,
                                    HTCAPTION, SWP_FRAMECHANGED, SWP_NOZORDER, SWP_SHOWWINDOW, WM_NCLBUTTONDOWN,
                                };
                                let mut pt = POINT { x: 0, y: 0 };
                                unsafe {
                                    let mut fs_rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                                    GetWindowRect(self.parent_hwnd as _, &mut fs_rc);
                                    let fs_w = (fs_rc.right - fs_rc.left).max(1);

                                    if GetCursorPos(&mut pt) != 0 {
                                        let ratio = ((pt.x - fs_rc.left) as f32 / fs_w as f32).clamp(0.05, 0.95);
                                        let target = self.last_windowed_rect
                                            .unwrap_or_else(|| Rect::from_min_size(
                                                Pos2::new(100.0, 80.0),
                                                Vec2::new(self.config.window_width.max(640.0), self.config.window_height.max(400.0)),
                                            ));
                                        let ppp = ctx.pixels_per_point().max(0.1);
                                        let win_w = (target.width() * ppp).round() as i32;
                                        let win_h = (target.height() * ppp).round() as i32;
                                        let new_left = pt.x - (win_w as f32 * ratio).round() as i32;
                                        let new_top = (pt.y - 15).max(0);

                                        SetWindowPos(
                                            self.parent_hwnd as _,
                                            0 as _,
                                            new_left,
                                            new_top,
                                            win_w,
                                            win_h,
                                            SWP_NOZORDER | SWP_SHOWWINDOW | SWP_FRAMECHANGED,
                                        );
                                        self.last_windowed_rect = Some(Rect::from_min_size(
                                            Pos2::new(new_left as f32 / ppp, new_top as f32 / ppp),
                                            Vec2::new(win_w as f32 / ppp, win_h as f32 / ppp),
                                        ));

                                        let lparam = (((pt.y as i16) as u32) << 16) | (((pt.x as i16) as u32) & 0xffff);
                                        ReleaseCapture();
                                        PostMessageW(self.parent_hwnd as _, WM_NCLBUTTONDOWN, HTCAPTION as _, lparam as isize);
                                    }
                                }
                            }
                            #[cfg(not(windows))]
                            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        } else {
                            let is_max = is_window_maximized_or_workarea(self.parent_hwnd, &ctx);
                            if is_max {
                                #[cfg(windows)]
                                if self.parent_hwnd != 0 {
                                    use windows_sys::Win32::Foundation::{POINT, RECT};
                                    use windows_sys::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
                                    use windows_sys::Win32::UI::WindowsAndMessaging::{
                                        GetCursorPos, GetWindowRect, PostMessageW, SetWindowPos, ShowWindow,
                                        HTCAPTION, SWP_FRAMECHANGED, SWP_NOZORDER, SWP_SHOWWINDOW, SW_RESTORE, WM_NCLBUTTONDOWN,
                                    };
                                    let mut pt = POINT { x: 0, y: 0 };
                                    unsafe {
                                        let mut max_rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                                        GetWindowRect(self.parent_hwnd as _, &mut max_rc);
                                        let max_w = (max_rc.right - max_rc.left).max(1);

                                        ShowWindow(self.parent_hwnd as _, SW_RESTORE);
                                        if GetCursorPos(&mut pt) != 0 {
                                            let ratio = ((pt.x - max_rc.left) as f32 / max_w as f32).clamp(0.05, 0.95);
                                            let target = self.last_windowed_rect
                                                .unwrap_or_else(|| Rect::from_min_size(
                                                    Pos2::new(100.0, 80.0),
                                                    Vec2::new(self.config.window_width.max(640.0), self.config.window_height.max(400.0)),
                                                ));
                                            let ppp = ctx.pixels_per_point().max(0.1);
                                            let win_w = (target.width() * ppp).round() as i32;
                                            let win_h = (target.height() * ppp).round() as i32;
                                            let new_left = pt.x - (win_w as f32 * ratio).round() as i32;
                                            let new_top = (pt.y - 15).max(0);

                                            SetWindowPos(
                                                self.parent_hwnd as _,
                                                0 as _,
                                                new_left,
                                                new_top,
                                                win_w,
                                                win_h,
                                                SWP_NOZORDER | SWP_SHOWWINDOW | SWP_FRAMECHANGED,
                                            );
                                            self.last_windowed_rect = Some(Rect::from_min_size(
                                                Pos2::new(new_left as f32 / ppp, new_top as f32 / ppp),
                                                Vec2::new(win_w as f32 / ppp, win_h as f32 / ppp),
                                            ));

                                            let lparam = (((pt.y as i16) as u32) << 16) | (((pt.x as i16) as u32) & 0xffff);
                                            ReleaseCapture();
                                            PostMessageW(self.parent_hwnd as _, WM_NCLBUTTONDOWN, HTCAPTION as _, lparam as isize);
                                        }
                                    }
                                }
                                #[cfg(not(windows))]
                                {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
                                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                                }
                            } else {
                                #[cfg(windows)]
                                if self.parent_hwnd != 0 {
                                    use windows_sys::Win32::Foundation::POINT;
                                    use windows_sys::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
                                    use windows_sys::Win32::UI::WindowsAndMessaging::{GetCursorPos, PostMessageW, WM_NCLBUTTONDOWN, HTCAPTION};
                                    let mut pt = POINT { x: 0, y: 0 };
                                    unsafe {
                                        GetCursorPos(&mut pt);
                                        let lparam = (((pt.y as i16) as u32) << 16) | (((pt.x as i16) as u32) & 0xffff);
                                        ReleaseCapture();
                                        PostMessageW(self.parent_hwnd as _, WM_NCLBUTTONDOWN, HTCAPTION as _, lparam as isize);
                                    }
                                } else {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                                }
                                #[cfg(not(windows))]
                                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                            }
                        }
                    }
                });
        }

        // =========================================================================
        // 2. BOTTOM PANEL: DOCKED CONTROL BAR (Adaptive height for Video / Music)
        // =========================================================================
        if show_bottom_menu {
            let is_song_mode = MusicBackgroundView::is_song(&stats);
            let bar_height = if is_song_mode {
                ControlBar::HEIGHT_MUSIC
            } else if is_fullscreen_active {
                ControlBar::HEIGHT_FULLSCREEN
            } else {
                ControlBar::HEIGHT_COMPACT
            };

            let is_docked_pl = self.show_playlist && !self.config.playlist_detached && !is_fullscreen_active;
            let sidebar_w = 330.0;
            let ctrl_w = if is_docked_pl { (screen_w - sidebar_w).max(0.0) } else { screen_w };

            show_bottom_control_surface(ui, &ctx, is_fullscreen_active, ctrl_w, screen_h, bar_height, |ui| {
                    let player_opt = self.player.as_ref().ok().map(|p| p.as_ref());
                    let actions = ControlBar::render(
                        ui,
                        player_opt,
                        &stats,
                        &mut *self.playlist.lock().unwrap(),
                        &mut self.bookmark_mgr,
                        &mut self.show_remaining_time,
                        self.music_view.cover_texture.as_ref(),
                        &self.config,
                        &mut self.seek_preview,
                        is_fullscreen_active,
                        self.show_playlist,
                    );

                        let active_tip_opt = actions.hover_tooltip.or(titlebar_tooltip);
                        if let Some((pos, ref tip)) = active_tip_opt {
                            // Universal cross-platform GPU tooltip renderer with crisp Vortex styling
                            let painter = ctx.layer_painter(egui::LayerId::new(
                                egui::Order::Tooltip,
                                egui::Id::new("control_bar_hover_tooltip"),
                            ));

                            let font_id = FontId::proportional(11.5);
                            let galley = ctx.fonts_mut(|f| f.layout_no_wrap(tip.clone(), font_id, Color32::WHITE));
                            let pad = Vec2::new(8.0, 4.5);
                            let tip_size = galley.size() + pad * 2.0;

                            let win_size = get_window_client_rect(&ctx).size();
                            let max_x = (win_size.x - tip_size.x - 8.0).max(8.0);
                            let tip_x = (pos.x - tip_size.x * 0.5).clamp(8.0, max_x);
                            
                            // If element is near top, display below element. If near bottom, display above element.
                            let tip_y = if pos.y < 60.0 {
                                let max_y = (win_size.y - tip_size.y - 8.0).max(4.0);
                                (pos.y + 4.0).clamp(4.0, max_y)
                            } else {
                                let max_y = (win_size.y - tip_size.y - 8.0).max(34.0);
                                (pos.y - tip_size.y - 6.0).clamp(34.0, max_y)
                            };

                            let tip_rect = Rect::from_min_size(Pos2::new(tip_x, tip_y), tip_size);
                            self.active_popup_rects.push(tip_rect);

                            painter.rect_filled(tip_rect, CornerRadius::same(4), Color32::from_rgb(18, 20, 26));
                            painter.rect_stroke(tip_rect, CornerRadius::same(4), Stroke::new(1.0, Color32::from_rgb(48, 52, 65)), eframe::egui::StrokeKind::Inside);
                            painter.galley(tip_rect.min + pad, galley, Color32::from_rgb(235, 238, 245));
                        }

                        if let Some(card_r) = actions.hover_card_rect {
                            self.active_popup_rects.push(card_r);
                        }

                        if actions.toggle_play {
                            if stats.is_idle {
                                if let Some(item) = self.playlist.lock().unwrap().current_item() {
                                    file_to_open = Some(item.path.clone());
                                } else if let Some(recent) = self.config.recent_files.first() {
                                    let p = PathBuf::from(recent);
                                    if p.exists() {
                                        file_to_open = Some(p);
                                    }
                                }
                            } else if let Ok(ref player) = self.player {
                                player.toggle_pause();
                                self.osd.show_play_pause(!stats.is_paused);
                            }
                        }
                        if actions.stop {
                            if let Ok(ref player) = self.player {
                                if !stats.file_path.is_empty() && stats.time_pos > 1.0 {
                                    let save_pos = if stats.duration > 0.0 && stats.time_pos >= stats.duration - 8.0 { 0.0 } else { stats.time_pos };
                                    self.config.save_resume_time(&stats.file_path, save_pos);
                                    let _ = self.config.save();
                                }
                                self.prev_was_playing = false;
                                player.stop();
                                self.osd.show("⏹ Stopped".to_string(), 1200);
                            }
                        }
                        if actions.close_file {
                            self.close_current_media(&mut stats, &ctx);
                        }
                        if actions.prev_track {
                            if let Some(p) = self.playlist.lock().unwrap().previous() {
                                file_to_open = Some(p);
                            }
                        }
                        if actions.next_track {
                            if let Some(p) = self.playlist.lock().unwrap().next() {
                                file_to_open = Some(p);
                            }
                        }
                        if actions.prev_chapter {
                            if let Ok(ref player) = self.player {
                                player.prev_chapter();
                                self.osd.show("Previous Chapter ⏮".to_string(), 1200);
                            }
                        }
                        if actions.next_chapter {
                            if let Ok(ref player) = self.player {
                                player.next_chapter();
                                self.osd.show("Next Chapter ⏭".to_string(), 1200);
                            }
                        }
                        if actions.skip_intro {
                            self.skip_intro_or_op(&stats);
                        }
                        if let Some(delta) = actions.seek_relative {
                            if let Ok(ref player) = self.player {
                                player.seek_relative(delta);
                            }
                        }
                        if actions.frame_step {
                            if let Ok(ref player) = self.player {
                                player.step_frame_forward();
                            }
                        }
                        if actions.open_file {
                            if let Some(file) = rfd::FileDialog::new().pick_file() {
                                file_to_open = Some(file);
                            }
                        }
                                                if actions.cycle_hdr_tone_mapping {
                            let curves = ["auto", "bt.2446a", "spline", "mobius", "reinhard", "hable"];
                            let curr = self.config.hdr_tone_mapping.to_lowercase();
                            let next_idx = match curves.iter().position(|&c| c == curr) {
                                Some(i) => (i + 1) % curves.len(),
                                None => 0,
                            };
                            let next_curve = curves[next_idx];
                            self.config.hdr_tone_mapping = next_curve.to_string();
                            let _ = self.config.save();
                            if let Ok(ref player) = self.player {
                                player.set_hdr_tone_mapping(next_curve);
                            }
                            let msg = format!("HDR Tone Mapping: {}", next_curve.to_uppercase());
                            self.osd.show(msg, 1500);
                        }
                                                if actions.toggle_hwdec {
                            let curr = self.config.hardware_decoding.clone();
                            let next_hw = if curr == "no" {
                                "auto-safe".to_string()
                            } else {
                                "no".to_string()
                            };
                            self.config.hardware_decoding = next_hw.clone();
                            let _ = self.config.save();
                            if let Ok(ref player) = self.player {
                                player.set_hwdec(&next_hw);
                            }
                            let msg = if next_hw != "no" {
                                "Hardware Decoding (H/W): Enabled (D3D11VA)"
                            } else {
                                "Hardware Decoding (S/W): Active (Software)"
                            };
                            self.osd.show(msg.to_string(), 1500);
                        }
                        if actions.toggle_wasapi_exclusive {
                            self.config.wasapi_exclusive = !self.config.wasapi_exclusive;
                            let _ = self.config.save();
                            if let Ok(ref player) = self.player {
                                player.set_wasapi_exclusive(self.config.wasapi_exclusive);
                            }
                            let msg = if self.config.wasapi_exclusive {
                                "WASAPI Exclusive: ON (Bit-Perfect Direct Output)"
                            } else {
                                "WASAPI Exclusive: OFF (Windows Shared Mixer)"
                            };
                            self.osd.show(msg.to_string(), 1500);
                        }
                        if actions.open_audio_channels {
                            self.show_audio_channels_popup = !self.show_audio_channels_popup;
                            self.audio_channels_popup_pos = actions.open_audio_channels_pos;
                            self.menu_opened_frame = self.frame_counter;
                        }
                        if actions.open_speed_menu {
                            self.show_speed_popup = !self.show_speed_popup;
                            self.speed_popup_pos = actions.open_speed_menu_pos;
                            self.menu_opened_frame = self.frame_counter;
                        }
                        if actions.toggle_playlist {
                            self.toggle_playlist_unified(&ctx);
                        }
                        if actions.toggle_control_panel {
                            self.show_control_panel = !self.show_control_panel;
                        }
                        if actions.toggle_preferences {
                            self.show_preferences = !self.show_preferences;
                        }
                        if actions.toggle_bookmark_overlay {
                            self.show_bookmark_overlay = !self.show_bookmark_overlay;
                        }
                        if actions.toggle_ab_loop {
                            if self.bookmark_mgr.ab_loop.is_active {
                                self.bookmark_mgr.clear_ab_loop();
                            } else {
                                let _ = self.bookmark_mgr.set_loop_a(stats.time_pos);
                            }
                        }
                        if actions.cycle_audio {
                            if let Ok(ref player) = self.player {
                                player.cycle_audio_track();
                            }
                        }
                        if actions.cycle_subtitle {
                            if let Ok(ref player) = self.player {
                                player.cycle_subtitle_track();
                            }
                        }
                        if actions.cycle_repeat {
                            let mode = self.playlist.lock().unwrap().cycle_repeat_mode();
                            self.config.playlist_repeat_mode = match mode {
                                crate::playlist::RepeatMode::RepeatAll => "RepeatAll".to_string(),
                                crate::playlist::RepeatMode::RepeatTrack => "RepeatTrack".to_string(),
                                _ => "Off".to_string(),
                            };
                            let _ = self.config.save();
                            let msg = match mode {
                                crate::playlist::RepeatMode::RepeatAll => "Repeat: All Tracks",
                                crate::playlist::RepeatMode::RepeatTrack => "Repeat: Current Track",
                                crate::playlist::RepeatMode::Off => "Repeat: Off",
                            };
                            self.osd.show(msg.to_string(), 1500);
                        }
                        if actions.toggle_shuffle {
                            let is_shuf = self.playlist.lock().unwrap().toggle_shuffle();
                            self.config.playlist_shuffle = is_shuf;
                            let _ = self.config.save();
                            let msg = if is_shuf { "Shuffle: ON" } else { "Shuffle: OFF" };
                            self.osd.show(msg.to_string(), 1500);
                        }
                        if actions.open_mediainfo {
                            self.show_mediainfo_dialog = true;
                            self.mediainfo_dialog.selected_tab = 1;
                        }
                        if actions.take_screenshot {
                            if let Ok(ref player) = self.player {
                                if let Some(pic_dir) = dirs::picture_dir() {
                                    let path = pic_dir.join(format!("VortexPlayer_{}.jpg", chrono::Utc::now().format("%Y%m%d_%H%M%S")));
                                    player.take_screenshot(&path);
                                    self.osd.show("📸 Snapshot saved to Pictures".to_string(), 1500);
                                }
                            }
                        }
                        if actions.cycle_speed {
                            if let Ok(ref player) = self.player {
                                let current_spd = stats.speed;
                                let next_spd = if (current_spd - 1.0).abs() < 0.05 {
                                    1.2
                                } else if (current_spd - 1.2).abs() < 0.05 {
                                    1.5
                                } else if (current_spd - 1.5).abs() < 0.05 {
                                    2.0
                                } else if (current_spd - 2.0).abs() < 0.05 {
                                    0.8
                                } else {
                                    1.0
                                };
                                player.set_speed(next_spd);
                                self.osd.show(format!("Speed: {:.1}x", next_spd), 1200);
                            }
                        }
                        if actions.reset_speed {
                            if let Ok(ref player) = self.player {
                                player.set_speed(1.0);
                                self.osd.show("Speed: 1.0x (Normal)".to_string(), 1200);
                            }
                        }
                        if actions.open_audio_tracks_popup {
                            if let Ok(ref player) = self.player {
                                player.cycle_audio_track();
                            }
                        }
                        if actions.open_subtitle_tracks_popup {
                            if let Ok(ref player) = self.player {
                                player.cycle_subtitle_track();
                            }
                        }
                });
        }

        // =========================================================================
        // 3. CENTRAL PANEL: VIDEO VIEWPORT
        // =========================================================================
        let is_docked_pl = self.show_playlist && !self.config.playlist_detached && !self.is_fullscreen;
        let sidebar_w = 330.0;
        let pl_x = (screen_w - sidebar_w).max(0.0);
        let video_w = if is_docked_pl { pl_x } else { screen_w };

        if let Ok(ref player) = self.player {
            let target_margin = if is_docked_pl && screen_w > 0.0 {
                (sidebar_w / screen_w).clamp(0.0, 0.85)
            } else {
                0.0
            };
            static LAST_MARGIN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(99999);
            let margin_bits = (target_margin * 10000.0) as u32;
            if LAST_MARGIN.swap(margin_bits, std::sync::atomic::Ordering::Relaxed) != margin_bits {
                player.set_property_string("video-margin-ratio-right", &format!("{:.4}", target_margin));
            }
        }

        let is_song_playback = MusicBackgroundView::is_song(&stats);
        let is_active_video = !stats.is_idle && !stats.file_path.is_empty() && !is_song_playback;
        let central_fill = if is_active_video {
            Color32::from_rgb(0, 0, 0)
        } else {
            Color32::from_rgb(10, 11, 14)
        };

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(central_fill).inner_margin(Margin::ZERO))
            .show(ui, |ui| {
                let is_fullscreen_active = self.is_fullscreen;
                let client_rect = get_window_client_rect(ui.ctx());
                let effective_rect = if is_docked_pl {
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(video_w, screen_h))
                } else {
                    client_rect
                };
                let rect = effective_rect;
                ui.set_clip_rect(effective_rect);
                let response = ui.allocate_rect(effective_rect, Sense::click_and_drag());
                self.last_video_rect = effective_rect;

                // Render video into OpenGL framebuffer via mpv_render_context
                // Panels use Order::Foreground so they paint ON TOP of this
                if is_active_video {
                    if let Ok(ref player) = self.player {

                        let player_clone = Arc::clone(player);
                        let egui_ctx = ui.ctx().clone();
                        let callback = egui_glow::CallbackFn::new(move |info, _painter| {
                            #[cfg(windows)]
                            {
                                static VSYNC_INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
                                if !VSYNC_INITIALIZED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                                    type WglGetProcAddressFn = unsafe extern "system" fn(*const u8) -> *const std::ffi::c_void;
                                    type WglSwapIntervalEXTFn = unsafe extern "system" fn(i32) -> i32;
                                    unsafe {
                                        use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
                                        let opengl32 = GetModuleHandleA(b"opengl32.dll\0".as_ptr());
                                        if opengl32 != std::ptr::null_mut() {
                                            if let Some(wgl_gpa) = GetProcAddress(opengl32, b"wglGetProcAddress\0".as_ptr()) {
                                                let wglGetProcAddress: WglGetProcAddressFn = std::mem::transmute(wgl_gpa);
                                                let ext_proc = wglGetProcAddress(b"wglSwapIntervalEXT\0".as_ptr());
                                                if !ext_proc.is_null() {
                                                    let swap_interval: WglSwapIntervalEXTFn = std::mem::transmute(ext_proc);
                                                    swap_interval(1);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            let _ = player_clone.ensure_render_context(&egui_ctx);
                            let [w, h] = info.screen_size_px;
                            player_clone.render_frame(0, w as i32, h as i32);
                            player_clone.report_swap();
                        });
                        ui.painter().add(egui::PaintCallback {
                            rect: effective_rect,
                            callback: Arc::new(callback),
                        });
                    }
                }

                let pointer_pos = ui.input(|i| i.pointer.hover_pos().or(i.pointer.latest_pos()));
                let over_bars = if is_fullscreen_active {
                    pointer_pos.map_or(false, |p| {
                        (show_top_menu && p.y <= 48.0) || (show_bottom_menu && p.y >= screen_h - (bar_h + 20.0))
                    })
                } else {
                    false
                };
                let in_ui_overlay = over_bars
                    || self.has_open_dialog()
                    || self.show_main_menu
                    || self.show_speed_popup
                    || self.show_audio_channels_popup;

                // Double-click gestures (Toggle Play / Pause)
                let pointer_dbl = ui.input(|i| {
                        i.pointer.button_double_clicked(egui::PointerButton::Primary)
                            && i.pointer.hover_pos().map_or(false, |p| effective_rect.contains(p))
                    });

                    if !in_ui_overlay && (response.double_clicked() || pointer_dbl) {
                        match self.config.double_click_action {
                            DoubleClickAction::ToggleFullscreen => {
                                self.toggle_fullscreen(ui.ctx());
                            }
                            DoubleClickAction::PlayPause => {
                                if let Ok(ref player) = self.player {
                                    player.toggle_pause();
                                    self.osd.show_play_pause(!stats.is_paused);
                                }
                            }
                            DoubleClickAction::MaximizeRestore => {
                                self.toggle_maximize_window(ui.ctx());
                            }
                        }
                    } else if !in_ui_overlay && response.clicked() {
                        if stats.is_idle {
                            if let Some(item) = self.playlist.lock().unwrap().current_item() {
                                file_to_open = Some(item.path.clone());
                            }
                        }
                    }

                    // 360 VR Drag Navigation
                    if !in_ui_overlay && self.is_vr_360 && response.dragged() {
                        let delta = ui.input(|i| i.pointer.delta());
                        self.vr_yaw += (delta.x as f64) * 0.004;
                        self.vr_pitch = (self.vr_pitch + (delta.y as f64) * 0.004).clamp(-1.5, 1.5);
                        if let Ok(ref player) = self.player {
                            player.set_video_pan_x(self.vr_yaw);
                            player.set_video_pan_y(self.vr_pitch);
                        }
                    }

                    // Middle-click action configured via Preferences
                    if !in_ui_overlay && response.middle_clicked() {
                        match self.config.mouse_middle_click_action.as_str() {
                            "Mute" => {
                                if let Ok(ref player) = self.player {
                                    player.toggle_mute();
                                    self.osd.show_volume(stats.volume, !stats.is_muted);
                                }
                            }
                            "100PercentSize" => {
                                self.resize_window_to_preset(ui.ctx(), 1.0, &stats);
                            }
                            "Fullscreen" => {
                                self.toggle_fullscreen(ui.ctx());
                            }
                            _ => {
                                if let Ok(ref player) = self.player {
                                    player.toggle_pause();
                                    self.osd.show_play_pause(!stats.is_paused);
                                }
                            }
                        }
                    }

                    // Mouse wheel over video canvas (Configurable: Volume, Seek, Speed, SubtitleDelay)
                    // Only active during playback; disabled on idle screen so recent files list scrolls freely
                    let scroll_delta = ui.input(|i| {
                        if i.smooth_scroll_delta.y.abs() > 0.01 {
                            i.smooth_scroll_delta.y
                        } else {
                            let mut sum = 0.0;
                            for ev in &i.raw.events {
                                if let egui::Event::MouseWheel { delta, .. } = ev {
                                    sum += delta.y;
                                }
                            }
                            sum
                        }
                    });

                    if !stats.is_idle && !stats.file_path.is_empty() && !in_ui_overlay && response.hovered() && scroll_delta.abs() > 0.1 {
                        if self.is_vr_360 {
                            self.vr_zoom = (self.vr_zoom + (scroll_delta as f64) * 0.05).clamp(-2.0, 3.0);
                            if let Ok(ref player) = self.player {
                                player.set_video_zoom(self.vr_zoom);
                            }
                        } else if let Ok(ref player) = self.player {
                            match self.config.mouse_wheel_action.as_str() {
                                "Seek" => {
                                    let delta_seek = if scroll_delta > 0.0 { 5.0 } else { -5.0 };
                                    player.seek_relative(delta_seek);
                                    self.osd.show_seek(stats.time_pos + delta_seek, stats.duration, Some(delta_seek));
                                }
                                "Speed" => {
                                    let delta_spd = if scroll_delta > 0.0 { 0.1 } else { -0.1 };
                                    let new_spd = (stats.speed + delta_spd).clamp(0.1, 4.0);
                                    player.set_speed(new_spd);
                                    self.config.playback_speed = new_spd;
                                    self.osd.show_speed(new_spd);
                                }
                                "SubtitleDelay" => {
                                    let delta_sub = if scroll_delta > 0.0 { 0.05 } else { -0.05 };
                                    let new_d = stats.subtitle_delay + delta_sub;
                                    player.set_subtitle_delay(new_d);
                                    self.osd.show_sub_delay(new_d);
                                }
                                _ => {
                                    let delta_vol = if scroll_delta > 0.0 { 2.0 } else { -2.0 };
                                    let next_vol = (stats.volume + delta_vol).clamp(0.0, self.config.volume_boost_limit);
                                    player.set_volume(next_vol);
                                    self.config.volume = next_vol;
                                    self.osd.show_volume(next_vol, stats.is_muted);
                                }
                            }
                            ctx.request_repaint();
                        }
                    }

                    // Modern Blurred Ambient Background View for Songs / Audio Playback
                    if let Ok(ref player) = self.player {
                        self.music_view.render(ui, player, effective_rect, &stats);
                    }

                    // Ultra-Modern Audio Visualizer Suite Overlay
                    if self.visualizer.is_visible {
                        self.visualizer.render(ui, effective_rect, &stats);
                    }

                    // Ultra-Modern Vortex Glassmorphic Idle Screen & Media Launchpad
                    if stats.is_idle || stats.file_path.is_empty() {
                        let top_bar_h = if show_top_menu { 32.0 } else { 0.0 };
                        let bottom_bar_h = if show_bottom_menu { bar_h } else { 0.0 };
                        let usable_top = effective_rect.top() + top_bar_h;
                        let usable_bottom = effective_rect.bottom() - bottom_bar_h;
                        let usable_h = (usable_bottom - usable_top).max(200.0);
                        let usable_center = Pos2::new(effective_rect.center().x, usable_top + usable_h * 0.5);

                        let has_recent = (!self.playback_history.entries.is_empty() || !self.config.recent_files.is_empty()) && self.config.show_recent_on_idle;

                        let entry_count = if has_recent {
                            if !self.playback_history.entries.is_empty() {
                                self.playback_history.entries.len().min(40)
                            } else {
                                self.config.recent_files.len().min(40)
                            }
                        } else {
                            0
                        };

                        let header_h = 100.0_f32; // Crest logo (32 radius), Title (22), Subtitle (14)
                        let gap = 14.0_f32;

                        let (logo_center_y, content_top_y, shelf_h_opt, is_recent_shelf) = if has_recent && entry_count > 0 {
                            let max_shelf_h = (usable_h * 0.74).clamp(380.0, 880.0);
                            let needed_h = (entry_count * 52 + 56) as f32;
                            let shelf_h = needed_h.clamp(160.0, max_shelf_h);
                            let total_block_h = header_h + gap + shelf_h;
                            let block_top = (usable_center.y - total_block_h * 0.5).max(usable_top + 16.0);
                            let logo_y = block_top + 32.0;
                            let shelf_top = block_top + header_h + gap;
                            (logo_y, shelf_top, Some(shelf_h), true)
                        } else {
                            let grid_h = 136.0_f32;
                            let total_block_h = header_h + gap + grid_h;
                            let block_top = (usable_center.y - total_block_h * 0.5).max(usable_top + 16.0);
                            let logo_y = block_top + 32.0;
                            let grid_top = block_top + header_h + gap;
                            (logo_y, grid_top, Some(grid_h), false)
                        };

                        let logo_pos = Pos2::new(usable_center.x, logo_center_y);

                        {
                            let painter = ui.painter();

                            // 1. Ambient Background Glow Behind Center Logo
                            let glow_radius = (usable_h * 0.35).min(rect.width() * 0.4).clamp(140.0, 260.0);
                            painter.circle_filled(
                                logo_pos,
                                glow_radius,
                                Color32::from_rgba_unmultiplied(245, 166, 35, 12),
                            );
                            painter.circle_filled(
                                logo_pos,
                                glow_radius * 0.65,
                                Color32::from_rgba_unmultiplied(60, 130, 240, 16),
                            );

                            // 2. Glowing Hero Vortex Crest
                            Icons::draw_vortex_hero_logo(painter, logo_pos, 32.0);

                            // 3. Bold Futuristic Title & Tagline
                            painter.text(
                                Pos2::new(usable_center.x, logo_center_y + 48.0),
                                Align2::CENTER_CENTER,
                                "VORTEX PLAYER",
                                FontId::proportional(22.0),
                                Color32::from_rgb(245, 248, 255),
                            );

                            painter.text(
                                Pos2::new(usable_center.x, logo_center_y + 72.0),
                                Align2::CENTER_CENTER,
                                "Ultra High-Fidelity Hardware Accelerated Media Engine",
                                FontId::proportional(12.0),
                                Color32::from_rgb(150, 160, 185),
                            );
                        }

                        if is_recent_shelf {
                            let shelf_h = shelf_h_opt.unwrap_or(240.0);
                            let shelf_w = (rect.width() * 0.78).clamp(960.0, 1480.0).min(rect.width() - 48.0);
                            let shelf_rect = Rect::from_min_size(
                                Pos2::new(usable_center.x - shelf_w * 0.5, content_top_y),
                                Vec2::new(shelf_w, shelf_h),
                            );

                            ui.painter().rect_filled(shelf_rect, CornerRadius::same(8), Color32::from_rgb(0, 0, 0));
                            ui.painter().rect_stroke(shelf_rect, CornerRadius::same(8), Stroke::new(1.0, Color32::from_rgb(38, 42, 56)), egui::StrokeKind::Inside);

                            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(shelf_rect.shrink(8.0)));
                            child_ui.horizontal(|ui| {
                                ui.label(RichText::new("🕒 RECENTLY PLAYED").size(12.0).strong().color(VortexTheme::current_skin().accent_primary));
                                let total_items = if !self.playback_history.entries.is_empty() {
                                    self.playback_history.entries.len()
                                } else {
                                    self.config.recent_files.len()
                                };
                                if total_items > 0 {
                                    ui.label(RichText::new(format!("({} items)", total_items)).size(11.0).color(Color32::from_rgb(140, 148, 170)));
                                }
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button(RichText::new("🗑 Clear History").size(11.0)).clicked() {
                                        self.config.recent_files.clear();
                                        let _ = self.config.save();
                                        self.playback_history.entries.clear();
                                        self.playback_history.save();
                                    }
                                    if ui.button(RichText::new("📂 Open File (Ctrl+O)").size(11.0).strong().color(Color32::from_rgb(255, 215, 80))).clicked() {
                                        if let Some(file) = rfd::FileDialog::new().pick_file() {
                                            file_to_open = Some(file);
                                        }
                                    }
                                    if ui.button(RichText::new("🌐 Stream URL (Ctrl+U)").size(11.0)).clicked() {
                                        self.show_stream_url_dialog = true;
                                    }
                                });
                            });
                            child_ui.add_space(6.0);

                            egui::ScrollArea::vertical()
                                .max_height(shelf_h - 44.0)
                                .show(&mut child_ui, |ui| {
                                    if !self.playback_history.entries.is_empty() {
                                        for entry in self.playback_history.entries.iter().take(40) {
                                            let path_str = &entry.file_path;
                                            let pb = PathBuf::from(path_str);
                                            let file_name = &entry.file_name;
                                            let dir_str = pb.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                                            let resume_pos = if entry.last_position > 1.0 { entry.last_position } else { self.config.get_resume_time(path_str).unwrap_or(0.0) };
                                            let pct = if entry.duration > 0.0 { (entry.last_position / entry.duration * 100.0).clamp(0.0, 100.0) as u32 } else { 0 };

                                            let card = ui.group(|ui| {
                                                let card_w = ui.available_width();
                                                let action_w = 330.0;
                                                let text_w = (card_w - action_w - 75.0).max(180.0);

                                                ui.horizontal(|ui| {
                                                    // 1. Progress Indicator Pill (Left)
                                                    if entry.completed {
                                                        ui.label(RichText::new("✓").size(13.0).color(Color32::from_rgb(60, 200, 100)));
                                                    } else if pct > 0 {
                                                        ui.label(RichText::new(format!("{:2}%", pct)).size(11.0).strong().color(VortexTheme::current_skin().accent_primary));
                                                    } else {
                                                        ui.label(RichText::new("•").size(15.0).color(Color32::from_rgb(140, 145, 165)));
                                                    }

                                                    // Format badge
                                                    let ext = pb.extension().and_then(|e| e.to_str()).unwrap_or("").to_uppercase();
                                                    if !ext.is_empty() {
                                                        ui.label(
                                                            RichText::new(format!(" {} ", ext))
                                                                .size(9.5)
                                                                .strong()
                                                                .color(Color32::from_rgb(175, 190, 220))
                                                                .background_color(Color32::from_rgb(26, 30, 42)),
                                                        );
                                                    }

                                                    // 2. Text Information (Wide space with title and path)
                                                    ui.allocate_ui_with_layout(Vec2::new(text_w, 32.0), egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                                        let max_chars = (text_w / 7.0) as usize;
                                                        let short_name = if file_name.chars().count() > max_chars {
                                                            let s: String = file_name.chars().take(max_chars.saturating_sub(3)).collect();
                                                            format!("{}...", s)
                                                        } else {
                                                            file_name.clone()
                                                        };
                                                        ui.label(RichText::new(short_name).strong().size(12.5).color(Color32::from_rgb(235, 238, 248)));

                                                        if !dir_str.is_empty() {
                                                            let max_dir_chars = (text_w / 6.5) as usize;
                                                            let short_dir = if dir_str.chars().count() > max_dir_chars {
                                                                let t: String = dir_str.chars().take(max_dir_chars.saturating_sub(3)).collect();
                                                                format!("{}...", t)
                                                            } else {
                                                                dir_str.clone()
                                                            };
                                                            ui.label(RichText::new(short_dir).size(10.5).color(Color32::from_rgb(130, 135, 150)));
                                                        }
                                                    });

                                                    // 3. Right-Aligned Actions & Metadata
                                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                        if ui.button(RichText::new("▶ Play").strong().color(Color32::BLACK)).clicked() {
                                                            file_to_open = Some(pb.clone());
                                                        }
                                                        if resume_pos > 1.0 && entry.duration > 0.0 {
                                                            ui.label(RichText::new(format!("Resume {} / {}", format_time(resume_pos), format_time(entry.duration))).size(10.5).color(VortexTheme::current_skin().accent_primary));
                                                        } else if resume_pos > 1.0 {
                                                            ui.label(RichText::new(format!("Resume {}", format_time(resume_pos))).size(10.5).color(VortexTheme::current_skin().accent_primary));
                                                        } else if entry.duration > 0.0 {
                                                            ui.label(RichText::new(format_time(entry.duration)).size(10.5).color(Color32::from_rgb(160, 170, 195)));
                                                        }
                                                        if entry.watch_count > 1 {
                                                            ui.label(RichText::new(format!("×{}", entry.watch_count)).small().color(Color32::from_rgb(180, 140, 255)));
                                                        }
                                                        if !entry.last_played.is_empty() {
                                                            ui.label(RichText::new(&entry.last_played).size(10.0).color(Color32::from_rgb(115, 122, 140)));
                                                        }
                                                    });
                                                });
                                            });

                                            if card.response.clicked() {
                                                file_to_open = Some(pb.clone());
                                            }
                                        }
                                    } else {
                                        for path_str in self.config.recent_files.iter().take(40) {
                                            let pb = PathBuf::from(path_str);
                                            let file_name = pb.file_name().and_then(|n| n.to_str()).unwrap_or(path_str).to_string();
                                            let dir_str = pb.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                                            let resume_time = self.config.get_resume_time(path_str);

                                            let card = ui.group(|ui| {
                                                let card_w = ui.available_width();
                                                let action_w = 330.0;
                                                let text_w = (card_w - action_w - 75.0).max(180.0);

                                                ui.horizontal(|ui| {
                                                    ui.label(RichText::new("•").size(15.0).color(VortexTheme::current_skin().accent_primary));

                                                    let ext = pb.extension().and_then(|e| e.to_str()).unwrap_or("").to_uppercase();
                                                    if !ext.is_empty() {
                                                        ui.label(
                                                            RichText::new(format!(" {} ", ext))
                                                                .size(9.5)
                                                                .strong()
                                                                .color(Color32::from_rgb(175, 190, 220))
                                                                .background_color(Color32::from_rgb(26, 30, 42)),
                                                        );
                                                    }

                                                    ui.allocate_ui_with_layout(Vec2::new(text_w, 32.0), egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                                        let max_chars = (text_w / 7.0) as usize;
                                                        let short_name = if file_name.chars().count() > max_chars {
                                                            let s: String = file_name.chars().take(max_chars.saturating_sub(3)).collect();
                                                            format!("{}...", s)
                                                        } else {
                                                            file_name.clone()
                                                        };
                                                        ui.label(RichText::new(short_name).strong().size(12.5).color(Color32::from_rgb(235, 238, 248)));

                                                        if !dir_str.is_empty() {
                                                            let max_dir_chars = (text_w / 6.5) as usize;
                                                            let short_dir = if dir_str.chars().count() > max_dir_chars {
                                                                let t: String = dir_str.chars().take(max_dir_chars.saturating_sub(3)).collect();
                                                                format!("{}...", t)
                                                            } else {
                                                                dir_str.clone()
                                                            };
                                                            ui.label(RichText::new(short_dir).size(10.5).color(Color32::from_rgb(130, 135, 150)));
                                                        }
                                                    });

                                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                        if ui.button(RichText::new("▶ Play").strong().color(Color32::BLACK)).clicked() {
                                                            file_to_open = Some(pb.clone());
                                                        }
                                                        if let Some(rt) = resume_time {
                                                            ui.label(RichText::new(format!("Resume {}", format_time(rt))).size(10.5).color(VortexTheme::current_skin().accent_primary));
                                                        }
                                                    });
                                                });
                                            });

                                            if card.response.clicked() {
                                                file_to_open = Some(pb.clone());
                                            }
                                        }
                                    }
                                });
                        } else {
                            // 4 Modern Interactive Quick-Action Cards
                            let grid_w = 480.0_f32.min(rect.width() - 32.0);
                            let grid_top = content_top_y;
                            let grid_h = shelf_h_opt.unwrap_or(136.0);
                            let grid_rect = Rect::from_min_size(
                                Pos2::new(usable_center.x - grid_w * 0.5, grid_top),
                                Vec2::new(grid_w, grid_h),
                            );

                            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(grid_rect));
                            child_ui.columns(2, |cols| {
                                // Card 1: Open Media File & Playlist
                                cols[0].vertical_centered(|ui| {
                                    if ui.add_sized(
                                        Vec2::new(ui.available_width(), 58.0),
                                        egui::Button::new(RichText::new("Open Media File\nCtrl + O").strong().size(12.5).color(Color32::from_rgb(255, 215, 80))),
                                    ).clicked() {
                                        if let Some(file) = rfd::FileDialog::new().pick_file() {
                                            file_to_open = Some(file);
                                        }
                                    }
                                    ui.add_space(8.0);
                                    if ui.add_sized(
                                        Vec2::new(ui.available_width(), 58.0),
                                        egui::Button::new(RichText::new("Playlist Manager\nF8 Key").strong().size(12.0).color(Color32::from_rgb(220, 225, 240))),
                                    ).clicked() {
                                        self.show_playlist = true;
                                    }
                                });

                                // Card 2: Stream URL & DSP Studio
                                cols[1].vertical_centered(|ui| {
                                    if ui.add_sized(
                                        Vec2::new(ui.available_width(), 58.0),
                                        egui::Button::new(RichText::new("Stream URL / Web\nCtrl + U").strong().size(12.5).color(Color32::from_rgb(100, 190, 255))),
                                    ).clicked() {
                                        self.show_stream_url_dialog = true;
                                    }
                                    ui.add_space(8.0);
                                    if ui.add_sized(
                                        Vec2::new(ui.available_width(), 58.0),
                                        egui::Button::new(RichText::new("DSP Equalizer Studio\nF7 Key").strong().size(12.0).color(Color32::from_rgb(220, 225, 240))),
                                    ).clicked() {
                                        self.show_control_panel = true;
                                    }
                                });
                            });
                        }

                        // 5. Bottom Keyboard Shortcuts Cheat Sheet Pill
                        let block_bottom = content_top_y + shelf_h_opt.unwrap_or(136.0);
                        if usable_bottom - block_bottom >= 38.0 {
                            let hint_w = 460.0_f32.min(rect.width() - 32.0);
                            let hint_y = ((block_bottom + usable_bottom) * 0.5).clamp(block_bottom + 16.0, usable_bottom - 14.0);
                            let hint_rect = Rect::from_center_size(
                                Pos2::new(usable_center.x, hint_y),
                                Vec2::new(hint_w, 22.0),
                            );
                            let p = ui.painter();
                            p.rect_filled(hint_rect, CornerRadius::same(11), Color32::from_rgba_unmultiplied(18, 20, 28, 190));
                            p.rect_stroke(hint_rect, CornerRadius::same(11), Stroke::new(0.8, Color32::from_rgb(40, 44, 58)), egui::StrokeKind::Inside);
                            p.text(
                                hint_rect.center(),
                                Align2::CENTER_CENTER,
                                "Space: Play/Pause  •  Enter: Fullscreen  •  M: Mute  •  [ / ]: Speed  •  Tab: Stats",
                                FontId::proportional(10.5),
                                Color32::from_rgb(150, 155, 175),
                            );
                        }
                    }

                    // Diagnostics HUD MPV stats update (rendering handled in top layer 4)
                    if let Ok(player) = &self.player {
                        let ppp = ui.ctx().pixels_per_point();
                        let win_w = (rect.width() * ppp).round() as u32;
                        let win_h = (rect.height() * ppp).round() as u32;
                        player.update_osd_diagnostics_hud(self.osd.show_media_info, &stats, win_w, win_h);
                    }

                    // Real-Time Visualizer Suite
                    self.visualizer.render(ui, rect, &stats);

                    // Synchronized Karaoke Lyrics Overlay
                    self.lyrics_overlay.render(ui, rect, stats.time_pos);

                    // A-B Split-Screen Video Comparison
                    self.split_compare.render(ui, rect);

                    // Floating PiP Mode Controls Overlay (Shown on hover)
                    if self.pip_mode.is_pip && response.hovered() {
                        let painter = ui.painter();
                        let pip_top_rect = Rect::from_min_size(rect.left_top(), Vec2::new(rect.width(), 32.0));
                        painter.rect_filled(
                            pip_top_rect,
                            CornerRadius::ZERO,
                            Color32::from_rgba_unmultiplied(12, 14, 20, 210),
                        );

                        painter.text(
                            pip_top_rect.left_center() + Vec2::new(10.0, 0.0),
                            Align2::LEFT_CENTER,
                            "Floating Player",
                            FontId::proportional(11.5),
                            Color32::from_rgb(180, 140, 255),
                        );

                        // Buttons: Exit PiP & Close
                        let btn_restore = Rect::from_min_size(
                            Pos2::new(rect.right() - 65.0, rect.top() + 4.0),
                            Vec2::new(26.0, 24.0),
                        );
                        let restore_resp = ui.interact(btn_restore, ui.id().with("pip_restore_btn"), Sense::click());
                        painter.rect_filled(
                            btn_restore,
                            CornerRadius::same(3),
                            if restore_resp.hovered() { Color32::from_rgb(80, 50, 150) } else { Color32::from_rgb(45, 35, 75) },
                        );
                        painter.text(
                            btn_restore.center(),
                            Align2::CENTER_CENTER,
                            "⛶",
                            FontId::proportional(12.0),
                            Color32::WHITE,
                        );
                        if restore_resp.clicked() {
                            toggle_pip_requested = true;
                        }

                        let btn_close = Rect::from_min_size(
                            Pos2::new(rect.right() - 32.0, rect.top() + 4.0),
                            Vec2::new(26.0, 24.0),
                        );
                        let close_resp = ui.interact(btn_close, ui.id().with("pip_close_btn"), Sense::click());
                        painter.rect_filled(
                            btn_close,
                            CornerRadius::same(3),
                            if close_resp.hovered() { Color32::from_rgb(200, 40, 40) } else { Color32::from_rgb(65, 25, 25) },
                        );
                        painter.text(
                            btn_close.center(),
                            Align2::CENTER_CENTER,
                            "✕",
                            FontId::proportional(11.0),
                            Color32::WHITE,
                        );
                        if close_resp.clicked() {
                            toggle_pip_requested = true;
                        }
                    }

                    // Right-Click Context Menu Trigger (Opens precisely at mouse pointer)
                    // Only opens when right-clicking on the active video canvas, NEVER on bottom bar, seekbar, topbar, or playlist
                    let right_clicked = response.secondary_clicked()
                        || ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Secondary) || i.pointer.secondary_clicked());

                    if right_clicked {
                        let click_pos = ctx.input(|i| {
                            i.pointer.latest_pos()
                                .or_else(|| i.pointer.hover_pos())
                                .or_else(|| i.pointer.press_origin())
                                .or_else(|| i.pointer.interact_pos())
                        });
                        if let Some(pos) = click_pos {
                            let bottom_bar_zone = if is_fullscreen_active {
                                bar_h + 36.0
                            } else if is_song_playback {
                                bar_h + 20.0
                            } else {
                                bar_h + 26.0
                            };
                            let is_over_bottom_bar = show_bottom_menu && pos.y >= screen_h - bottom_bar_zone;
                            let is_over_top_bar = show_top_menu && pos.y <= 38.0;
                            let is_over_docked_pl = is_docked_pl && pos.x >= pl_x;
                            let is_over_popups = self.active_popup_rects.iter().any(|r| r.contains(pos));
                            let is_over_dialog = self.has_open_dialog() || self.show_speed_popup || self.show_audio_channels_popup;
                            let is_in_video_rect = effective_rect.contains(pos);

                            if is_in_video_rect && !is_over_bottom_bar && !is_over_top_bar && !is_over_docked_pl && !is_over_popups && !is_over_dialog {
                                self.menu_position = Some(pos);
                                self.menu_anchor_pos = None;
                                self.show_main_menu = true;
                                self.menu_opened_frame = self.frame_counter;
                            }
                        }
                    }
                });

        // =========================================================================
        // 4. DOCKED SIDEBAR PLAYLIST (Exact width 330.0, sits below 32px titlebar)
        // =========================================================================
        if is_docked_pl {
            let pl_top = 32.0;
            let pl_h = (screen_h - pl_top).max(100.0);
            let pl_rect = Rect::from_min_size(Pos2::new(pl_x, pl_top), Vec2::new(sidebar_w, pl_h));
            self.active_popup_rects.push(pl_rect);

            egui::Area::new(egui::Id::new("vortex_docked_playlist"))
                .fixed_pos(Pos2::new(pl_x, pl_top))
                .order(egui::Order::Foreground)
                .show(&ctx, |ui| {
                    ui.allocate_ui(Vec2::new(sidebar_w, pl_h), |ui| {
                        ui.set_width(sidebar_w);
                        ui.set_height(pl_h);
                        egui::Frame::new()
                            .fill(Color32::from_rgb(14, 15, 18))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(32, 34, 42)))
                            .inner_margin(Margin { left: 6, right: 6, top: 4, bottom: 6 })
                            .show(ui, |ui| {
                                let pl_actions = PlaylistPanel::render(
                                    ui,
                                    &mut *self.playlist.lock().unwrap(),
                                    Some(&stats.file_path),
                                    false,
                                    false,
                                    &mut self.drawer_tab,
                                    &stats,
                                    &self.bookmark_mgr,
                                    &self.subtitle_explorer.lines,
                                    &mut self.drawer_browser_dir,
                                    &mut self.drawer_browser_search,
                                    &mut self.drawer_sub_search,
                                    self.config.restore_last_playlist,
                                );
                                if let Some(path) = pl_actions.play_path {
                                    file_to_open = Some(path);
                                }
                                if let Some(path) = pl_actions.enqueue_path {
                                    self.playlist.lock().unwrap().add_item(path);
                                    self.osd.show("Added to Playlist".to_string(), 1200);
                                }
                                if let Some(seek_t) = pl_actions.seek_to {
                                    if let Ok(ref player) = self.player {
                                        player.seek_absolute(seek_t);
                                    }
                                }
                                if pl_actions.add_bookmark {
                                    self.bookmark_mgr.add_bookmark(stats.time_pos, None);
                                    self.osd.show(format!("Bookmark Added: {}", format_time(stats.time_pos)), 1500);
                                }
                                if let Some(b_idx) = pl_actions.delete_bookmark_idx {
                                    if b_idx < self.bookmark_mgr.bookmarks.len() {
                                        self.bookmark_mgr.bookmarks.remove(b_idx);
                                    }
                                }
                                if pl_actions.add_files {
                                    if let Some(files) = rfd::FileDialog::new().pick_files() {
                                        self.playlist.lock().unwrap().add_items(files);
                                        self.config.last_playlist = self.playlist.lock().unwrap().items.iter().map(|i| i.path.to_string_lossy().to_string()).collect();
                                        self.config.last_playlist_index = self.playlist.lock().unwrap().current_index;
                                        let _ = self.config.save();
                                    }
                                }
                                if pl_actions.add_folder {
                                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                        self.open_folder(folder);
                                    }
                                }
                                if pl_actions.clear_playlist {
                                    self.playlist.lock().unwrap().items.clear();
                                    self.playlist.lock().unwrap().current_index = None;
                                    self.config.last_playlist.clear();
                                    self.config.last_playlist_index = None;
                                    let _ = self.config.save();
                                }
                                if pl_actions.add_url {
                                    self.show_stream_url_dialog = true;
                                }
                                if pl_actions.cycle_repeat {
                                    let mode = self.playlist.lock().unwrap().repeat_mode;
                                    self.config.playlist_repeat_mode = match mode {
                                        crate::playlist::RepeatMode::RepeatAll => "RepeatAll".to_string(),
                                        crate::playlist::RepeatMode::RepeatTrack => "RepeatTrack".to_string(),
                                        _ => "Off".to_string(),
                                    };
                                    let _ = self.config.save();
                                    let msg = match mode {
                                        crate::playlist::RepeatMode::RepeatAll => "Repeat: All Tracks",
                                        crate::playlist::RepeatMode::RepeatTrack => "Repeat: Current Track",
                                        crate::playlist::RepeatMode::Off => "Repeat: Off",
                                    };
                                    self.osd.show(msg.to_string(), 1500);
                                }
                                if pl_actions.toggle_shuffle {
                                    let is_shuf = self.playlist.lock().unwrap().is_shuffle();
                                    self.config.playlist_shuffle = is_shuf;
                                    let _ = self.config.save();
                                    let msg = if is_shuf { "Shuffle: ON" } else { "Shuffle: OFF" };
                                    self.osd.show(msg.to_string(), 1500);
                                }
                                if pl_actions.toggle_restore_prev {
                                    self.config.restore_last_playlist = !self.config.restore_last_playlist;
                                    let _ = self.config.save();
                                    let msg = if self.config.restore_last_playlist {
                                        "Restore Prev Playlist: Enabled"
                                    } else {
                                        "Restore Prev Playlist: Disabled"
                                    };
                                    self.osd.show(msg.to_string(), 1500);
                                }
                                if pl_actions.toggle_detach {
                                    self.config.playlist_detached = true;
                                    let _ = self.config.save();
                                    self.toast.info("Playlist: Detached (Floating Window)");
                                }
                                if pl_actions.close_playlist {
                                    self.toggle_playlist_unified(&ctx);
                                }
                            });
                    });
                });
        }

        // =========================================================================
        // EXTENDED POWER DIALOGS & OVERLAYS
        // =========================================================================
        let mut seek_target: Option<f64> = None;

        if self.show_peq_dialog {
            if let Ok(ref player) = self.player {
                if let Some(r) = self.peq_dialog.render(&ctx, &mut self.show_peq_dialog, &mut self.config.parametric_eq, player) {
                    self.active_popup_rects.push(r);
                }
            }
        }

        if self.show_jump_time_dialog {
            if let Ok(ref player) = self.player {
                if let Some(r) = self.jump_time_dialog.render(&ctx, &mut self.show_jump_time_dialog, &stats, player) {
                    self.active_popup_rects.push(r);
                }
            }
        }

        if self.bookmark_studio.is_open {
            self.bookmark_studio.render(&ctx, &mut self.bookmark_mgr, &stats.file_path, &mut seek_target);
        }

        if self.subtitle_explorer.is_open {
            self.subtitle_explorer.render(&ctx, &mut seek_target);
        }

        if self.settings_inspector.is_open {
            if let Some(r) = self.settings_inspector.render(&ctx) {
                self.active_popup_rects.push(r);
            }
        }

        if self.input_editor.is_open {
            self.input_editor.render(&ctx, &mut self.input_profiles);
        }

        if self.file_navigator.is_open {
            self.file_navigator.render(&ctx, &mut file_to_open);
        }

        if self.show_library_view {
            let (p, rect_opt) = self.library_view.render(&ctx, &mut self.show_library_view, &mut self.library);
            if let Some(p) = p {
                file_to_open = Some(p);
            }
            if let Some(r) = rect_opt {
                self.active_popup_rects.push(r);
            }
        }

        if self.network_browser.is_open {
            self.network_browser.render(&ctx, &mut file_to_open);
        }

                if let Ok(ref player) = self.player {
            self.record_dialog.render(&ctx, player);
        }
        self.contact_sheet_dialog.render(&ctx, &stats);

                self.gif_maker.render(&ctx, &stats);
        if let Ok(ref player) = self.player {
            self.device_capture.render(&ctx, player);
        }
        if let Some(r) = self.subtitle_lookup.render(&ctx, &self.config.subtitle_search_engine) {
            self.active_popup_rects.push(r);
        }

                if let Ok(ref player) = self.player {
            self.shader_studio.render(&ctx, player);
            self.stereo_3d_dialog.render(&ctx, player);
            self.karaoke_studio.render(&ctx, player);
            self.subtitle_studio.render(&ctx, player);
            self.screen_capture.render(&ctx, player);
            self.iptv_guide.render(&ctx, player);
            self.logo_watermark.render(&ctx, player);
            self.disc_nav.render(&ctx, player);
            self.cd_ripper.render(&ctx, player);
            self.bda_tuner.render(&ctx, player);
            self.bdj_dialog.render(&ctx, player);
            self.vst_winamp.render(&ctx, player);
            self.web_remote.render(&ctx, player, &stats);
            self.media_server.render(&ctx, player);
            self.cast_renderer.render(&ctx, player, &stats);
            self.zoom_magnifier.render(&ctx, player);
            self.radio_directory.render(&ctx, player);
            self.video_puzzle.render(&ctx, player);
            self.vr_360_studio.render(&ctx, player);
            self.damaged_file_repair.render(&ctx, player, &stats);
            self.binaural_crossfeed.render(&ctx, player);
            self.video_wall_matrix.render(&ctx, player);
            self.chapter_marker_dialog.render(&ctx, player, &stats, &mut self.config);
            self.audio_compressor.render(&ctx, player);
            self.video_crop.render(&ctx, player);
            self.goto_frame.render(&ctx, player, &stats);
            self.deinterlace_dialog.render(&ctx, player);
            self.ab_repeat_dialog.render(&ctx, player, &stats);
            self.motion_interpolation.render(&ctx, player, &stats);
            self.color_lut_dialog.render(&ctx, player);
            self.ai_upscaling.render(&ctx, player, &stats);
            self.ai_audio.render(&ctx, player);
            self.loudness_radar.render(&ctx, player);
            self.frame_dumper.render(&ctx, player, &stats);
            self.color_blindness.render(&ctx, player);
            self.motion_vector_inspector.render(&ctx, player, &stats);
            self.rich_bookmark_notes.render(&ctx, player, &stats);
            self.pitch_formant.render(&ctx, player);
        }

        self.boss_key_dialog.render(&ctx);
        self.subtitle_translator.render(&ctx, &stats);
        self.ai_subtitle.render(&ctx, &stats);
        self.cinema_matte.render(&ctx);

        self.detached_windows.render(&ctx);



        self.discord_rpc.render(&ctx, &stats);
        self.scrobbler_dialog.render(&ctx);
        self.ambilight_dialog.render(&ctx);
        self.transcoder_dialog.render(&ctx, &stats);
        if let Some(r) = self.playback_history.render(&ctx, &mut file_to_open) {
            self.active_popup_rects.push(r);
        }
        if let Some(r) = self.media_tag_editor.render(&ctx, &stats) {
            self.active_popup_rects.push(r);
        }






        self.broadcast_dialog.render(&ctx, &stats);
        self.channel_matrix_dialog.render(&ctx, &mut self.channel_matrix_config);
        if self.channel_matrix_dialog.open_surround_eq {
            self.channel_matrix_dialog.open_surround_eq = false;
            self.show_surround_eq_dialog = true;
        }
        self.chain_editor.render(&ctx, &mut self.video_chain, &mut self.audio_chain);
        if let Some(r) = self.auto_skip_dialog.render(
            &ctx,
            &mut self.bookmark_mgr,
            &mut self.config,
            &stats.chapters,
            self.player.as_ref().ok(),
            stats.time_pos,
            stats.duration,
        ) {
            self.active_popup_rects.push(r);
        }

        if self.show_telemetry_hud {
            TelemetryHudView::render(&ctx, self.show_telemetry_hud, &stats);
        }

        if let Some(st) = seek_target {
            if let Ok(ref player) = self.player {
                player.seek_absolute(st);
            }
        }

        // System Media Transport Controls (SMTC)
        if !self.smtc.is_initialized && self.parent_hwnd != 0 {
            self.smtc.init(self.parent_hwnd);
        }

        for cmd in self.smtc.poll_events() {
            if let Ok(ref player) = self.player {
                match cmd {
                    crate::engine::SmtcCommand::Play => player.play(),
                    crate::engine::SmtcCommand::Pause => player.pause(),
                    crate::engine::SmtcCommand::Toggle => {
                        player.toggle_pause();
                        self.osd.show_play_pause(!stats.is_paused);
                    }
                    crate::engine::SmtcCommand::Stop => player.stop(),
                    crate::engine::SmtcCommand::Next => {
                        let next_opt = self.playlist.lock().unwrap().next();
                        if let Some(next_file) = next_opt {
                            self.open_media_file(next_file);
                        }
                    }
                    crate::engine::SmtcCommand::Prev => {
                        let prev_opt = self.playlist.lock().unwrap().previous();
                        if let Some(prev_file) = prev_opt {
                            self.open_media_file(prev_file);
                        }
                    }
                    crate::engine::SmtcCommand::Seek(pos) => {
                        player.seek_absolute(pos.as_secs_f64());
                    }
                }
            }
        }

        let display_title = if !stats.title.is_empty() {
            stats.title.clone()
        } else if !stats.file_path.is_empty() {
            std::path::Path::new(&stats.file_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "VortexPlayer".to_string())
        } else {
            "VortexPlayer".to_string()
        };

        let artist_opt = if !stats.artist.is_empty() {
            Some(stats.artist.as_str())
        } else {
            None
        };

        let album_opt = if !stats.album.is_empty() {
            Some(stats.album.as_str())
        } else {
            None
        };

        let cover_url_str = self.music_view.cached_cover_path.as_ref().and_then(|p| {
            if p.exists() {
                Some(format!("file:///{}", p.to_string_lossy().replace('\\', "/")))
            } else {
                None
            }
        });

        self.smtc.update_metadata(
            &display_title,
            artist_opt,
            album_opt,
            cover_url_str.as_deref(),
            stats.duration,
            stats.time_pos,
            stats.is_paused,
            stats.is_idle,
        );

        if self.parent_hwnd != 0 {
            self.taskbar.update_progress(
                self.parent_hwnd,
                stats.time_pos,
                stats.duration,
                stats.is_paused,
                stats.is_idle,
            );
            self.taskbar.update_thumbbar(self.parent_hwnd, stats.is_paused, stats.is_idle);
        }

        if stats.duration > 0.0 {
            let mut pl = self.playlist.lock().unwrap();
            if let Some(cur_idx) = pl.current_index {
                if let Some(item) = pl.items.get_mut(cur_idx) {
                    if item.duration_secs.is_none() || item.duration_secs == Some(0.0) {
                        item.duration_secs = Some(stats.duration);
                    }
                }
            }
        }

        // 4. OSD & TOAST NOTIFICATIONS OVERLAY (Top layer)
        // =========================================================================
        self.osd.is_new_message = false;
        self.osd.render_ctx(&ctx, self.last_video_rect, &stats);

        // Floating "Skip Intro / OP" Button (Netflix / Crunchyroll style)
        let mut do_skip_intro = false;
        let mut skip_intro_active_rect: Option<Rect> = None;
        if !is_song_mode && !stats.is_idle && stats.duration > 30.0 {
            let current_ch_idx = stats.chapters.iter().position(|c| {
                if let Some(next_c) = stats.chapters.iter().find(|n| n.index == c.index + 1) {
                    stats.time_pos >= c.time_pos && stats.time_pos < next_c.time_pos
                } else {
                    stats.time_pos >= c.time_pos
                }
            });

            let current_ch = current_ch_idx.and_then(|idx| stats.chapters.get(idx));

            let is_op_or_intro = if let Some(ch) = current_ch {
                self.config.matches_skip_chapter(&ch.title)
            } else {
                stats.time_pos < 90.0 && stats.duration > 180.0
            };

            let seg_key = if is_op_or_intro {
                Some(format!("{}:{:?}", stats.file_path, current_ch_idx))
            } else {
                None
            };

            if let Some(key) = seg_key {
                // If entering an intro segment for the first time, start 5-second timer
                if self.intro_segment_key.as_ref() != Some(&key) {
                    self.intro_segment_key = Some(key);
                    self.intro_btn_shown_at = Some(std::time::Instant::now());
                } else if stats.time_pos + 4.0 < self.intro_last_time_pos || (stats.time_pos < 2.0 && self.intro_last_time_pos >= 2.0) {
                    // User sought backwards significantly or back to start: re-show for 5 seconds
                    self.intro_btn_shown_at = Some(std::time::Instant::now());
                }
            } else {
                self.intro_segment_key = None;
                self.intro_btn_shown_at = None;
            }
            self.intro_last_time_pos = stats.time_pos;

            if let Some(shown_at) = self.intro_btn_shown_at {
                let bar_h = if is_fullscreen_active { ControlBar::HEIGHT_FULLSCREEN } else { ControlBar::HEIGHT_COMPACT };
                let btn_w = 132.0;
                let btn_h = 32.0;
                let btn_pos = Pos2::new(screen_w - btn_w - 24.0, screen_h - bar_h - btn_h - 16.0);
                let btn_rect = Rect::from_min_size(btn_pos, Vec2::new(btn_w, btn_h));

                let is_hovered = ctx.input(|i| {
                    i.pointer.hover_pos().map_or(false, |pos| btn_rect.contains(pos))
                });

                // While user hovers over the button, hold the timer so it doesn't vanish while clicking
                if is_hovered {
                    self.intro_btn_shown_at = Some(std::time::Instant::now() - std::time::Duration::from_millis(3500));
                }

                let elapsed = shown_at.elapsed().as_secs_f32();
                if elapsed < 5.0 || is_hovered {
                    skip_intro_active_rect = Some(btn_rect);

                    // Smoothly fade out during the last 0.5s (between 4.5s and 5.0s)
                    let alpha = if elapsed >= 4.5 && !is_hovered {
                        ((5.0 - elapsed) / 0.5).clamp(0.0, 1.0)
                    } else {
                        1.0
                    };

                    egui::Area::new(egui::Id::new("vortex_floating_skip_intro_btn"))
                        .fixed_pos(btn_pos)
                        .order(egui::Order::Foreground)
                        .show(&ctx, |ui| {
                            let resp = ui.interact(btn_rect, ui.id().with("btn_skip_intro_osd"), egui::Sense::click());
                            let p = ui.painter();
                            let bg = if resp.hovered() {
                                Color32::from_rgba_unmultiplied(32, 38, 52, (240.0 * alpha) as u8)
                            } else {
                                Color32::from_rgba_unmultiplied(18, 22, 32, (220.0 * alpha) as u8)
                            };
                            let stroke_col = if resp.hovered() {
                                Color32::from_rgba_unmultiplied(255, 197, 36, (255.0 * alpha) as u8)
                            } else {
                                Color32::from_rgba_unmultiplied(255, 255, 255, (60.0 * alpha) as u8)
                            };
                            let text_col = if resp.hovered() {
                                Color32::from_rgba_unmultiplied(255, 197, 36, (255.0 * alpha) as u8)
                            } else {
                                Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * alpha) as u8)
                            };

                            p.rect_filled(btn_rect, CornerRadius::same(6), bg);
                            p.rect_stroke(btn_rect, CornerRadius::same(6), Stroke::new(1.0, stroke_col), egui::StrokeKind::Inside);
                            p.text(
                                btn_rect.center(),
                                Align2::CENTER_CENTER,
                                "⏭ Skip Intro (S)",
                                FontId::proportional(12.0),
                                text_col,
                            );

                            if resp.clicked() {
                                do_skip_intro = true;
                            }
                        });

                    // Continuously repaint while countdown/fade is active
                    ctx.request_repaint_after(std::time::Duration::from_millis(50));
                }
            }
        }

        // Render toasts with collision avoidance against floating playback controls
        let screen_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(screen_w, screen_h));
        self.toast.anchor = self.config.toast_position;
        self.toast.render_with_avoidance(&ctx, screen_rect, skip_intro_active_rect);

        if do_skip_intro {
            self.skip_intro_or_op(&stats);
        }

        // =========================================================================
        // 5. MODAL DIALOGS & POPUPS
        // =========================================================================
        if self.show_main_menu {
            let win_size = get_window_client_rect(&ctx).size();
            let raw_pos = self.menu_position.unwrap_or(Pos2::new(6.0, 32.0));
            let menu_w = 260.0;
            let cached_h: Option<f32> = ctx.data(|d| d.get_temp(egui::Id::new("vortex_context_menu_height")));
            let full_menu_h = cached_h.unwrap_or(440.0);

            // X: open directly at cursor / button, flip left if overflowing right edge
            let clamped_x = if raw_pos.x + menu_w > win_size.x - 6.0 {
                (raw_pos.x - menu_w).max(4.0)
            } else {
                raw_pos.x.max(4.0)
            };

            // Y: If full menu fits in window height, shift up just enough so it never overflows bottom edge.
            // When menu fits, we NEVER want it to scroll!
            let available_window_h = (win_size.y - 12.0).max(100.0);
            let menu_fits_in_window = available_window_h >= full_menu_h;

            let clamped_y = if menu_fits_in_window {
                if raw_pos.y + full_menu_h > win_size.y - 6.0 {
                    (win_size.y - full_menu_h - 6.0).max(4.0)
                } else {
                    raw_pos.y.max(4.0)
                }
            } else {
                4.0
            };

            let menu_pos = Pos2::new(clamped_x, clamped_y);
            let available_menu_h = (win_size.y - clamped_y - 6.0).max(120.0);
            self.active_popup_rects.push(Rect::from_min_size(menu_pos, Vec2::new(menu_w, available_menu_h)));
            let player_opt = self.player.as_ref().ok().map(|p| p.as_ref());
            let mut close_menu = false;
            let mut menu_toggle_fullscreen = false;
            let mut menu_exit_fullscreen = false;
            let mut menu_resize_window_factor: Option<f32> = None;
            let mut menu_center_window = false;
            let mut menu_skip_intro = false;
            let mut menu_toggle_playlist = false;
            let mut menu_close_file = false;
            let mut menu_toggle_pip = false;

            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_menu = true;
            }

            let area_resp = egui::Area::new(egui::Id::new("vortex_context_menu").with(self.menu_opened_frame))
                .order(egui::Order::Tooltip)
                .fixed_pos(menu_pos)
                .pivot(egui::Align2::LEFT_TOP)
                .movable(false)
                .show(&ctx, |ui| {
                    ui.style_mut().animation_time = 0.0;
                    egui::Frame::new()
                        .fill(Color32::from_rgba_premultiplied(16, 18, 26, 252))
                        .stroke(Stroke::NONE)
                        .corner_radius(CornerRadius::ZERO)
                        .shadow(egui::Shadow {
                            offset: [0, 8],
                            blur: 32,
                            spread: 2,
                            color: Color32::from_black_alpha(210),
                        })
                        .inner_margin(Margin::symmetric(4, 6))
                        .show(ui, |ui| {
                    let menu_actions = if menu_fits_in_window {
                        // When the menu fits, render it directly without ScrollArea so it NEVER scrolls or truncates!
                        VortexMenu::render(
                            ui,
                            player_opt,
                            &stats,
                            &mut *self.playlist.lock().unwrap(),
                            &mut self.config,
                            &mut self.bookmark_mgr,
                        )
                    } else {
                        // Only scroll if window is physically too small to fit the menu
                        egui::ScrollArea::vertical()
                            .max_height(available_menu_h)
                            .auto_shrink([true, true])
                            .show(ui, |ui| {
                                VortexMenu::render(
                                    ui,
                                    player_opt,
                                    &stats,
                                    &mut *self.playlist.lock().unwrap(),
                                    &mut self.config,
                                    &mut self.bookmark_mgr,
                                )
                            })
                            .inner
                    };

                    if menu_actions.close_menu {
                        self.show_main_menu = false;
                    }

                    // All fullscreen entry/exit requests go through VortexApp so
                    // the native video child and egui use the same bounds.
                    if menu_actions.toggle_fullscreen {
                        menu_toggle_fullscreen = true;
                    }
                    if menu_actions.exit_fullscreen {
                        menu_exit_fullscreen = true;
                    }
                    if let Some(factor) = menu_actions.resize_window_factor {
                        menu_resize_window_factor = Some(factor);
                    }
                    if menu_actions.center_window {
                        menu_center_window = true;
                    }

                    if menu_actions.open_file {
                        self.show_main_menu = false;
                        if let Some(file) = rfd::FileDialog::new().pick_file() {
                            file_to_open = Some(file);
                        }
                    }
                    if menu_actions.open_folder {
                        self.show_main_menu = false;
                        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                            folder_to_open = Some(folder);
                        }
                    }
                    if menu_actions.open_url {
                        self.show_main_menu = false;
                        self.show_stream_url_dialog = true;
                    }
                    if menu_actions.load_subtitle {
                        self.show_main_menu = false;
                        if let Some(sub) = rfd::FileDialog::new().add_filter("Subtitles", &["srt", "ass", "ssa", "vtt", "smi"]).pick_file() {
                            if let Some(p) = player_opt {
                                if sub.to_string_lossy().ends_with(".smi") {
                                    if let Ok(srt) = SamiParser::load_sami_file_as_srt(&sub) {
                                        let temp_srt = std::env::temp_dir().join("vortex_loaded_smi.srt");
                                        let _ = std::fs::write(&temp_srt, srt);
                                        p.load_external_subtitle(&temp_srt.to_string_lossy());
                                    }
                                } else {
                                    p.load_external_subtitle(&sub.to_string_lossy());
                                }
                            }
                        }
                    }
                    if menu_actions.toggle_playlist {
                            self.show_main_menu = false;
                            menu_toggle_playlist = true;
                        }
                        if menu_actions.toggle_control_panel {
                            self.show_main_menu = false;
                            self.show_control_panel = !self.show_control_panel;
                        }
                        if menu_actions.toggle_preferences {
                            self.show_main_menu = false;
                            self.show_preferences = !self.show_preferences;
                        }
                        if menu_actions.open_subtitle_preferences {
                            self.show_main_menu = false;
                            self.show_preferences = true;
                            self.preferences_dialog.active_category = "Subtitles".to_string();
                            self.preferences_dialog.active_sub_category = "Font & Typography".to_string();
                        }
                        if menu_actions.toggle_bookmark_overlay {
                            self.show_main_menu = false;
                            self.show_bookmark_overlay = !self.show_bookmark_overlay;
                        }
                        if menu_actions.toggle_media_info {
                            self.show_main_menu = false;
                            self.show_mediainfo_dialog = true;
                        }
                        if menu_actions.open_file_info {
                            self.show_main_menu = false;
                            self.show_mediainfo_dialog = true;
                            self.mediainfo_dialog.selected_tab = 1;
                        }
                        if menu_actions.toggle_stereo_3d {
                            self.show_main_menu = false;
                            self.stereo_3d_dialog.is_open = true;
                        }
                        if menu_actions.toggle_karaoke {
                            self.show_main_menu = false;
                            self.karaoke_studio.is_open = true;
                        }
                        if menu_actions.toggle_subtitle_studio {
                            self.show_main_menu = false;
                            self.subtitle_studio.is_open = true;
                        }
                        if menu_actions.toggle_screen_capture {
                            self.show_main_menu = false;
                            self.screen_capture.is_open = true;
                        }
                        if menu_actions.toggle_iptv_guide {
                            self.show_main_menu = false;
                            self.iptv_guide.is_open = true;
                        }
                        if menu_actions.toggle_logo_watermark {
                            self.show_main_menu = false;
                            self.logo_watermark.is_open = true;
                        }
                        if menu_actions.toggle_disc_nav {
                            self.show_main_menu = false;
                            self.disc_nav.is_open = true;
                        }
                        if menu_actions.toggle_cd_ripper {
                            self.show_main_menu = false;
                            self.cd_ripper.is_open = true;
                        }
                        if menu_actions.toggle_bda_tuner {
                            self.show_main_menu = false;
                            self.bda_tuner.is_open = true;
                        }
                        if menu_actions.toggle_bdj_studio {
                            self.show_main_menu = false;
                            self.bdj_dialog.is_open = true;
                        }
                        if menu_actions.toggle_vst_winamp {
                            self.show_main_menu = false;
                            self.vst_winamp.is_open = true;
                        }
                        if menu_actions.toggle_detached_windows {
                            self.show_main_menu = false;
                            self.detached_windows.is_open = true;
                        }
                        if menu_actions.toggle_discord_rpc {
                            self.show_main_menu = false;
                            self.discord_rpc.is_open = true;
                        }
                        if menu_actions.toggle_web_remote {
                            self.show_main_menu = false;
                            self.web_remote.is_open = true;
                        }
                        if menu_actions.toggle_scrobbler {
                            self.show_main_menu = false;
                            self.scrobbler_dialog.is_open = true;
                        }
                        if menu_actions.toggle_media_server {
                            self.show_main_menu = false;
                            self.media_server.is_open = true;
                        }
                        if menu_actions.toggle_ambilight {
                            self.show_main_menu = false;
                            self.ambilight_dialog.is_open = true;
                        }
                        if menu_actions.toggle_cast_renderer {
                            self.show_main_menu = false;
                            self.cast_renderer.is_open = true;
                        }
                        if menu_actions.toggle_transcoder {
                            self.show_main_menu = false;
                            self.transcoder_dialog.is_open = true;
                        }
                        if menu_actions.toggle_zoom_magnifier {
                            self.show_main_menu = false;
                            self.zoom_magnifier.is_open = true;
                        }
                        if menu_actions.toggle_radio_directory {
                            self.show_main_menu = false;
                            self.radio_directory.is_open = true;
                        }
                        if menu_actions.toggle_video_puzzle {
                            self.show_main_menu = false;
                            self.video_puzzle.is_open = true;
                        }
                        if menu_actions.toggle_vr_360 {
                            self.show_main_menu = false;
                            self.vr_360_studio.is_open = true;
                        }
                        if menu_actions.toggle_damaged_file_repair {
                            self.show_main_menu = false;
                            self.damaged_file_repair.is_open = true;
                        }
                        if menu_actions.toggle_binaural_crossfeed {
                            self.show_main_menu = false;
                            self.binaural_crossfeed.is_open = true;
                        }
                        if menu_actions.toggle_video_wall {
                            self.show_main_menu = false;
                            self.video_wall_matrix.is_open = true;
                        }
                        if menu_actions.toggle_chapter_marker {
                            self.show_main_menu = false;
                            self.chapter_marker_dialog.is_open = true;
                        }
                        if menu_actions.toggle_auto_skip_dialog {
                            self.show_main_menu = false;
                            self.auto_skip_dialog.is_open = true;
                            ctx.request_repaint();
                        }
                        if menu_actions.skip_intro {
                            self.show_main_menu = false;
                            menu_skip_intro = true;
                        }
                        if menu_actions.toggle_audio_compressor {
                            self.show_main_menu = false;
                            self.audio_compressor.is_open = true;
                        }
                        if menu_actions.toggle_video_crop {
                            self.show_main_menu = false;
                            self.video_crop.is_open = true;
                        }
                        if menu_actions.toggle_goto_frame {
                            self.show_main_menu = false;
                            self.goto_frame.is_open = true;
                        }
                        if menu_actions.toggle_playback_history {
                            self.show_main_menu = false;
                            self.playback_history.is_open = true;
                            ctx.request_repaint();
                        }
                        if menu_actions.toggle_media_tag_editor {
                            self.show_main_menu = false;
                            self.media_tag_editor.is_open = true;
                            ctx.request_repaint();
                        }
                        if menu_actions.close_file {
                            self.show_main_menu = false;
                            menu_close_file = true;
                        }
                        if menu_actions.toggle_deinterlace {
                            self.show_main_menu = false;
                            self.deinterlace_dialog.is_open = true;
                        }
                        if menu_actions.toggle_ab_repeat {
                            self.show_main_menu = false;
                            self.ab_repeat_dialog.is_open = true;
                        }
                        if menu_actions.toggle_boss_key {
                            self.show_main_menu = false;
                            if let Ok(player) = &self.player {
                                self.boss_key_dialog.trigger_boss_key(player, &ctx);
                            }
                            self.boss_key_dialog.is_open = true;
                        }
                        if menu_actions.toggle_motion_interpolation {
                            self.show_main_menu = false;
                            self.motion_interpolation.is_open = true;
                        }
                        if menu_actions.toggle_color_lut {
                            self.show_main_menu = false;
                            self.color_lut_dialog.is_open = true;
                        }
                        if menu_actions.toggle_subtitle_translator {
                            self.show_main_menu = false;
                            self.subtitle_translator.is_open = true;
                        }
                        if menu_actions.toggle_ai_subtitle {
                            self.show_main_menu = false;
                            self.ai_subtitle.is_open = true;
                        }
                        if menu_actions.toggle_ai_upscaling {
                            self.show_main_menu = false;
                            self.ai_upscaling.is_open = true;
                        }
                        if menu_actions.toggle_ai_audio {
                            self.show_main_menu = false;
                            self.ai_audio.is_open = true;
                        }
                        if menu_actions.toggle_loudness_radar {
                            self.show_main_menu = false;
                            self.loudness_radar.is_open = true;
                        }
                        if menu_actions.toggle_frame_dumper {
                            self.show_main_menu = false;
                            self.frame_dumper.is_open = true;
                        }
                        if menu_actions.toggle_color_blindness {
                            self.show_main_menu = false;
                            self.color_blindness.is_open = true;
                        }
                        if menu_actions.toggle_cinema_matte {
                            self.show_main_menu = false;
                            self.cinema_matte.is_open = true;
                        }
                        if menu_actions.toggle_motion_vector_inspector {
                            self.show_main_menu = false;
                            self.motion_vector_inspector.is_open = true;
                        }
                        if menu_actions.toggle_rich_bookmark_notes {
                            self.show_main_menu = false;
                            self.rich_bookmark_notes.is_open = true;
                        }
                        if menu_actions.toggle_pitch_formant {
                            self.show_main_menu = false;
                            self.pitch_formant.is_open = true;
                        }
                        if menu_actions.show_about {
                            self.show_main_menu = false;
                            self.show_about_dialog = true;
                        }
                        if menu_actions.toggle_record_dialog {
                            self.show_main_menu = false;
                            self.record_dialog.is_open = true;
                        }
                        if menu_actions.toggle_gif_maker {
                            self.show_main_menu = false;
                            self.gif_maker.is_open = true;
                        }
                        if menu_actions.toggle_device_capture {
                            self.show_main_menu = false;
                            self.device_capture.is_open = true;
                        }
                        if menu_actions.toggle_subtitle_lookup {
                            self.show_main_menu = false;
                            self.subtitle_lookup.is_open = true;
                        }
                        if menu_actions.toggle_shader_studio {
                            self.show_main_menu = false;
                            self.shader_studio.is_open = true;
                        }
                        if menu_actions.toggle_broadcast {
                            self.show_main_menu = false;
                            self.broadcast_dialog.is_open = true;
                        }
                        if menu_actions.toggle_contact_sheet {
                            self.show_main_menu = false;
                            self.contact_sheet_dialog.is_open = true;
                        }
                        if menu_actions.toggle_jump_time {
                            self.show_main_menu = false;
                            self.show_jump_time_dialog = true;
                        }
                        if menu_actions.toggle_pip {
                            self.show_main_menu = false;
                            menu_toggle_pip = true;
                        }
                        if menu_actions.minimize_window {
                            self.show_main_menu = false;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                        if menu_actions.maximize_window {
                            self.show_main_menu = false;
                            let is_max = is_window_maximized_or_workarea(self.parent_hwnd, &ctx);
                            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_max));
                        }

                        if menu_actions.take_screenshot {
                            self.show_main_menu = false;
                            if let Some(pic_dir) = dirs::picture_dir() {
                                let path = pic_dir.join(format!("VertexPlayer_{}.jpg", chrono::Utc::now().format("%Y%m%d_%H%M%S")));
                                if let Some(p) = player_opt {
                                    p.take_screenshot(&path);
                                }
                                self.osd.show("📸 Snapshot saved to Pictures".to_string(), 1500);
                            }
                        }
                        if menu_actions.sub_sync_changed {
                            if player_opt.is_some() {
                                self.osd.show_sub_delay(stats.subtitle_delay);
                            }
                        }
                        if let Some(p) = menu_actions.play_path {
                            self.show_main_menu = false;
                            file_to_open = Some(p);
                        }
                        if menu_actions.toggle_sleep_timer {
                            self.show_main_menu = false;
                            self.show_sleep_timer_dialog = !self.show_sleep_timer_dialog;
                        }
                        if menu_actions.toggle_visualizer {
                            self.show_main_menu = false;
                            self.visualizer.toggle();
                            let state = if self.visualizer.is_visible { "ON" } else { "OFF" };
                            self.osd.show(format!("Visualizer Suite: {}", state), 1200);
                        }
                        if menu_actions.cycle_visualizer_mode {
                            self.show_main_menu = false;
                            self.visualizer.is_visible = true;
                            let mode = self.visualizer.cycle_mode();
                            self.osd.show(format!("Visualizer Mode: {}", mode.display_name()), 1200);
                        }
                        if let Some(mode) = menu_actions.set_visualizer_mode {
                            self.show_main_menu = false;
                            self.visualizer.mode = mode;
                            self.visualizer.is_visible = true;
                            self.osd.show(format!("Visualizer Mode: {}", mode.display_name()), 1200);
                        }
                        if menu_actions.check_updates {
                            self.show_main_menu = false;
                            self.show_update_dialog = true;
                            self.update_dialog.check_for_updates();
                        }
                        if menu_actions.toggle_surround_eq {
                            self.show_main_menu = false;
                            self.show_surround_eq_dialog = !self.show_surround_eq_dialog;
                        }

                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.show_main_menu = false;
                    }
                });
            });

            let main_menu_rect = area_resp.response.rect;
            if main_menu_rect.height() > 50.0 {
                ctx.data_mut(|d| d.insert_temp(egui::Id::new("vortex_context_menu_height"), main_menu_rect.height()));
            }
            self.active_popup_rects.push(main_menu_rect);

            if menu_toggle_fullscreen {
                self.show_main_menu = false;
                self.toggle_fullscreen(&ctx);
            }
            if menu_exit_fullscreen {
                self.show_main_menu = false;
                self.set_fullscreen(&ctx, false);
            }
            if let Some(factor) = menu_resize_window_factor {
                self.show_main_menu = false;
                self.resize_window_to_preset(&ctx, factor, &stats);
            }
            if menu_center_window {
                self.show_main_menu = false;
                self.center_window_with_monitor_aspect_ratio(&ctx);
            }
            if menu_toggle_playlist {
                self.toggle_playlist_unified(&ctx);
            }
            if menu_skip_intro {
                self.show_main_menu = false;
                self.skip_intro_or_op(&stats);
            }
            if menu_close_file {
                self.show_main_menu = false;
                self.close_current_media(&mut stats, &ctx);
            }
            if menu_toggle_pip {
                self.show_main_menu = false;
                self.toggle_pip(&ctx);
            }

            let latest_sub_rect: Option<Rect> = ctx.data(|d| d.get_temp(egui::Id::new("latest_active_submenu_rect"))).flatten();
            if let Some(sub_r) = latest_sub_rect {
                self.active_popup_rects.push(sub_r);
            }

            // Dismiss menu when clicking anywhere outside of the context menu and submenus (ignore the opening frame)
            let just_opened = self.menu_opened_frame >= self.frame_counter.saturating_sub(1);
            if !just_opened {
                if let Some(pos) = ctx.input(|i| {
                    if i.pointer.button_clicked(egui::PointerButton::Primary) || i.pointer.primary_clicked() {
                        i.pointer.latest_pos().or(i.pointer.hover_pos())
                    } else {
                        None
                    }
                }) {
                    let inside_main = main_menu_rect.contains(pos);
                    let inside_sub = latest_sub_rect.map(|r| r.contains(pos)).unwrap_or(false);
                    let inside_any_popup = self.active_popup_rects.iter().any(|r| r.contains(pos));
                    if !inside_main && !inside_sub && !inside_any_popup {
                        close_menu = true;
                    }
                }
            }

            if close_menu {
                self.show_main_menu = false;
                self.menu_position = None;
                ctx.data_mut(|d| d.insert_temp(egui::Id::new("latest_active_submenu_rect"), None::<Rect>));
            }
        }

        if self.show_audio_channels_popup {
            let win_size = get_window_client_rect(&ctx).size();
            let raw_pos = self.audio_channels_popup_pos.unwrap_or(Pos2::new(win_size.x * 0.5, win_size.y - 60.0));
            let popup_w = 285.0;
            let popup_h = 300.0;

            let max_x = (win_size.x - popup_w - 8.0).max(8.0);
            let max_y = (win_size.y - popup_h - 8.0).max(34.0);
            let clamped_x = (raw_pos.x - popup_w * 0.5).clamp(8.0, max_x);
            let clamped_y = (raw_pos.y - popup_h - 8.0).clamp(34.0, max_y);
            let popup_pos = Pos2::new(clamped_x, clamped_y);
            self.active_popup_rects.push(Rect::from_min_size(popup_pos, Vec2::new(popup_w, popup_h)));

            let mut close_channels_popup = false;
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_channels_popup = true;
            }

            let area_resp = egui::Area::new(egui::Id::new("audio_channels_selector_popup"))
                .order(egui::Order::Tooltip)
                .fixed_pos(popup_pos)
                .show(&ctx, |ui| {
                    egui::Frame::new()
                        .fill(Color32::from_rgb(16, 18, 24))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(44, 48, 60)))
                        .corner_radius(CornerRadius::same(6))
                        .shadow(egui::Shadow {
                            offset: [0, 8],
                            blur: 28,
                            spread: 2,
                            color: Color32::from_black_alpha(210),
                        })
                        .inner_margin(Margin::symmetric(8, 8))
                        .show(ui, |ui| {
                            ui.set_width(popup_w - 16.0);
                            ui.spacing_mut().item_spacing = Vec2::new(0.0, 2.0);

                            // Header
                            ui.horizontal(|ui| {
                                let (icon_r, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), Sense::hover());
                                Icons::draw_volume(ui.painter(), icon_r, VortexTheme::current_skin().accent_primary, false, 100.0);
                                ui.add_space(2.0);
                                ui.label(egui::RichText::new("Audio Channel Configuration").size(12.0).color(Color32::WHITE).strong());
                            });
                            ui.add_space(2.0);
                            ui.separator();
                            ui.add_space(2.0);

                            let curr_ch = self.config.audio_channels.clone();
                            let is_same = curr_ch == "auto" || curr_ch == "auto-safe" || curr_ch.is_empty();

                            let btn = |ui: &mut egui::Ui, is_sel: bool, icon: &str, label: &str, subtext: &str| -> bool {
                                let w = ui.available_width();
                                let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 24.0), Sense::click());
                                let is_hovered = resp.hovered();

                                if is_sel {
                                    ui.painter().rect_filled(rect, CornerRadius::same(4), Color32::from_rgb(32, 45, 68));
                                    ui.painter().rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.0, Color32::from_rgb(55, 85, 140)), eframe::egui::StrokeKind::Inside);
                                } else if is_hovered {
                                    ui.painter().rect_filled(rect, CornerRadius::same(4), Color32::from_rgb(26, 30, 42));
                                }

                                // Vector Radio Indicator
                                let radio_c = Pos2::new(rect.left() + 11.0, rect.center().y);
                                Icons::draw_radio(ui.painter(), radio_c, 4.8, is_sel, is_hovered, VortexTheme::current_skin().accent_primary);

                                // Icon
                                let mut text_x = rect.left() + 23.0;
                                if !icon.is_empty() {
                                    ui.painter().text(
                                        Pos2::new(text_x, rect.center().y),
                                        Align2::LEFT_CENTER,
                                        icon,
                                        FontId::proportional(11.0),
                                        if is_sel { VortexTheme::current_skin().accent_primary } else if is_hovered { Color32::WHITE } else { Color32::from_rgb(160, 165, 185) },
                                    );
                                    text_x += 16.0;
                                }

                                // Label
                                let label_col = if is_sel {
                                    Color32::WHITE
                                } else if is_hovered {
                                    Color32::from_rgb(240, 242, 250)
                                } else {
                                    Color32::from_rgb(215, 218, 230)
                                };
                                ui.painter().text(
                                    Pos2::new(text_x, rect.center().y),
                                    Align2::LEFT_CENTER,
                                    label,
                                    FontId::proportional(11.5),
                                    label_col,
                                );

                                // Subtext on right
                                if !subtext.is_empty() {
                                    ui.painter().text(
                                        Pos2::new(rect.right() - 6.0, rect.center().y),
                                        Align2::RIGHT_CENTER,
                                        subtext,
                                        FontId::proportional(10.5),
                                        if is_sel { Color32::from_rgb(175, 195, 235) } else { Color32::from_rgb(130, 136, 155) },
                                    );
                                }

                                resp.clicked()
                            };

                            let is_exclusive = self.config.wasapi_exclusive;
                            if btn(ui, is_exclusive, "⚡", "WASAPI Exclusive", if is_exclusive { "Bit-Perfect (ON)" } else { "Shared (OFF)" }) {
                                self.config.wasapi_exclusive = !self.config.wasapi_exclusive;
                                if let Ok(ref player) = self.player {
                                    player.set_wasapi_exclusive(self.config.wasapi_exclusive);
                                }
                                let _ = self.config.save();
                                let msg = if self.config.wasapi_exclusive {
                                    "WASAPI Exclusive: ON (Bit-Perfect Direct Output)"
                                } else {
                                    "WASAPI Exclusive: OFF (Windows Shared Mixer)"
                                };
                                self.osd.show(msg.to_string(), 1500);
                                close_channels_popup = true;
                            }
                            ui.add_space(1.0);
                            ui.separator();
                            ui.add_space(1.0);

                            if btn(ui, is_same, "⚡", "Same as input", "Source Passthrough") {
                                self.config.audio_channels = "auto".to_string();
                                if let Ok(ref player) = self.player {
                                    player.set_audio_channels("auto");
                                }
                                let _ = self.config.save();
                                self.osd.show("Audio Channels: Same as Input (Auto Passthrough)".to_string(), 1500);
                                close_channels_popup = true;
                            }
                            ui.add_space(1.0);
                            ui.separator();
                            ui.add_space(1.0);

                            let ch_options = [
                                ("mono", "🔈", "1.0 Mono", "1 Channel"),
                                ("stereo", "🎧", "2.0 Stereo", "Default 2ch"),
                                ("2.1", "📻", "2.1 Stereo", "+ Subwoofer"),
                                ("4.0", "🎛️", "4.0 Quadraphonic", "4 Channels"),
                                ("5.1", "🎬", "5.1 Surround", "6 Channels"),
                                ("7.1", "🌌", "7.1 Surround", "8 Channels"),
                            ];
                            for (id, icon, label, sub) in ch_options {
                                let is_sel = curr_ch == id;
                                if btn(ui, is_sel, icon, label, sub) {
                                    self.config.audio_channels = id.to_string();
                                    if let Ok(ref player) = self.player {
                                        player.set_audio_channels(id);
                                    }
                                    let _ = self.config.save();
                                    let msg = format!("Audio Channels: {}", label);
                                    self.osd.show(msg, 1500);
                                    close_channels_popup = true;
                                }
                            }
                            ui.add_space(1.0);
                            ui.separator();
                            ui.add_space(1.0);

                            let is_bs2b = curr_ch == "bs2b";
                            if btn(ui, is_bs2b, "🎧", "Virtual Headphone", "Bauer BS2B") {
                                self.config.audio_channels = "bs2b".to_string();
                                if let Ok(ref player) = self.player {
                                    player.set_audio_channels("bs2b");
                                }
                                let _ = self.config.save();
                                self.osd.show("Audio Mode: Virtual Headphone (BS2B)".to_string(), 1500);
                                close_channels_popup = true;
                            }
                            let is_sofa = curr_ch == "sofalizer";
                            if btn(ui, is_sofa, "🌐", "Virtual 3D Surround", "HRTF Spatial") {
                                self.config.audio_channels = "sofalizer".to_string();
                                if let Ok(ref player) = self.player {
                                    player.set_audio_channels("sofalizer");
                                }
                                let _ = self.config.save();
                                self.osd.show("Audio Mode: Virtual 3D Surround (HRTF)".to_string(), 1500);
                                close_channels_popup = true;
                            }
                        });
                });

            let actual_rect = area_resp.response.rect;
            self.active_popup_rects.push(actual_rect);

            let just_opened = self.menu_opened_frame >= self.frame_counter.saturating_sub(1);
            if !just_opened && ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary) || i.pointer.button_clicked(egui::PointerButton::Secondary)) {
                if let Some(pos) = ctx.pointer_latest_pos().or_else(|| ctx.pointer_hover_pos()) {
                    if !actual_rect.contains(pos) {
                        close_channels_popup = true;
                    }
                }
            }

            if close_channels_popup {
                self.show_audio_channels_popup = false;
            }
        }

        if self.show_speed_popup {
            let win_size = get_window_client_rect(&ctx).size();
            let raw_pos = self.speed_popup_pos.unwrap_or(Pos2::new(win_size.x * 0.5, win_size.y - 60.0));
            let popup_w = 260.0;
            let popup_h = 390.0;

            let max_x = (win_size.x - popup_w - 8.0).max(8.0);
            let max_y = (win_size.y - popup_h - 8.0).max(34.0);
            let clamped_x = (raw_pos.x - popup_w * 0.5).clamp(8.0, max_x);
            let clamped_y = (raw_pos.y - popup_h - 8.0).clamp(34.0, max_y);
            let popup_pos = Pos2::new(clamped_x, clamped_y);
            self.active_popup_rects.push(Rect::from_min_size(popup_pos, Vec2::new(popup_w, popup_h)));

            let mut close_speed_popup = false;
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_speed_popup = true;
            }

            let area_resp = egui::Area::new(egui::Id::new("speed_selector_popup"))
                .order(egui::Order::Tooltip)
                .fixed_pos(popup_pos)
                .show(&ctx, |ui| {
                    egui::Frame::new()
                        .fill(Color32::from_rgb(16, 18, 24))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(44, 48, 60)))
                        .corner_radius(CornerRadius::same(6))
                        .shadow(egui::Shadow {
                            offset: [0, 8],
                            blur: 28,
                            spread: 2,
                            color: Color32::from_black_alpha(210),
                        })
                        .inner_margin(Margin::symmetric(10, 8))
                        .show(ui, |ui| {
                            ui.set_width(popup_w - 20.0);
                            ui.spacing_mut().item_spacing = Vec2::new(0.0, 2.0);

                            // Header
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("⚡").color(VortexTheme::current_skin().accent_primary).size(13.0));
                                ui.add_space(2.0);
                                ui.label(egui::RichText::new("Playback Speed").size(12.0).color(Color32::WHITE).strong());
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let is_custom = (stats.speed - 1.0).abs() > 0.03;
                                    let badge_col = if is_custom { VortexTheme::current_skin().accent_primary } else { Color32::from_rgb(160, 165, 180) };
                                    ui.label(egui::RichText::new(format!("{:.2}x", stats.speed)).monospace().size(11.0).color(badge_col).strong());
                                });
                            });
                            ui.add_space(3.0);
                            ui.separator();
                            ui.add_space(3.0);

                            // Presets list
                            let speed_presets: &[(f64, &str, &str, &str)] = &[
                                (0.2, "0.2x", "🐌", "Slowest"),
                                (0.5, "0.5x", "🐢", "Half Speed"),
                                (0.75, "0.75x", "🔉", "Slower"),
                                (0.8, "0.8x", "🔉", "0.8x"),
                                (1.0, "1.0x", "⚡", "Normal Speed (Default)"),
                                (1.2, "1.2x", "⏩", "1.2x"),
                                (1.25, "1.25x", "⏩", "Faster"),
                                (1.5, "1.5x", "🚀", "1.5x Fast"),
                                (1.75, "1.75x", "🚀", "1.75x"),
                                (2.0, "2.0x", "🔥", "Double Speed"),
                                (3.0, "3.0x", "⚡", "Triple Speed"),
                                (4.0, "4.0x", "✨", "Max Speed (4.0x)"),
                            ];

                            let btn = |ui: &mut egui::Ui, is_sel: bool, icon: &str, label: &str, subtext: &str| -> bool {
                                let w = ui.available_width();
                                let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 22.0), Sense::click());
                                let is_hovered = resp.hovered();

                                if is_sel {
                                    ui.painter().rect_filled(rect, CornerRadius::same(4), Color32::from_rgb(32, 45, 68));
                                    ui.painter().rect_stroke(rect, CornerRadius::same(4), Stroke::new(1.0, Color32::from_rgb(55, 85, 140)), eframe::egui::StrokeKind::Inside);
                                } else if is_hovered {
                                    ui.painter().rect_filled(rect, CornerRadius::same(4), Color32::from_rgb(26, 30, 42));
                                }

                                // Vector Radio Indicator
                                let radio_c = Pos2::new(rect.left() + 10.0, rect.center().y);
                                Icons::draw_radio(ui.painter(), radio_c, 4.5, is_sel, is_hovered, VortexTheme::current_skin().accent_primary);

                                let mut text_x = rect.left() + 22.0;
                                if !icon.is_empty() {
                                    ui.painter().text(
                                        Pos2::new(text_x, rect.center().y),
                                        Align2::LEFT_CENTER,
                                        icon,
                                        FontId::proportional(11.0),
                                        if is_sel { VortexTheme::current_skin().accent_primary } else if is_hovered { Color32::WHITE } else { Color32::from_rgb(160, 165, 185) },
                                    );
                                    text_x += 16.0;
                                }

                                let label_col = if is_sel {
                                    Color32::WHITE
                                } else if is_hovered {
                                    Color32::from_rgb(240, 242, 250)
                                } else {
                                    Color32::from_rgb(215, 218, 230)
                                };
                                ui.painter().text(
                                    Pos2::new(text_x, rect.center().y),
                                    Align2::LEFT_CENTER,
                                    label,
                                    FontId::proportional(11.5),
                                    label_col,
                                );

                                if !subtext.is_empty() {
                                    ui.painter().text(
                                        Pos2::new(rect.right() - 6.0, rect.center().y),
                                        Align2::RIGHT_CENTER,
                                        subtext,
                                        FontId::proportional(10.0),
                                        if is_sel { Color32::from_rgb(175, 195, 235) } else { Color32::from_rgb(130, 136, 155) },
                                    );
                                }

                                resp.clicked()
                            };

                            for &(spd_val, label, icon, sub) in speed_presets {
                                let is_sel = (stats.speed - spd_val).abs() < 0.03;
                                if btn(ui, is_sel, icon, label, sub) {
                                    if let Ok(ref player) = self.player {
                                        player.set_speed(spd_val);
                                    }
                                    let msg = format!("Playback Speed: {:.2}x", spd_val);
                                    self.osd.show(msg, 1200);
                                    close_speed_popup = true;
                                }
                            }

                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(4.0);

                            // Quick adjustment buttons row: [-0.1x] [↺ 1.0x] [+0.1x]
                            ui.horizontal(|ui| {
                                let btn_w = (ui.available_width() - 8.0) / 3.0;
                                if ui.add_sized([btn_w, 22.0], egui::Button::new("-0.1x")).clicked() {
                                    if let Ok(ref player) = self.player {
                                        let new_spd = (stats.speed - 0.1).clamp(0.1, 10.0);
                                        player.set_speed(new_spd);
                                    }
                                }
                                let reset_text = egui::RichText::new("↺ 1.0x").color(if (stats.speed - 1.0).abs() > 0.03 { VortexTheme::current_skin().accent_primary } else { Color32::WHITE });
                                if ui.add_sized([btn_w, 22.0], egui::Button::new(reset_text)).on_hover_text("Reset to normal 1.0x speed").clicked() {
                                    if let Ok(ref player) = self.player {
                                        player.reset_speed();
                                    }
                                    close_speed_popup = true;
                                }
                                if ui.add_sized([btn_w, 22.0], egui::Button::new("+0.1x")).clicked() {
                                    if let Ok(ref player) = self.player {
                                        let new_spd = (stats.speed + 0.1).clamp(0.1, 10.0);
                                        player.set_speed(new_spd);
                                    }
                                }
                            });
                        });
                });

            let actual_rect = area_resp.response.rect;
            self.active_popup_rects.push(actual_rect);

            let just_opened = self.menu_opened_frame >= self.frame_counter.saturating_sub(1);
            if !just_opened && ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary) || i.pointer.button_clicked(egui::PointerButton::Secondary)) {
                if let Some(pos) = ctx.pointer_latest_pos().or_else(|| ctx.pointer_hover_pos()) {
                    if !actual_rect.contains(pos) {
                        close_speed_popup = true;
                    }
                }
            }

            if close_speed_popup {
                self.show_speed_popup = false;
            }
        }

        if self.show_control_panel {
            if let Ok(ref player) = self.player {
                if let Some(r) = self.control_panel.render(
                    &ctx,
                    &mut self.show_control_panel,
                    player,
                    &stats,
                    &mut self.config,
                    &mut self.bookmark_mgr,
                    &mut *self.playlist.lock().unwrap(),
                ) {
                    self.active_popup_rects.push(r);
                }
            }
        }

        if self.show_preferences {
            let player_ref = self.player.as_ref().map(|p| &**p).ok();
            self.preferences_dialog.render(
                &ctx,
                &mut self.show_preferences,
                &mut self.config,
                player_ref,
            );
        }

        if self.show_bookmark_overlay {
            if let Ok(ref player) = self.player {
                if let Some(r) = BookmarkOverlay::render(
                    &ctx,
                    &mut self.show_bookmark_overlay,
                    player,
                    &stats,
                    &mut self.bookmark_mgr,
                ) {
                    self.active_popup_rects.push(r);
                }
            }
        }

        if self.show_capture_dialog {
            if let Ok(ref player) = self.player {
                if let Some(r) = self.capture_dialog.render(
                    &ctx,
                    &mut self.show_capture_dialog,
                    &mut self.capture_config,
                    &mut self.burst_engine,
                    player,
                ) {
                    self.active_popup_rects.push(r);
                }
            }
        }

        if self.show_stream_url_dialog {
            if let Ok(ref player) = self.player {
                if let Some(r) = self.stream_url_dialog.render(
                    &ctx,
                    &mut self.show_stream_url_dialog,
                    player,
                ) {
                    self.active_popup_rects.push(r);
                }
            }
        }

        if self.show_mediainfo_dialog {
            self.mediainfo_dialog.render(&ctx, &mut self.show_mediainfo_dialog, &stats);
            if self.mediainfo_dialog.toggle_wasapi_exclusive {
                self.mediainfo_dialog.toggle_wasapi_exclusive = false;
                self.config.wasapi_exclusive = !self.config.wasapi_exclusive;
                let _ = self.config.save();
                if let Ok(ref player) = self.player {
                    player.set_wasapi_exclusive(self.config.wasapi_exclusive);
                }
                let msg = if self.config.wasapi_exclusive {
                    "WASAPI Exclusive: ON (Bit-Perfect Direct Output)"
                } else {
                    "WASAPI Exclusive: OFF (Windows Shared Mixer)"
                };
                self.osd.show(msg.to_string(), 1500);
            }
        }

        if self.show_about_dialog {
            let mut close_about = false;
            let window_resp = egui::Window::new("About VortexPlayer")
                .open(&mut self.show_about_dialog)
                .resizable(false)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .frame(
                    egui::Frame::new()
                        .fill(Color32::from_rgb(0, 0, 0))
                        .stroke(Stroke::new(1.0, VortexTheme::current_skin().accent_primary))
                        .corner_radius(CornerRadius::same(8)),
                )
                .show(&ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(8.0);
                        let (logo_rect, _) = ui.allocate_exact_size(Vec2::splat(50.0), Sense::hover());
                        Icons::draw_vertex_logo(ui.painter(), logo_rect.center(), 24.0);

                        ui.add_space(8.0);
                        ui.label(RichText::new("VortexPlayer v1.1.0").size(16.0).strong().color(VortexTheme::VORTEX_CYAN));
                        ui.label(RichText::new("High-Quality Hardware Accelerated Media Player").size(11.5).color(Color32::from_rgb(170, 160, 200)));

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(6.0);

                        ui.label(RichText::new("Built with Rust, eframe/egui & libmpv").size(10.5).color(Color32::from_rgb(140, 130, 170)));
                        ui.label(RichText::new("AV1, HEVC 10-bit/12-bit D3D11 Acceleration").size(10.0).color(Color32::from_rgb(120, 110, 150)));
                        ui.label(RichText::new("18-Band Parametric Equalizer & Audio DSP").size(10.0).color(Color32::from_rgb(120, 110, 150)));
                        ui.label(RichText::new("Multi-Theme Engine (Dark / Midnight / Slate)").size(10.0).color(Color32::from_rgb(120, 110, 150)));

                        ui.add_space(12.0);
                        if ui.button(RichText::new("  Close  ").size(11.0)).clicked() {
                            close_about = true;
                        }
                        ui.add_space(4.0);
                    });
                });
            if let Some(inner) = window_resp {
                self.active_popup_rects.push(inner.response.rect);
            }
            if close_about {
                self.show_about_dialog = false;
            }
        }

        if self.show_sleep_timer_dialog {
            if let Some(r) = self.sleep_timer_dialog.render(&ctx, &mut self.show_sleep_timer_dialog, &mut self.sleep_timer) {
                self.active_popup_rects.push(r);
            }
        }

        if self.show_update_dialog {
            if let Some(r) = self.update_dialog.render(&ctx, &mut self.show_update_dialog) {
                self.active_popup_rects.push(r);
            }
        }

        if self.show_surround_eq_dialog {
            if let Ok(ref player) = self.player {
                if let Some(r) = self.surround_eq_dialog.render(&ctx, &mut self.show_surround_eq_dialog, &mut self.config.surround_eq, player) {
                    self.active_popup_rects.push(r);
                }
            }
        }

        // =========================================================================
        // DETACHED PLAYLIST WINDOW (Independent Multi-Monitor Top-Level Native Viewport)
        // =========================================================================
        if self.show_playlist && self.config.playlist_detached {
            let viewport_id = egui::ViewportId::from_hash_of("vortex_detached_playlist_window");
            let mut builder = egui::ViewportBuilder::default()
                .with_title("Playlist")
                .with_inner_size([380.0, 620.0])
                .with_min_inner_size([280.0, 360.0])
                .with_resizable(true)
                .with_decorations(true);

            if self.config.playlist_pinned {
                builder = builder.with_always_on_top();
            }

            let mut close_detached = false;
            let mut detach_toggle = false;
            let mut pin_toggle = false;
            let is_pinned = self.config.playlist_pinned;
            let active_path = stats.file_path.clone();

            ctx.show_viewport_immediate(viewport_id, builder, |ctx, _class| {
                if ctx.input(|i| i.viewport().close_requested() || i.key_pressed(egui::Key::Escape)) {
                    close_detached = true;
                }

                // Handle files dragged and dropped into Playlist window
                let dropped = ctx.input(|i| i.raw.dropped_files.clone());
                if !dropped.is_empty() {
                    let mut paths = Vec::new();
                    for d in dropped {
                        let p = d.path().to_path_buf();
                        if !p.as_os_str().is_empty() {
                            paths.push(p);
                        }
                    }
                    if !paths.is_empty() {
                        self.playlist.lock().unwrap().add_items(paths);
                        self.config.last_playlist = self.playlist.lock().unwrap().items.iter().map(|i| i.path.to_string_lossy().to_string()).collect();
                        let _ = self.config.save();
                    }
                }

                egui::CentralPanel::default()
                    .frame(
                        egui::Frame::new()
                            .fill(Color32::from_rgb(0, 0, 0))
                            .inner_margin(Margin::same(6)),
                    )
                    .show(ctx, |ui| {
                        let pl_actions = PlaylistPanel::render(
                            ui,
                            &mut *self.playlist.lock().unwrap(),
                            Some(&active_path),
                            true,
                            is_pinned,
                            &mut self.drawer_tab,
                            &stats,
                            &self.bookmark_mgr,
                            &self.subtitle_explorer.lines,
                            &mut self.drawer_browser_dir,
                            &mut self.drawer_browser_search,
                            &mut self.drawer_sub_search,
                            self.config.restore_last_playlist,
                        );
                        if let Some(path) = pl_actions.play_path {
                            file_to_open = Some(path);
                        }
                        if let Some(path) = pl_actions.enqueue_path {
                            self.playlist.lock().unwrap().add_item(path);
                            self.osd.show("Added to Playlist".to_string(), 1200);
                        }
                        if let Some(seek_t) = pl_actions.seek_to {
                            if let Ok(ref player) = self.player {
                                player.seek_absolute(seek_t);
                            }
                        }
                        if pl_actions.add_bookmark {
                            self.bookmark_mgr.add_bookmark(stats.time_pos, None);
                            self.osd.show(format!("Bookmark Added: {}", format_time(stats.time_pos)), 1500);
                        }
                        if let Some(b_idx) = pl_actions.delete_bookmark_idx {
                            if b_idx < self.bookmark_mgr.bookmarks.len() {
                                self.bookmark_mgr.bookmarks.remove(b_idx);
                            }
                        }
                        if pl_actions.add_files {
                            if let Some(files) = rfd::FileDialog::new().pick_files() {
                                self.playlist.lock().unwrap().add_items(files);
                                self.config.last_playlist = self.playlist.lock().unwrap().items.iter().map(|i| i.path.to_string_lossy().to_string()).collect();
                                self.config.last_playlist_index = self.playlist.lock().unwrap().current_index;
                                let _ = self.config.save();
                            }
                        }
                        if pl_actions.add_folder {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                self.open_folder(folder);
                            }
                        }
                        if pl_actions.clear_playlist {
                            self.playlist.lock().unwrap().items.clear();
                            self.playlist.lock().unwrap().current_index = None;
                            self.config.last_playlist.clear();
                            self.config.last_playlist_index = None;
                            let _ = self.config.save();
                        }
                        if pl_actions.add_url {
                            self.show_stream_url_dialog = true;
                        }
                        if pl_actions.cycle_repeat {
                            let mode = self.playlist.lock().unwrap().repeat_mode;
                            self.config.playlist_repeat_mode = match mode {
                                crate::playlist::RepeatMode::RepeatAll => "RepeatAll".to_string(),
                                crate::playlist::RepeatMode::RepeatTrack => "RepeatTrack".to_string(),
                                _ => "Off".to_string(),
                            };
                            let _ = self.config.save();
                            let msg = match mode {
                                crate::playlist::RepeatMode::RepeatAll => "Repeat: All Tracks",
                                crate::playlist::RepeatMode::RepeatTrack => "Repeat: Current Track",
                                crate::playlist::RepeatMode::Off => "Repeat: Off",
                            };
                            self.osd.show(msg.to_string(), 1500);
                        }
                        if pl_actions.toggle_shuffle {
                            let is_shuf = self.playlist.lock().unwrap().is_shuffle();
                            self.config.playlist_shuffle = is_shuf;
                            let _ = self.config.save();
                            let msg = if is_shuf { "Shuffle: ON" } else { "Shuffle: OFF" };
                            self.osd.show(msg.to_string(), 1500);
                        }
                        if pl_actions.toggle_restore_prev {
                            self.config.restore_last_playlist = !self.config.restore_last_playlist;
                            let _ = self.config.save();
                            let msg = if self.config.restore_last_playlist {
                                "Restore Prev Playlist: Enabled"
                            } else {
                                "Restore Prev Playlist: Disabled"
                            };
                            self.osd.show(msg.to_string(), 1500);
                        }
                        if pl_actions.toggle_detach {
                            detach_toggle = true;
                        }
                        if pl_actions.toggle_pin {
                            pin_toggle = true;
                        }
                        if pl_actions.close_playlist {
                            close_detached = true;
                        }
                    });
            });

            if close_detached {
                self.show_playlist = false;
            }
            if detach_toggle {
                self.config.playlist_detached = false;
                let _ = self.config.save();
                self.toast.info("Playlist: Docked (Sidebar)");
                // Expand window by sidebar width since we're going floating -> docked
                if !self.is_fullscreen {
                    let is_max = is_window_maximized_or_workarea(self.parent_hwnd, &ctx);
                    if !is_max {
                        if let Some(cur) = ctx.input(|i| i.viewport().inner_rect) {
                            let new_w = cur.width() + 330.0;
                            let cur_h = cur.height();
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(new_w, cur_h)));
                            #[cfg(windows)]
                            if self.parent_hwnd != 0 {
                                use windows_sys::Win32::UI::WindowsAndMessaging::{SetWindowPos, SWP_NOMOVE, SWP_NOZORDER, SWP_FRAMECHANGED, SWP_SHOWWINDOW};
                                let ppp = ctx.pixels_per_point().max(0.1);
                                let pw = (new_w * ppp).round() as i32;
                                let ph = (cur_h * ppp).round() as i32;
                                unsafe {
                                    SetWindowPos(self.parent_hwnd as _, 0 as _, 0, 0, pw, ph, SWP_NOMOVE | SWP_NOZORDER | SWP_FRAMECHANGED | SWP_SHOWWINDOW);
                                }
                            }
                        }
                    }
                }
            }
            if pin_toggle {
                self.config.playlist_pinned = !self.config.playlist_pinned;
                let _ = self.config.save();
                let msg = if self.config.playlist_pinned { "Playlist: Pinned Always on Top" } else { "Playlist: Unpinned" };
                self.toast.info(msg);
            }
        }

        // Apply any pending restored audio / subtitle tracks once demuxer populates tracks
        if let Ok(ref player) = self.player {
            if let Some(aid) = self.pending_audio_track {
                if !stats.audio_tracks.is_empty() {
                    if stats.selected_audio_track != aid {
                        player.set_audio_track(aid);
                    }
                    self.pending_audio_track = None;
                }
            }
            if let Some(sid) = self.pending_subtitle_track {
                if !stats.subtitle_tracks.is_empty() {
                    if stats.selected_subtitle_track != sid {
                        player.set_subtitle_track(sid);
                    }
                    self.pending_subtitle_track = None;
                }
            }
        }

        // =========================================================================
        // 6. PROCESS FILE / FOLDER OPENS
        // =========================================================================
        if let Some(path) = file_to_open {
            self.open_media_file(path);
        }
        if let Some(folder) = folder_to_open {
            self.open_folder(folder);
        }

        if toggle_pip_requested {
            self.toggle_pip(&ctx);
        }

        // Smart Zero-GPU Power Management:
        // - When Minimized: Deep sleep (1000ms), dropping GPU usage & wattage to true 0.0W
        // - When in Background / Not on top: Throttles to 100ms (10 FPS)
        // - When Active / On Top playing: 30 FPS (33ms) fluid timeline
        // - When User Interacting (mouse moving, dragging): 60 FPS (16ms)
        // - When Paused / Idle: 500ms sleep
        #[cfg(windows)]
        let is_minimized = (self.parent_hwnd != 0 && unsafe {
            windows_sys::Win32::UI::WindowsAndMessaging::IsIconic(self.parent_hwnd as _) != 0
        }) || ctx.input(|i| i.viewport().minimized.unwrap_or(false));
        #[cfg(not(windows))]
        let is_minimized = ctx.input(|i| i.viewport().minimized.unwrap_or(false));

        #[cfg(windows)]
        let is_foreground = (self.parent_hwnd != 0 && unsafe {
            windows_sys::Win32::UI::WindowsAndMessaging::GetForegroundWindow() == (self.parent_hwnd as _)
        }) || ctx.input(|i| i.viewport().focused.unwrap_or(true));
        #[cfg(not(windows))]
        let is_foreground = ctx.input(|i| i.viewport().focused.unwrap_or(true));

        let has_dialog_open = self.show_playlist
            || self.show_control_panel
            || self.show_preferences
            || self.show_main_menu
            || self.show_mediainfo_dialog
            || self.show_about_dialog
            || self.show_stream_url_dialog
            || self.show_bookmark_overlay
            || self.show_peq_dialog
            || self.show_library_view
            || self.show_telemetry_hud
            || self.show_jump_time_dialog
            || self.stereo_3d_dialog.is_open
            || self.karaoke_studio.is_open
            || self.subtitle_studio.is_open
            || self.screen_capture.is_open
            || self.iptv_guide.is_open
            || self.logo_watermark.is_open
            || self.disc_nav.is_open
            || self.cd_ripper.is_open
            || self.bda_tuner.is_open
            || self.bdj_dialog.is_open
            || self.vst_winamp.is_open
            || self.detached_windows.is_open
            || self.discord_rpc.is_open
            || self.web_remote.is_open
            || self.scrobbler_dialog.is_open
            || self.media_server.is_open
            || self.ambilight_dialog.is_open
            || self.cast_renderer.is_open
            || self.transcoder_dialog.is_open
            || self.zoom_magnifier.is_open
            || self.radio_directory.is_open
            || self.video_puzzle.is_open
            || self.vr_360_studio.is_open
            || self.damaged_file_repair.is_open
            || self.binaural_crossfeed.is_open
            || self.video_wall_matrix.is_open
            || self.chapter_marker_dialog.is_open
            || self.audio_compressor.is_open
            || self.video_crop.is_open
            || self.goto_frame.is_open
            || self.playback_history.is_open
            || self.media_tag_editor.is_open
            || self.deinterlace_dialog.is_open
            || self.ab_repeat_dialog.is_open
            || self.boss_key_dialog.is_open
            || self.motion_interpolation.is_open
            || self.color_lut_dialog.is_open
            || self.subtitle_translator.is_open
            || self.ai_subtitle.is_open
            || self.ai_upscaling.is_open
            || self.ai_audio.is_open
            || self.loudness_radar.is_open
            || self.frame_dumper.is_open
            || self.color_blindness.is_open
            || self.cinema_matte.is_open
            || self.motion_vector_inspector.is_open
            || self.rich_bookmark_notes.is_open
            || self.pitch_formant.is_open
            || self.bookmark_studio.is_open







            || self.subtitle_explorer.is_open
            || self.settings_inspector.is_open
            || self.input_editor.is_open
            || self.file_navigator.is_open
            || self.network_browser.is_open
            || self.record_dialog.is_open
            || self.contact_sheet_dialog.is_open
            || self.gif_maker.is_open
            || self.device_capture.is_open
            || self.subtitle_lookup.is_open
            || self.shader_studio.is_open
            || self.broadcast_dialog.is_open
            || self.channel_matrix_dialog.is_open
            || self.chain_editor.is_open
            || self.auto_skip_dialog.is_open;

        // ── Ultra Low-Power Vortex-Grade Frame Pacing & Wattage Management ──
        let has_osd_animating = (self.osd.message.is_some() && std::time::Instant::now() < self.osd.expires_at)
            || self.osd.center_indicator.is_some()
            || !self.toast.toasts.is_empty();

        let is_pointer_active = ctx.input(|i| {
            i.pointer.is_moving()
                || i.pointer.any_down()
                || i.pointer.any_pressed()
                || i.pointer.any_released()
                || i.smooth_scroll_delta != Vec2::ZERO
        });
        let has_recent_mouse = self.last_mouse_activity.elapsed().as_millis() < 2000;
        let is_hovering_controls = if !self.is_fullscreen {
            mouse_pos.map_or(false, |p| p.y < 45.0 || p.y > screen_h - 75.0 || (self.show_playlist && p.x > screen_w - 340.0))
        } else {
            self.is_bottom_hovered || self.is_top_hovered
        };
        let is_active_ui = is_pointer_active 
            || has_recent_mouse 
            || is_hovering_controls 
            || has_osd_animating 
            || has_dialog_open 
            || self.osd.show_media_info 
            || self.show_control_panel 
            || self.show_main_menu
            || self.show_audio_channels_popup
            || self.show_speed_popup;

        let repaint_for_fps = |fps: u32| {
            std::time::Duration::from_secs_f32(1.0 / fps.max(1) as f32)
        };

        if is_minimized {
            if !stats.is_paused && !stats.is_idle {
                ctx.request_repaint_after(std::time::Duration::from_millis(100));
            } else {
                ctx.request_repaint_after(std::time::Duration::from_millis(250));
            }
        } else if is_active_ui {
            // Full interaction budget for silky-smooth responsive controls
            ctx.request_repaint_after(repaint_for_fps(self.config.fps_interaction));
        } else if !stats.is_paused && !stats.is_idle {
            if is_foreground {
                // Display-synced playback rate matching monitor refresh rate (60Hz / 120Hz / 144Hz)
                let display_hz = if stats.display_fps >= 50.0 { stats.display_fps.round() as u32 } else { 60 };
                let target_fps = self.config.fps_playback.max(display_hz);
                ctx.request_repaint_after(repaint_for_fps(target_fps));
            } else {
                ctx.request_repaint_after(repaint_for_fps(self.config.fps_background));
            }
        } else {
            // Stationary mouse when paused / idle: smooth 100ms idle tick (never a 1-sec freeze)
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }

        #[cfg(windows)]
        if self.parent_hwnd != 0 {
            let ppp = ctx.pixels_per_point();
            let is_song_playback = MusicBackgroundView::is_song(&stats);
            let is_active_video = !stats.is_idle && !stats.file_path.is_empty() && !is_song_playback;
            let clip = if is_active_video {
                let r = self.last_video_rect;
                Some([
                    (r.min.x * ppp).round() as i32,
                    (r.min.y * ppp).round() as i32,
                    (r.max.x * ppp).round() as i32,
                    (r.max.y * ppp).round() as i32,
                ])
            } else {
                None
            };
            self.taskbar.set_thumbnail_clip(self.parent_hwnd, clip);
        }
        static LAST_FRAME: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
        if LAST_FRAME.swap(false, std::sync::atomic::Ordering::Relaxed) {
            crate::log_step("34. VortexApp::ui frame 1 finished successfully!");
        }
    }

    fn on_exit(&mut self, _gl: std::option::Option<&eframe::glow::Context>) {
        crate::log_step("35. VortexApp on_exit called!");
        self.save_current_playback_state();
        let _ = self.config.save();
    }

}

// ── Native Win32 child window helpers (legacy, retained for reference) ────
#[allow(dead_code)]
#[cfg(windows)]
fn native_monitor_physical_size(hwnd: isize) -> Option<(i32, i32)> {
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::Graphics::Gdi::*;
    if hwnd == 0 {
        return None;
    }
    unsafe {
        let h_mon = MonitorFromWindow(hwnd as HWND, MONITOR_DEFAULTTONEAREST);
        if h_mon.is_null() {
            return None;
        }
        let mut mi = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            dwFlags: 0,
        };
        if GetMonitorInfoW(h_mon, &mut mi) != 0 {
            let w = mi.rcMonitor.right - mi.rcMonitor.left;
            let h = mi.rcMonitor.bottom - mi.rcMonitor.top;
            if w > 0 && h > 0 {
                return Some((w, h));
            }
        }
        None
    }
}

#[allow(dead_code)]
#[cfg(windows)]
fn native_client_physical_size(hwnd: isize) -> Option<(i32, i32)> {
    use windows_sys::Win32::Foundation::RECT;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetClientRect;

    if hwnd == 0 {
        return None;
    }

    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    let ok = unsafe { GetClientRect(hwnd as _, &mut rect) } != 0;
    if ok && rect.right > rect.left && rect.bottom > rect.top {
        Some((rect.right - rect.left, rect.bottom - rect.top))
    } else {
        None
    }
}

#[allow(dead_code)]
#[cfg(windows)]
unsafe extern "system" fn video_host_wndproc(
    hwnd: windows_sys::Win32::Foundation::HWND,
    msg: u32,
    wparam: windows_sys::Win32::Foundation::WPARAM,
    lparam: windows_sys::Win32::Foundation::LPARAM,
) -> windows_sys::Win32::Foundation::LRESULT {
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    match msg {
        WM_ERASEBKGND => 1,
        WM_NCHITTEST => HTTRANSPARENT as isize,
        WM_MOUSEMOVE
        | WM_LBUTTONDOWN
        | WM_LBUTTONUP
        | WM_RBUTTONDOWN
        | WM_RBUTTONUP
        | WM_LBUTTONDBLCLK
        | WM_MOUSEWHEEL
        | WM_MBUTTONDOWN
        | WM_MBUTTONUP => {
            let parent = unsafe { GetParent(hwnd) };
            if !parent.is_null() {
                unsafe {
                    PostMessageW(parent, msg, wparam, lparam);
                }
            }
            0
        }
        WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP => {
            let parent = unsafe { GetParent(hwnd) };
            if !parent.is_null() {
                unsafe {
                    PostMessageW(parent, msg, wparam, lparam);
                }
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

#[allow(dead_code)]
#[cfg(windows)]
fn create_child_video_window(parent_hwnd: isize) -> isize {
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;

    static REGISTER: std::sync::Once = std::sync::Once::new();
    let class_name: Vec<u16> = "VertexPlayer_VideoHost_Class\0".encode_utf16().collect();

    REGISTER.call_once(|| unsafe {
        let hinstance = GetModuleHandleW(null());
        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(video_host_wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: null_mut(),
        };
        RegisterClassExW(&wc);
    });

    unsafe {
        let child = CreateWindowExW(
            0,
            class_name.as_ptr(),
            null(),
            WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
            0,
            0,
            1920,
            1080,
            parent_hwnd as _,
            null_mut(),
            GetModuleHandleW(null()),
            null_mut(),
        );
        child as isize
    }
}

#[allow(dead_code)]
#[cfg(windows)]
static LAST_CHILD_GEOM: std::sync::Mutex<(isize, i32, i32, i32, i32, bool, Vec<(i32, i32, i32, i32)>)> =
    std::sync::Mutex::new((0, -1, -1, -1, -1, false, Vec::new()));

#[allow(dead_code)]
#[cfg(windows)]
pub fn reset_child_geom_cache() {
    if let Ok(mut geom_guard) = LAST_CHILD_GEOM.lock() {
        *geom_guard = (0, -1, -1, -1, -1, false, Vec::new());
    }
}
#[allow(dead_code)]
#[cfg(not(windows))]
pub fn reset_child_geom_cache() {}

#[allow(dead_code)]
#[cfg(windows)]
fn update_child_window_geometry(
    child_hwnd: isize,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    is_visible: bool,
    exclusion_rects: &[Rect],
    ppp: f32,
) {
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::Graphics::Gdi::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;

    unsafe {
        if w <= 0 || h <= 0 || !is_visible {
            ShowWindow(child_hwnd as _, SW_HIDE);
            return;
        }

        let raw_exclusions: Vec<(i32, i32, i32, i32)> = exclusion_rects
            .iter()
            .map(|r| {
                (
                    (r.min.x * ppp).floor() as i32,
                    (r.min.y * ppp).floor() as i32,
                    (r.width() * ppp).ceil() as i32,
                    (r.height() * ppp).ceil() as i32,
                )
            })
            .collect();

        if let Ok(mut geom_guard) = LAST_CHILD_GEOM.lock() {
            if *geom_guard == (child_hwnd, x, y, w, h, is_visible, raw_exclusions.clone()) {
                return;
            }
            *geom_guard = (child_hwnd, x, y, w, h, is_visible, raw_exclusions.clone());
        }

        SetWindowPos(
            child_hwnd as _,
            0 as _, // HWND_TOP
            x,
            y,
            w,
            h,
            SWP_NOACTIVATE | SWP_SHOWWINDOW,
        );
        BringWindowToTop(child_hwnd as HWND);

        let mut valid_holes: Vec<(i32, i32, i32, i32)> = Vec::new();
        for &(rx, ry, rw, rh) in &raw_exclusions {
            let ox1 = rx.max(x);
            let oy1 = ry.max(y);
            let ox2 = (rx + rw).min(x + w);
            let oy2 = if (ry + rh) >= (y + h - 15) {
                y + h
            } else {
                (ry + rh).min(y + h)
            };
            if ox2 > ox1 && oy2 > oy1 {
                valid_holes.push((ox1 - x, oy1 - y, ox2 - x, oy2 - y));
            }
        }

        let is_fs = IS_NATIVE_FULLSCREEN.load(std::sync::atomic::Ordering::Relaxed);
        static HAD_CUSTOM_RGN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if is_fs || valid_holes.is_empty() {
            // Restore full unclipped window only if a custom region was actually previously set
            if HAD_CUSTOM_RGN.swap(false, std::sync::atomic::Ordering::Relaxed) {
                SetWindowRgn(child_hwnd as HWND, 0 as _, 1);
            }
        } else {
            let full_rgn = CreateRectRgn(0, 0, w, h);
            if !full_rgn.is_null() {
                for (hx1, hy1, hx2, hy2) in valid_holes {
                    let hole_rgn = CreateRectRgn(hx1, hy1, hx2, hy2);
                    if !hole_rgn.is_null() {
                        CombineRgn(full_rgn, full_rgn, hole_rgn, RGN_DIFF as i32);
                        DeleteObject(hole_rgn as _);
                    }
                }
                // SetWindowRgn takes ownership of full_rgn only on success
                if SetWindowRgn(child_hwnd as HWND, full_rgn, 1) == 0 {
                    DeleteObject(full_rgn as _);
                } else {
                    HAD_CUSTOM_RGN.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct XRectangle {
    x: i16,
    y: i16,
    width: u16,
    height: u16,
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy)]
struct X11Host {
    display: *mut std::ffi::c_void,
    x_create_simple_window: unsafe extern "C" fn(*mut std::ffi::c_void, u64, i32, i32, u32, u32, u32, u64, u64) -> u64,
    x_map_window: unsafe extern "C" fn(*mut std::ffi::c_void, u64) -> i32,
    x_unmap_window: unsafe extern "C" fn(*mut std::ffi::c_void, u64) -> i32,
    x_move_resize_window: unsafe extern "C" fn(*mut std::ffi::c_void, u64, i32, i32, u32, u32) -> i32,
    x_lower_window: unsafe extern "C" fn(*mut std::ffi::c_void, u64) -> i32,
    x_flush: unsafe extern "C" fn(*mut std::ffi::c_void) -> i32,
    x_shape_combine_rectangles: Option<unsafe extern "C" fn(*mut std::ffi::c_void, u64, i32, i32, i32, *const std::ffi::c_void, i32, i32, i32)>,
}

#[cfg(target_os = "linux")]
unsafe impl Send for X11Host {}
#[cfg(target_os = "linux")]
unsafe impl Sync for X11Host {}

#[cfg(target_os = "linux")]
static X11_HOST: std::sync::Mutex<Option<X11Host>> = std::sync::Mutex::new(None);
#[cfg(target_os = "linux")]
static LAST_X11_CHILD_GEOM: std::sync::Mutex<(isize, i32, i32, i32, i32, bool, Vec<(i32, i32, i32, i32)>)> = std::sync::Mutex::new((0, -1, -1, -1, -1, false, Vec::new()));

#[cfg(target_os = "linux")]
fn create_child_video_window(parent_hwnd: isize) -> isize {
    let mut host_guard = X11_HOST.lock().unwrap_or_else(|e| e.into_inner());
    if host_guard.is_none() {
        let lib_paths = [
            "libX11.so.6",
            "libX11.so",
            "/lib/x86_64-linux-gnu/libX11.so.6",
            "/usr/lib/x86_64-linux-gnu/libX11.so.6",
        ];
        let mut shape_combine = None;
        let lib_ext_paths = [
            "libXext.so.6",
            "libXext.so",
            "/lib/x86_64-linux-gnu/libXext.so.6",
            "/usr/lib/x86_64-linux-gnu/libXext.so.6",
        ];
        for path in lib_ext_paths {
            if let Ok(lib) = unsafe { libloading::Library::new(path) } {
                let lib = Box::leak(Box::new(lib));
                if let Ok(sym) = unsafe { lib.get::<unsafe extern "C" fn(*mut std::ffi::c_void, u64, i32, i32, i32, *const std::ffi::c_void, i32, i32, i32)>(b"XShapeCombineRectangles\0") } {
                    shape_combine = Some(*sym);
                    break;
                }
            }
        }

        for path in lib_paths {
            if let Ok(lib) = unsafe { libloading::Library::new(path) } {
                let lib = Box::leak(Box::new(lib));
                if let (Ok(open_dsp), Ok(create_win), Ok(map_win), Ok(unmap_win), Ok(move_win), Ok(lower_win), Ok(flush_win)) = unsafe { (
                    lib.get::<unsafe extern "C" fn(*const std::os::raw::c_char) -> *mut std::ffi::c_void>(b"XOpenDisplay\0"),
                    lib.get::<unsafe extern "C" fn(*mut std::ffi::c_void, u64, i32, i32, u32, u32, u32, u64, u64) -> u64>(b"XCreateSimpleWindow\0"),
                    lib.get::<unsafe extern "C" fn(*mut std::ffi::c_void, u64) -> i32>(b"XMapWindow\0"),
                    lib.get::<unsafe extern "C" fn(*mut std::ffi::c_void, u64) -> i32>(b"XUnmapWindow\0"),
                    lib.get::<unsafe extern "C" fn(*mut std::ffi::c_void, u64, i32, i32, u32, u32) -> i32>(b"XMoveResizeWindow\0"),
                    lib.get::<unsafe extern "C" fn(*mut std::ffi::c_void, u64) -> i32>(b"XLowerWindow\0"),
                    lib.get::<unsafe extern "C" fn(*mut std::ffi::c_void) -> i32>(b"XFlush\0"),
                ) } {
                    let display = unsafe { open_dsp(std::ptr::null()) };
                    if !display.is_null() {
                        *host_guard = Some(X11Host {
                            display,
                            x_create_simple_window: *create_win,
                            x_map_window: *map_win,
                            x_unmap_window: *unmap_win,
                            x_move_resize_window: *move_win,
                            x_lower_window: *lower_win,
                            x_flush: *flush_win,
                            x_shape_combine_rectangles: shape_combine,
                        });
                        break;
                    }
                }
            }
        }
    }

    if let Some(host) = *host_guard {
        unsafe {
            let child = (host.x_create_simple_window)(host.display, parent_hwnd as u64, 0, 32, 1920, 1080, 0, 0, 0);
            if child != 0 {
                (host.x_map_window)(host.display, child);
                (host.x_lower_window)(host.display, child);
                // Make child window transparent to all mouse input so egui on parent window receives all cursor & clicks
                if let Some(shape_fn) = host.x_shape_combine_rectangles {
                    shape_fn(host.display, child, 2 /* ShapeInput */, 0, 0, std::ptr::null(), 0, 0 /* ShapeSet */, 0 /* Unsorted */);
                }
                (host.x_flush)(host.display);
                return child as isize;
            }
        }
    }
    0
}

#[cfg(target_os = "linux")]
fn update_child_window_geometry(
    child_hwnd: isize,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    is_visible: bool,
    exclusion_rects: &[Rect],
    ppp: f32,
) {
    let raw_exclusions: Vec<(i32, i32, i32, i32)> = exclusion_rects
        .iter()
        .map(|r| {
            (
                (r.min.x * ppp).round() as i32,
                (r.min.y * ppp).round() as i32,
                (r.width() * ppp).round() as i32,
                (r.height() * ppp).round() as i32,
            )
        })
        .collect();

    if let Ok(mut geom_guard) = LAST_X11_CHILD_GEOM.lock() {
        if *geom_guard == (child_hwnd, x, y, w, h, is_visible, raw_exclusions.clone()) {
            return;
        }
        *geom_guard = (child_hwnd, x, y, w, h, is_visible, raw_exclusions.clone());
    }

    let host_guard = X11_HOST.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(host) = *host_guard {
        unsafe {
            if is_visible && w > 0 && h > 0 {
                (host.x_move_resize_window)(host.display, child_hwnd as u64, x, y, w as u32, h as u32);
                (host.x_map_window)(host.display, child_hwnd as u64);
                (host.x_lower_window)(host.display, child_hwnd as u64);

                if let Some(shape_fn) = host.x_shape_combine_rectangles {
                    // 1. Reset visible shape to full video window bounds
                    let full_rect = XRectangle {
                        x: 0,
                        y: 0,
                        width: w.clamp(1, 32767) as u16,
                        height: h.clamp(1, 32767) as u16,
                    };
                    shape_fn(
                        host.display,
                        child_hwnd as u64,
                        0, /* ShapeBounding */
                        0,
                        0,
                        &full_rect as *const _ as *const _,
                        1,
                        0, /* ShapeSet */
                        0, /* Unsorted */
                    );

                    // 2. Punch cutout holes for any open popups / context menus so egui draws on top
                    for &(rx, ry, rw, rh) in &raw_exclusions {
                        let ox1 = rx.max(x);
                        let oy1 = ry.max(y);
                        let ox2 = (rx + rw).min(x + w);
                        let oy2 = (ry + rh).min(y + h);
                        if ox2 > ox1 && oy2 > oy1 {
                            let hole = XRectangle {
                                x: (ox1 - x) as i16,
                                y: (oy1 - y) as i16,
                                width: (ox2 - ox1) as u16,
                                height: (oy2 - oy1) as u16,
                            };
                            shape_fn(
                                host.display,
                                child_hwnd as u64,
                                0, /* ShapeBounding */
                                0,
                                0,
                                &hole as *const _ as *const _,
                                1,
                                3, /* ShapeSubtract */
                                0, /* Unsorted */
                            );
                        }
                    }

                    // 3. Make child window transparent to mouse input so egui on parent receives clicks
                    shape_fn(
                        host.display,
                        child_hwnd as u64,
                        2, /* ShapeInput */
                        0,
                        0,
                        std::ptr::null(),
                        0,
                        0, /* ShapeSet */
                        0, /* Unsorted */
                    );
                }
                (host.x_flush)(host.display);
            } else {
                (host.x_unmap_window)(host.display, child_hwnd as u64);
                (host.x_flush)(host.display);
            }
        }
    }
}

#[cfg(not(any(windows, target_os = "linux")))]
fn create_child_video_window(_parent_hwnd: isize) -> isize {
    0
}

#[cfg(not(any(windows, target_os = "linux")))]
fn update_child_window_geometry(
    _child_hwnd: isize,
    _x: i32,
    _y: i32,
    _w: i32,
    _h: i32,
    _is_visible: bool,
    _exclusion_rects: &[Rect],
    _ppp: f32,
) {}




#[cfg(windows)]
static IS_NATIVE_FULLSCREEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(windows)]
static PRE_FULLSCREEN_RECT: std::sync::Mutex<windows_sys::Win32::Foundation::RECT> =
    std::sync::Mutex::new(windows_sys::Win32::Foundation::RECT { left: 100, top: 80, right: 1100, bottom: 700 });

#[cfg(windows)]
pub fn set_native_fullscreen(
    hwnd: isize,
    _child_hwnd: isize,
    fullscreen: bool,
    is_maximized: bool,
    saved_rect: Option<windows_sys::Win32::Foundation::RECT>,
    always_on_top: bool,
) {
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::Graphics::Gdi::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    unsafe {
        let hwnd = hwnd as HWND;
        if hwnd.is_null() { return; }
        crate::platform::set_native_fullscreen_state(fullscreen);
        IS_NATIVE_FULLSCREEN.store(fullscreen, std::sync::atomic::Ordering::Relaxed);

        if fullscreen {
            if let Some(r) = saved_rect {
                if (r.right - r.left) >= 200 && (r.bottom - r.top) >= 100 {
                    if let Ok(mut lock) = PRE_FULLSCREEN_RECT.lock() {
                        *lock = r;
                    }
                }
            } else {
                let mut cur_rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                if GetWindowRect(hwnd, &mut cur_rc) != 0 && (cur_rc.right - cur_rc.left) >= 200 && (cur_rc.bottom - cur_rc.top) >= 100 {
                    let h_mon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
                    let mut mi = MONITORINFO {
                        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                        rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                        rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                        dwFlags: 0,
                    };
                    GetMonitorInfoW(h_mon, &mut mi);
                    let is_already_mon = (cur_rc.right - cur_rc.left) >= (mi.rcMonitor.right - mi.rcMonitor.left)
                        && (cur_rc.bottom - cur_rc.top) >= (mi.rcMonitor.bottom - mi.rcMonitor.top);
                    if !is_already_mon {
                        if let Ok(mut lock) = PRE_FULLSCREEN_RECT.lock() {
                            *lock = cur_rc;
                        }
                    }
                }
            }

            let mut monitor_info = MONITORINFO {
                cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                dwFlags: 0,
            };
            let h_monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            GetMonitorInfoW(h_monitor, &mut monitor_info);

            let rc = monitor_info.rcMonitor;
            let mon_w = rc.right - rc.left;
            let mon_h = rc.bottom - rc.top;

            let style = GetWindowLongW(hwnd, GWL_STYLE) as u32;
            let fs_style = (style & !WS_THICKFRAME) | WS_POPUP;
            if fs_style != style {
                SetWindowLongW(hwnd, GWL_STYLE, fs_style as i32);
            }

            let hwnd_isize = hwnd as isize;
            std::thread::spawn(move || {
                crate::platform::mark_fullscreen_window(hwnd_isize, true);
            });

            crate::platform::apply_fullscreen_border_suppression(hwnd as _);
            let z_order = if always_on_top { -1 as isize as HWND } else { 0 as isize as HWND };
            let flags = SWP_NOCOPYBITS | SWP_NOACTIVATE | SWP_FRAMECHANGED | (if always_on_top { 0 } else { SWP_NOZORDER });
            SetWindowPos(
                hwnd,
                z_order,
                rc.left,
                rc.top,
                mon_w,
                mon_h,
                flags,
            );
            crate::platform::apply_fullscreen_border_suppression(hwnd as _);
        } else {
            let style = GetWindowLongW(hwnd, GWL_STYLE) as u32;
            let win_style = style | WS_THICKFRAME;
            if win_style != style {
                SetWindowLongW(hwnd, GWL_STYLE, win_style as i32);
            }

            let hwnd_isize = hwnd as isize;
            std::thread::spawn(move || {
                crate::platform::mark_fullscreen_window(hwnd_isize, false);
            });

            crate::platform::apply_border_suppression(hwnd as _);
            let z_order = if always_on_top { -1 as isize as HWND } else { -2 as isize as HWND };
            let flags = SWP_NOCOPYBITS | SWP_NOACTIVATE | SWP_FRAMECHANGED | (if always_on_top { 0 } else { SWP_NOZORDER });
            if is_maximized {
                let h_monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
                let mut monitor_info = MONITORINFO {
                    cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                    rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                    rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                    dwFlags: 0,
                };
                GetMonitorInfoW(h_monitor, &mut monitor_info);
                let rw = monitor_info.rcWork;
                let w = rw.right - rw.left;
                let h = rw.bottom - rw.top;
                SetWindowPos(
                    hwnd,
                    z_order,
                    rw.left,
                    rw.top,
                    w,
                    h,
                    flags,
                );
            } else {
                let fallback = PRE_FULLSCREEN_RECT.lock().map(|l| *l).unwrap_or(windows_sys::Win32::Foundation::RECT { left: 100, top: 80, right: 1100, bottom: 700 });
                let rc = saved_rect.unwrap_or(fallback);
                let w = (rc.right - rc.left).max(320);
                let h = (rc.bottom - rc.top).max(200);
                SetWindowPos(
                    hwnd,
                    z_order,
                    rc.left,
                    rc.top,
                    w,
                    h,
                    flags,
                );
            }
            crate::platform::apply_border_suppression(hwnd as _);
        }
    }
}
#[cfg(not(windows))]
pub fn set_native_fullscreen(_hwnd: isize, _child_hwnd: isize, _fullscreen: bool, _is_maximized: bool, _saved_rect: Option<()>, _always_on_top: bool) {}

#[cfg(windows)]
static PRE_PIP_RECT: std::sync::Mutex<windows_sys::Win32::Foundation::RECT> =
    std::sync::Mutex::new(windows_sys::Win32::Foundation::RECT { left: 100, top: 80, right: 1100, bottom: 700 });

#[cfg(windows)]
pub fn set_native_pip(hwnd: isize, enable: bool) {
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::Graphics::Gdi::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    unsafe {
        let hwnd = hwnd as HWND;
        if hwnd.is_null() { return; }
        if enable {
            let mut cur_rc = RECT { left: 100, top: 80, right: 1100, bottom: 700 };
            GetWindowRect(hwnd, &mut cur_rc);
            if let Ok(mut lock) = PRE_PIP_RECT.lock() {
                *lock = cur_rc;
            }
            let mut monitor_info = MONITORINFO {
                cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
                dwFlags: 0,
            };
            let h_monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            GetMonitorInfoW(h_monitor, &mut monitor_info);
            let w = 460;
            let h = 260;
            let x = monitor_info.rcWork.right - w - 24;
            let y = monitor_info.rcWork.bottom - h - 24;
            SetWindowPos(hwnd, HWND_TOPMOST, x, y, w, h, SWP_SHOWWINDOW | SWP_FRAMECHANGED);
        } else {
            let rc = PRE_PIP_RECT.lock().map(|l| *l).unwrap_or(windows_sys::Win32::Foundation::RECT { left: 100, top: 80, right: 1100, bottom: 700 });
            SetWindowPos(
                hwnd,
                HWND_NOTOPMOST,
                rc.left,
                rc.top,
                (rc.right - rc.left).max(640),
                (rc.bottom - rc.top).max(400),
                SWP_SHOWWINDOW | SWP_FRAMECHANGED,
            );
        }
    }
}
#[cfg(not(windows))]
pub fn set_native_pip(_hwnd: isize, _enable: bool) {}

#[cfg(windows)]
fn enable_native_borderless_resizing(parent_hwnd: isize) {
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    unsafe {
        let hwnd = parent_hwnd as HWND;
        crate::platform::attach_subclass(parent_hwnd);

        SetWindowPos(hwnd, 0 as _, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED | SWP_SHOWWINDOW);
        ShowWindow(hwnd, SW_SHOW);
        SetForegroundWindow(hwnd);
        crate::platform::apply_border_suppression(hwnd);
        crate::log_step("trace: ShowWindow SW_SHOW and SetForegroundWindow called successfully!");
    }
}


#[cfg(not(windows))]
fn enable_native_borderless_resizing(_parent_hwnd: isize) {}

#[cfg(windows)]
fn create_native_hicon() -> Option<windows_sys::Win32::UI::WindowsAndMessaging::HICON> {
    use windows_sys::Win32::Graphics::Gdi::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;

    const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");
    if let Ok(img) = image::load_from_memory(ICON_BYTES) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let raw = rgba.as_raw();

        unsafe {
            let bi = BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            };

            let hdc = GetDC(0 as _);
            let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
            let hbm_color = CreateDIBSection(
                hdc,
                &bi as *const _ as *const BITMAPINFO,
                DIB_RGB_COLORS,
                &mut bits,
                0 as _,
                0,
            );

            if !bits.is_null() && !hbm_color.is_null() {
                let total_pixels = (width * height) as usize;
                let dst = std::slice::from_raw_parts_mut(bits as *mut u8, total_pixels * 4);
                for i in 0..total_pixels {
                    let src_idx = i * 4;
                    dst[src_idx] = raw[src_idx + 2];     // Blue
                    dst[src_idx + 1] = raw[src_idx + 1]; // Green
                    dst[src_idx + 2] = raw[src_idx];     // Red
                    dst[src_idx + 3] = raw[src_idx + 3]; // Alpha
                }
            }

            let hbm_mask = CreateCompatibleBitmap(hdc, width as i32, height as i32);
            ReleaseDC(0 as _, hdc);

            let mut ii = ICONINFO {
                fIcon: 1,
                xHotspot: 0,
                yHotspot: 0,
                hbmMask: hbm_mask,
                hbmColor: hbm_color,
            };

            let h_icon = CreateIconIndirect(&mut ii);
            if !hbm_color.is_null() { DeleteObject(hbm_color as _); }
            if !hbm_mask.is_null() { DeleteObject(hbm_mask as _); }

            if !h_icon.is_null() {
                return Some(h_icon);
            }
        }
    }
    None
}
