fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rerun-if-changed=assets/icon.ico");
        println!("cargo:rerun-if-changed=build.rs");

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let icon_path = std::path::Path::new(&manifest_dir).join("assets").join("icon.ico");

        let manifest_path = std::path::Path::new(&manifest_dir).join("assets").join("vortex.manifest");
        if icon_path.exists() {
            let out_dir = std::env::var("OUT_DIR").unwrap();
            let icon_str = icon_path.to_string_lossy().replace('\\', "/");
            let manifest_line = if manifest_path.exists() {
                let m_str = manifest_path.to_string_lossy().replace('\\', "/");
                format!("1 24 \"{}\"\r\n", m_str)
            } else {
                String::new()
            };

            let rc_content = format!(
                "1 ICON \"{}\"\r\n{}1 VERSIONINFO\r\nFILEVERSION 1,1,0,0\r\nPRODUCTVERSION 1,1,0,0\r\nBEGIN\r\n  BLOCK \"StringFileInfo\"\r\n  BEGIN\r\n    BLOCK \"040904B0\"\r\n    BEGIN\r\n      VALUE \"CompanyName\", \"Vortex Media\\0\"\r\n      VALUE \"FileDescription\", \"VortexPlayer - Modern Media Player\\0\"\r\n      VALUE \"FileVersion\", \"1.1.0.0\\0\"\r\n      VALUE \"InternalName\", \"VortexPlayer\\0\"\r\n      VALUE \"OriginalFilename\", \"vortex-player-egui.exe\\0\"\r\n      VALUE \"ProductName\", \"VortexPlayer\\0\"\r\n      VALUE \"ProductVersion\", \"1.1.0\\0\"\r\n    END\r\n  END\r\n  BLOCK \"VarFileInfo\"\r\n  BEGIN\r\n    VALUE \"Translation\", 0x0409, 0x04B0\r\n  END\r\nEND\r\n",
                icon_str, manifest_line
            );

            let rc_file = std::path::Path::new(&out_dir).join("vortex_res.rc");
            let res_file = std::path::Path::new(&out_dir).join("vortex_res.res");
            let _ = std::fs::write(&rc_file, rc_content);

            let rc_exe_candidates = [
                r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\rc.exe",
                r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64\rc.exe",
                r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.22000.0\x64\rc.exe",
                r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64\rc.exe",
                "rc.exe",
            ];

            let mut compiled = false;
            for rc_exe in &rc_exe_candidates {
                let status = std::process::Command::new(rc_exe)
                    .arg("/nologo")
                    .arg(format!("/fo{}", res_file.display()))
                    .arg(&rc_file)
                    .status();

                if let Ok(s) = status {
                    if s.success() && res_file.exists() {
                        println!("cargo:rustc-link-arg={}", res_file.display());
                        compiled = true;
                        break;
                    }
                }
            }

            if !compiled {
                let mut res = winres::WindowsResource::new();
                res.set_icon(&icon_str);
                res.set("ProductName", "VertexPlayer");
                res.set("FileDescription", "VertexPlayer");
                let _ = res.compile();
            }
        }
    }
}
