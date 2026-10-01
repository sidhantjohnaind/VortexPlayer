#![allow(dead_code)]

#[cfg(windows)]
use std::env;

pub struct FileAssociations;

impl FileAssociations {
    pub const EXTENSIONS: &'static [&'static str] = &[
        "mkv", "mp4", "webm", "avi", "mov", "wmv", "flv", "m4v", "ts", "m2ts",
        "mp3", "flac", "wav", "aac", "ogg", "m4a", "opus"
    ];

    #[cfg(windows)]
    pub fn register_associations() -> Result<(), String> {
        let exe_path = env::current_exe().map_err(|e| e.to_string())?;
        let exe_str = exe_path.to_str().ok_or("Invalid exe path")?;

        let prog_id = "VertexPlayer.Media";
        
        // 1. HKCU\Software\Classes\VertexPlayer.Media
        let _ = crate::platform::silent_command("reg")
            .args(&["add", &format!(r"HKCU\Software\Classes\{}", prog_id), "/ve", "/d", "VertexPlayer Media File", "/f"])
            .output();

        // 2. HKCU\Software\Classes\VertexPlayer.Media\shell\open\command
        let cmd_val = format!(r#""{}" "%1""#, exe_str);
        let _ = crate::platform::silent_command("reg")
            .args(&["add", &format!(r"HKCU\Software\Classes\{}\shell\open\command", prog_id), "/ve", "/d", &cmd_val, "/f"])
            .output();

        // 3. Register each file extension in HKCU\Software\Classes
        for ext in Self::EXTENSIONS {
            let _ = crate::platform::silent_command("reg")
                .args(&["add", &format!(r"HKCU\Software\Classes\.{}", ext), "/ve", "/d", prog_id, "/f"])
                .output();
        }

        Ok(())
    }

    #[cfg(not(windows))]
    pub fn register_associations() -> Result<(), String> {
        Ok(())
    }
}
