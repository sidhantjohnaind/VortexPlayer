#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::{
    apply_border_suppression, apply_fullscreen_border_suppression, attach_subclass, build_button_icons,
    install_startup_cbt_hook, is_native_fullscreen, is_taskbar_autohide, mark_fullscreen_window,
    set_native_fullscreen_state, uninstall_startup_cbt_hook, GlobalHotkeyEvent, GlobalHotkeyManager,
    SmtcEvent, TaskbarProgressState, WindowsSmtcAdapter, WindowsTaskbarAdapter,
};

#[cfg(not(windows))]
pub mod linux;

#[cfg(not(windows))]
pub use linux::*;

/// Creates a `std::process::Command` that runs completely silently without popping up a console/cmd window on Windows.
pub fn silent_command<S: AsRef<std::ffi::OsStr>>(program: S) -> std::process::Command {
    #[allow(unused_mut)]
    let mut cmd = std::process::Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    cmd
}
