#![allow(dead_code, non_snake_case, unsafe_op_in_unsafe_fn)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarProgressState {
    NoProgress,
    Indeterminate,
    Normal,
    Error,
    Paused,
}

// Win32 Taskbar Constants
pub const THB_BITMAP: u32 = 0x0001;
pub const THB_ICON: u32 = 0x0002;
pub const THB_TOOLTIP: u32 = 0x0004;
pub const THB_FLAGS: u32 = 0x0008;

pub const THBF_ENABLED: u32 = 0x0000;
pub const THBF_DISABLED: u32 = 0x0001;
pub const THBF_DISMISSONCLICK: u32 = 0x0002;
pub const THBF_NOBACKGROUND: u32 = 0x0004;
pub const THBF_HIDDEN: u32 = 0x0008;
pub const THBF_NONINTERACTIVE: u32 = 0x0010;

pub const THBN_CLICKED: u16 = 0x1800;

// Thumbnail toolbar button IDs
pub const THUMB_CMD_PREV: u16 = 1001;
pub const THUMB_CMD_REWIND: u16 = 1002;
pub const THUMB_CMD_STOP: u16 = 1003;
pub const THUMB_CMD_PLAY_PAUSE: u16 = 1004;
pub const THUMB_CMD_FORWARD: u16 = 1005;
pub const THUMB_CMD_NEXT: u16 = 1006;
pub const THUMB_CMD_FULLSCREEN: u16 = 1007;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct THUMBBUTTON {
    pub dwMask: u32,
    pub iId: u32,
    pub iBitmap: u32,
    pub hIcon: *mut std::ffi::c_void,
    pub szTip: [u16; 260],
    pub dwFlags: u32,
}

#[cfg(windows)]
static THUMB_COMMAND_QUEUE: std::sync::Mutex<Vec<u16>> = std::sync::Mutex::new(Vec::new());

#[cfg(windows)]
type SUBCLASSPROC = unsafe extern "system" fn(
    hwnd: windows_sys::Win32::Foundation::HWND,
    msg: u32,
    wparam: windows_sys::Win32::Foundation::WPARAM,
    lparam: windows_sys::Win32::Foundation::LPARAM,
    uid_subclass: usize,
    ref_data: usize,
) -> windows_sys::Win32::Foundation::LRESULT;

#[cfg(windows)]
#[link(name = "comctl32")]
unsafe extern "system" {
    fn SetWindowSubclass(
        hWnd: windows_sys::Win32::Foundation::HWND,
        pfnSubclass: SUBCLASSPROC,
        uIdSubclass: usize,
        dwRefData: usize,
    ) -> windows_sys::Win32::Foundation::BOOL;
    fn DefSubclassProc(
        hWnd: windows_sys::Win32::Foundation::HWND,
        uMsg: u32,
        wParam: windows_sys::Win32::Foundation::WPARAM,
        lParam: windows_sys::Win32::Foundation::LPARAM,
    ) -> windows_sys::Win32::Foundation::LRESULT;
}

#[cfg(windows)]
#[cfg(windows)]
const SUBCLASS_ID: usize = 0x5442; // "TB"

#[cfg(windows)]
static CBT_HOOK: std::sync::atomic::AtomicIsize = std::sync::atomic::AtomicIsize::new(0);
static IS_NATIVE_FULLSCREEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn set_native_fullscreen_state(v: bool) {
    IS_NATIVE_FULLSCREEN.store(v, std::sync::atomic::Ordering::Relaxed);
}

pub fn is_native_fullscreen() -> bool {
    IS_NATIVE_FULLSCREEN.load(std::sync::atomic::Ordering::Relaxed)
}

#[cfg(windows)]
pub unsafe fn apply_border_suppression(hwnd: windows_sys::Win32::Foundation::HWND) {
    let is_fullscreen_or_zoomed = is_native_fullscreen() || windows_sys::Win32::UI::WindowsAndMessaging::IsZoomed(hwnd) != 0 || {
        use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST};
        use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowRect;
        use windows_sys::Win32::Foundation::RECT;
        let mut wr = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        if GetWindowRect(hwnd, &mut wr) != 0 {
            let hmon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            let mut mi: MONITORINFO = std::mem::zeroed();
            mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            if GetMonitorInfoW(hmon, &mut mi) != 0 {
                (wr.right - wr.left) >= (mi.rcMonitor.right - mi.rcMonitor.left)
                    && (wr.bottom - wr.top) >= (mi.rcMonitor.bottom - mi.rcMonitor.top)
            } else {
                false
            }
        } else {
            false
        }
    };
    let corner = if is_fullscreen_or_zoomed {
        1 /* DWMWCP_DONOTROUND - completely square, no rounded corners in fullscreen or maximized */
    } else {
        2 /* DWMWCP_ROUND - circular / rounded borders for normal windowed mode */
    };
    apply_border_suppression_internal(hwnd, corner);
}

#[cfg(windows)]
pub unsafe fn apply_fullscreen_border_suppression(hwnd: windows_sys::Win32::Foundation::HWND) {
    apply_border_suppression_internal(hwnd, 1 /* DWMWCP_DONOTROUND - square for fullscreen */);
}

#[cfg(windows)]
unsafe fn apply_border_suppression_internal(
    hwnd: windows_sys::Win32::Foundation::HWND,
    corner_pref: u32,
) {
    if hwnd.is_null() {
        return;
    }

    #[link(name = "dwmapi")]
    unsafe extern "system" {
        fn DwmSetWindowAttribute(
            hwnd: windows_sys::Win32::Foundation::HWND,
            dw_attribute: u32,
            pv_attribute: *const std::ffi::c_void,
            cb_attribute: u32,
        ) -> i32;
    }

    // 1. DWMWA_USE_IMMERSIVE_DARK_MODE (20 in modern Win10/11, 19 in early Win10)
    let dark_mode: u32 = 1;
    let _ = DwmSetWindowAttribute(hwnd, 20, &dark_mode as *const _ as _, 4);
    let _ = DwmSetWindowAttribute(hwnd, 19, &dark_mode as *const _ as _, 4);

    // 2. DWMWA_WINDOW_CORNER_PREFERENCE (33)
    // 2 = DWMWCP_ROUND (circular / rounded borders for modern Windows 11 windowed mode)
    // 1 = DWMWCP_DONOTROUND (square for fullscreen mode)
    let _ = DwmSetWindowAttribute(hwnd, 33, &corner_pref as *const _ as _, 4);

    // 3. DWMWA_BORDER_COLOR (34)
    // First try DWMWA_COLOR_NONE (0xFFFFFFFE) to suppress border completely in Windows 11 build 22621+
    let border_none: u32 = 0xFFFFFFFE;
    let hr = DwmSetWindowAttribute(hwnd, 34, &border_none as *const _ as _, 4);
    if hr != 0 {
        // If DWMWA_COLOR_NONE is not accepted, force dark border matching window dark theme
        // (RGB 18, 19, 24 -> COLORREF 0x00181312), preventing Windows from ever using the blue accent color
        let dark_border: u32 = 0x00181312;
        let _ = DwmSetWindowAttribute(hwnd, 34, &dark_border as *const _ as _, 4);
    }

    // 4. DWMWA_CAPTION_COLOR (35) -> Dark color matching background (0x00181312)
    // Prevents DWM from drawing any default accent/blue caption background or top line
    let caption_color: u32 = 0x00181312;
    let _ = DwmSetWindowAttribute(hwnd, 35, &caption_color as *const _ as _, 4);

    // 5. DWMWA_SYSTEMBACKDROP_TYPE (38) -> DWMSBT_NONE (1)
    // Disables Mica/Acrylic material frame rendering
    let backdrop: u32 = 1;
    let _ = DwmSetWindowAttribute(hwnd, 38, &backdrop as *const _ as _, 4);
}

#[cfg(not(windows))]
pub unsafe fn apply_border_suppression(_hwnd: *mut std::ffi::c_void) {}
#[cfg(not(windows))]
pub unsafe fn apply_fullscreen_border_suppression(_hwnd: *mut std::ffi::c_void) {}

#[cfg(windows)]
pub unsafe fn clean_window_styles(hwnd: windows_sys::Win32::Foundation::HWND) {
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    if hwnd.is_null() {
        return;
    }
    let style = GetWindowLongW(hwnd, GWL_STYLE) as u32;
    let clean_style = (style & !(WS_BORDER | WS_DLGFRAME | WS_CAPTION))
        | WS_POPUP | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_CLIPCHILDREN | WS_CLIPSIBLINGS;
    if clean_style != style {
        SetWindowLongW(hwnd, GWL_STYLE, clean_style as i32);
    }
    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
    let clean_ex = (ex_style & !(WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE | WS_EX_STATICEDGE | WS_EX_DLGMODALFRAME))
        | WS_EX_APPWINDOW;
    if clean_ex != ex_style {
        SetWindowLongW(hwnd, GWL_EXSTYLE, clean_ex as i32);
    }
    if clean_style != style || clean_ex != ex_style {
        SetWindowPos(hwnd, 0 as _, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);
    }
}

#[cfg(not(windows))]
pub unsafe fn clean_window_styles(_hwnd: *mut std::ffi::c_void) {}

#[cfg(windows)]
pub fn attach_subclass(hwnd: isize) {
    if hwnd == 0 {
        return;
    }
    unsafe {
        let hwnd_val = hwnd as windows_sys::Win32::Foundation::HWND;
        clean_window_styles(hwnd_val);
        apply_border_suppression(hwnd_val);
        SetWindowSubclass(hwnd_val, thumbbar_subclass_proc, SUBCLASS_ID, 0);
        apply_border_suppression(hwnd_val);
    }
}

#[cfg(not(windows))]
pub fn attach_subclass(_hwnd: isize) {}

#[cfg(windows)]
unsafe extern "system" fn cbt_hook_proc(
    n_code: i32,
    wparam: windows_sys::Win32::Foundation::WPARAM,
    lparam: windows_sys::Win32::Foundation::LPARAM,
) -> windows_sys::Win32::Foundation::LRESULT {
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    if n_code == HCBT_CREATEWND as i32 {
        let hwnd = wparam as windows_sys::Win32::Foundation::HWND;
        apply_border_suppression(hwnd);
        clean_window_styles(hwnd);
        attach_subclass(hwnd as isize);
        uninstall_startup_cbt_hook();
    }
    CallNextHookEx(0 as _, n_code, wparam, lparam)
}

#[cfg(windows)]
pub fn install_startup_cbt_hook() {
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    use windows_sys::Win32::System::Threading::GetCurrentThreadId;
    unsafe {
        let thread_id = GetCurrentThreadId();
        let hook = SetWindowsHookExW(
            WH_CBT,
            Some(cbt_hook_proc),
            std::ptr::null_mut(),
            thread_id,
        );
        CBT_HOOK.store(hook as isize, std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(not(windows))]
pub fn install_startup_cbt_hook() {}

#[cfg(windows)]
pub fn uninstall_startup_cbt_hook() {
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    let hook = CBT_HOOK.swap(0, std::sync::atomic::Ordering::Relaxed);
    if hook != 0 {
        unsafe {
            UnhookWindowsHookEx(hook as _);
        }
    }
}

#[cfg(not(windows))]
pub fn uninstall_startup_cbt_hook() {}

#[cfg(windows)]
unsafe extern "system" fn thumbbar_subclass_proc(
    hwnd: windows_sys::Win32::Foundation::HWND,
    msg: u32,
    wparam: windows_sys::Win32::Foundation::WPARAM,
    lparam: windows_sys::Win32::Foundation::LPARAM,
    _uid_subclass: usize,
    _ref_data: usize,
) -> windows_sys::Win32::Foundation::LRESULT {
    use windows_sys::Win32::UI::WindowsAndMessaging::*;

    // 1. Taskbar thumbnail button commands
    if msg == WM_COMMAND {
        let hi = ((wparam >> 16) & 0xffff) as u16;
        let lo = (wparam & 0xffff) as u16;
        if hi == THBN_CLICKED {
            if let Ok(mut q) = THUMB_COMMAND_QUEUE.lock() {
                q.push(lo);
            }
            return 0;
        }
    }

    // 2. Suppress Windows 11 system accent border (blue outline when active/moving)
    if msg == WM_NCACTIVATE {
        apply_border_suppression(hwnd);
        // Returning 1 directly acknowledges window activation without invoking
        // DefWindowProc's non-client painting that draws the system blue accent border.
        return 1;
    }
    if msg == WM_NCPAINT {
        // Prevent Windows from painting default nonclient borders
        apply_border_suppression(hwnd);
        return 0;
    }
    if msg == WM_ERASEBKGND {
        // Suppress background erase flicker during resize/move
        return 1;
    }

    // Handle native hardware window resizing on all 4 borders and 4 corners
    if msg == WM_NCHITTEST {
        if !is_native_fullscreen() && IsZoomed(hwnd) == 0 {
            use windows_sys::Win32::Foundation::RECT;
            let mut wr = RECT { left: 0, top: 0, right: 0, bottom: 0 };
            if GetWindowRect(hwnd, &mut wr) != 0 {
                let x = ((lparam & 0xffff) as i16) as i32;
                let y = (((lparam >> 16) & 0xffff) as i16) as i32;
                let b = 8; // 8px resize edge for reliable mouse grabbing on left, right, bottom
                let bt = 4; // 4px top edge so titlebar dragging is never interrupted

                let on_left = x >= wr.left - 2 && x < wr.left + b;
                let on_right = x <= wr.right + 2 && x > wr.right - b;
                let on_top = y >= wr.top - 2 && y < wr.top + bt;
                let on_bottom = y <= wr.bottom + 2 && y > wr.bottom - b;

                if on_top && on_left { return HTTOPLEFT as _; }
                if on_top && on_right { return HTTOPRIGHT as _; }
                if on_bottom && on_left { return HTBOTTOMLEFT as _; }
                if on_bottom && on_right { return HTBOTTOMRIGHT as _; }
                if on_left { return HTLEFT as _; }
                if on_right { return HTRIGHT as _; }
                if on_bottom { return HTBOTTOM as _; }
                if on_top { return HTTOP as _; }
            }
        }
    }

    // Direct native routing of non-client mouse clicks to DefWindowProcW so Windows
    // executes modal sizing/moving loops without interference from winit/subclasses
    if msg == WM_NCLBUTTONDOWN {
        use windows_sys::Win32::UI::WindowsAndMessaging::DefWindowProcW;
        return DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    // Remove 1-pixel top line inset caused by Windows DefWindowProc on borderless windows,
    // and when maximized, align the client rectangle exactly to the monitor work area.
    // In fullscreen mode, client rectangle is always the 100% full monitor bounds.
    if msg == WM_NCCALCSIZE && wparam != 0 {
        if !is_native_fullscreen() && IsZoomed(hwnd) != 0 {
            use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST};
            let hmon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            let mut mi: MONITORINFO = std::mem::zeroed();
            mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            if GetMonitorInfoW(hmon, &mut mi) != 0 {
                let params = lparam as *mut windows_sys::Win32::UI::WindowsAndMessaging::NCCALCSIZE_PARAMS;
                (*params).rgrc[0] = mi.rcWork;
                return 0;
            }
        }
        return 0;
    }

    // Undocumented non-client themed frame/caption draw messages (WM_NCUAHDRAWCAPTION / WM_NCUAHDRAWFRAME)
    // Suppress without invoking heavy synchronous DWM RPC calls on hover
    if msg == 0x00AE || msg == 0x00AF {
        return 0;
    }

    // Re-apply border suppression on window state and theme changes, but NEVER during
    // continuous move/size loops (WM_MOVING, WM_MOVE, WM_WINDOWPOSCHANGING) which would flood
    // synchronous cross-process DWM RPC calls and cause heavy window dragging lag.
    if msg == WM_ENTERSIZEMOVE
        || msg == WM_EXITSIZEMOVE
        || msg == WM_ACTIVATE
        || msg == WM_ACTIVATEAPP
        || msg == WM_SETFOCUS
        || msg == WM_SHOWWINDOW
        || msg == WM_CREATE
        || msg == WM_THEMECHANGED
        || msg == WM_SETTINGCHANGE
        || msg == 0x0320 /* WM_DWMCOLORIZATIONCOLORCHANGED */
    {
        apply_border_suppression(hwnd);
    }

    DefSubclassProc(hwnd, msg, wparam, lparam)
}

pub fn to_wide_chars(s: &str) -> [u16; 260] {
    let mut buf = [0u16; 260];
    let mut i = 0;
    for c in s.encode_utf16() {
        if i >= 259 {
            break;
        }
        buf[i] = c;
        i += 1;
    }
    buf[i] = 0;
    buf
}

// 20x20 Icon Canvas for high-contrast crisp taskbar thumbnail preview buttons
struct IconCanvas {
    w: usize,
    h: usize,
    pixels: Vec<u8>, // RGBA
}

impl IconCanvas {
    fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            pixels: vec![0; w * h * 4],
        }
    }

    fn put_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8) {
        if x < self.w && y < self.h {
            let idx = (y * self.w + x) * 4;
            self.pixels[idx] = r;
            self.pixels[idx + 1] = g;
            self.pixels[idx + 2] = b;
            self.pixels[idx + 3] = a;
        }
    }

    fn fill_rect(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, r: u8, g: u8, b: u8, a: u8) {
        for y in y1..=y2 {
            for x in x1..=x2 {
                self.put_pixel(x, y, r, g, b, a);
            }
        }
    }
}

#[cfg(windows)]
unsafe fn create_hicon_from_rgba(width: u32, height: u32, rgba: &[u8]) -> *mut std::ffi::c_void {
    use windows_sys::Win32::Graphics::Gdi::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;

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
            // RGBA -> BGRA
            dst[src_idx] = rgba[src_idx + 2];     // Blue
            dst[src_idx + 1] = rgba[src_idx + 1]; // Green
            dst[src_idx + 2] = rgba[src_idx];     // Red
            dst[src_idx + 3] = rgba[src_idx + 3]; // Alpha
        }
    }

    let mask_bytes = vec![0u8; (((width + 15) / 16) * 2 * height) as usize];
    let hbm_mask = CreateBitmap(width as i32, height as i32, 1, 1, mask_bytes.as_ptr() as _);
    ReleaseDC(0 as _, hdc);

    let mut ii = ICONINFO {
        fIcon: 1,
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: hbm_mask,
        hbmColor: hbm_color,
    };

    let h_icon = CreateIconIndirect(&mut ii);
    if !hbm_color.is_null() {
        DeleteObject(hbm_color as _);
    }
    if !hbm_mask.is_null() {
        DeleteObject(hbm_mask as _);
    }

    h_icon as *mut std::ffi::c_void
}

#[cfg(windows)]
pub fn build_button_icons() -> [isize; 8] {
    let fg = (235, 240, 252, 255);

    // 0: Prev (|◀)
    let mut c_prev = IconCanvas::new(20, 20);
    c_prev.fill_rect(3, 4, 4, 15, fg.0, fg.1, fg.2, fg.3);
    for x in 5..=15 {
        let frac = (x - 5) as f32 / 10.0;
        let y1 = (9.5 - 5.5 * frac).round().max(4.0) as usize;
        let y2 = (10.5 + 5.5 * frac).round().min(15.0) as usize;
        c_prev.fill_rect(x, y1, x, y2, fg.0, fg.1, fg.2, fg.3);
    }

    // 1: Rewind (◀◀)
    let mut c_rew = IconCanvas::new(20, 20);
    for x in 3..=9 {
        let frac = (x - 3) as f32 / 6.0;
        let y1 = (9.5 - 5.0 * frac).round().max(4.0) as usize;
        let y2 = (10.5 + 5.0 * frac).round().min(15.0) as usize;
        c_rew.fill_rect(x, y1, x, y2, fg.0, fg.1, fg.2, fg.3);
    }
    for x in 10..=16 {
        let frac = (x - 10) as f32 / 6.0;
        let y1 = (9.5 - 5.0 * frac).round().max(4.0) as usize;
        let y2 = (10.5 + 5.0 * frac).round().min(15.0) as usize;
        c_rew.fill_rect(x, y1, x, y2, fg.0, fg.1, fg.2, fg.3);
    }

    // 2: Stop (■)
    let mut c_stop = IconCanvas::new(20, 20);
    c_stop.fill_rect(5, 5, 14, 14, fg.0, fg.1, fg.2, fg.3);

    // 3: Play (▶)
    let mut c_play = IconCanvas::new(20, 20);
    for x in 6..=15 {
        let frac = (15 - x) as f32 / 9.0;
        let y1 = (9.5 - 5.5 * frac).round().max(4.0) as usize;
        let y2 = (10.5 + 5.5 * frac).round().min(15.0) as usize;
        c_play.fill_rect(x, y1, x, y2, fg.0, fg.1, fg.2, fg.3);
    }

    // 4: Pause (⏸)
    let mut c_pause = IconCanvas::new(20, 20);
    c_pause.fill_rect(5, 4, 7, 15, fg.0, fg.1, fg.2, fg.3);
    c_pause.fill_rect(12, 4, 14, 15, fg.0, fg.1, fg.2, fg.3);

    // 5: Forward (▶▶)
    let mut c_fwd = IconCanvas::new(20, 20);
    for x in 3..=9 {
        let frac = (9 - x) as f32 / 6.0;
        let y1 = (9.5 - 5.0 * frac).round().max(4.0) as usize;
        let y2 = (10.5 + 5.0 * frac).round().min(15.0) as usize;
        c_fwd.fill_rect(x, y1, x, y2, fg.0, fg.1, fg.2, fg.3);
    }
    for x in 10..=16 {
        let frac = (16 - x) as f32 / 6.0;
        let y1 = (9.5 - 5.0 * frac).round().max(4.0) as usize;
        let y2 = (10.5 + 5.0 * frac).round().min(15.0) as usize;
        c_fwd.fill_rect(x, y1, x, y2, fg.0, fg.1, fg.2, fg.3);
    }

    // 6: Next (▶|)
    let mut c_next = IconCanvas::new(20, 20);
    for x in 4..=14 {
        let frac = (14 - x) as f32 / 10.0;
        let y1 = (9.5 - 5.5 * frac).round().max(4.0) as usize;
        let y2 = (10.5 + 5.5 * frac).round().min(15.0) as usize;
        c_next.fill_rect(x, y1, x, y2, fg.0, fg.1, fg.2, fg.3);
    }
    c_next.fill_rect(15, 4, 16, 15, fg.0, fg.1, fg.2, fg.3);

    // 7: Fullscreen (⤢)
    let mut c_fs = IconCanvas::new(20, 20);
    // Corners: top-left
    c_fs.fill_rect(4, 4, 8, 5, fg.0, fg.1, fg.2, fg.3);
    c_fs.fill_rect(4, 4, 5, 8, fg.0, fg.1, fg.2, fg.3);
    // top-right
    c_fs.fill_rect(11, 4, 15, 5, fg.0, fg.1, fg.2, fg.3);
    c_fs.fill_rect(14, 4, 15, 8, fg.0, fg.1, fg.2, fg.3);
    // bottom-left
    c_fs.fill_rect(4, 14, 8, 15, fg.0, fg.1, fg.2, fg.3);
    c_fs.fill_rect(4, 11, 5, 15, fg.0, fg.1, fg.2, fg.3);
    // bottom-right
    c_fs.fill_rect(11, 14, 15, 15, fg.0, fg.1, fg.2, fg.3);
    c_fs.fill_rect(14, 11, 15, 15, fg.0, fg.1, fg.2, fg.3);

    unsafe {
        [
            create_hicon_from_rgba(20, 20, &c_prev.pixels) as isize,
            create_hicon_from_rgba(20, 20, &c_rew.pixels) as isize,
            create_hicon_from_rgba(20, 20, &c_stop.pixels) as isize,
            create_hicon_from_rgba(20, 20, &c_play.pixels) as isize,
            create_hicon_from_rgba(20, 20, &c_pause.pixels) as isize,
            create_hicon_from_rgba(20, 20, &c_fwd.pixels) as isize,
            create_hicon_from_rgba(20, 20, &c_next.pixels) as isize,
            create_hicon_from_rgba(20, 20, &c_fs.pixels) as isize,
        ]
    }
}

pub struct WindowsTaskbarAdapter {
    pub current_state: TaskbarProgressState,
    pub current_progress: f64,
    p_taskbar: isize,
    thumbbar_initialized: bool,
    subclass_attached: bool,
    last_is_paused: bool,
    last_pct: u64,
    last_state: u32,
    last_clip: Option<[i32; 4]>,
    icons: Option<[isize; 8]>,
}

unsafe impl Send for WindowsTaskbarAdapter {}
unsafe impl Sync for WindowsTaskbarAdapter {}

impl Default for WindowsTaskbarAdapter {
    fn default() -> Self {
        Self {
            current_state: TaskbarProgressState::NoProgress,
            current_progress: 0.0,
            p_taskbar: 0,
            thumbbar_initialized: false,
            subclass_attached: false,
            last_is_paused: true,
            last_pct: 0,
            last_state: 0,
            last_clip: None,
            icons: None,
        }
    }
}

impl WindowsTaskbarAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_thumbbar_initialized(&self) -> bool {
        self.thumbbar_initialized
    }

    #[cfg(windows)]
    fn ensure_taskbar_com(&mut self) -> bool {
        if self.p_taskbar != 0 {
            return true;
        }

        type HRESULT = i32;
        type LPVOID = *mut std::ffi::c_void;

        #[repr(C)]
        struct GUID {
            data1: u32,
            data2: u16,
            data3: u16,
            data4: [u8; 8],
        }

        const CLSID_TASKBAR_LIST: GUID = GUID {
            data1: 0x56FDF344,
            data2: 0xFD6D,
            data3: 0x11D0,
            data4: [0x95, 0x8A, 0x00, 0x60, 0x97, 0xC9, 0xA0, 0x90],
        };
        const IID_ITASKBAR_LIST3: GUID = GUID {
            data1: 0xEA1AFB91,
            data2: 0x9E28,
            data3: 0x4B86,
            data4: [0x90, 0xE9, 0x9E, 0x9F, 0x8A, 0x5E, 0xEF, 0xAF],
        };

        #[link(name = "ole32")]
        unsafe extern "system" {
            fn CoInitialize(pv_reserved: LPVOID) -> HRESULT;
            fn CoCreateInstance(
                rclsid: *const GUID,
                p_unk_outer: LPVOID,
                dw_cls_context: u32,
                riid: *const GUID,
                ppv: *mut LPVOID,
            ) -> HRESULT;
        }

        unsafe {
            let _ = CoInitialize(std::ptr::null_mut());
            let mut ptr: LPVOID = std::ptr::null_mut();
            let hr = CoCreateInstance(
                &CLSID_TASKBAR_LIST,
                std::ptr::null_mut(),
                1, // CLSCTX_INPROC_SERVER
                &IID_ITASKBAR_LIST3,
                &mut ptr,
            );
            if hr == 0 && !ptr.is_null() {
                let vtbl = *(ptr as *const *const usize);
                let hr_init: unsafe extern "system" fn(this: LPVOID) -> HRESULT =
                    std::mem::transmute(*vtbl.add(3));
                let _ = hr_init(ptr);
                self.p_taskbar = ptr as isize;
                true
            } else {
                false
            }
        }
    }

    #[cfg(windows)]
    pub fn init_thumbbar(&mut self, hwnd: isize) {
        if hwnd == 0 || self.thumbbar_initialized {
            return;
        }
        if !self.ensure_taskbar_com() {
            return;
        }

        // 1. Subclass window to intercept WM_COMMAND from taskbar buttons
        if !self.subclass_attached {
            attach_subclass(hwnd);
            self.subclass_attached = true;
        }

        // 2. Prepare button icons
        let icons = self.icons.get_or_insert_with(build_button_icons);

        // 3. Register the 7 thumbnail toolbar buttons (matches Vortex layout)
        let buttons = [
            THUMBBUTTON {
                dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
                iId: THUMB_CMD_PREV as u32,
                iBitmap: 0,
                hIcon: icons[0] as *mut std::ffi::c_void,
                szTip: to_wide_chars("Previous Track (Page Up)"),
                dwFlags: THBF_ENABLED,
            },
            THUMBBUTTON {
                dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
                iId: THUMB_CMD_REWIND as u32,
                iBitmap: 0,
                hIcon: icons[1] as *mut std::ffi::c_void,
                szTip: to_wide_chars("Rewind 10s (Left Arrow)"),
                dwFlags: THBF_ENABLED,
            },
            THUMBBUTTON {
                dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
                iId: THUMB_CMD_STOP as u32,
                iBitmap: 0,
                hIcon: icons[2] as *mut std::ffi::c_void,
                szTip: to_wide_chars("Stop / Return to Home (Ctrl+S / Ctrl+W)"),
                dwFlags: THBF_ENABLED,
            },
            THUMBBUTTON {
                dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
                iId: THUMB_CMD_PLAY_PAUSE as u32,
                iBitmap: 0,
                hIcon: icons[3] as *mut std::ffi::c_void, // Initially Play icon
                szTip: to_wide_chars("Play (Space)"),
                dwFlags: THBF_ENABLED,
            },
            THUMBBUTTON {
                dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
                iId: THUMB_CMD_FORWARD as u32,
                iBitmap: 0,
                hIcon: icons[5] as *mut std::ffi::c_void,
                szTip: to_wide_chars("Forward 10s (Right Arrow)"),
                dwFlags: THBF_ENABLED,
            },
            THUMBBUTTON {
                dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
                iId: THUMB_CMD_NEXT as u32,
                iBitmap: 0,
                hIcon: icons[6] as *mut std::ffi::c_void,
                szTip: to_wide_chars("Next Track (Page Down)"),
                dwFlags: THBF_ENABLED,
            },
            THUMBBUTTON {
                dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
                iId: THUMB_CMD_FULLSCREEN as u32,
                iBitmap: 0,
                hIcon: icons[7] as *mut std::ffi::c_void,
                szTip: to_wide_chars("Toggle Fullscreen (Enter)"),
                dwFlags: THBF_ENABLED,
            },
        ];

        unsafe {
            let vtbl = *(self.p_taskbar as *const *const usize);
            // vtbl[4] is AddTab(this, hwnd) - register the undecorated borderless window on the Windows taskbar
            let add_tab: unsafe extern "system" fn(
                this: *mut std::ffi::c_void,
                hwnd: *mut std::ffi::c_void,
            ) -> i32 = std::mem::transmute(*vtbl.add(4));
            let hr_tab = add_tab(
                self.p_taskbar as *mut std::ffi::c_void,
                hwnd as *mut std::ffi::c_void,
            );

            // vtbl[15] is ThumbBarAddButtons(this, hwnd, cButtons, pButton)
            let thumb_add: unsafe extern "system" fn(
                this: *mut std::ffi::c_void,
                hwnd: *mut std::ffi::c_void,
                c_buttons: u32,
                p_button: *const THUMBBUTTON,
            ) -> i32 = std::mem::transmute(*vtbl.add(15));

            let hr = thumb_add(
                self.p_taskbar as *mut std::ffi::c_void,
                hwnd as *mut std::ffi::c_void,
                7,
                buttons.as_ptr(),
            );
            static ATTEMPT_CNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let cnt = ATTEMPT_CNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if cnt < 3 || hr == 0 {
                crate::log_step(&format!("10i. icon handles: {:?}, AddTab: hr=0x{:08X}, ThumbBarAddButtons: hwnd={}, hr=0x{:08X}", icons, hr_tab as u32, hwnd, hr as u32));
            }
            // 0 = S_OK, 0x800401e5 = CO_E_ALREADYINITIALIZED, 0x80070057 = E_INVALIDARG (buttons already present)
            if hr == 0 || hr == 0x800401e5_u32 as i32 || hr == 0x80070057_u32 as i32 || cnt >= 5 {
                self.thumbbar_initialized = true;
            }
        }
    }

    #[cfg(not(windows))]
    pub fn init_thumbbar(&mut self, _hwnd: isize) {}

    #[cfg(windows)]
    pub fn update_thumbbar(&mut self, hwnd: isize, is_paused: bool, _is_idle: bool) {
        if hwnd == 0 || !self.thumbbar_initialized || self.p_taskbar == 0 {
            return;
        }

        if is_paused != self.last_is_paused {
            self.last_is_paused = is_paused;
            let icons = match self.icons {
                Some(ref ic) => ic,
                None => return,
            };

            let play_pause_icon = if is_paused { icons[3] } else { icons[4] };
            let tip = if is_paused { "Play (Space)" } else { "Pause (Space)" };

            let btn = THUMBBUTTON {
                dwMask: THB_ICON | THB_TOOLTIP | THB_FLAGS,
                iId: THUMB_CMD_PLAY_PAUSE as u32,
                iBitmap: 0,
                hIcon: play_pause_icon as *mut std::ffi::c_void,
                szTip: to_wide_chars(tip),
                dwFlags: THBF_ENABLED,
            };

            unsafe {
                let vtbl = *(self.p_taskbar as *const *const usize);
                // vtbl[16] is ThumbBarUpdateButtons(this, hwnd, cButtons, pButton)
                let thumb_update: unsafe extern "system" fn(
                    this: *mut std::ffi::c_void,
                    hwnd: *mut std::ffi::c_void,
                    c_buttons: u32,
                    p_button: *const THUMBBUTTON,
                ) -> i32 = std::mem::transmute(*vtbl.add(16));

                let _ = thumb_update(
                    self.p_taskbar as *mut std::ffi::c_void,
                    hwnd as *mut std::ffi::c_void,
                    1,
                    &btn,
                );
            }
        }
    }

    #[cfg(not(windows))]
    pub fn update_thumbbar(&mut self, _hwnd: isize, _is_paused: bool, _is_idle: bool) {}

    #[cfg(windows)]
    pub fn update_progress(
        &mut self,
        hwnd: isize,
        current_sec: f64,
        total_sec: f64,
        is_paused: bool,
        is_idle: bool,
    ) {
        if hwnd == 0 || !self.ensure_taskbar_com() {
            return;
        }

        unsafe {
            let vtbl = *(self.p_taskbar as *const *const usize);
            let set_val: unsafe extern "system" fn(
                this: *mut std::ffi::c_void,
                hwnd: *mut std::ffi::c_void,
                ull_completed: u64,
                ull_total: u64,
            ) -> i32 = std::mem::transmute(*vtbl.add(9));

            let set_state: unsafe extern "system" fn(
                this: *mut std::ffi::c_void,
                hwnd: *mut std::ffi::c_void,
                tbp_flags: u32,
            ) -> i32 = std::mem::transmute(*vtbl.add(10));

            if total_sec <= 0.0 || is_idle {
                if self.last_state != 0 {
                    self.last_state = 0;
                    self.last_pct = 0;
                    let _ = set_state(self.p_taskbar as _, hwnd as _, 0 /* TBPF_NOPROGRESS */);
                }
                return;
            }

            let completed = (current_sec.clamp(0.0, total_sec) * 100.0) as u64;
            let total = (total_sec * 100.0) as u64;
            let pct = if total > 0 { (completed * 100) / total } else { 0 };

            // TBPF_NORMAL = 2 (Green), TBPF_PAUSED = 8 (Yellow)
            let prog_state: u32 = if is_paused { 8 } else { 2 };

            if prog_state != self.last_state {
                self.last_state = prog_state;
                let _ = set_state(self.p_taskbar as _, hwnd as _, prog_state);
            }

            if pct != self.last_pct {
                self.last_pct = pct;
                let _ = set_val(self.p_taskbar as _, hwnd as _, completed, total);
            }
        }
    }

    #[cfg(not(windows))]
    pub fn update_progress(
        &mut self,
        _hwnd: isize,
        _current_sec: f64,
        _total_sec: f64,
        _is_paused: bool,
        _is_idle: bool,
    ) {}

    #[cfg(windows)]
    pub fn clear_progress(&mut self, hwnd: isize) {
        if hwnd == 0 || self.p_taskbar == 0 {
            return;
        }
        self.last_state = 0;
        self.last_pct = 0;
        unsafe {
            let vtbl = *(self.p_taskbar as *const *const usize);
            let set_state: unsafe extern "system" fn(
                this: *mut std::ffi::c_void,
                hwnd: *mut std::ffi::c_void,
                tbp_flags: u32,
            ) -> i32 = std::mem::transmute(*vtbl.add(10));
            let _ = set_state(self.p_taskbar as _, hwnd as _, 0);
        }
        self.set_thumbnail_clip(hwnd, None);
    }

    #[cfg(not(windows))]
    pub fn clear_progress(&mut self, _hwnd: isize) {}

    #[cfg(windows)]
    pub fn set_thumbnail_clip(&mut self, hwnd: isize, clip_rect: Option<[i32; 4]>) {
        if hwnd == 0 || !self.ensure_taskbar_com() {
            return;
        }
        if clip_rect == self.last_clip {
            return;
        }
        self.last_clip = clip_rect;

        unsafe {
            use windows_sys::Win32::Foundation::RECT;
            let vtbl = *(self.p_taskbar as *const *const usize);
            // vtbl[20] is SetThumbnailClip(this, hwnd, prcClip)
            let set_clip: unsafe extern "system" fn(
                this: *mut std::ffi::c_void,
                hwnd: *mut std::ffi::c_void,
                prc_clip: *const RECT,
            ) -> i32 = std::mem::transmute(*vtbl.add(20));

            if let Some([left, top, right, bottom]) = clip_rect {
                let rc = RECT { left, top, right, bottom };
                let _ = set_clip(self.p_taskbar as _, hwnd as _, &rc);
            } else {
                let _ = set_clip(self.p_taskbar as _, hwnd as _, std::ptr::null());
            }
        }
    }

    #[cfg(not(windows))]
    pub fn set_thumbnail_clip(&mut self, _hwnd: isize, _clip_rect: Option<[i32; 4]>) {}

    #[cfg(windows)]
    pub fn poll_commands(&mut self) -> Vec<u16> {
        if let Ok(mut q) = THUMB_COMMAND_QUEUE.lock() {
            std::mem::take(&mut *q)
        } else {
            Vec::new()
        }
    }

    #[cfg(not(windows))]
    pub fn poll_commands(&mut self) -> Vec<u16> {
        Vec::new()
    }
}

impl Drop for WindowsTaskbarAdapter {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::UI::WindowsAndMessaging::DestroyIcon;
            if let Some(icons) = self.icons {
                for icon in icons {
                    if icon != 0 {
                        DestroyIcon(icon as _);
                    }
                }
            }
            if self.p_taskbar != 0 {
                let vtbl = *(self.p_taskbar as *const *const usize);
                let release: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> u32 =
                    std::mem::transmute(*vtbl.add(2));
                let _ = release(self.p_taskbar as *mut std::ffi::c_void);
                self.p_taskbar = 0;
            }
        }
    }
}

#[cfg(windows)]
pub fn is_taskbar_autohide() -> bool {
    use windows_sys::Win32::UI::Shell::{SHAppBarMessage, APPBARDATA, ABM_GETSTATE, ABS_AUTOHIDE};
    unsafe {
        let mut abd: APPBARDATA = std::mem::zeroed();
        abd.cbSize = std::mem::size_of::<APPBARDATA>() as u32;
        let state = SHAppBarMessage(ABM_GETSTATE, &mut abd);
        (state & (ABS_AUTOHIDE as usize)) != 0
    }
}
#[cfg(not(windows))]
pub fn is_taskbar_autohide() -> bool {
    false
}

#[cfg(windows)]
pub fn mark_fullscreen_window(hwnd: isize, fullscreen: bool) {
    if hwnd == 0 {
        return;
    }
    type HRESULT = i32;
    type LPVOID = *mut std::ffi::c_void;
    type HWND = *mut std::ffi::c_void;
    type BOOL = i32;

    #[repr(C)]
    struct GUID {
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    }

    const CLSID_TASKBAR_LIST: GUID = GUID {
        data1: 0x56FDF344,
        data2: 0xFD6D,
        data3: 0x11D0,
        data4: [0x95, 0x8A, 0x00, 0x60, 0x97, 0xC9, 0xA0, 0x90],
    };
    const IID_ITASKBAR_LIST3: GUID = GUID {
        data1: 0xEA1AFB91,
        data2: 0x9E28,
        data3: 0x4B86,
        data4: [0x90, 0xE9, 0x9E, 0x9F, 0x8A, 0x5E, 0xEF, 0xAF],
    };

    #[link(name = "ole32")]
    unsafe extern "system" {
        fn CoInitialize(pv_reserved: LPVOID) -> HRESULT;
        fn CoCreateInstance(
            rclsid: *const GUID,
            p_unk_outer: LPVOID,
            dw_cls_context: u32,
            riid: *const GUID,
            ppv: *mut LPVOID,
        ) -> HRESULT;
    }

    unsafe {
        CoInitialize(std::ptr::null_mut());
        let mut p_taskbar: LPVOID = std::ptr::null_mut();
        let hr = CoCreateInstance(
            &CLSID_TASKBAR_LIST,
            std::ptr::null_mut(),
            1, // CLSCTX_INPROC_SERVER
            &IID_ITASKBAR_LIST3,
            &mut p_taskbar,
        );
        if hr == 0 && !p_taskbar.is_null() {
            let vtbl = *(p_taskbar as *const *const usize);
            // vtbl[3] is HrInit
            let hr_init: unsafe extern "system" fn(this: LPVOID) -> HRESULT =
                std::mem::transmute(*vtbl.add(3));
            let _ = hr_init(p_taskbar);
            // vtbl[8] is MarkFullscreenWindow(this, hwnd, fFullscreen)
            let mark_fs: unsafe extern "system" fn(this: LPVOID, hwnd: HWND, f_fullscreen: BOOL) -> HRESULT =
                std::mem::transmute(*vtbl.add(8));
            let _ = mark_fs(p_taskbar, hwnd as HWND, if fullscreen { 1 } else { 0 });
            // vtbl[2] is Release
            let release: unsafe extern "system" fn(this: LPVOID) -> u32 =
                std::mem::transmute(*vtbl.add(2));
            let _ = release(p_taskbar);
        }
    }
}
#[cfg(not(windows))]
pub fn mark_fullscreen_window(_hwnd: isize, _fullscreen: bool) {}
