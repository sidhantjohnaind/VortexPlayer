//! win32_tooltip.rs — Native Win32 WS_EX_TOPMOST Floating Tooltip Window
//! Exact Vortex architecture: creates an independent OS popup HWND that sits on top
//! of the video window (child HWND) without any occlusion or clipping.

#![allow(dead_code)]

#[cfg(windows)]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(windows)]
use std::sync::Mutex;
#[cfg(windows)]
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
#[cfg(windows)]
use windows_sys::Win32::Graphics::Gdi::*;
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::*;

#[cfg(windows)]
static CLASS_REGISTERED: AtomicBool = AtomicBool::new(false);
#[cfg(windows)]
static TOOLTIP_TEXT: Mutex<String> = Mutex::new(String::new());

#[cfg(windows)]
unsafe extern "system" fn tooltip_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_ERASEBKGND => 1,
            WM_PAINT => {
                let mut ps = std::mem::zeroed::<PAINTSTRUCT>();
                let hdc = BeginPaint(hwnd, &mut ps);
                if !hdc.is_null() {
                    let mut client_rc = std::mem::zeroed::<RECT>();
                    GetClientRect(hwnd, &mut client_rc);

                    let w = client_rc.right - client_rc.left;
                    let h = client_rc.bottom - client_rc.top;
                    if w <= 0 || h <= 0 {
                        EndPaint(hwnd, &ps);
                        return 0;
                    }

                    // 1. Double buffering
                    let mem_dc = CreateCompatibleDC(hdc);
                    if mem_dc.is_null() {
                        EndPaint(hwnd, &ps);
                        return 0;
                    }
                    let mem_bm = CreateCompatibleBitmap(hdc, w, h);
                    if mem_bm.is_null() {
                        DeleteDC(mem_dc);
                        EndPaint(hwnd, &ps);
                        return 0;
                    }
                    let old_bm = SelectObject(mem_dc, mem_bm);

                    // 2. Draw sleek dark tooltip background with subtle border
                    // Win32 GDI COLORREF: 0x00BBGGRR
                    let bg_brush = CreateSolidBrush(0x001A1412); // Dark slate #12141A (RGB: 18, 20, 26)
                    let border_pen = CreatePen(PS_SOLID as i32, 1, 0x00443632); // Dark slate border #323644 (RGB: 50, 54, 68)
                    let old_brush = SelectObject(mem_dc, bg_brush);
                    let old_pen = SelectObject(mem_dc, border_pen);

                    RoundRect(mem_dc, 0, 0, w, h, 6, 6);

                    // 3. Draw Text
                    SetBkMode(mem_dc, TRANSPARENT as i32);
                    SetTextColor(mem_dc, 0x00F8F2F0); // Crisp light text #F0F2F8 (RGB: 240, 242, 248)

                    let font_name: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
                    let font = CreateFontW(
                        -12, 0, 0, 0,
                        FW_NORMAL as i32,
                        0, 0, 0,
                        DEFAULT_CHARSET as u32,
                        OUT_DEFAULT_PRECIS as u32,
                        CLIP_DEFAULT_PRECIS as u32,
                        CLEARTYPE_QUALITY as u32,
                        DEFAULT_PITCH as u32,
                        font_name.as_ptr(),
                    );
                    let old_font = SelectObject(mem_dc, font);

                    let wide_text: Vec<u16> = {
                        let guard = TOOLTIP_TEXT.lock().unwrap_or_else(|e| e.into_inner());
                        guard.encode_utf16().collect()
                    };

                    let mut text_rc = RECT {
                        left: 8,
                        top: 4,
                        right: w - 8,
                        bottom: h - 4,
                    };
                    DrawTextW(
                        mem_dc,
                        wide_text.as_ptr(),
                        wide_text.len() as i32,
                        &mut text_rc,
                        DT_LEFT as u32 | DT_NOPREFIX as u32,
                    );

                    // 4. BitBlt to screen
                    BitBlt(hdc, 0, 0, w, h, mem_dc, 0, 0, SRCCOPY);

                    // Cleanup
                    SelectObject(mem_dc, old_font);
                    DeleteObject(font);
                    SelectObject(mem_dc, old_brush);
                    DeleteObject(bg_brush);
                    SelectObject(mem_dc, old_pen);
                    DeleteObject(border_pen);
                    SelectObject(mem_dc, old_bm);
                    DeleteObject(mem_bm);
                    DeleteDC(mem_dc);

                    EndPaint(hwnd, &ps);
                }
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

pub struct Win32TooltipEngine {
    #[cfg(windows)]
    hwnd: HWND,
    #[cfg(windows)]
    last_text: String,
}

impl Win32TooltipEngine {
    pub fn new() -> Self {
        #[cfg(windows)]
        {
            let mut engine = Self {
                hwnd: std::ptr::null_mut(),
                last_text: String::new(),
            };
            engine.ensure_created();
            engine
        }
        #[cfg(not(windows))]
        {
            Self {}
        }
    }

    #[cfg(windows)]
    fn ensure_created(&mut self) {
        if !self.hwnd.is_null() {
            return;
        }

        unsafe {
            let class_name: Vec<u16> = "Vortex_Win32TooltipClass\0".encode_utf16().collect();
            if !CLASS_REGISTERED.load(Ordering::Relaxed) {
                let hinstance = windows_sys::Win32::System::LibraryLoader::GetModuleHandleW(std::ptr::null());
                let mut wc = std::mem::zeroed::<WNDCLASSEXW>();
                wc.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
                wc.style = CS_DROPSHADOW | CS_HREDRAW | CS_VREDRAW;
                wc.lpfnWndProc = Some(tooltip_wndproc);
                wc.hInstance = hinstance;
                wc.hCursor = LoadCursorW(std::ptr::null_mut(), IDC_ARROW);
                wc.lpszClassName = class_name.as_ptr();

                RegisterClassExW(&wc);
                CLASS_REGISTERED.store(true, Ordering::Relaxed);
            }

            let hwnd = CreateWindowExW(
                WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
                class_name.as_ptr(),
                std::ptr::null(),
                WS_POPUP,
                0, 0, 10, 10,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            );

            self.hwnd = hwnd;
        }
    }

    pub fn show(&mut self, _parent_hwnd: isize, _text: &str, _client_x: f32, _client_y: f32) {
        #[cfg(windows)]
        unsafe {
            self.ensure_created();
            if self.hwnd.is_null() {
                return;
            }

            // Convert client coordinate to screen coordinate
            let mut pt = POINT {
                x: _client_x as i32,
                y: _client_y as i32,
            };
            if _parent_hwnd != 0 {
                ClientToScreen(_parent_hwnd as HWND, &mut pt);
            }

            {
                let mut guard = TOOLTIP_TEXT.lock().unwrap_or_else(|e| e.into_inner());
                *guard = _text.to_string();
            }
            self.last_text = _text.to_string();

            // Calculate exact text dimensions
            let hdc = GetDC(self.hwnd);
            if !hdc.is_null() {
                let font_name: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
                let font = CreateFontW(
                    -12, 0, 0, 0,
                    FW_NORMAL as i32,
                    0, 0, 0,
                    DEFAULT_CHARSET as u32,
                    OUT_DEFAULT_PRECIS as u32,
                    CLIP_DEFAULT_PRECIS as u32,
                    CLEARTYPE_QUALITY as u32,
                    DEFAULT_PITCH as u32,
                    font_name.as_ptr(),
                );
                let old_font = SelectObject(hdc, font);

                let wide_text: Vec<u16> = {
                    let guard = TOOLTIP_TEXT.lock().unwrap_or_else(|e| e.into_inner());
                    guard.encode_utf16().collect()
                };
                let mut calc_rc = RECT { left: 0, top: 0, right: 400, bottom: 0 };
                DrawTextW(
                    hdc,
                    wide_text.as_ptr(),
                    wide_text.len() as i32,
                    &mut calc_rc,
                    DT_CALCRECT as u32 | DT_LEFT as u32 | DT_NOPREFIX as u32,
                );

                SelectObject(hdc, old_font);
                DeleteObject(font);
                ReleaseDC(self.hwnd, hdc);

                let tip_w = (calc_rc.right - calc_rc.left + 16).max(40);
                let tip_h = (calc_rc.bottom - calc_rc.top + 8).max(22);

                let tip_x = pt.x - tip_w / 2;
                let tip_y = pt.y - tip_h - 4;

                SetWindowPos(
                    self.hwnd,
                    -1 as _, // HWND_TOPMOST
                    tip_x,
                    tip_y,
                    tip_w,
                    tip_h,
                    SWP_SHOWWINDOW | SWP_NOACTIVATE,
                );
                InvalidateRect(self.hwnd, std::ptr::null(), 1);
            }
        }
    }

    pub fn hide(&mut self) {
        #[cfg(windows)]
        unsafe {
            if !self.hwnd.is_null() {
                ShowWindow(self.hwnd, SW_HIDE);
            }
        }
    }
}

impl Drop for Win32TooltipEngine {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            if !self.hwnd.is_null() {
                DestroyWindow(self.hwnd);
                self.hwnd = std::ptr::null_mut();
            }
        }
    }
}
