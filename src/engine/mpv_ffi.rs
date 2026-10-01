#![allow(dead_code)]

use libloading::{Library, Symbol};
use std::ffi::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::sync::Arc;

pub const MPV_FORMAT_NONE: c_int = 0;
pub const MPV_FORMAT_STRING: c_int = 1;
pub const MPV_FORMAT_OSD_STRING: c_int = 2;
pub const MPV_FORMAT_FLAG: c_int = 3;
pub const MPV_FORMAT_INT64: c_int = 4;
pub const MPV_FORMAT_DOUBLE: c_int = 5;
pub const MPV_FORMAT_NODE: c_int = 6;
pub const MPV_FORMAT_NODE_ARRAY: c_int = 7;
pub const MPV_FORMAT_NODE_MAP: c_int = 8;
pub const MPV_FORMAT_BYTE_ARRAY: c_int = 9;

pub const MPV_EVENT_NONE: c_int = 0;
pub const MPV_EVENT_SHUTDOWN: c_int = 1;
pub const MPV_EVENT_LOG_MESSAGE: c_int = 2;
pub const MPV_EVENT_GET_PROPERTY_REPLY: c_int = 3;
pub const MPV_EVENT_SET_PROPERTY_REPLY: c_int = 4;
pub const MPV_EVENT_COMMAND_REPLY: c_int = 5;
pub const MPV_EVENT_START_FILE: c_int = 6;
pub const MPV_EVENT_END_FILE: c_int = 7;
pub const MPV_EVENT_FILE_LOADED: c_int = 8;
pub const MPV_EVENT_CLIENT_MESSAGE: c_int = 16;
pub const MPV_EVENT_VIDEO_RECONFIG: c_int = 17;
pub const MPV_EVENT_AUDIO_RECONFIG: c_int = 18;
pub const MPV_EVENT_SEEK: c_int = 20;
pub const MPV_EVENT_PLAYBACK_RESTART: c_int = 21;
pub const MPV_EVENT_PROPERTY_CHANGE: c_int = 22;

#[repr(C)]
pub struct MpvEvent {
    pub event_id: c_int,
    pub error: c_int,
    pub reply_userdata: u64,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct MpvEventProperty {
    pub name: *const c_char,
    pub format: c_int,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct MpvEventEndFile {
    pub reason: c_int,
    pub error: c_int,
    pub playlist_entry_id: i64,
    pub playlist_insert_id: i64,
    pub playlist_insert_num_entries: c_int,
}

pub type FnMpvCreate = unsafe extern "C" fn() -> *mut c_void;
pub type FnMpvInitialize = unsafe extern "C" fn(ctx: *mut c_void) -> c_int;
pub type FnMpvDestroy = unsafe extern "C" fn(ctx: *mut c_void);
pub type FnMpvTerminateDestroy = unsafe extern "C" fn(ctx: *mut c_void);
pub type FnMpvCommand = unsafe extern "C" fn(ctx: *mut c_void, args: *mut *const c_char) -> c_int;
pub type FnMpvCommandString = unsafe extern "C" fn(ctx: *mut c_void, args: *const c_char) -> c_int;
pub type FnMpvSetOption = unsafe extern "C" fn(ctx: *mut c_void, name: *const c_char, format: c_int, data: *mut c_void) -> c_int;
pub type FnMpvSetOptionString = unsafe extern "C" fn(ctx: *mut c_void, name: *const c_char, data: *const c_char) -> c_int;
pub type FnMpvGetProperty = unsafe extern "C" fn(ctx: *mut c_void, name: *const c_char, format: c_int, data: *mut c_void) -> c_int;
pub type FnMpvGetPropertyString = unsafe extern "C" fn(ctx: *mut c_void, name: *const c_char) -> *mut c_char;
pub type FnMpvSetProperty = unsafe extern "C" fn(ctx: *mut c_void, name: *const c_char, format: c_int, data: *mut c_void) -> c_int;
pub type FnMpvSetPropertyString = unsafe extern "C" fn(ctx: *mut c_void, name: *const c_char, data: *const c_char) -> c_int;
pub type FnMpvObserveProperty = unsafe extern "C" fn(ctx: *mut c_void, reply_userdata: u64, name: *const c_char, format: c_int) -> c_int;
pub type FnMpvUnobserveProperty = unsafe extern "C" fn(ctx: *mut c_void, registered_reply_userdata: u64) -> c_int;
pub type FnMpvFree = unsafe extern "C" fn(data: *mut c_void);
pub type FnMpvWaitEvent = unsafe extern "C" fn(ctx: *mut c_void, timeout: f64) -> *mut MpvEvent;
pub type FnMpvEventName = unsafe extern "C" fn(event_id: c_int) -> *const c_char;
pub type FnMpvErrorString = unsafe extern "C" fn(error: c_int) -> *const c_char;

#[derive(Clone)]
pub struct MpvFfi {
    _lib: Arc<Library>,
    pub mpv_create: FnMpvCreate,
    pub mpv_initialize: FnMpvInitialize,
    pub mpv_destroy: FnMpvDestroy,
    pub mpv_terminate_destroy: FnMpvTerminateDestroy,
    pub mpv_command: FnMpvCommand,
    pub mpv_command_string: FnMpvCommandString,
    pub mpv_set_option: FnMpvSetOption,
    pub mpv_set_option_string: FnMpvSetOptionString,
    pub mpv_get_property: FnMpvGetProperty,
    pub mpv_get_property_string: FnMpvGetPropertyString,
    pub mpv_set_property: FnMpvSetProperty,
    pub mpv_set_property_string: FnMpvSetPropertyString,
    pub mpv_observe_property: FnMpvObserveProperty,
    pub mpv_unobserve_property: FnMpvUnobserveProperty,
    pub mpv_free: FnMpvFree,
    pub mpv_wait_event: FnMpvWaitEvent,
    pub mpv_event_name: FnMpvEventName,
    pub mpv_error_string: FnMpvErrorString,
}

impl MpvFfi {
    pub fn find_mpv_dll() -> Result<PathBuf, String> {
        #[cfg(target_os = "windows")]
        let search_paths = vec![
            PathBuf::from("libmpv-2.dll"),
            PathBuf::from("mpv-dev/libmpv-2.dll"),
            PathBuf::from("target/debug/libmpv-2.dll"),
            PathBuf::from("target/release/libmpv-2.dll"),
            PathBuf::from("libmpv-1.dll"),
            PathBuf::from("mpv-2.dll"),
            PathBuf::from("mpv-1.dll"),
        ];

        #[cfg(target_os = "linux")]
        let search_paths = vec![
            PathBuf::from("libmpv.so.2"),
            PathBuf::from("libmpv.so.1"),
            PathBuf::from("libmpv.so"),
            PathBuf::from("/lib/x86_64-linux-gnu/libmpv.so.2"),
            PathBuf::from("/lib/x86_64-linux-gnu/libmpv.so.1"),
            PathBuf::from("/lib/x86_64-linux-gnu/libmpv.so"),
            PathBuf::from("/usr/lib/x86_64-linux-gnu/libmpv.so.2"),
            PathBuf::from("/usr/lib/x86_64-linux-gnu/libmpv.so.1"),
            PathBuf::from("/usr/lib/x86_64-linux-gnu/libmpv.so"),
            PathBuf::from("/usr/lib/libmpv.so.2"),
            PathBuf::from("/usr/lib/libmpv.so.1"),
            PathBuf::from("/usr/lib/libmpv.so"),
            PathBuf::from("/lib/libmpv.so.2"),
            PathBuf::from("/lib/libmpv.so"),
            PathBuf::from("/usr/local/lib/libmpv.so"),
        ];

        #[cfg(target_os = "macos")]
        let search_paths = vec![
            PathBuf::from("libmpv.2.dylib"),
            PathBuf::from("libmpv.dylib"),
            PathBuf::from("/opt/homebrew/lib/libmpv.dylib"),
            PathBuf::from("/usr/local/lib/libmpv.dylib"),
        ];

        // Check relative to current exe
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                #[cfg(target_os = "windows")]
                let exe_candidates = [
                    exe_dir.join("libmpv-2.dll"),
                    exe_dir.join("libmpv-1.dll"),
                    exe_dir.join("mpv-dev").join("libmpv-2.dll"),
                ];

                #[cfg(target_os = "linux")]
                let exe_candidates = [
                    exe_dir.join("libmpv.so.2"),
                    exe_dir.join("libmpv.so.1"),
                    exe_dir.join("libmpv.so"),
                ];

                #[cfg(target_os = "macos")]
                let exe_candidates = [
                    exe_dir.join("libmpv.2.dylib"),
                    exe_dir.join("libmpv.dylib"),
                ];

                #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
                let exe_candidates: [PathBuf; 0] = [];

                for candidate in &exe_candidates {
                    if candidate.exists() {
                        return Ok(candidate.clone());
                    }
                }
            }
        }

        // Check relative paths
        for path in &search_paths {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        #[cfg(target_os = "windows")]
        return Ok(PathBuf::from("libmpv-2.dll"));

        #[cfg(target_os = "linux")]
        return Ok(PathBuf::from("libmpv.so.2"));

        #[cfg(target_os = "macos")]
        return Ok(PathBuf::from("libmpv.2.dylib"));

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        Ok(PathBuf::from("libmpv.so"))
    }

    pub fn load() -> Result<Self, String> {
        let dll_path = Self::find_mpv_dll()?;
        let lib = unsafe {
            Library::new(&dll_path)
                .map_err(|e| format!("Failed to load mpv library at {:?}: {}", dll_path, e))?
        };
        let lib = Arc::new(lib);

        unsafe {
            let mpv_create: Symbol<FnMpvCreate> = lib.get(b"mpv_create\0")
                .map_err(|e| format!("Symbol mpv_create not found: {}", e))?;
            let mpv_initialize: Symbol<FnMpvInitialize> = lib.get(b"mpv_initialize\0")
                .map_err(|e| format!("Symbol mpv_initialize not found: {}", e))?;
            let mpv_destroy: Symbol<FnMpvDestroy> = lib.get(b"mpv_destroy\0")
                .map_err(|e| format!("Symbol mpv_destroy not found: {}", e))?;
            let mpv_terminate_destroy: Symbol<FnMpvTerminateDestroy> = lib.get(b"mpv_terminate_destroy\0")
                .map_err(|e| format!("Symbol mpv_terminate_destroy not found: {}", e))?;
            let mpv_command: Symbol<FnMpvCommand> = lib.get(b"mpv_command\0")
                .map_err(|e| format!("Symbol mpv_command not found: {}", e))?;
            let mpv_command_string: Symbol<FnMpvCommandString> = lib.get(b"mpv_command_string\0")
                .map_err(|e| format!("Symbol mpv_command_string not found: {}", e))?;
            let mpv_set_option: Symbol<FnMpvSetOption> = lib.get(b"mpv_set_option\0")
                .map_err(|e| format!("Symbol mpv_set_option not found: {}", e))?;
            let mpv_set_option_string: Symbol<FnMpvSetOptionString> = lib.get(b"mpv_set_option_string\0")
                .map_err(|e| format!("Symbol mpv_set_option_string not found: {}", e))?;
            let mpv_get_property: Symbol<FnMpvGetProperty> = lib.get(b"mpv_get_property\0")
                .map_err(|e| format!("Symbol mpv_get_property not found: {}", e))?;
            let mpv_get_property_string: Symbol<FnMpvGetPropertyString> = lib.get(b"mpv_get_property_string\0")
                .map_err(|e| format!("Symbol mpv_get_property_string not found: {}", e))?;
            let mpv_set_property: Symbol<FnMpvSetProperty> = lib.get(b"mpv_set_property\0")
                .map_err(|e| format!("Symbol mpv_set_property not found: {}", e))?;
            let mpv_set_property_string: Symbol<FnMpvSetPropertyString> = lib.get(b"mpv_set_property_string\0")
                .map_err(|e| format!("Symbol mpv_set_property_string not found: {}", e))?;
            let mpv_observe_property: Symbol<FnMpvObserveProperty> = lib.get(b"mpv_observe_property\0")
                .map_err(|e| format!("Symbol mpv_observe_property not found: {}", e))?;
            let mpv_unobserve_property: Symbol<FnMpvUnobserveProperty> = lib.get(b"mpv_unobserve_property\0")
                .map_err(|e| format!("Symbol mpv_unobserve_property not found: {}", e))?;
            let mpv_free: Symbol<FnMpvFree> = lib.get(b"mpv_free\0")
                .map_err(|e| format!("Symbol mpv_free not found: {}", e))?;
            let mpv_wait_event: Symbol<FnMpvWaitEvent> = lib.get(b"mpv_wait_event\0")
                .map_err(|e| format!("Symbol mpv_wait_event not found: {}", e))?;
            let mpv_event_name: Symbol<FnMpvEventName> = lib.get(b"mpv_event_name\0")
                .map_err(|e| format!("Symbol mpv_event_name not found: {}", e))?;
            let mpv_error_string: Symbol<FnMpvErrorString> = lib.get(b"mpv_error_string\0")
                .map_err(|e| format!("Symbol mpv_error_string not found: {}", e))?;

            Ok(Self {
                mpv_create: *mpv_create,
                mpv_initialize: *mpv_initialize,
                mpv_destroy: *mpv_destroy,
                mpv_terminate_destroy: *mpv_terminate_destroy,
                mpv_command: *mpv_command,
                mpv_command_string: *mpv_command_string,
                mpv_set_option: *mpv_set_option,
                mpv_set_option_string: *mpv_set_option_string,
                mpv_get_property: *mpv_get_property,
                mpv_get_property_string: *mpv_get_property_string,
                mpv_set_property: *mpv_set_property,
                mpv_set_property_string: *mpv_set_property_string,
                mpv_observe_property: *mpv_observe_property,
                mpv_unobserve_property: *mpv_unobserve_property,
                mpv_free: *mpv_free,
                mpv_wait_event: *mpv_wait_event,
                mpv_event_name: *mpv_event_name,
                mpv_error_string: *mpv_error_string,
                _lib: lib,
            })
        }
    }
}
