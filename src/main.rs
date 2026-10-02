#![windows_subsystem = "windows"]

mod app;
mod bookmark;
mod capture;
pub mod commands;
mod config;
mod engine;
pub mod library;
pub mod network;
pub mod platform;
pub mod plugin;
mod playlist;
mod subtitles;
mod ui;

use app::PotApp;
use config::AppConfig;
use eframe::egui::{IconData, ViewportBuilder};
use std::env;
use std::path::PathBuf;

// Instruct NVIDIA and AMD drivers that this is a 2D desktop utility (not a 3D game)
#[unsafe(no_mangle)]
pub static mut NvOptimusEnablement: u32 = 0x00000000;

#[unsafe(no_mangle)]
pub static mut AmdPowerXpressRequestHighPerformance: i32 = 0;

// RISC-V 64 Linux glibc CRT compatibility stubs for cross-compilation
#[cfg(all(target_os = "linux", target_arch = "riscv64"))]
#[unsafe(no_mangle)]
pub extern "C" fn _init() {}

#[cfg(all(target_os = "linux", target_arch = "riscv64"))]
#[unsafe(no_mangle)]
pub extern "C" fn _fini() {}

fn main() {
    log_step("1. main entered");

    // Set panic hook to log any unhandled crashes to disk & display an error dialog box
    std::panic::set_hook(Box::new(|info| {
        let bt = std::backtrace::Backtrace::force_capture();
        let panic_msg = format!("VortexPlayer encountered a fatal error:\n\n{}\n\nLocation: {:?}\n\nBacktrace:\n{:?}", info, info.location(), bt);
        log_step(&format!("PANIC HOOK FIRED: {}", panic_msg));
        let panic_dir = dirs::data_local_dir()
            .unwrap_or_else(|| std::env::temp_dir())
            .join("VortexPlayer");
        let _ = std::fs::create_dir_all(&panic_dir);
        let panic_file = panic_dir.join("vortex_panic.log");
        let _ = std::fs::write(&panic_file, &panic_msg);
        eprintln!("{}", panic_msg);
        #[cfg(windows)]
        show_error_box("VortexPlayer - Fatal Error", &panic_msg);
    }));
    log_step("2. panic hook set");

    // Suppress NVIDIA GeForce Experience / ShadowPlay gaming overlay hooks
    unsafe {
        std::env::set_var("NV_DISABLE_GAMING_OVERLAY", "1");
        std::env::set_var("SHADOWPLAY_DISABLE", "1");
        std::env::set_var("GFE_OVERLAY_DISABLE", "1");
        std::env::set_var("NVIDIA_OVERLAY_DISABLE", "1");
        std::env::set_var("NV_GEFORCE_SHARE_DISABLE", "1");
        std::env::set_var("GFE_IN_GAME_OVERLAY_DISABLE", "1");
        std::env::set_var("SHADOWPLAY_RECORDING_DISABLE", "1");
        std::env::set_var("NvCameraEnable", "0");
        std::env::set_var("__GL_THREADED_OPTIMIZATIONS", "0");
    }

    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID;
        let app_id: Vec<u16> = "VortexPlayer.AudioVideoPlayer.1\0".encode_utf16().collect();
        let _ = SetCurrentProcessExplicitAppUserModelID(app_id.as_ptr());
    }

    let args: Vec<String> = env::args().collect();
    log_step(&format!("2a. args = {:?}", args));

    // Check for CLI flags
    if args.len() > 1 {
        let first_arg = &args[1];
        if first_arg == "-v" || first_arg == "--version" {
            println!("VortexPlayer - egui v1.0.0 (Rust Edition)");
            return;
        }
        if first_arg == "-h" || first_arg == "--help" {
            println!("VortexPlayer - High Quality Modern Rust Media Player");
            println!("Usage:");
            println!("  vortexplayer [options] [file_or_url]");
            println!("Options:");
            println!("  -v, --version    Show version info");
            println!("  -h, --help       Show help information");
            return;
        }
    }

    log_step("3. loading config");
    let config = AppConfig::load();
    log_step("3a. config loaded");

    // Copy libmpv-2.dll next to exe on Windows if needed
    #[cfg(windows)]
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let target_dll = exe_dir.join("libmpv-2.dll");
            if !target_dll.exists() {
                for candidate in &[PathBuf::from("libmpv-2.dll"), PathBuf::from("mpv-dev/libmpv-2.dll")] {
                    if candidate.exists() {
                        let _ = std::fs::copy(candidate, &target_dll);
                        break;
                    }
                }
            }
        }
    }

    let mut viewport = ViewportBuilder::default()
        .with_title("VortexPlayer - egui")
        .with_app_id("VortexPlayer.AudioVideoPlayer.1")
        .with_inner_size([config.window_width, config.window_height])
        .with_min_inner_size([380.0, 180.0])
        .with_position([100.0, 80.0])
        .with_visible(true)
        .with_decorations(false)
        .with_resizable(true)
        .with_drag_and_drop(true);

    if config.always_on_top {
        viewport = viewport.with_window_level(eframe::egui::WindowLevel::AlwaysOnTop);
    }
    viewport = viewport.with_active(true);

    // Generate VortexPlayer icon
    let icon = create_vertex_icon();
    if let Some(icon_data) = icon {
        viewport = viewport.with_icon(icon_data);
    }

    let options = eframe::NativeOptions {
        viewport,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    let initial_file = if args.len() > 1 && !args[1].starts_with('-') {
        Some(PathBuf::from(&args[1]))
    } else {
        None
    };

    log_step("3b. preparing options");

    // Install thread-level CBT hook before eframe creates the window, so that
    // DWM border suppression and window subclassing are applied inside CreateWindowExW
    // before the window is ever shown or paints for the first time.
    #[cfg(windows)]
    crate::platform::install_startup_cbt_hook();

    log_step("4. calling eframe::run_native");
    let res = eframe::run_native(
        "VortexPlayer - egui",
        options,
        Box::new(move |cc| {
            log_step("5. inside eframe app creator closure");
            #[cfg(windows)]
            {
                use windows_sys::Win32::Foundation::*;
                use windows_sys::Win32::UI::WindowsAndMessaging::*;
                use windows_sys::Win32::System::Threading::GetCurrentThreadId;
                unsafe extern "system" fn enum_thread_wnd(hwnd: HWND, _lparam: LPARAM) -> BOOL {
                    crate::platform::attach_subclass(hwnd as isize);
                    1
                }
                unsafe {
                    EnumThreadWindows(GetCurrentThreadId(), Some(enum_thread_wnd), 0);
                }
            }
            setup_unicode_fonts(&cc.egui_ctx);
            log_step("6. setup_unicode_fonts done");
            let app = PotApp::new(cc, initial_file);
            log_step("7. PotApp::new returned successfully");
            Ok(Box::new(app))
        }),
    );
    #[cfg(windows)]
    crate::platform::uninstall_startup_cbt_hook();
    log_step(&format!("8. eframe::run_native exited with result: {:?}", res.as_ref().map(|_| ())));

    if let Err(e) = res {
        let err_msg = format!(
            "VortexPlayer failed to initialize display window.\n\nError code: {:?}\n\nPlease check display driver and GPU support.",
            e
        );
        let err_dir = dirs::data_local_dir()
            .unwrap_or_else(|| std::env::temp_dir())
            .join("VortexPlayer");
        let _ = std::fs::create_dir_all(&err_dir);
        let _ = std::fs::write(err_dir.join("vortex_err.log"), &err_msg);
        eprintln!("{}", err_msg);
        #[cfg(windows)]
        show_error_box("VortexPlayer - Launch Error", &err_msg);
    }
}

pub fn log_step(_msg: &str) {
    use std::io::Write;
    let path = std::env::temp_dir().join("vortex_startup.log");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "[{:?}] [PID {}] {}", std::time::SystemTime::now(), std::process::id(), _msg);
    }
}

#[cfg(windows)]
pub fn show_error_box(title: &str, message: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK, MB_SYSTEMMODAL, MB_SETFOREGROUND};

    let title_w: Vec<u16> = OsStr::new(title).encode_wide().chain(std::iter::once(0)).collect();
    let msg_w: Vec<u16> = OsStr::new(message).encode_wide().chain(std::iter::once(0)).collect();
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            msg_w.as_ptr(),
            title_w.as_ptr(),
            MB_OK | MB_ICONERROR | MB_SYSTEMMODAL | MB_SETFOREGROUND,
        );
    }
}

#[cfg(not(windows))]
pub fn show_error_box(title: &str, message: &str) {
    eprintln!("{}: {}", title, message);
}

/// Configures egui with Windows system Unicode, UI Symbol & CJK fallback fonts (Japanese, Chinese, Korean, Cyrillic, etc.)
fn setup_unicode_fonts(ctx: &eframe::egui::Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();

    // 0. Primary Bundled Modern UI Font: Inter Medium & Regular (embedded, zero runtime OS dependency)
    fonts.font_data.insert(
        "inter_medium".to_owned(),
        std::sync::Arc::new(eframe::egui::FontData::from_static(include_bytes!(
            "../assets/fonts/Inter-Medium.ttf"
        ))),
    );
    fonts.font_data.insert(
        "inter_regular".to_owned(),
        std::sync::Arc::new(eframe::egui::FontData::from_static(include_bytes!(
            "../assets/fonts/Inter-Regular.ttf"
        ))),
    );
    if let Some(prop) = fonts.families.get_mut(&eframe::egui::FontFamily::Proportional) {
        prop.insert(0, "inter_medium".to_owned());
        prop.insert(1, "inter_regular".to_owned());
    }

    // 1. Primary Latin UI Font: Segoe UI or DejaVu / Roboto / FreeSans on Linux (fallback)
    let latin_candidates = [
        "C:\\Windows\\Fonts\\segoeui.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
        "/usr/share/fonts/truetype/ubuntu/Ubuntu-R.ttf",
    ];

    for path in latin_candidates {
        if let Ok(data) = std::fs::read(path) {
            fonts.font_data.insert(
                "primary_ui_font".to_owned(),
                std::sync::Arc::new(eframe::egui::FontData::from_owned(data)),
            );
            if let Some(prop) = fonts.families.get_mut(&eframe::egui::FontFamily::Proportional) {
                prop.push("primary_ui_font".to_owned());
            }
            break;
        }
    }

    // 2. UI Symbol Fallback Font (Segoe UI Symbol / Segoe UI Historic) - contains checkmarks (✓), media controls, arrows, UI glyphs
    let symbol_candidates = [
        "C:\\Windows\\Fonts\\seguisym.ttf",
        "C:\\Windows\\Fonts\\seguihis.ttf",
        "/usr/share/fonts/truetype/noto/NotoSansSymbols-Regular.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ];

    for path in symbol_candidates {
        if let Ok(data) = std::fs::read(path) {
            fonts.font_data.insert(
                "ui_symbol_font".to_owned(),
                std::sync::Arc::new(eframe::egui::FontData::from_owned(data)),
            );
            if let Some(prop) = fonts.families.get_mut(&eframe::egui::FontFamily::Proportional) {
                prop.push("ui_symbol_font".to_owned());
            }
            if let Some(mono) = fonts.families.get_mut(&eframe::egui::FontFamily::Monospace) {
                mono.push("ui_symbol_font".to_owned());
            }
            break;
        }
    }

    // 3. Emoji Fallback Font (Segoe UI Emoji / Noto Color Emoji)
    let emoji_candidates = [
        "C:\\Windows\\Fonts\\seguiemj.ttf",
        "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
        "/usr/share/fonts/truetype/noto/NotoEmoji-Regular.ttf",
    ];

    for path in emoji_candidates {
        if let Ok(data) = std::fs::read(path) {
            fonts.font_data.insert(
                "ui_emoji_font".to_owned(),
                std::sync::Arc::new(eframe::egui::FontData::from_owned(data)),
            );
            if let Some(prop) = fonts.families.get_mut(&eframe::egui::FontFamily::Proportional) {
                prop.push("ui_emoji_font".to_owned());
            }
            if let Some(mono) = fonts.families.get_mut(&eframe::egui::FontFamily::Monospace) {
                mono.push("ui_emoji_font".to_owned());
            }
            break;
        }
    }

    // 4. Global Universal CJK & Unicode Fallback (Microsoft YaHei / Malgun Gothic / Noto Sans CJK / MS Gothic / Arial)
    let cjk_candidates = [
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\msyh.ttf",
        "C:\\Windows\\Fonts\\malgun.ttf",
        "C:\\Windows\\Fonts\\msgothic.ttc",
        "C:\\Windows\\Fonts\\arial.ttf",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
    ];

    for path in cjk_candidates {
        if let Ok(data) = std::fs::read(path) {
            fonts.font_data.insert(
                "unicode_cjk_fallback".to_owned(),
                std::sync::Arc::new(eframe::egui::FontData::from_owned(data)),
            );
            if let Some(prop) = fonts.families.get_mut(&eframe::egui::FontFamily::Proportional) {
                prop.push("unicode_cjk_fallback".to_owned());
            }
            if let Some(mono) = fonts.families.get_mut(&eframe::egui::FontFamily::Monospace) {
                mono.push("unicode_cjk_fallback".to_owned());
            }
            break;
        }
    }

    ctx.set_fonts(fonts);
}

/// Creates the RGBA VertexPlayer window & taskbar icon from embedded 256x256 asset
fn create_vertex_icon() -> Option<IconData> {
    const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");
    if let Ok(img) = image::load_from_memory(ICON_BYTES) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        return Some(IconData {
            rgba: rgba.into_raw(),
            width,
            height,
        });
    }

    // Fallback if image decode fails
    let width: u32 = 32;
    let height: u32 = 32;
    let mut rgba: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - 15.5;
            let dy = y as f32 - 15.5;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= 14.5 {
                if dist >= 13.5 {
                    rgba.extend_from_slice(&[180, 110, 15, 255]);
                } else if dist >= 11.5 && dist < 13.0 {
                    rgba.extend_from_slice(&[255, 215, 70, 255]);
                } else {
                    rgba.extend_from_slice(&[245, 166, 35, 255]);
                }
            } else {
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }

    Some(IconData {
        rgba,
        width,
        height,
    })
}
