#![allow(dead_code)]

#[cfg(windows)]
use windows_sys::Win32::Graphics::Gdi::*;

#[derive(Debug, Clone, Copy)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

pub struct DisplaySyncManager {
    original_mode: Option<DisplayMode>,
}

impl Default for DisplaySyncManager {
    fn default() -> Self {
        Self { original_mode: None }
    }
}

impl DisplaySyncManager {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(windows)]
    pub fn get_current_mode(&mut self) -> Option<DisplayMode> {
        unsafe {
            let mut devmode: DEVMODEW = std::mem::zeroed();
            devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

            if EnumDisplaySettingsW(std::ptr::null(), ENUM_CURRENT_SETTINGS, &mut devmode) != 0 {
                let mode = DisplayMode {
                    width: devmode.dmPelsWidth,
                    height: devmode.dmPelsHeight,
                    refresh_rate: devmode.dmDisplayFrequency,
                };
                if self.original_mode.is_none() {
                    self.original_mode = Some(mode);
                }
                return Some(mode);
            }
        }
        None
    }

    #[cfg(windows)]
    pub fn switch_for_fps(&mut self, video_fps: f64) -> bool {
        if video_fps < 10.0 {
            return false;
        }

        // Target refresh rates based on video FPS to eliminate judder
        let target_hz: u32 = if (video_fps - 23.976).abs() < 0.2 || (video_fps - 24.0).abs() < 0.2 {
            // 24fps -> 24Hz, 120Hz, or 144Hz
            120
        } else if (video_fps - 25.0).abs() < 0.2 || (video_fps - 50.0).abs() < 0.2 {
            // 25/50fps -> 50Hz or 100Hz
            100
        } else if (video_fps - 29.97).abs() < 0.2 || (video_fps - 30.0).abs() < 0.2 || (video_fps - 60.0).abs() < 0.2 {
            // 30/60fps -> 60Hz or 120Hz
            120
        } else {
            60
        };

        unsafe {
            let mut devmode: DEVMODEW = std::mem::zeroed();
            devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

            if EnumDisplaySettingsW(std::ptr::null(), ENUM_CURRENT_SETTINGS, &mut devmode) != 0 {
                if self.original_mode.is_none() {
                    self.original_mode = Some(DisplayMode {
                        width: devmode.dmPelsWidth,
                        height: devmode.dmPelsHeight,
                        refresh_rate: devmode.dmDisplayFrequency,
                    });
                }

                if devmode.dmDisplayFrequency != target_hz {
                    devmode.dmDisplayFrequency = target_hz;
                    devmode.dmFields = DM_DISPLAYFREQUENCY;

                    let res = ChangeDisplaySettingsExW(
                        std::ptr::null(),
                        &mut devmode,
                        0 as _,
                        CDS_UPDATEREGISTRY,
                        std::ptr::null(),
                    );
                    return res == DISP_CHANGE_SUCCESSFUL;
                }
            }
        }
        false
    }

    #[cfg(windows)]
    pub fn restore_original(&mut self) -> bool {
        if let Some(orig) = self.original_mode.take() {
            unsafe {
                let mut devmode: DEVMODEW = std::mem::zeroed();
                devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

                if EnumDisplaySettingsW(std::ptr::null(), ENUM_CURRENT_SETTINGS, &mut devmode) != 0 {
                    devmode.dmDisplayFrequency = orig.refresh_rate;
                    devmode.dmFields = DM_DISPLAYFREQUENCY;

                    let res = ChangeDisplaySettingsExW(
                        std::ptr::null(),
                        &mut devmode,
                        0 as _,
                        CDS_UPDATEREGISTRY,
                        std::ptr::null(),
                    );
                    return res == DISP_CHANGE_SUCCESSFUL;
                }
            }
        }
        false
    }

    #[cfg(not(windows))]
    pub fn switch_for_fps(&mut self, _fps: f64) -> bool { false }
    #[cfg(not(windows))]
    pub fn restore_original(&mut self) -> bool { false }
}
