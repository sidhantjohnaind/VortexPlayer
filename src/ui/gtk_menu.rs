#![allow(dead_code)]

#[cfg(target_os = "linux")]
use crate::config::{AppConfig, ThemeMode};
#[cfg(target_os = "linux")]
use crate::engine::audio_dsp::POT_EQ_PRESETS;
#[cfg(target_os = "linux")]
use crate::engine::MediaStats;
use crate::ui::win32_menu::*;
#[cfg(target_os = "linux")]
use std::ffi::{c_char, c_void, CString};
#[cfg(target_os = "linux")]
use std::sync::atomic::{AtomicU32, Ordering};
#[cfg(target_os = "linux")]
use std::sync::Mutex;

#[cfg(target_os = "linux")]
static SELECTED_COMMAND: AtomicU32 = AtomicU32::new(0);

#[cfg(target_os = "linux")]
struct GtkMenuApi {
    gtk_init_check: unsafe extern "C" fn(*mut i32, *mut *mut *mut c_char) -> i32,
    gtk_menu_new: unsafe extern "C" fn() -> *mut c_void,
    gtk_menu_item_new_with_label: unsafe extern "C" fn(*const c_char) -> *mut c_void,
    gtk_check_menu_item_new_with_label: unsafe extern "C" fn(*const c_char) -> *mut c_void,
    gtk_check_menu_item_set_active: unsafe extern "C" fn(*mut c_void, i32),
    gtk_separator_menu_item_new: unsafe extern "C" fn() -> *mut c_void,
    gtk_menu_item_set_submenu: unsafe extern "C" fn(*mut c_void, *mut c_void),
    gtk_menu_shell_append: unsafe extern "C" fn(*mut c_void, *mut c_void),
    gtk_widget_show_all: unsafe extern "C" fn(*mut c_void),
    gtk_menu_popup: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, u32, u32),
    gtk_main: unsafe extern "C" fn(),
    gtk_main_quit: unsafe extern "C" fn(),
    gtk_widget_destroy: unsafe extern "C" fn(*mut c_void),
    g_signal_connect_data: unsafe extern "C" fn(*mut c_void, *const c_char, unsafe extern "C" fn(*mut c_void, *mut c_void), *mut c_void, *mut c_void, i32) -> u64,
}

#[cfg(target_os = "linux")]
unsafe impl Send for GtkMenuApi {}
#[cfg(target_os = "linux")]
unsafe impl Sync for GtkMenuApi {}

#[cfg(target_os = "linux")]
static GTK_API: std::sync::OnceLock<Option<GtkMenuApi>> = std::sync::OnceLock::new();

#[cfg(target_os = "linux")]
unsafe extern "C" fn on_item_activate(_widget: *mut c_void, data: *mut c_void) {
    let cmd = data as usize as u32;
    SELECTED_COMMAND.store(cmd, Ordering::SeqCst);
    if let Some(api) = load_gtk_api() {
        unsafe {
            (api.gtk_main_quit)();
        }
    }
}

#[cfg(target_os = "linux")]
unsafe extern "C" fn on_menu_deactivate(_widget: *mut c_void, _data: *mut c_void) {
    if let Some(api) = load_gtk_api() {
        unsafe {
            (api.gtk_main_quit)();
        }
    }
}

#[cfg(target_os = "linux")]
fn load_gtk_api() -> Option<&'static GtkMenuApi> {
    GTK_API.get_or_init(|| {
        let gtk_libs = ["libgtk-3.so.0", "libgtk-3.so", "/usr/lib/x86_64-linux-gnu/libgtk-3.so.0", "/lib/x86_64-linux-gnu/libgtk-3.so.0"];
        let gobject_libs = ["libgobject-2.0.so.0", "libgobject-2.0.so", "/usr/lib/x86_64-linux-gnu/libgobject-2.0.so.0", "/lib/x86_64-linux-gnu/libgobject-2.0.so.0"];

        let mut lib_gtk = None;
        for path in gtk_libs {
            if let Ok(lib) = unsafe { libloading::Library::new(path) } {
                lib_gtk = Some(Box::leak(Box::new(lib)));
                break;
            }
        }
        let mut lib_gobject = None;
        for path in gobject_libs {
            if let Ok(lib) = unsafe { libloading::Library::new(path) } {
                lib_gobject = Some(Box::leak(Box::new(lib)));
                break;
            }
        }

        if let (Some(gtk), Some(gobj)) = (lib_gtk, lib_gobject) {
            unsafe {
                if let (
                    Ok(init_check),
                    Ok(menu_new),
                    Ok(item_new),
                    Ok(check_item_new),
                    Ok(check_set_act),
                    Ok(sep_new),
                    Ok(set_submenu),
                    Ok(shell_append),
                    Ok(show_all),
                    Ok(popup),
                    Ok(main),
                    Ok(main_quit),
                    Ok(destroy),
                    Ok(signal_connect),
                ) = (
                    gtk.get::<unsafe extern "C" fn(*mut i32, *mut *mut *mut c_char) -> i32>(b"gtk_init_check\0"),
                    gtk.get::<unsafe extern "C" fn() -> *mut c_void>(b"gtk_menu_new\0"),
                    gtk.get::<unsafe extern "C" fn(*const c_char) -> *mut c_void>(b"gtk_menu_item_new_with_label\0"),
                    gtk.get::<unsafe extern "C" fn(*const c_char) -> *mut c_void>(b"gtk_check_menu_item_new_with_label\0"),
                    gtk.get::<unsafe extern "C" fn(*mut c_void, i32)>(b"gtk_check_menu_item_set_active\0"),
                    gtk.get::<unsafe extern "C" fn() -> *mut c_void>(b"gtk_separator_menu_item_new\0"),
                    gtk.get::<unsafe extern "C" fn(*mut c_void, *mut c_void)>(b"gtk_menu_item_set_submenu\0"),
                    gtk.get::<unsafe extern "C" fn(*mut c_void, *mut c_void)>(b"gtk_menu_shell_append\0"),
                    gtk.get::<unsafe extern "C" fn(*mut c_void)>(b"gtk_widget_show_all\0"),
                    gtk.get::<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *mut c_void, *mut c_void, u32, u32)>(b"gtk_menu_popup\0"),
                    gtk.get::<unsafe extern "C" fn()>(b"gtk_main\0"),
                    gtk.get::<unsafe extern "C" fn()>(b"gtk_main_quit\0"),
                    gtk.get::<unsafe extern "C" fn(*mut c_void)>(b"gtk_widget_destroy\0"),
                    gobj.get::<unsafe extern "C" fn(*mut c_void, *const c_char, unsafe extern "C" fn(*mut c_void, *mut c_void), *mut c_void, *mut c_void, i32) -> u64>(b"g_signal_connect_data\0"),
                ) {
                    return Some(GtkMenuApi {
                        gtk_init_check: *init_check,
                        gtk_menu_new: *menu_new,
                        gtk_menu_item_new_with_label: *item_new,
                        gtk_check_menu_item_new_with_label: *check_item_new,
                        gtk_check_menu_item_set_active: *check_set_act,
                        gtk_separator_menu_item_new: *sep_new,
                        gtk_menu_item_set_submenu: *set_submenu,
                        gtk_menu_shell_append: *shell_append,
                        gtk_widget_show_all: *show_all,
                        gtk_menu_popup: *popup,
                        gtk_main: *main,
                        gtk_main_quit: *main_quit,
                        gtk_widget_destroy: *destroy,
                        g_signal_connect_data: *signal_connect,
                    });
                }
            }
        }
        None
    }).as_ref()
}

#[cfg(target_os = "linux")]
struct MenuBuilder<'a> {
    api: &'a GtkMenuApi,
}

#[cfg(target_os = "linux")]
impl<'a> MenuBuilder<'a> {
    fn new(api: &'a GtkMenuApi) -> Self {
        Self { api }
    }

    unsafe fn create_menu(&self) -> *mut c_void {
        unsafe { (self.api.gtk_menu_new)() }
    }

    unsafe fn add_item(&self, parent: *mut c_void, label: &str, cmd: u32) {
        if let Ok(cstr) = CString::new(label) {
            unsafe {
                let item = (self.api.gtk_menu_item_new_with_label)(cstr.as_ptr());
                if cmd > 0 {
                    let sig = CString::new("activate").unwrap();
                    (self.api.g_signal_connect_data)(item, sig.as_ptr(), on_item_activate, cmd as usize as *mut c_void, std::ptr::null_mut(), 0);
                }
                (self.api.gtk_menu_shell_append)(parent, item);
            }
        }
    }

    unsafe fn add_check_item(&self, parent: *mut c_void, label: &str, is_checked: bool, cmd: u32) {
        if let Ok(cstr) = CString::new(label) {
            unsafe {
                let item = (self.api.gtk_check_menu_item_new_with_label)(cstr.as_ptr());
                (self.api.gtk_check_menu_item_set_active)(item, if is_checked { 1 } else { 0 });
                if cmd > 0 {
                    let sig = CString::new("activate").unwrap();
                    (self.api.g_signal_connect_data)(item, sig.as_ptr(), on_item_activate, cmd as usize as *mut c_void, std::ptr::null_mut(), 0);
                }
                (self.api.gtk_menu_shell_append)(parent, item);
            }
        }
    }

    unsafe fn add_separator(&self, parent: *mut c_void) {
        unsafe {
            let sep = (self.api.gtk_separator_menu_item_new)();
            (self.api.gtk_menu_shell_append)(parent, sep);
        }
    }

    unsafe fn add_submenu(&self, parent: *mut c_void, label: &str, submenu: *mut c_void) {
        if let Ok(cstr) = CString::new(label) {
            unsafe {
                let item = (self.api.gtk_menu_item_new_with_label)(cstr.as_ptr());
                (self.api.gtk_menu_item_set_submenu)(item, submenu);
                (self.api.gtk_menu_shell_append)(parent, item);
            }
        }
    }
}

#[cfg(target_os = "linux")]
pub fn show_native_popup_menu(
    stats: &MediaStats,
    config: &AppConfig,
) -> u32 {
    let api = match load_gtk_api() {
        Some(a) => a,
        None => return 0,
    };

    unsafe {
        (api.gtk_init_check)(std::ptr::null_mut(), std::ptr::null_mut());
        let b = MenuBuilder::new(api);

        let root = b.create_menu();

        // 1. OPEN / FILE SUBMENU
        let file_menu = b.create_menu();
        b.add_item(file_menu, "Open File...\tCtrl+O", CMD_OPEN_FILE);
        b.add_item(file_menu, "Open Folder...\tCtrl+F", CMD_OPEN_FOLDER);
        b.add_item(file_menu, "Open URL / Stream...\tCtrl+U", CMD_OPEN_URL);
        b.add_item(file_menu, "Close Playback\tCtrl+W", CMD_CLOSE_PLAYBACK);
        b.add_separator(file_menu);

        let recent_menu = b.create_menu();
        if config.recent_files.is_empty() {
            b.add_item(recent_menu, "(No Recent Files)", 0);
        } else {
            for (i, path) in config.recent_files.iter().take(20).enumerate() {
                let name = std::path::Path::new(path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.clone());
                let label = format!("{}. {}", i + 1, name);
                b.add_item(recent_menu, &label, CMD_RECENT_BASE + i as u32);
            }
        }
        b.add_submenu(file_menu, "Recent Files", recent_menu);

        let fav_menu = b.create_menu();
        b.add_item(fav_menu, "Add Current File to Favorites", CMD_FAVORITES_ADD_ITEM);
        b.add_item(fav_menu, "Add Current Folder to Favorites", CMD_FAVORITES_ADD_FOLDER);
        b.add_item(fav_menu, "Edit Favorites...", CMD_FAVORITES_EDIT);
        b.add_submenu(file_menu, "Favorites", fav_menu);

        b.add_submenu(root, "Open", file_menu);

        // 2. PLAYBACK SUBMENU
        let play_menu = b.create_menu();
        let play_label = if stats.is_paused { "Play\tSpace" } else { "Pause\tSpace" };
        b.add_item(play_menu, play_label, CMD_PLAY_PAUSE);
        b.add_item(play_menu, "Stop\tCtrl+S", CMD_STOP);
        b.add_separator(play_menu);
        b.add_item(play_menu, "Previous File\tPageUp", CMD_PREV_FILE);
        b.add_item(play_menu, "Next File\tPageDown", CMD_NEXT_FILE);
        b.add_separator(play_menu);
        b.add_item(play_menu, "Previous Chapter\tShift+H", CMD_PREV_CHAPTER);
        b.add_item(play_menu, "Next Chapter\tH", CMD_NEXT_CHAPTER);
        b.add_separator(play_menu);
        b.add_item(play_menu, "Jump Forward 5s\tRight", CMD_SEEK_FWD_SHORT);
        b.add_item(play_menu, "Jump Backward 5s\tLeft", CMD_SEEK_BACK_SHORT);
        b.add_item(play_menu, "Jump Forward 30s\tCtrl+Right", CMD_SEEK_FWD_MED);
        b.add_item(play_menu, "Jump Backward 30s\tCtrl+Left", CMD_SEEK_BACK_MED);
        b.add_separator(play_menu);
        b.add_item(play_menu, "Frame Step Forward\tF", CMD_FRAME_STEP_FWD);
        b.add_item(play_menu, "Frame Step Backward\tD", CMD_FRAME_STEP_BACK);
        b.add_separator(play_menu);

        let speed_menu = b.create_menu();
        b.add_item(speed_menu, "Normal Speed (1.0×)\tZ", CMD_SPEED_NORMAL);
        b.add_item(speed_menu, "Slower (-0.1×)\tX", CMD_SPEED_DOWN);
        b.add_item(speed_menu, "Faster (+0.1×)\tC", CMD_SPEED_UP);
        b.add_submenu(play_menu, "Playback Speed", speed_menu);

        let ab_menu = b.create_menu();
        b.add_item(ab_menu, "Set Point A\t[", CMD_AB_LOOP_A);
        b.add_item(ab_menu, "Set Point B\t]", CMD_AB_LOOP_B);
        b.add_item(ab_menu, "Clear A-B Repeat\t\\", CMD_AB_LOOP_CLEAR);
        b.add_submenu(play_menu, "A-B Repeat", ab_menu);

        let chap_menu = b.create_menu();
        b.add_item(chap_menu, "Previous Chapter\tShift+H", CMD_PREV_CHAPTER);
        b.add_item(chap_menu, "Next Chapter\tH", CMD_NEXT_CHAPTER);
        b.add_item(chap_menu, "Chapter Timeline Manager...\tCtrl+Alt+J", CMD_CHAPTER_MARKER);
        b.add_separator(chap_menu);
        if stats.chapters.is_empty() {
            b.add_item(chap_menu, "(No chapters)", 0);
        } else {
            for (i, ch) in stats.chapters.iter().take(50).enumerate() {
                let time_str = crate::bookmark::format_time(ch.time_pos);
                let label = if ch.title.trim().is_empty() {
                    format!("Chapter {} ({})", i + 1, time_str)
                } else {
                    format!("{} ({})", ch.title.trim(), time_str)
                };
                b.add_item(chap_menu, &label, CMD_CHAPTER_BASE + i as u32);
            }
        }
        b.add_submenu(play_menu, "Chapters", chap_menu);

        b.add_submenu(root, "Playback", play_menu);

        // 3. SUBTITLES SUBMENU
        let sub_menu = b.create_menu();
        b.add_check_item(sub_menu, "Show Subtitles\tAlt+H", stats.subtitles_visible, CMD_SUBTITLE_TOGGLE);
        b.add_item(sub_menu, "Load Subtitle File...\tAlt+O", CMD_SUBTITLE_LOAD);
        b.add_separator(sub_menu);

        let sub_tracks_menu = b.create_menu();
        b.add_check_item(sub_tracks_menu, "None (Disabled)", stats.selected_subtitle_track <= 0, CMD_SUBTITLE_TRACK_BASE + 999);
        for (i, track) in stats.subtitle_tracks.iter().take(20).enumerate() {
            let is_sel = track.id == stats.selected_subtitle_track;
            let label = if track.title.is_empty() {
                format!("Track {}: {} [{}]", track.id, track.lang, track.codec)
            } else {
                format!("Track {}: {}", track.id, track.title)
            };
            b.add_check_item(sub_tracks_menu, &label, is_sel, CMD_SUBTITLE_TRACK_BASE + i as u32);
        }
        b.add_submenu(sub_menu, "Select Subtitle Track", sub_tracks_menu);
        b.add_separator(sub_menu);
        b.add_item(sub_menu, "Subtitle Sync -0.5s\t<", CMD_SUBTITLE_DELAY_DOWN);
        b.add_item(sub_menu, "Subtitle Sync +0.5s\t>", CMD_SUBTITLE_DELAY_UP);
        b.add_submenu(root, "Subtitles", sub_menu);

        // 4. VIDEO SUBMENU
        let vid_menu = b.create_menu();
        let ar_menu = b.create_menu();
        let curr_ar = &config.aspect_ratio;
        b.add_check_item(ar_menu, "Auto (Original AR)", curr_ar == "auto" || curr_ar.is_empty(), CMD_AR_AUTO);
        b.add_check_item(ar_menu, "16:9 Widescreen", curr_ar == "16:9", CMD_AR_16_9);
        b.add_check_item(ar_menu, "4:3 Standard", curr_ar == "4:3", CMD_AR_4_3);
        b.add_check_item(ar_menu, "1.85:1 Academy", curr_ar == "1.85:1", CMD_AR_185);
        b.add_check_item(ar_menu, "2.35:1 Anamorphic", curr_ar == "2.35:1", CMD_AR_235);
        b.add_check_item(ar_menu, "Fit to Window", curr_ar == "none", CMD_AR_FILL);
        b.add_submenu(vid_menu, "Aspect Ratio", ar_menu);

        let hw_menu = b.create_menu();
        let curr_hw = &config.hardware_decoding;
        b.add_check_item(hw_menu, "Auto Hardware Acceleration", curr_hw == "auto" || curr_hw == "auto-safe", CMD_HWDEC_AUTO);
        b.add_check_item(hw_menu, "VA-API (Intel / AMD Linux)", curr_hw == "vaapi", CMD_HWDEC_D3D11VA);
        b.add_check_item(hw_menu, "NVDEC (NVIDIA Linux)", curr_hw == "nvdec", CMD_HWDEC_NVDEC);
        b.add_check_item(hw_menu, "Disable (Software Decoding)", curr_hw == "no" || curr_hw.is_empty(), CMD_HWDEC_NO);
        b.add_submenu(vid_menu, "Hardware Decoding", hw_menu);

        let rot_menu = b.create_menu();
        b.add_check_item(rot_menu, "0° (Normal)", config.video_rotation == 0, CMD_ROT_0);
        b.add_check_item(rot_menu, "90° Clockwise", config.video_rotation == 90, CMD_ROT_90);
        b.add_check_item(rot_menu, "180° Inverted", config.video_rotation == 180, CMD_ROT_180);
        b.add_check_item(rot_menu, "270° Counter-Clockwise", config.video_rotation == 270, CMD_ROT_270);
        b.add_submenu(vid_menu, "Rotation", rot_menu);

        b.add_item(vid_menu, "Capture Video Screenshot\tCtrl+E", CMD_SNAPSHOT);
        b.add_item(vid_menu, "Reset Video Colors\tQ", CMD_RESET_VIDEO);
        b.add_submenu(root, "Video", vid_menu);

        // 5. AUDIO SUBMENU
        let aud_menu = b.create_menu();
        b.add_item(aud_menu, "Cycle Audio Stream\tAlt+A", CMD_AUDIO_CYCLE);

        let aud_tracks_menu = b.create_menu();
        if stats.audio_tracks.is_empty() {
            b.add_item(aud_tracks_menu, "(No Audio Tracks)", 0);
        } else {
            for (i, track) in stats.audio_tracks.iter().take(20).enumerate() {
                let is_sel = track.id == stats.selected_audio_track;
                let label = if track.title.is_empty() {
                    format!("Track {}: {} [{} ch]", track.id, track.lang, track.channels)
                } else {
                    format!("Track {}: {}", track.id, track.title)
                };
                b.add_check_item(aud_tracks_menu, &label, is_sel, CMD_AUDIO_TRACK_BASE + i as u32);
            }
        }
        b.add_submenu(aud_menu, "Select Audio Stream", aud_tracks_menu);

        let ch_menu = b.create_menu();
        let curr_ch = &config.audio_channels;
        let is_same = curr_ch == "auto" || curr_ch == "auto-safe" || curr_ch.is_empty();
        b.add_check_item(ch_menu, "Same as input", is_same, CMD_CH_SAME_AS_INPUT);
        b.add_separator(ch_menu);
        b.add_check_item(ch_menu, "1.0 mono (Single Channel)", curr_ch == "mono" || curr_ch == "1.0", CMD_CH_1_0);
        b.add_check_item(ch_menu, "2.0 stereo (Default)", curr_ch == "stereo" || curr_ch == "2.0", CMD_CH_2_0);
        b.add_check_item(ch_menu, "2.1 stereo + LFE", curr_ch == "2.1", CMD_CH_2_1);
        b.add_separator(ch_menu);
        b.add_check_item(ch_menu, "3.0 (Front L, R, C)", curr_ch == "3.0", CMD_CH_3_0);
        b.add_check_item(ch_menu, "4.0 Quadraphonic", curr_ch == "4.0", CMD_CH_4_0);
        b.add_check_item(ch_menu, "5.1 Surround (6 Channels)", curr_ch == "5.1", CMD_CH_5_1);
        b.add_check_item(ch_menu, "6.1 Surround", curr_ch == "6.1", CMD_CH_6_1);
        b.add_check_item(ch_menu, "7.1 Surround (8 Channels)", curr_ch == "7.1", CMD_CH_7_1);
        b.add_separator(ch_menu);
        b.add_check_item(ch_menu, "Dolby Surround / Pro Logic II", curr_ch == "surround", CMD_CH_DOLBY_PL2);
        b.add_check_item(ch_menu, "Virtual Headphone (BS2B)", curr_ch == "bs2b", CMD_CH_VIRTUAL_HEADPHONE);
        b.add_check_item(ch_menu, "Virtual 3D Surround (Sofalizer)", curr_ch == "sofalizer", CMD_CH_VIRTUAL_SURROUND);
        b.add_submenu(aud_menu, "Audio Channels / Speakers", ch_menu);

        let dev_menu = b.create_menu();
        let curr_dev = &config.audio_device;
        let is_auto_dev = curr_dev == "auto" || curr_dev.is_empty();
        b.add_check_item(dev_menu, "Auto (System Default)", is_auto_dev, CMD_AUDIO_DEV_AUTO);
        b.add_separator(dev_menu);
        let cur_ao = &stats.current_ao;
        let mut seen_names = std::collections::HashSet::new();
        for (i, dev) in stats.audio_device_list.iter().take(40).enumerate() {
            if dev.name == "auto" { continue; }
            if !cur_ao.is_empty() && dev.name.contains('/') && !dev.name.starts_with(cur_ao) {
                continue;
            }
            let label = if dev.description.is_empty() {
                dev.name.clone()
            } else {
                dev.description.clone()
            };
            if !seen_names.insert(label.clone()) {
                continue;
            }
            let is_sel = dev.name == *curr_dev || curr_dev.ends_with(&dev.name) || dev.name.ends_with(curr_dev);
            b.add_check_item(dev_menu, &label, is_sel, CMD_AUDIO_DEV_BASE + i as u32);
        }
        b.add_submenu(aud_menu, "Audio Output Device", dev_menu);

        let eq_menu = b.create_menu();
        for (i, preset) in POT_EQ_PRESETS.iter().enumerate() {
            b.add_check_item(eq_menu, preset.name, config.eq_preset == preset.name, CMD_AUDIO_EQ_BASE + i as u32);
        }
        b.add_submenu(aud_menu, "18-Band Equalizer Presets", eq_menu);
        b.add_item(aud_menu, "Parametric Equalizer (PEQ)\tF9", CMD_AUDIO_PEQ);
        b.add_separator(aud_menu);
        b.add_check_item(aud_menu, "Loudness Normalization (dynaudnorm)", config.audio_normalize, CMD_AUDIO_NORMALIZE);
        b.add_check_item(aud_menu, "Karaoke Center Vocal Remover", config.vocal_remover, CMD_VOCAL_REMOVER);
        b.add_check_item(aud_menu, "Voice & Dialogue Clarity Enhancer", config.voice_enhance, CMD_VOICE_ENHANCE);

        let rev_menu = b.create_menu();
        b.add_item(rev_menu, "Off (Dry)", CMD_REVERB_OFF);
        b.add_item(rev_menu, "Studio Room", CMD_REVERB_STUDIO);
        b.add_item(rev_menu, "Living Room", CMD_REVERB_LIVING_ROOM);
        b.add_item(rev_menu, "Concert Hall", CMD_REVERB_CONCERT_HALL);
        b.add_item(rev_menu, "Stadium Arena", CMD_REVERB_ARENA);
        b.add_submenu(aud_menu, "3D Acoustic Reverb", rev_menu);
        b.add_separator(aud_menu);
        b.add_item(aud_menu, "Mute / Unmute\tM", CMD_AUDIO_MUTE);
        b.add_item(aud_menu, "Audio Sync -50ms\tShift+D", CMD_AUDIO_DELAY_DOWN);
        b.add_item(aud_menu, "Audio Sync +50ms\tShift+F", CMD_AUDIO_DELAY_UP);
        b.add_submenu(root, "Audio", aud_menu);

        // 6. FILTERS SUBMENU
        let filter_menu = b.create_menu();
        b.add_item(filter_menu, "Control Center (Filters & EQ)\tF7", CMD_CONTROL_PANEL);
        b.add_item(filter_menu, "Parametric EQ & AutoEQ\tF9", CMD_AUDIO_PEQ);
        b.add_submenu(root, "Filters", filter_menu);

        // 7. SKINS SUBMENU
        let skins_menu = b.create_menu();
        for (i, mode) in ThemeMode::ALL.iter().enumerate() {
            b.add_check_item(skins_menu, mode.display_name(), config.theme_mode == *mode, CMD_SKIN_BASE + i as u32);
        }
        b.add_submenu(root, "Skins", skins_menu);

        // 8. MISC SUBMENU
        let misc_menu = b.create_menu();
        b.add_item(misc_menu, "Mobile Wi-Fi Web Remote...\tCtrl+Alt+W", CMD_WEB_REMOTE);
        b.add_item(misc_menu, "Bookmark Manager\tCtrl+Shift+B", CMD_BOOKMARKS);
        b.add_item(misc_menu, "Add Bookmark\tP", CMD_ADD_BOOKMARK);
        b.add_submenu(root, "Misc", misc_menu);

        b.add_separator(root);

        // Root Section 3
        let frame_size_menu = b.create_menu();
        b.add_item(frame_size_menu, "0.5× Size\tNumPad 1", CMD_FRAME_SIZE_05);
        b.add_item(frame_size_menu, "1.0× Normal Size\tNumPad 2", CMD_FRAME_SIZE_10);
        b.add_item(frame_size_menu, "1.5× Size\tNumPad 3", CMD_FRAME_SIZE_15);
        b.add_item(frame_size_menu, "2.0× Double Size\tNumPad 4", CMD_FRAME_SIZE_20);
        b.add_item(frame_size_menu, "Fit to Screen\tNumPad 5", CMD_FRAME_SIZE_FIT);
        b.add_submenu(root, "Frame Size", frame_size_menu);

        b.add_item(root, "Fullscreen\tEnter", CMD_FULLSCREEN_KEEP);
        b.add_item(root, "Picture-in-Picture\tAlt+P", CMD_PIP);
        b.add_separator(root);

        // Root Section 4
        b.add_item(root, "Preferences...\tF5", CMD_PREFERENCES);
        b.add_item(root, "Playlist...\tF6", CMD_PLAYLIST);
        b.add_item(root, "Control Panel...\tF7", CMD_CONTROL_PANEL);
        b.add_item(root, "Playback/System Info...\tCtrl+F1", CMD_MEDIA_INFO);
        b.add_item(root, "About...\tF1", CMD_ABOUT);
        b.add_separator(root);
        b.add_item(root, "Exit\tAlt+F4", CMD_EXIT);

        SELECTED_COMMAND.store(0, Ordering::SeqCst);
        let deact_sig = CString::new("deactivate").unwrap();
        (api.g_signal_connect_data)(root, deact_sig.as_ptr(), on_menu_deactivate, std::ptr::null_mut(), std::ptr::null_mut(), 0);

        (api.gtk_widget_show_all)(root);
        (api.gtk_menu_popup)(root, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), 3, 0);

        (api.gtk_main)();

        let selected = SELECTED_COMMAND.load(Ordering::SeqCst);
        (api.gtk_widget_destroy)(root);
        selected
    }
}

#[cfg(not(target_os = "linux"))]
pub fn show_native_popup_menu(
    _stats: &crate::engine::MediaStats,
    _config: &crate::config::AppConfig,
) -> u32 {
    0
}
