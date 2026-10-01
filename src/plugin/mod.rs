#![allow(dead_code)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

pub trait VertexPlugin: Send + Sync {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn on_init(&mut self) -> Result<(), String>;
    fn on_file_loaded(&mut self, _path: &Path) {}
    fn on_shutdown(&mut self) {}
}

pub struct DynamicLibraryPlugin {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    lib: libloading::Library,
}

impl DynamicLibraryPlugin {
    pub unsafe fn load(path: PathBuf) -> Result<Self, String> {
        let lib = unsafe { libloading::Library::new(&path) }.map_err(|e| format!("Failed to load dll: {}", e))?;

        let name = if let Ok(name_fn) = unsafe { lib.get::<unsafe extern "C" fn() -> *const c_char>(b"vertex_plugin_name\0") } {
            let ptr = unsafe { name_fn() };
            if !ptr.is_null() {
                unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string()
            } else {
                path.file_stem().unwrap_or_default().to_string_lossy().to_string()
            }
        } else {
            path.file_stem().unwrap_or_default().to_string_lossy().to_string()
        };

        let version = if let Ok(ver_fn) = unsafe { lib.get::<unsafe extern "C" fn() -> *const c_char>(b"vertex_plugin_version\0") } {
            let ptr = unsafe { ver_fn() };
            if !ptr.is_null() {
                unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string()
            } else {
                "1.0.0".to_string()
            }
        } else {
            "1.0.0".to_string()
        };

        // Call init if present
        if let Ok(init_fn) = unsafe { lib.get::<unsafe extern "C" fn() -> i32>(b"vertex_plugin_init\0") } {
            let res = unsafe { init_fn() };
            if res != 0 {
                return Err(format!("Plugin init returned error code: {}", res));
            }
        }

        Ok(Self {
            name,
            version,
            path,
            lib,
        })
    }

    pub fn notify_file_loaded(&self, file_path: &Path) {
        if let Ok(file_fn) = unsafe { self.lib.get::<unsafe extern "C" fn(*const c_char)>(b"vertex_plugin_on_file_loaded\0") } {
            if let Ok(c_str) = CString::new(file_path.to_string_lossy().as_bytes()) {
                unsafe { file_fn(c_str.as_ptr()); }
            }
        }
    }
}

impl Drop for DynamicLibraryPlugin {
    fn drop(&mut self) {
        if let Ok(shutdown_fn) = unsafe { self.lib.get::<unsafe extern "C" fn()>(b"vertex_plugin_shutdown\0") } {
            unsafe { shutdown_fn(); }
        }
    }
}

pub struct PluginRegistry {
    pub plugins: Vec<DynamicLibraryPlugin>,
    pub plugin_dir: PathBuf,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        let p_dir = base.join("VertexPlayer").join("plugins");
        let _ = std::fs::create_dir_all(&p_dir);
        Self {
            plugins: Vec::new(),
            plugin_dir: p_dir,
        }
    }
}

impl PluginRegistry {
    pub fn new() -> Self {
        let mut reg = Self::default();
        reg.load_installed_plugins();
        reg
    }

    pub fn load_installed_plugins(&mut self) {
        if let Ok(entries) = std::fs::read_dir(&self.plugin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("dll") {
                    unsafe {
                        match DynamicLibraryPlugin::load(path.clone()) {
                            Ok(plugin) => {
                                eprintln!("Loaded plugin: {} v{} from {:?}", plugin.name, plugin.version, path);
                                self.plugins.push(plugin);
                            }
                            Err(e) => {
                                eprintln!("Failed to load plugin at {:?}: {}", path, e);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn broadcast_file_loaded(&self, path: &Path) {
        for plugin in &self.plugins {
            plugin.notify_file_loaded(path);
        }
    }
}
