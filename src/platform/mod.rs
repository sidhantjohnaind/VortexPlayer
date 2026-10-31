#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::{
    apply_border_suppression, apply_fullscreen_border_suppression, attach_subclass, build_button_icons,
    get_system_refresh_rate, install_startup_cbt_hook, is_native_fullscreen, is_taskbar_autohide, mark_fullscreen_window,
    set_native_fullscreen_state, set_window_corner_style, uninstall_startup_cbt_hook, GlobalHotkeyEvent, GlobalHotkeyManager,
    SmtcEvent, TaskbarProgressState, WindowsSmtcAdapter, WindowsTaskbarAdapter,
};

#[cfg(not(windows))]
pub mod linux;

#[cfg(not(windows))]
pub use linux::*;

/// Resolves the absolute path to `ffmpeg.exe` if available, checking:
/// 1. Directory of the currently running executable
/// 2. Current working directory and its parent
/// 3. Windows WinGet package directories
/// 4. Fallback to `"ffmpeg"` for standard PATH resolution
pub fn resolve_ffmpeg_path() -> std::path::PathBuf {
    // 1. Next to current running executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("ffmpeg.exe");
            if candidate.is_file() {
                return candidate;
            }
            let candidate_non_win = dir.join("ffmpeg");
            if candidate_non_win.is_file() {
                return candidate_non_win;
            }
        }
    }

    // 2. Current working directory and parent
    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd.join("ffmpeg.exe");
        if candidate.is_file() {
            return candidate;
        }
        let candidate_non_win = cwd.join("ffmpeg");
        if candidate_non_win.is_file() {
            return candidate_non_win;
        }
        if let Some(parent) = cwd.parent() {
            let candidate_parent = parent.join("ffmpeg.exe");
            if candidate_parent.is_file() {
                return candidate_parent;
            }
        }
    }

    // 3. WinGet package directory on Windows
    #[cfg(windows)]
    {
        if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
            let packages_dir = std::path::Path::new(&local_appdata)
                .join("Microsoft")
                .join("WinGet")
                .join("Packages");
            if packages_dir.is_dir() {
                if let Ok(entries) = std::fs::read_dir(&packages_dir) {
                    for entry in entries.flatten() {
                        let folder_name = entry.file_name().to_string_lossy().to_string();
                        if folder_name.to_lowercase().contains("ffmpeg") {
                            // Search depth 2
                            if let Ok(sub1) = std::fs::read_dir(entry.path()) {
                                for s1 in sub1.flatten() {
                                    let bin_exe = s1.path().join("bin").join("ffmpeg.exe");
                                    if bin_exe.is_file() {
                                        return bin_exe;
                                    }
                                    let direct_exe = s1.path().join("ffmpeg.exe");
                                    if direct_exe.is_file() {
                                        return direct_exe;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    std::path::PathBuf::from("ffmpeg")
}

/// Creates a `std::process::Command` that runs completely silently without popping up a console/cmd window on Windows.
pub fn silent_command<S: AsRef<std::ffi::OsStr>>(program: S) -> std::process::Command {
    let prog_str = program.as_ref().to_string_lossy();
    let prog_path = if prog_str.eq_ignore_ascii_case("ffmpeg") || prog_str.eq_ignore_ascii_case("ffmpeg.exe") {
        resolve_ffmpeg_path().into_os_string()
    } else {
        program.as_ref().to_os_string()
    };

    #[allow(unused_mut)]
    let mut cmd = std::process::Command::new(prog_path);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    cmd
}
