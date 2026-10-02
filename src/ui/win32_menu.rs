#![allow(dead_code)]

pub const CMD_MOTION_INTERP: u32 = 5800;
pub const CMD_SHADER_STUDIO: u32 = 5801;
pub const CMD_BROADCAST: u32 = 5802;
pub const CMD_3D_SUB_DEPTH_0: u32 = 5810;
pub const CMD_3D_SUB_DEPTH_1: u32 = 5811;
pub const CMD_3D_SUB_DEPTH_2: u32 = 5812;
pub const CMD_3D_SUB_DEPTH_3: u32 = 5813;

pub const CMD_STEREO_3D_STUDIO: u32 = 5820;
pub const CMD_KARAOKE_STUDIO: u32 = 5821;
pub const CMD_SUBTITLE_STUDIO: u32 = 5822;
pub const CMD_SCREEN_CAPTURE: u32 = 5823;
pub const CMD_IPTV_GUIDE: u32 = 5824;
pub const CMD_LOGO_WATERMARK: u32 = 5825;
pub const CMD_CD_RIPPER: u32 = 5826;
pub const CMD_DISC_NAV: u32 = 5827;
pub const CMD_BDA_TUNER: u32 = 5828;
pub const CMD_BDJ_STUDIO: u32 = 5829;
pub const CMD_VST_WINAMP: u32 = 5830;
pub const CMD_DETACHED_WINDOWS: u32 = 5831;
pub const CMD_DISCORD_RPC: u32 = 5832;
pub const CMD_WEB_REMOTE: u32 = 5833;
pub const CMD_SCROBBLER: u32 = 5834;
pub const CMD_MEDIA_SERVER: u32 = 5835;
pub const CMD_AMBILIGHT: u32 = 5836;
pub const CMD_CAST_RENDERER: u32 = 5837;
pub const CMD_TRANSCODER: u32 = 5838;
pub const CMD_ZOOM_MAGNIFIER: u32 = 5839;
pub const CMD_RADIO_DIRECTORY: u32 = 5840;
pub const CMD_VIDEO_PUZZLE: u32 = 5841;
pub const CMD_VR_360_STUDIO: u32 = 5842;
pub const CMD_DAMAGED_FILE_REPAIR: u32 = 5843;
pub const CMD_BINAURAL_CROSSFEED: u32 = 5844;
pub const CMD_VIDEO_WALL_MATRIX: u32 = 5845;
pub const CMD_CHAPTER_MARKER: u32 = 5846;
pub const CMD_AUDIO_COMPRESSOR: u32 = 5847;
pub const CMD_VIDEO_CROP: u32 = 5848;
pub const CMD_GOTO_FRAME: u32 = 5849;
pub const CMD_PLAYBACK_HISTORY: u32 = 5850;
pub const CMD_MEDIA_TAG_EDITOR: u32 = 5851;
pub const CMD_DEINTERLACE: u32 = 5852;
pub const CMD_AB_REPEAT: u32 = 5853;
pub const CMD_BOSS_KEY: u32 = 5854;
pub const CMD_MOTION_INTERPOLATION: u32 = 5855;
pub const CMD_COLOR_LUT: u32 = 5856;
pub const CMD_SUBTITLE_TRANSLATOR: u32 = 5857;
pub const CMD_AI_SUBTITLE: u32 = 5858;
pub const CMD_AI_UPSCALING: u32 = 5859;
pub const CMD_AI_AUDIO: u32 = 5860;
pub const CMD_LOUDNESS_RADAR: u32 = 5861;
pub const CMD_FRAME_DUMPER: u32 = 5862;
pub const CMD_COLOR_BLINDNESS: u32 = 5863;
pub const CMD_CINEMA_MATTE: u32 = 5864;
pub const CMD_MOTION_VECTOR_INSPECTOR: u32 = 5865;
pub const CMD_RICH_BOOKMARK_NOTES: u32 = 5866;
pub const CMD_PITCH_FORMANT: u32 = 5867;

pub const CMD_GIF_MAKER: u32 = 5700;
pub const CMD_WIN_DEFAULT: u32 = 5880;
pub const CMD_WIN_MAXIMIZE: u32 = 5881;
pub const CMD_WIN_MINIMIZE: u32 = 5882;
pub const CMD_DEVICE_CAPTURE: u32 = 5701;
pub const CMD_SUBTITLE_LOOKUP: u32 = 5702;
pub const CMD_AUDIO_PASSTHROUGH: u32 = 5703;
pub const CMD_REVERB_OFF: u32 = 5710;
pub const CMD_REVERB_STUDIO: u32 = 5711;
pub const CMD_REVERB_LIVING_ROOM: u32 = 5712;
pub const CMD_REVERB_CONCERT_HALL: u32 = 5713;
pub const CMD_REVERB_ARENA: u32 = 5714;

pub const CMD_SEC_SUB_TRACK_BASE: u32 = 5500;
pub const CMD_SEC_SUB_NONE: u32 = 5521;
pub const CMD_3D_OFF: u32 = 5600;
pub const CMD_3D_SBS_2D: u32 = 5601;
pub const CMD_3D_TAB_2D: u32 = 5602;
pub const CMD_3D_SBS_ANAGLYPH: u32 = 5603;
pub const CMD_3D_TAB_ANAGLYPH: u32 = 5604;
pub const CMD_3D_SBS_AMBER_BLUE: u32 = 5605;
pub const CMD_360_VR: u32 = 5606;
pub const CMD_SEAMLESS_STITCH: u32 = 5607;
pub const CMD_RECORD_STREAM: u32 = 5650;
pub const CMD_CONTACT_SHEET: u32 = 5651;
pub const CMD_REGISTER_ASSOC: u32 = 5652;
pub const CMD_PIP: u32 = 5660;

// Pan & Scan / Zoom Commands
pub const CMD_ZOOM_50: u32 = 4101;
pub const CMD_ZOOM_75: u32 = 4102;
pub const CMD_ZOOM_100: u32 = 4103;
pub const CMD_ZOOM_125: u32 = 4104;
pub const CMD_ZOOM_150: u32 = 4105;
pub const CMD_ZOOM_200: u32 = 4106;
pub const CMD_PANSCAN_RESET: u32 = 4107;
pub const CMD_CROP_16_9: u32 = 4108;
pub const CMD_CROP_235: u32 = 4109;

// Playback Jumps & Navigation
pub const CMD_JUMP_TIME: u32 = 2200;
pub const CMD_JUMP_PERCENT_BASE: u32 = 2210; // 2210..2220
pub const CMD_SEEK_KEYFRAME_FWD: u32 = 2221;
pub const CMD_SEEK_KEYFRAME_BACK: u32 = 2222;
pub const CMD_SUB_SEEK_PREV: u32 = 2223;
pub const CMD_SUB_SEEK_NEXT: u32 = 2224;


#[cfg(windows)]
use crate::config::{AppConfig, ThemeMode};
#[cfg(windows)]
use crate::engine::audio_dsp::VORTEX_EQ_PRESETS;
#[cfg(windows)]
use crate::engine::{MediaStats, Player};
#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
#[cfg(windows)]
use windows_sys::Win32::Foundation::POINT;
#[cfg(windows)]
use windows_sys::Win32::Graphics::Gdi::ClientToScreen;
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::*;

pub const CMD_OPEN_FILE: u32 = 1001;
pub const CMD_OPEN_FOLDER: u32 = 1002;
pub const CMD_OPEN_URL: u32 = 1003;
pub const CMD_CLOSE_PLAYBACK: u32 = 1004;
pub const CMD_FAVORITES_ADD_ITEM: u32 = 1005;
pub const CMD_FAVORITES_ADD_FOLDER: u32 = 1006;
pub const CMD_FAVORITES_EDIT: u32 = 1007;
pub const CMD_RECENT_BASE: u32 = 1100; // 1100..1130

pub const CMD_PLAY_PAUSE: u32 = 2001;
pub const CMD_STOP: u32 = 2002;
pub const CMD_PREV_FILE: u32 = 2003;
pub const CMD_NEXT_FILE: u32 = 2004;
pub const CMD_SEEK_FWD_SHORT: u32 = 2005;
pub const CMD_SEEK_BACK_SHORT: u32 = 2006;
pub const CMD_SEEK_FWD_MED: u32 = 2007;
pub const CMD_SEEK_BACK_MED: u32 = 2008;
pub const CMD_SPEED_NORMAL: u32 = 2009;
pub const CMD_SPEED_DOWN: u32 = 2010;
pub const CMD_SPEED_UP: u32 = 2011;
pub const CMD_FRAME_STEP_FWD: u32 = 2012;
pub const CMD_FRAME_STEP_BACK: u32 = 2013;
pub const CMD_AB_LOOP_A: u32 = 2014;
pub const CMD_AB_LOOP_B: u32 = 2015;
pub const CMD_AB_LOOP_CLEAR: u32 = 2016;
pub const CMD_PREV_CHAPTER: u32 = 2017;
pub const CMD_NEXT_CHAPTER: u32 = 2018;
pub const CMD_CHAPTER_BASE: u32 = 2100; // 2100..2150

pub const CMD_SUBTITLE_TOGGLE: u32 = 3001;
pub const CMD_SUBTITLE_LOAD: u32 = 3002;
pub const CMD_SUBTITLE_DELAY_DOWN: u32 = 3003;
pub const CMD_SUBTITLE_DELAY_UP: u32 = 3004;
pub const CMD_SUBTITLE_TRACK_BASE: u32 = 3100;
pub const CMD_SUBTITLE_CYCLE: u32 = 3200;

pub const CMD_AR_AUTO: u32 = 4001;
pub const CMD_AR_16_9: u32 = 4002;
pub const CMD_AR_4_3: u32 = 4003;
pub const CMD_AR_185: u32 = 4004;
pub const CMD_AR_235: u32 = 4005;
pub const CMD_AR_FILL: u32 = 4006;
pub const CMD_HWDEC_AUTO: u32 = 4010;
pub const CMD_HWDEC_D3D11VA: u32 = 4011;
pub const CMD_HWDEC_DXVA2: u32 = 4012;
pub const CMD_HWDEC_NVDEC: u32 = 4013;
pub const CMD_HWDEC_NO: u32 = 4014;
pub const CMD_ROT_0: u32 = 4020;
pub const CMD_ROT_90: u32 = 4021;
pub const CMD_ROT_180: u32 = 4022;
pub const CMD_ROT_270: u32 = 4023;
pub const CMD_SNAPSHOT: u32 = 4030;
pub const CMD_RESET_VIDEO: u32 = 4031;

pub const CMD_AUDIO_MUTE: u32 = 5001;
pub const CMD_AUDIO_DELAY_DOWN: u32 = 5002;
pub const CMD_AUDIO_DELAY_UP: u32 = 5003;
pub const CMD_AUDIO_TRACK_BASE: u32 = 5100;
pub const CMD_AUDIO_EQ_BASE: u32 = 5200;
pub const CMD_AUDIO_NORMALIZE: u32 = 5300;
pub const CMD_AUDIO_WASAPI: u32 = 5301;
pub const CMD_AUDIO_CYCLE: u32 = 5302;
pub const CMD_AUDIO_PEQ: u32 = 5303;
pub const CMD_VOCAL_REMOVER: u32 = 5310;
pub const CMD_VOICE_ENHANCE: u32 = 5311;
pub const CMD_AUDIO_DEV_AUTO: u32 = 5900;
pub const CMD_AUDIO_DEV_BASE: u32 = 5901; // 5901..5950

// Audio Channels Constants
pub const CMD_CH_SAME_AS_INPUT: u32 = 5401;
pub const CMD_CH_1_0: u32 = 5402;
pub const CMD_CH_2_0: u32 = 5403;
pub const CMD_CH_2_1: u32 = 5404;
pub const CMD_CH_3_0: u32 = 5405;
pub const CMD_CH_3_0_SURR: u32 = 5406;
pub const CMD_CH_3_1_SURR: u32 = 5407;
pub const CMD_CH_4_0: u32 = 5408;
pub const CMD_CH_4_0_SURR: u32 = 5409;
pub const CMD_CH_4_1_SURR: u32 = 5410;
pub const CMD_CH_5_0: u32 = 5411;
pub const CMD_CH_5_1: u32 = 5412;
pub const CMD_CH_6_1: u32 = 5413;
pub const CMD_CH_7_1: u32 = 5414;
pub const CMD_CH_VIRTUAL_HEADPHONE: u32 = 5415;
pub const CMD_CH_VIRTUAL_SURROUND: u32 = 5416;
pub const CMD_CH_VIRTUAL_DOLBY: u32 = 5417;
pub const CMD_CH_DOLBY_PL2: u32 = 5418;
pub const CMD_CH_DOLBY_PL2_LFE: u32 = 5419;
pub const CMD_CH_BOTH: u32 = 5420;
pub const CMD_CH_LEFT: u32 = 5421;
pub const CMD_CH_RIGHT: u32 = 5422;

pub const CMD_SKIN_BASE: u32 = 6000;

pub const CMD_FRAME_SIZE_05: u32 = 7011;
pub const CMD_FRAME_SIZE_10: u32 = 7012;
pub const CMD_FRAME_SIZE_15: u32 = 7013;
pub const CMD_FRAME_SIZE_20: u32 = 7014;
pub const CMD_FRAME_SIZE_FIT: u32 = 7015;

pub const CMD_FULLSCREEN_KEEP: u32 = 7001;
pub const CMD_FULLSCREEN_STRETCH: u32 = 7002;

pub const CMD_PREFERENCES: u32 = 8001;
pub const CMD_PLAYLIST: u32 = 8002;
pub const CMD_CONTROL_PANEL: u32 = 8003;
pub const CMD_MEDIA_INFO: u32 = 8004;
pub const CMD_ABOUT: u32 = 8005;
pub const CMD_BOOKMARKS: u32 = 8006;
pub const CMD_ADD_BOOKMARK: u32 = 8007;

pub const CMD_EXIT: u32 = 9999;

#[cfg(windows)]
fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

#[cfg(windows)]
pub fn show_audio_channels_popup_menu(hwnd: isize, selected_id: u32) -> u32 {
    unsafe {
        let menu = CreatePopupMenu();
        if menu.is_null() { return 0; }

        let is_same_input = selected_id == CMD_CH_SAME_AS_INPUT || selected_id == 0;
        AppendMenuW(menu, if is_same_input { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_SAME_AS_INPUT as usize, to_wide("Same as input").as_ptr());
        AppendMenuW(menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(menu, if selected_id == CMD_CH_1_0 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_1_0 as usize, to_wide("1.0 mono (Single Channel)").as_ptr());
        AppendMenuW(menu, if selected_id == CMD_CH_2_0 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_2_0 as usize, to_wide("2.0 stereo (Default)").as_ptr());
        AppendMenuW(menu, if selected_id == CMD_CH_2_1 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_2_1 as usize, to_wide("2.1 stereo + LFE").as_ptr());
        AppendMenuW(menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(menu, if selected_id == CMD_CH_3_0 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_3_0 as usize, to_wide("3.0 (Front L, R, C)").as_ptr());
        AppendMenuW(menu, if selected_id == CMD_CH_4_0 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_4_0 as usize, to_wide("4.0 Quadraphonic").as_ptr());
        AppendMenuW(menu, if selected_id == CMD_CH_5_1 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_5_1 as usize, to_wide("5.1 Surround (6 Channels)").as_ptr());
        AppendMenuW(menu, if selected_id == CMD_CH_6_1 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_6_1 as usize, to_wide("6.1 Surround").as_ptr());
        AppendMenuW(menu, if selected_id == CMD_CH_7_1 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_7_1 as usize, to_wide("7.1 Surround (8 Channels)").as_ptr());
        AppendMenuW(menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(menu, if selected_id == CMD_CH_DOLBY_PL2 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_DOLBY_PL2 as usize, to_wide("Dolby Surround / Pro Logic II").as_ptr());
        AppendMenuW(menu, if selected_id == CMD_CH_VIRTUAL_HEADPHONE { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_VIRTUAL_HEADPHONE as usize, to_wide("Virtual Headphone (Bauer bs2b)").as_ptr());
        AppendMenuW(menu, if selected_id == CMD_CH_VIRTUAL_SURROUND { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_VIRTUAL_SURROUND as usize, to_wide("Virtual 3D Surround (HRTF Sofalizer)").as_ptr());

        let mut pt: POINT = std::mem::zeroed();
        GetCursorPos(&mut pt);

        if hwnd != 0 {
            SetForegroundWindow(hwnd as _);
        }

        let cmd = TrackPopupMenuEx(
            menu,
            TPM_RETURNCMD | TPM_LEFTALIGN | TPM_BOTTOMALIGN | TPM_RIGHTBUTTON,
            pt.x,
            pt.y,
            hwnd as _,
            std::ptr::null(),
        );

        if hwnd != 0 {
            PostMessageW(hwnd as _, WM_NULL, 0, 0);
        }

        DestroyMenu(menu);
        cmd as u32
    }
}

#[cfg(not(windows))]
pub fn show_audio_channels_popup_menu(_hwnd: isize, _selected: u32) -> u32 { 0 }

#[cfg(not(windows))]
pub fn show_native_popup_menu(
    _hwnd: isize,
    _stats: &crate::engine::MediaStats,
    _config: &crate::config::AppConfig,
    _custom_pos: Option<(i32, i32)>,
) -> u32 { 0 }

#[cfg(windows)]
pub fn show_native_popup_menu(
    hwnd: isize,
    stats: &MediaStats,
    config: &AppConfig,
    custom_pos: Option<(i32, i32)>,
) -> u32 {
    unsafe {
        // Try enabling dark mode for Win32 menus
        let uxtheme = windows_sys::Win32::System::LibraryLoader::LoadLibraryA(b"uxtheme.dll\0".as_ptr());
        if !uxtheme.is_null() {
            type SetPreferredAppModeFn = unsafe extern "system" fn(i32) -> i32;
            let set_mode: Option<SetPreferredAppModeFn> = std::mem::transmute(
                windows_sys::Win32::System::LibraryLoader::GetProcAddress(uxtheme, 135 as _),
            );
            if let Some(f) = set_mode {
                f(2); // ForceDark
            }
        }

        let root_menu = CreatePopupMenu();
        if root_menu.is_null() {
            return 0;
        }

        // 1. OPEN SUBMENU
        let open_menu = CreatePopupMenu();
        AppendMenuW(open_menu, MF_STRING, CMD_OPEN_FILE as usize, to_wide("Open File(s)...\tCtrl+O").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_OPEN_FOLDER as usize, to_wide("Open Folder...\tCtrl+F").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_OPEN_URL as usize, to_wide("Open URL / Stream...\tCtrl+U").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_DEVICE_CAPTURE as usize, to_wide("Device Capture / Webcam / HDMI\tCtrl+W").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_SCREEN_CAPTURE as usize, to_wide("Desktop Screen & Region Capture...\tCtrl+Shift+S").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_IPTV_GUIDE as usize, to_wide("IPTV & EPG Digital Broadcasts...\tCtrl+I").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_DISC_NAV as usize, to_wide("DVD / Blu-ray Disc Browser...\tCtrl+D").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_CD_RIPPER as usize, to_wide("Audio CD (CDDA) Ripper Studio...\tCtrl+Shift+D").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_BDA_TUNER as usize, to_wide("Digital TV Tuner (BDA) Scanning...\tCtrl+Alt+T").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_BDJ_STUDIO as usize, to_wide("Java BD-J Interactive Blu-ray Menu...\tCtrl+Alt+B").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_MEDIA_SERVER as usize, to_wide("Plex / Jellyfin / Emby Cloud Browser...\tCtrl+Alt+P").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_TRANSCODER as usize, to_wide("Convert / Transcode Media...\tCtrl+R").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_RADIO_DIRECTORY as usize, to_wide("Icecast & Shoutcast Internet Radio...\tCtrl+Alt+I").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_DAMAGED_FILE_REPAIR as usize, to_wide("Damaged Media Repair & Codec Doctor...\tCtrl+Alt+F").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_PLAYBACK_HISTORY as usize, to_wide("Playback History & Statistics...\tCtrl+Alt+Y").as_ptr());
        AppendMenuW(open_menu, MF_STRING, CMD_MEDIA_TAG_EDITOR as usize, to_wide("Media Metadata / Tag Editor...\tCtrl+Alt+E").as_ptr());

        AppendMenuW(open_menu, MF_SEPARATOR, 0, std::ptr::null());








        let recent_menu = CreatePopupMenu();
        if config.recent_files.is_empty() {
            AppendMenuW(recent_menu, MF_STRING | MF_GRAYED, 0, to_wide("(No recent files)").as_ptr());
        } else {
            for (i, path_str) in config.recent_files.iter().take(20).enumerate() {
                let name = std::path::Path::new(path_str).file_name().and_then(|n| n.to_str()).unwrap_or(path_str);
                AppendMenuW(recent_menu, MF_STRING, (CMD_RECENT_BASE + i as u32) as usize, to_wide(name).as_ptr());
            }
        }
        AppendMenuW(open_menu, MF_POPUP, recent_menu as usize, to_wide("Recent Files").as_ptr());

        // ALBUM/FAVORITES SUBMENU (Matching media_1787017395599.png exactly)
        let fav_menu = CreatePopupMenu();
        AppendMenuW(fav_menu, MF_STRING, CMD_FAVORITES_ADD_ITEM as usize, to_wide("Add Current Item to Favorites\tAlt+Insert").as_ptr());
        AppendMenuW(fav_menu, MF_STRING, CMD_FAVORITES_ADD_FOLDER as usize, to_wide("Add Current Folder to Favorites\tCtrl+Insert").as_ptr());
        AppendMenuW(fav_menu, MF_STRING, CMD_FAVORITES_EDIT as usize, to_wide("Edit Favorites...").as_ptr());

        // Root Section 1 (Matching screenshot exactly)
        AppendMenuW(root_menu, MF_STRING, CMD_OPEN_FILE as usize, to_wide("Open File(s)...\tF3").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, open_menu as usize, to_wide("Open").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, fav_menu as usize, to_wide("Album/Favorites").as_ptr());
        AppendMenuW(root_menu, MF_STRING, CMD_CLOSE_PLAYBACK as usize, to_wide("Close Playback\tF4").as_ptr());
        AppendMenuW(root_menu, MF_SEPARATOR, 0, std::ptr::null());

        // 2. PLAYBACK SUBMENU
        let play_menu = CreatePopupMenu();
        let play_text = if stats.is_paused { "Play\tSpace" } else { "Pause\tSpace" };
        AppendMenuW(play_menu, MF_STRING, CMD_PLAY_PAUSE as usize, to_wide(play_text).as_ptr());
        AppendMenuW(play_menu, MF_STRING, CMD_STOP as usize, to_wide("Stop\tCtrl+Space").as_ptr());
        AppendMenuW(play_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(play_menu, MF_STRING, CMD_PREV_FILE as usize, to_wide("Previous File\tPage Up").as_ptr());
        AppendMenuW(play_menu, MF_STRING, CMD_NEXT_FILE as usize, to_wide("Next File\tPage Down").as_ptr());
        AppendMenuW(play_menu, MF_SEPARATOR, 0, std::ptr::null());
        let has_chapters = !stats.chapters.is_empty();
        let ch_nav_flag = if has_chapters { MF_STRING } else { MF_STRING | MF_GRAYED };
        AppendMenuW(play_menu, ch_nav_flag, CMD_PREV_CHAPTER as usize, to_wide("Previous Chapter\tShift+H").as_ptr());
        AppendMenuW(play_menu, ch_nav_flag, CMD_NEXT_CHAPTER as usize, to_wide("Next Chapter\tH").as_ptr());
        AppendMenuW(play_menu, MF_SEPARATOR, 0, std::ptr::null());

        // CHAPTERS SUBMENU
        let ch_menu = CreatePopupMenu();
        AppendMenuW(ch_menu, ch_nav_flag, CMD_PREV_CHAPTER as usize, to_wide("Previous Chapter\tShift+H").as_ptr());
        AppendMenuW(ch_menu, ch_nav_flag, CMD_NEXT_CHAPTER as usize, to_wide("Next Chapter\tH").as_ptr());
        AppendMenuW(ch_menu, MF_STRING, CMD_CHAPTER_MARKER as usize, to_wide("Chapter Timeline Manager...\tCtrl+Alt+J").as_ptr());
        AppendMenuW(ch_menu, MF_SEPARATOR, 0, std::ptr::null());
        if stats.chapters.is_empty() {
            AppendMenuW(ch_menu, MF_STRING | MF_GRAYED, 0, to_wide("(No chapters)").as_ptr());
        } else {
            for (i, ch) in stats.chapters.iter().take(50).enumerate() {
                let flags = if stats.current_chapter == Some(ch.index) { MF_STRING | MF_CHECKED } else { MF_STRING };
                let time_str = crate::bookmark::format_time(ch.time_pos);
                let label = if ch.title.trim().is_empty() {
                    format!("Chapter {} ({})", i + 1, time_str)
                } else {
                    format!("{} ({})", ch.title.trim(), time_str)
                };
                AppendMenuW(ch_menu, flags, (CMD_CHAPTER_BASE + i as u32) as usize, to_wide(&label).as_ptr());
            }
        }
        AppendMenuW(play_menu, MF_POPUP, ch_menu as usize, to_wide("Chapters").as_ptr());
        AppendMenuW(play_menu, MF_SEPARATOR, 0, std::ptr::null());

        // JUMP / NAVIGATION SUBMENU
        let jump_menu = CreatePopupMenu();
        AppendMenuW(jump_menu, MF_STRING, CMD_CHAPTER_MARKER as usize, to_wide("Chapter Timeline Manager...\tCtrl+Alt+J").as_ptr());
        AppendMenuW(jump_menu, MF_STRING, CMD_JUMP_TIME as usize, to_wide("Jump to Time...\tG").as_ptr());
        AppendMenuW(jump_menu, MF_STRING, CMD_GOTO_FRAME as usize, to_wide("Go To Frame Number...\tCtrl+Shift+G").as_ptr());
        AppendMenuW(jump_menu, MF_STRING, CMD_AB_REPEAT as usize, to_wide("A-B Repeat Interval Looper...\tCtrl+Shift+L").as_ptr());
        AppendMenuW(jump_menu, MF_STRING, CMD_RICH_BOOKMARK_NOTES as usize, to_wide("Rich Bookmarks & Study Notes...\tCtrl+Alt+8").as_ptr());
        AppendMenuW(jump_menu, MF_SEPARATOR, 0, std::ptr::null());




        AppendMenuW(jump_menu, MF_STRING, (CMD_JUMP_PERCENT_BASE + 0) as usize, to_wide("Jump to Start (0%)\t0").as_ptr());
        for p in 1..=9 {
            AppendMenuW(jump_menu, MF_STRING, (CMD_JUMP_PERCENT_BASE + p) as usize, to_wide(&format!("Jump to {}0%\t{}", p, p)).as_ptr());
        }
        AppendMenuW(jump_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(jump_menu, MF_STRING, CMD_SEEK_KEYFRAME_BACK as usize, to_wide("Keyframe Backward\tCtrl+←").as_ptr());
        AppendMenuW(jump_menu, MF_STRING, CMD_SEEK_KEYFRAME_FWD as usize, to_wide("Keyframe Forward\tCtrl+→").as_ptr());
        AppendMenuW(jump_menu, MF_STRING, CMD_SUB_SEEK_PREV as usize, to_wide("Previous Subtitle Line\t[").as_ptr());
        AppendMenuW(jump_menu, MF_STRING, CMD_SUB_SEEK_NEXT as usize, to_wide("Next Subtitle Line\t]").as_ptr());
        AppendMenuW(play_menu, MF_POPUP, jump_menu as usize, to_wide("Jump / Navigation").as_ptr());
        AppendMenuW(play_menu, MF_SEPARATOR, 0, std::ptr::null());

        AppendMenuW(play_menu, MF_STRING, CMD_SEEK_FWD_SHORT as usize, to_wide("Jump Forward (+5s)\t→").as_ptr());
        AppendMenuW(play_menu, MF_STRING, CMD_SEEK_BACK_SHORT as usize, to_wide("Jump Backward (-5s)\t←").as_ptr());
        AppendMenuW(play_menu, MF_STRING, CMD_SEEK_FWD_MED as usize, to_wide("Jump Forward (+30s)\tCtrl+→").as_ptr());
        AppendMenuW(play_menu, MF_STRING, CMD_SEEK_BACK_MED as usize, to_wide("Jump Backward (-30s)\tCtrl+←").as_ptr());
        AppendMenuW(play_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(play_menu, MF_STRING, CMD_SPEED_NORMAL as usize, to_wide("Speed: Normal (1.0×)\tZ").as_ptr());
        AppendMenuW(play_menu, MF_STRING, CMD_SPEED_DOWN as usize, to_wide("Speed: Slower (-0.1×)\tX").as_ptr());
        AppendMenuW(play_menu, MF_STRING, CMD_SPEED_UP as usize, to_wide("Speed: Faster (+0.1×)\tC").as_ptr());

        // 3. SUBTITLES SUBMENU
        let sub_menu = CreatePopupMenu();
        AppendMenuW(sub_menu, MF_STRING, CMD_SUBTITLE_TOGGLE as usize, to_wide("Show/Hide Subtitles\tAlt+H").as_ptr());
        AppendMenuW(sub_menu, MF_STRING, CMD_SUBTITLE_CYCLE as usize, to_wide("Cycle Subtitle Track\tS").as_ptr());
        AppendMenuW(sub_menu, MF_STRING, CMD_SUBTITLE_LOAD as usize, to_wide("Add/Load Subtitles...\tAlt+O").as_ptr());
        AppendMenuW(sub_menu, MF_SEPARATOR, 0, std::ptr::null());
        let sub_tracks_menu = CreatePopupMenu();
        if stats.subtitle_tracks.is_empty() {
            AppendMenuW(sub_tracks_menu, MF_STRING | MF_GRAYED, 0, to_wide("(None)").as_ptr());
        } else {
            for (i, track) in stats.subtitle_tracks.iter().take(20).enumerate() {
                let flags = if track.id == stats.selected_subtitle_track { MF_STRING | MF_CHECKED } else { MF_STRING };
                AppendMenuW(sub_tracks_menu, flags, (CMD_SUBTITLE_TRACK_BASE + i as u32) as usize, to_wide(&track.title).as_ptr());
            }
        }
        AppendMenuW(sub_menu, MF_POPUP, sub_tracks_menu as usize, to_wide("Select Subtitle Track").as_ptr());
        // 2nd Subtitle Track (Top)
        let sec_sub_tracks_menu = CreatePopupMenu();
        AppendMenuW(sec_sub_tracks_menu, MF_STRING, CMD_SEC_SUB_NONE as usize, to_wide("None (Disabled)").as_ptr());
        AppendMenuW(sec_sub_tracks_menu, MF_SEPARATOR, 0, std::ptr::null());
        for (i, track) in stats.subtitle_tracks.iter().take(15).enumerate() {
            AppendMenuW(sec_sub_tracks_menu, MF_STRING, (CMD_SEC_SUB_TRACK_BASE + i as u32) as usize, to_wide(&track.title).as_ptr());
        }
        AppendMenuW(sub_menu, MF_POPUP, sec_sub_tracks_menu as usize, to_wide("2nd Subtitle Track (Top Dual Subtitle)").as_ptr());

        AppendMenuW(sub_menu, MF_STRING, CMD_SUBTITLE_STUDIO as usize, to_wide("Subtitle Studio & Timeline Editor...\tCtrl+T").as_ptr());
        AppendMenuW(sub_menu, MF_STRING, CMD_SUBTITLE_TRANSLATOR as usize, to_wide("Subtitle Word Translator & Dictionary...\tCtrl+Alt+W").as_ptr());
        AppendMenuW(sub_menu, MF_STRING, CMD_AI_SUBTITLE as usize, to_wide("AI Live Speech-to-Text Transcriber (Whisper)...\tCtrl+Alt+3").as_ptr());
        AppendMenuW(sub_menu, MF_SEPARATOR, 0, std::ptr::null());


        AppendMenuW(sub_menu, MF_STRING, CMD_SUBTITLE_DELAY_DOWN as usize, to_wide("Subtitle Sync -0.5s\t<").as_ptr());
        AppendMenuW(sub_menu, MF_STRING, CMD_SUBTITLE_DELAY_UP as usize, to_wide("Subtitle Sync +0.5s\t>").as_ptr());

        // 4. VIDEO SUBMENU
        let vid_menu = CreatePopupMenu();
        let ar_menu = CreatePopupMenu();
        AppendMenuW(ar_menu, if config.aspect_ratio == "auto" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_AR_AUTO as usize, to_wide("Auto (Keep Aspect Ratio)").as_ptr());
        AppendMenuW(ar_menu, if config.aspect_ratio == "16:9" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_AR_16_9 as usize, to_wide("16:9 Wide").as_ptr());
        AppendMenuW(ar_menu, if config.aspect_ratio == "4:3" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_AR_4_3 as usize, to_wide("4:3 Standard").as_ptr());
        AppendMenuW(ar_menu, if config.aspect_ratio == "1.85:1" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_AR_185 as usize, to_wide("1.85:1 Cinema").as_ptr());
        AppendMenuW(ar_menu, if config.aspect_ratio == "2.35:1" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_AR_235 as usize, to_wide("2.35:1 Anamorphic").as_ptr());
        AppendMenuW(ar_menu, if config.aspect_ratio == "fill" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_AR_FILL as usize, to_wide("Fit to Window").as_ptr());
        AppendMenuW(vid_menu, MF_POPUP, ar_menu as usize, to_wide("Aspect Ratio").as_ptr());

        // PAN & SCAN / ZOOM SUBMENU
        let zoom_menu = CreatePopupMenu();
        AppendMenuW(zoom_menu, MF_STRING, CMD_ZOOM_50 as usize, to_wide("50% Half Size").as_ptr());
        AppendMenuW(zoom_menu, MF_STRING, CMD_ZOOM_75 as usize, to_wide("75% Size").as_ptr());
        AppendMenuW(zoom_menu, MF_STRING, CMD_ZOOM_100 as usize, to_wide("100% Original Size\tNumPad 5").as_ptr());
        AppendMenuW(zoom_menu, MF_STRING, CMD_ZOOM_125 as usize, to_wide("125% Size").as_ptr());
        AppendMenuW(zoom_menu, MF_STRING, CMD_ZOOM_150 as usize, to_wide("150% Size").as_ptr());
        AppendMenuW(zoom_menu, MF_STRING, CMD_ZOOM_200 as usize, to_wide("200% Double Size").as_ptr());
        AppendMenuW(zoom_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(zoom_menu, MF_STRING, CMD_CROP_16_9 as usize, to_wide("Crop to 16:9 Wide").as_ptr());
        AppendMenuW(zoom_menu, MF_STRING, CMD_CROP_235 as usize, to_wide("Crop to 2.35:1 Cinema").as_ptr());
        AppendMenuW(zoom_menu, MF_STRING, CMD_PANSCAN_RESET as usize, to_wide("Reset Pan & Scan\tNumPad 5").as_ptr());
        AppendMenuW(vid_menu, MF_POPUP, zoom_menu as usize, to_wide("Pan & Scan / Zoom").as_ptr());

        let hw_menu = CreatePopupMenu();
        AppendMenuW(hw_menu, if config.hardware_decoding == "auto-safe" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_HWDEC_AUTO as usize, to_wide("auto-safe").as_ptr());
        AppendMenuW(hw_menu, if config.hardware_decoding == "d3d11va" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_HWDEC_D3D11VA as usize, to_wide("d3d11va (Direct3D 11)").as_ptr());
        AppendMenuW(hw_menu, if config.hardware_decoding == "dxva2" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_HWDEC_DXVA2 as usize, to_wide("dxva2 (DirectX Video Accel)").as_ptr());
        AppendMenuW(hw_menu, if config.hardware_decoding == "nvdec" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_HWDEC_NVDEC as usize, to_wide("nvdec (NVIDIA PureVideo)").as_ptr());
        AppendMenuW(hw_menu, if config.hardware_decoding == "no" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_HWDEC_NO as usize, to_wide("no (Software CPU)").as_ptr());
        AppendMenuW(vid_menu, MF_POPUP, hw_menu as usize, to_wide("Hardware Acceleration").as_ptr());

        let rot_menu = CreatePopupMenu();
        AppendMenuW(rot_menu, if config.video_rotation == 0 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_ROT_0 as usize, to_wide("0° (Normal)").as_ptr());
        AppendMenuW(rot_menu, if config.video_rotation == 90 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_ROT_90 as usize, to_wide("90° Clockwise").as_ptr());
        AppendMenuW(rot_menu, if config.video_rotation == 180 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_ROT_180 as usize, to_wide("180° Inverted").as_ptr());
        AppendMenuW(rot_menu, if config.video_rotation == 270 { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_ROT_270 as usize, to_wide("270° Counter-Clockwise").as_ptr());
        AppendMenuW(vid_menu, MF_POPUP, rot_menu as usize, to_wide("Rotation").as_ptr());
        // 3D Video Modes
        let menu_3d = CreatePopupMenu();
        AppendMenuW(menu_3d, MF_STRING, CMD_3D_OFF as usize, to_wide("2D (Off / Normal)").as_ptr());
        AppendMenuW(menu_3d, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(menu_3d, MF_STRING, CMD_3D_SBS_2D as usize, to_wide("3D Side-by-Side (SBS) -> 2D Monoscopic").as_ptr());
        AppendMenuW(menu_3d, MF_STRING, CMD_3D_TAB_2D as usize, to_wide("3D Top-and-Bottom (TAB) -> 2D Monoscopic").as_ptr());
        AppendMenuW(menu_3d, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(menu_3d, MF_STRING, CMD_3D_SBS_ANAGLYPH as usize, to_wide("3D SBS -> Red/Cyan Anaglyph (3D Glasses)").as_ptr());
        AppendMenuW(menu_3d, MF_STRING, CMD_3D_TAB_ANAGLYPH as usize, to_wide("3D TAB -> Red/Cyan Anaglyph (3D Glasses)").as_ptr());
        AppendMenuW(menu_3d, MF_STRING, CMD_3D_SBS_AMBER_BLUE as usize, to_wide("3D SBS -> Amber/Blue ColorCode").as_ptr());
        AppendMenuW(vid_menu, MF_POPUP, menu_3d as usize, to_wide("3D Video Output Modes").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_STEREO_3D_STUDIO as usize, to_wide("Stereoscopic 3D Studio...\tCtrl+Alt+3").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_LOGO_WATERMARK as usize, to_wide("Video Watermark & Idle Logo Studio...").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_360_VR as usize, to_wide("360° VR Spherical Video Mode").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_SEAMLESS_STITCH as usize, to_wide("Seamless Multi-Part Video Stitching").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_RECORD_STREAM as usize, to_wide("Live Stream / Video Recording\tAlt+C").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_GIF_MAKER as usize, to_wide("Animated GIF & Video Clip Maker\tCtrl+G").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_MOTION_INTERP as usize, to_wide("Motion Interpolation (60fps/144fps Smooth Motion)").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_SHADER_STUDIO as usize, to_wide("Pixel Shader Studio & GLSL Editor...").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_BROADCAST as usize, to_wide("Live RTMP Broadcast Studio\tCtrl+B").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_ZOOM_MAGNIFIER as usize, to_wide("Live Zoom Magnifier Lens...\tCtrl+Alt+M").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_VIDEO_PUZZLE as usize, to_wide("Interactive Video Puzzle Game...\tCtrl+Alt+Z").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_VR_360_STUDIO as usize, to_wide("360° VR Spherical Video Studio...\tCtrl+Alt+1").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_VIDEO_WALL_MATRIX as usize, to_wide("Multi-Screen Video Wall Matrix...\tCtrl+Alt+2").as_ptr());




        AppendMenuW(vid_menu, MF_STRING, CMD_CONTACT_SHEET as usize, to_wide("Storyboard Contact Sheet Generator...").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_VIDEO_CROP as usize, to_wide("Video Crop & Pan-Scan...\tCtrl+Shift+C").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_DEINTERLACE as usize, to_wide("Deinterlace & Post-Processing...\tCtrl+Shift+I").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_MOTION_INTERPOLATION as usize, to_wide("SmoothMotion & Frame Interpolation...\tCtrl+Shift+M").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_COLOR_LUT as usize, to_wide("3D LUT Color Studio & Gamut Calibration...\tCtrl+Shift+T").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_AI_UPSCALING as usize, to_wide("AI Neural Super Resolution & RTX VSR...\tCtrl+Alt+4").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_FRAME_DUMPER as usize, to_wide("Continuous Burst Frame Dumper Studio...\tCtrl+Alt+5").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_COLOR_BLINDNESS as usize, to_wide("Color Blindness & Vision Accessibility...\tCtrl+Alt+9").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_CINEMA_MATTE as usize, to_wide("Cinema Screen Matte & Ambient Curtains...\tCtrl+Alt+0").as_ptr());
        AppendMenuW(vid_menu, MF_STRING, CMD_MOTION_VECTOR_INSPECTOR as usize, to_wide("Motion Vector & Codec Bitstream Inspector...").as_ptr());

        AppendMenuW(vid_menu, MF_SEPARATOR, 0, std::ptr::null());



        AppendMenuW(vid_menu, MF_STRING, CMD_SNAPSHOT as usize, to_wide("Capture Video / Screenshot\tCtrl+E").as_ptr());

        AppendMenuW(vid_menu, MF_STRING, CMD_RESET_VIDEO as usize, to_wide("Reset Video Colors\tQ").as_ptr());

        // 5. AUDIO SUBMENU
        let aud_menu = CreatePopupMenu();
        AppendMenuW(aud_menu, MF_STRING, CMD_KARAOKE_STUDIO as usize, to_wide("Karaoke & Pitch/Tempo Studio...\tCtrl+K").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_AUDIO_CYCLE as usize, to_wide("Cycle Audio Stream\tAlt+A").as_ptr());

        let aud_tracks_menu = CreatePopupMenu();
        AppendMenuW(aud_tracks_menu, MF_STRING, CMD_AUDIO_CYCLE as usize, to_wide("Cycle Audio Stream\tAlt+A").as_ptr());
        if stats.audio_tracks.is_empty() {
            AppendMenuW(aud_tracks_menu, MF_STRING | MF_GRAYED, 0, to_wide("(None)").as_ptr());
        } else {
            for (i, track) in stats.audio_tracks.iter().take(20).enumerate() {
                let flags = if track.id == stats.selected_audio_track { MF_STRING | MF_CHECKED } else { MF_STRING };
                AppendMenuW(aud_tracks_menu, flags, (CMD_AUDIO_TRACK_BASE + i as u32) as usize, to_wide(&track.title).as_ptr());
            }
        }
        AppendMenuW(aud_tracks_menu, MF_SEPARATOR, 0, std::ptr::null());
        let curr_ch = &config.audio_channels;
        let is_both = curr_ch != "left" && curr_ch != "right";
        let is_left = curr_ch == "left";
        let is_right = curr_ch == "right";
        AppendMenuW(aud_tracks_menu, if is_both { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_BOTH as usize, to_wide("Both").as_ptr());
        AppendMenuW(aud_tracks_menu, if is_left { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_LEFT as usize, to_wide("Left channel").as_ptr());
        AppendMenuW(aud_tracks_menu, if is_right { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_RIGHT as usize, to_wide("Right channel").as_ptr());

        AppendMenuW(aud_menu, MF_POPUP, aud_tracks_menu as usize, to_wide("Select Audio Stream").as_ptr());

        let ch_menu = CreatePopupMenu();
        let curr_ch = &config.audio_channels;
        let is_same = curr_ch == "auto" || curr_ch == "auto-safe" || curr_ch.is_empty();
        AppendMenuW(ch_menu, if is_same { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_SAME_AS_INPUT as usize, to_wide("Same as input").as_ptr());
        AppendMenuW(ch_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(ch_menu, if curr_ch == "mono" || curr_ch == "1.0" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_1_0 as usize, to_wide("1.0 mono (Single Channel)").as_ptr());
        AppendMenuW(ch_menu, if curr_ch == "stereo" || curr_ch == "2.0" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_2_0 as usize, to_wide("2.0 stereo (Default)").as_ptr());
        AppendMenuW(ch_menu, if curr_ch == "2.1" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_2_1 as usize, to_wide("2.1 stereo + LFE").as_ptr());
        AppendMenuW(ch_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(ch_menu, if curr_ch == "3.0" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_3_0 as usize, to_wide("3.0 (Front L, R, C)").as_ptr());
        AppendMenuW(ch_menu, if curr_ch == "4.0" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_4_0 as usize, to_wide("4.0 Quadraphonic").as_ptr());
        AppendMenuW(ch_menu, if curr_ch == "5.1" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_5_1 as usize, to_wide("5.1 Surround (6 Channels)").as_ptr());
        AppendMenuW(ch_menu, if curr_ch == "6.1" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_6_1 as usize, to_wide("6.1 Surround").as_ptr());
        AppendMenuW(ch_menu, if curr_ch == "7.1" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_7_1 as usize, to_wide("7.1 Surround (8 Channels)").as_ptr());
        AppendMenuW(ch_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(ch_menu, if curr_ch == "surround" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_DOLBY_PL2 as usize, to_wide("Dolby Surround / Pro Logic II").as_ptr());
        AppendMenuW(ch_menu, if curr_ch == "bs2b" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_VIRTUAL_HEADPHONE as usize, to_wide("Virtual Headphone (BS2B)").as_ptr());
        AppendMenuW(ch_menu, if curr_ch == "sofalizer" { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_CH_VIRTUAL_SURROUND as usize, to_wide("Virtual 3D Surround (HRTF Sofalizer)").as_ptr());

        AppendMenuW(aud_menu, MF_POPUP, ch_menu as usize, to_wide("Audio Channels / Speakers").as_ptr());

        let dev_menu = CreatePopupMenu();
        let curr_dev = &config.audio_device;
        let is_auto_dev = curr_dev == "auto" || curr_dev.is_empty();
        AppendMenuW(dev_menu, if is_auto_dev { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_AUDIO_DEV_AUTO as usize, to_wide("Auto (System Default)").as_ptr());
        AppendMenuW(dev_menu, MF_SEPARATOR, 0, std::ptr::null());
        for (i, dev) in stats.audio_device_list.iter().take(30).enumerate() {
            let is_sel = dev.name == *curr_dev;
            let label = if dev.description.is_empty() {
                dev.name.clone()
            } else {
                dev.description.clone()
            };
            AppendMenuW(dev_menu, if is_sel { MF_STRING | MF_CHECKED } else { MF_STRING }, (CMD_AUDIO_DEV_BASE + i as u32) as usize, to_wide(&label).as_ptr());
        }
        AppendMenuW(aud_menu, MF_POPUP, dev_menu as usize, to_wide("Audio Output Device").as_ptr());

        let eq_menu = CreatePopupMenu();
        for (i, preset) in VORTEX_EQ_PRESETS.iter().enumerate() {
            let flags = if config.eq_preset == preset.name { MF_STRING | MF_CHECKED } else { MF_STRING };
            AppendMenuW(eq_menu, flags, (CMD_AUDIO_EQ_BASE + i as u32) as usize, to_wide(preset.name).as_ptr());
        }
        AppendMenuW(aud_menu, MF_POPUP, eq_menu as usize, to_wide("18-Band Equalizer Presets").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_AUDIO_PEQ as usize, to_wide("Parametric Equalizer (PEQ)\tF9").as_ptr());
        AppendMenuW(aud_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(aud_menu, if config.audio_normalize { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_AUDIO_NORMALIZE as usize, to_wide("Loudness Normalization (dynaudnorm)").as_ptr());
        AppendMenuW(aud_menu, if config.vocal_remover { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_VOCAL_REMOVER as usize, to_wide("Karaoke Center Vocal Remover (Stereo)").as_ptr());
        AppendMenuW(aud_menu, if config.voice_enhance { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_VOICE_ENHANCE as usize, to_wide("Voice & Dialogue Clarity Enhancer").as_ptr());
        AppendMenuW(aud_menu, if config.wasapi_exclusive { MF_STRING | MF_CHECKED } else { MF_STRING }, CMD_AUDIO_WASAPI as usize, to_wide("WASAPI Exclusive Output").as_ptr());
        // 3D Reverb & Passthrough
        let rev_menu = CreatePopupMenu();
        AppendMenuW(rev_menu, MF_STRING, CMD_REVERB_OFF as usize, to_wide("Off (Dry)").as_ptr());
        AppendMenuW(rev_menu, MF_STRING, CMD_REVERB_STUDIO as usize, to_wide("Studio Room").as_ptr());
        AppendMenuW(rev_menu, MF_STRING, CMD_REVERB_LIVING_ROOM as usize, to_wide("Living Room").as_ptr());
        AppendMenuW(rev_menu, MF_STRING, CMD_REVERB_CONCERT_HALL as usize, to_wide("Concert Hall").as_ptr());
        AppendMenuW(rev_menu, MF_STRING, CMD_REVERB_ARENA as usize, to_wide("Stadium Arena").as_ptr());
        AppendMenuW(aud_menu, MF_POPUP, rev_menu as usize, to_wide("3D Acoustic Space Reverb").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_AUDIO_PASSTHROUGH as usize, to_wide("HDMI / SPDIF Bitstream Passthrough").as_ptr());

        AppendMenuW(aud_menu, MF_STRING, CMD_VST_WINAMP as usize, to_wide("VST2 / VST3 & Winamp DSP Host...\tCtrl+Alt+V").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_BINAURAL_CROSSFEED as usize, to_wide("Bauer BS2B Headphone Crossfeed & HRTF...\tCtrl+Alt+H").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_AUDIO_COMPRESSOR as usize, to_wide("Dynamic Range Compressor & Night Mode...\tCtrl+Shift+N").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_AI_AUDIO as usize, to_wide("AI Neural Voice Isolation & RNNoise...\tCtrl+Alt+6").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_LOUDNESS_RADAR as usize, to_wide("EBU R128 & LUFS Loudness Radar...\tCtrl+Alt+7").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_PITCH_FORMANT as usize, to_wide("Musical Pitch & Formant Shifter...\tCtrl+Alt+P").as_ptr());
        AppendMenuW(aud_menu, MF_SEPARATOR, 0, std::ptr::null());




        AppendMenuW(aud_menu, MF_STRING, CMD_AUDIO_MUTE as usize, to_wide("Mute / Unmute\tM").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_AUDIO_DELAY_DOWN as usize, to_wide("Audio Sync -50ms\tShift+D").as_ptr());
        AppendMenuW(aud_menu, MF_STRING, CMD_AUDIO_DELAY_UP as usize, to_wide("Audio Sync +50ms\tShift+F").as_ptr());

        // 6. FILTERS SUBMENU
        let filter_menu = CreatePopupMenu();
        AppendMenuW(filter_menu, MF_STRING, CMD_CONTROL_PANEL as usize, to_wide("Control Center (Filters & EQ)\tF7").as_ptr());
        AppendMenuW(filter_menu, MF_STRING, CMD_AUDIO_PEQ as usize, to_wide("Parametric EQ & AutoEQ\tF9").as_ptr());

        // 7. SKINS SUBMENU
        let skins_menu = CreatePopupMenu();
        for (i, mode) in ThemeMode::ALL.iter().enumerate() {
            let flags = if config.theme_mode == *mode { MF_STRING | MF_CHECKED } else { MF_STRING };
            AppendMenuW(skins_menu, flags, (CMD_SKIN_BASE + i as u32) as usize, to_wide(mode.display_name()).as_ptr());
        }

        // 8. MISC SUBMENU
        let misc_menu = CreatePopupMenu();
        AppendMenuW(misc_menu, MF_STRING, CMD_DETACHED_WINDOWS as usize, to_wide("Multi-Monitor Detached Windows...\tCtrl+Alt+D").as_ptr());
        AppendMenuW(misc_menu, MF_STRING, CMD_DISCORD_RPC as usize, to_wide("Discord Rich Presence (RPC)...\tCtrl+Alt+R").as_ptr());
        AppendMenuW(misc_menu, MF_STRING, CMD_WEB_REMOTE as usize, to_wide("Mobile Wi-Fi Web Remote...\tCtrl+Alt+W").as_ptr());
        AppendMenuW(misc_menu, MF_STRING, CMD_SCROBBLER as usize, to_wide("Trakt.tv & AniList Scrobbler...\tCtrl+Alt+S").as_ptr());
        AppendMenuW(misc_menu, MF_STRING, CMD_AMBILIGHT as usize, to_wide("Ambilight & Razer Chroma Lighting...\tCtrl+Alt+L").as_ptr());
        AppendMenuW(misc_menu, MF_STRING, CMD_CAST_RENDERER as usize, to_wide("Cast to Chromecast / AirPlay...\tCtrl+Alt+C").as_ptr());
        AppendMenuW(misc_menu, MF_STRING, CMD_BOOKMARKS as usize, to_wide("Bookmark Manager\tCtrl+Shift+B").as_ptr());
        AppendMenuW(misc_menu, MF_STRING, CMD_BOSS_KEY as usize, to_wide("Boss Key & Privacy Disguise...\tCtrl+Alt+K").as_ptr());

        AppendMenuW(misc_menu, MF_STRING, CMD_ADD_BOOKMARK as usize, to_wide("Add Bookmark\tP").as_ptr());

        AppendMenuW(misc_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(misc_menu, MF_STRING, CMD_REGISTER_ASSOC as usize, to_wide("Register Windows File Associations").as_ptr());




        // Root Section 2 (Matching Vortex screenshot order exactly)
        AppendMenuW(root_menu, MF_POPUP, play_menu as usize, to_wide("Playback").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, sub_menu as usize, to_wide("Subtitles").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, vid_menu as usize, to_wide("Video").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, aud_menu as usize, to_wide("Audio").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, filter_menu as usize, to_wide("Filters").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, skins_menu as usize, to_wide("Skins").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, misc_menu as usize, to_wide("Misc").as_ptr());
        AppendMenuW(root_menu, MF_SEPARATOR, 0, std::ptr::null());

        // Root Section 3 (Frame Size, AR, Window Size, Fullscreen)
        let frame_size_menu = CreatePopupMenu();
        AppendMenuW(frame_size_menu, MF_STRING, CMD_FRAME_SIZE_05 as usize, to_wide("0.5× Size\tNumPad 1").as_ptr());
        AppendMenuW(frame_size_menu, MF_STRING, CMD_FRAME_SIZE_10 as usize, to_wide("1.0× Normal Size\tNumPad 2").as_ptr());
        AppendMenuW(frame_size_menu, MF_STRING, CMD_FRAME_SIZE_15 as usize, to_wide("1.5× Size\tNumPad 3").as_ptr());
        AppendMenuW(frame_size_menu, MF_STRING, CMD_FRAME_SIZE_20 as usize, to_wide("2.0× Double Size\tNumPad 4").as_ptr());
        AppendMenuW(frame_size_menu, MF_STRING, CMD_FRAME_SIZE_FIT as usize, to_wide("Fit to Screen\tNumPad 5").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, frame_size_menu as usize, to_wide("Frame Size").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, ar_menu as usize, to_wide("Aspect Ratio").as_ptr());

        let win_size_menu = CreatePopupMenu();
        AppendMenuW(win_size_menu, MF_STRING, CMD_WIN_DEFAULT as usize, to_wide("Default Size").as_ptr());
        AppendMenuW(win_size_menu, MF_STRING, CMD_WIN_MAXIMIZE as usize, to_wide("Maximize Window").as_ptr());
        AppendMenuW(win_size_menu, MF_STRING, CMD_WIN_MINIMIZE as usize, to_wide("Minimize Window").as_ptr());
        AppendMenuW(root_menu, MF_POPUP, win_size_menu as usize, to_wide("Window Size").as_ptr());

        AppendMenuW(root_menu, MF_STRING, CMD_FULLSCREEN_KEEP as usize, to_wide("Fullscreen\tEnter").as_ptr());
        AppendMenuW(root_menu, MF_STRING, CMD_PIP as usize, to_wide("Picture-in-Picture (Floating Window)\tAlt+P").as_ptr());
        AppendMenuW(root_menu, MF_SEPARATOR, 0, std::ptr::null());

        // Root Section 4 (Preferences, Playlist, Control Panel, Playback Info, About, Exit)
        AppendMenuW(root_menu, MF_STRING, CMD_PREFERENCES as usize, to_wide("Preferences...\tF5").as_ptr());
        AppendMenuW(root_menu, MF_STRING, CMD_PLAYLIST as usize, to_wide("Playlist...\tF6").as_ptr());
        AppendMenuW(root_menu, MF_STRING, CMD_CONTROL_PANEL as usize, to_wide("Control Panel...\tF7").as_ptr());
        AppendMenuW(root_menu, MF_STRING, CMD_MEDIA_INFO as usize, to_wide("Playback/System Info...\tCtrl+F1").as_ptr());
        AppendMenuW(root_menu, MF_STRING, CMD_ABOUT as usize, to_wide("About...\tF1").as_ptr());
        AppendMenuW(root_menu, MF_SEPARATOR, 0, std::ptr::null());
        AppendMenuW(root_menu, MF_STRING, CMD_EXIT as usize, to_wide("Exit\tAlt+F4").as_ptr());

        // Determine coordinates in screen space
        let (pos_x, pos_y) = if let Some((cx, cy)) = custom_pos {
            if hwnd != 0 {
                let mut pt = POINT { x: cx, y: cy };
                ClientToScreen(hwnd as _, &mut pt);
                (pt.x, pt.y)
            } else {
                (cx, cy)
            }
        } else {
            let mut pt: POINT = std::mem::zeroed();
            GetCursorPos(&mut pt);
            (pt.x, pt.y)
        };

        if hwnd != 0 {
            SetForegroundWindow(hwnd as _);
        }

        let cmd = TrackPopupMenuEx(
            root_menu,
            TPM_RETURNCMD | TPM_LEFTALIGN | TPM_TOPALIGN | TPM_RIGHTBUTTON,
            pos_x,
            pos_y,
            hwnd as _,
            std::ptr::null(),
        );

        if hwnd != 0 {
            PostMessageW(hwnd as _, WM_NULL, 0, 0);
        }

        DestroyMenu(root_menu);
        cmd as u32
    }
}
