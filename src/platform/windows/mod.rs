pub mod global_hotkeys;
pub mod smtc;
pub mod taskbar;

pub use global_hotkeys::{GlobalHotkeyEvent, GlobalHotkeyManager};
pub use smtc::{SmtcEvent, WindowsSmtcAdapter};
pub use taskbar::{
    apply_border_suppression, apply_fullscreen_border_suppression, attach_subclass, build_button_icons,
    install_startup_cbt_hook, is_native_fullscreen, is_taskbar_autohide, mark_fullscreen_window,
    set_native_fullscreen_state, uninstall_startup_cbt_hook, TaskbarProgressState, WindowsTaskbarAdapter,
};
