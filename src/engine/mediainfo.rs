use crate::engine::MediaStats;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::thread;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

static MEDIAINFO_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
static PENDING_REQUESTS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn get_cache() -> &'static Mutex<HashMap<String, String>> {
    MEDIAINFO_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn get_pending() -> &'static Mutex<HashSet<String>> {
    PENDING_REQUESTS.get_or_init(|| Mutex::new(HashSet::new()))
}

/// Checks if an asynchronous MediaInfo scan is currently in progress for the specified file path.
pub fn is_mediainfo_pending(file_path: &str) -> bool {
    if file_path.is_empty() {
        return false;
    }
    let pending = get_pending().lock().unwrap_or_else(|e| e.into_inner());
    pending.contains(file_path)
}

/// Locates the `MediaInfo.exe` binary across bundled paths, working directories, and system paths.
pub fn find_mediainfo_exe() -> Option<PathBuf> {
    // 1. Next to current running executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let candidate = parent.join("MediaInfo.exe");
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    // 2. In project root / current working directory
    let cwd_candidate = PathBuf::from("MediaInfo.exe");
    if cwd_candidate.is_file() {
        return Some(cwd_candidate);
    }

    // 3. Known project directory
    let proj_candidate = PathBuf::from(r"d:\Programming\VIDPLAYER-egui\MediaInfo.exe");
    if proj_candidate.is_file() {
        return Some(proj_candidate);
    }

    // 4. In target build output folders
    for target_dir in &["D:\\temp\\rust\\target\\debug\\MediaInfo.exe", "D:\\temp\\rust\\target\\release\\MediaInfo.exe"] {
        let p = PathBuf::from(target_dir);
        if p.is_file() {
            return Some(p);
        }
    }

    // 5. In WinGet installation packages directory
    if let Some(local_app_data) = dirs::data_local_dir() {
        let winget_path = local_app_data.join(r"Microsoft\WinGet\Packages\MediaArea.MediaInfo_Microsoft.Winget.Source_8wekyb3d8bbwe\MediaInfo.exe");
        if winget_path.is_file() {
            return Some(winget_path);
        }
    }

    // 6. In PATH environment variable
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let p1 = dir.join("MediaInfo.exe");
            if p1.is_file() {
                return Some(p1);
            }
            let p2 = dir.join("mediainfo.exe");
            if p2.is_file() {
                return Some(p2);
            }
        }
    }

    None
}

/// Invokes the MediaInfo CLI without displaying any console window on Windows.
pub fn query_mediainfo_cli(exe_path: &Path, file_path: &str) -> Option<String> {
    let mut cmd = Command::new(exe_path);
    cmd.arg(file_path);

    #[cfg(windows)]
    {
        // 0x08000000 = CREATE_NO_WINDOW
        cmd.creation_flags(0x08000000);
    }

    let output = cmd.output().ok()?;
    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout).to_string();
        if !text.trim().is_empty() {
            return Some(text);
        }
    }
    None
}

/// Retrieves the complete MediaInfo string for the given file.
/// If cached, returns instantly. Otherwise, spawns a background fetch thread and provides
/// an immediate synthesized fallback so the UI never stalls.
pub fn get_mediainfo_text(file_path: &str, stats: &MediaStats) -> String {
    if file_path.is_empty() {
        return "No media file currently loaded.\n\nOpen a video or audio file to inspect detailed container and stream metadata.".to_string();
    }

    // Fast path: memory cache hit
    {
        let cache = get_cache().lock().unwrap_or_else(|e| e.into_inner());
        if let Some(cached_text) = cache.get(file_path) {
            return cached_text.clone();
        }
    }

    // If file does not exist on disk (e.g. streaming URL like http://, rtmp://), synthesize directly
    let is_local_file = Path::new(file_path).exists();
    if !is_local_file {
        let synthetic = generate_synthesized_file_info(stats);
        let mut cache = get_cache().lock().unwrap_or_else(|e| e.into_inner());
        cache.insert(file_path.to_string(), synthetic.clone());
        return synthetic;
    }

    // Trigger asynchronous background fetch if not already in flight
    let should_spawn = {
        let mut pending = get_pending().lock().unwrap_or_else(|e| e.into_inner());
        if pending.contains(file_path) {
            false
        } else {
            pending.insert(file_path.to_string());
            true
        }
    };

    if should_spawn {
        let file_path_owned = file_path.to_string();
        let stats_clone = stats.clone();

        thread::spawn(move || {
            let cli_result = if let Some(exe) = find_mediainfo_exe() {
                query_mediainfo_cli(&exe, &file_path_owned)
            } else {
                None
            };

            let final_output = cli_result.unwrap_or_else(|| {
                generate_synthesized_file_info(&stats_clone)
            });

            // Store in cache
            {
                let mut cache = get_cache().lock().unwrap_or_else(|e| e.into_inner());
                // Cap cache to 50 items to avoid unbounded memory growth
                if cache.len() > 50 {
                    cache.clear();
                }
                cache.insert(file_path_owned.clone(), final_output);
            }

            // Remove from pending set
            {
                let mut pending = get_pending().lock().unwrap_or_else(|e| e.into_inner());
                pending.remove(&file_path_owned);
            }
        });
    }

    // Return immediate high-fidelity synthesized info while background MediaInfo CLI finishes
    generate_synthesized_file_info(stats)
}

/// High-fidelity synthesized MediaInfo formatter adhering strictly to the official MediaInfo 31-character column alignment.
pub fn generate_synthesized_file_info(stats: &MediaStats) -> String {
    if stats.file_path.is_empty() {
        return "No media file currently loaded.\n\nOpen a video or audio file to inspect detailed container and stream metadata.".to_string();
    }

    let path = &stats.file_path;
    let p = Path::new(path);
    let _filename = p.file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());

    let ext = p.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let (container_format, format_version) = match ext.as_str() {
        "mkv" => ("Matroska", Some("Version 4")),
        "webm" => ("WebM", None),
        "mp4" | "m4v" => ("MPEG-4", Some("Base Media / ISO 14496-12")),
        "m4a" => ("MPEG-4", Some("Apple iTunes")),
        "mov" => ("QuickTime", None),
        "flac" => ("FLAC", None),
        "mp3" => ("MPEG Audio", Some("Version 1 / Layer 3")),
        "wav" => ("Wave", None),
        "ogg" => ("Ogg", None),
        "opus" => ("Ogg", None),
        "ts" | "m2ts" | "mts" => ("BDAV", Some("MPEG-2 Transport Stream")),
        "avi" => ("AVI", None),
        "wmv" | "wma" => ("Windows Media", None),
        other => (other, None),
    };

    let file_size_bytes = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let file_size_str = if file_size_bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GiB", file_size_bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if file_size_bytes >= 1024 * 1024 {
        format!("{} MiB", (file_size_bytes as f64 / (1024.0 * 1024.0)).round() as u64)
    } else if file_size_bytes >= 1024 {
        format!("{:.1} KiB", file_size_bytes as f64 / 1024.0)
    } else {
        format!("{} bytes", file_size_bytes)
    };

    let duration_secs = stats.duration;
    let hours = (duration_secs / 3600.0) as u64;
    let mins = ((duration_secs % 3600.0) / 60.0) as u64;
    let secs = (duration_secs % 60.0) as u64;
    let dur_str = if hours > 0 {
        format!("{} h {} min {} s", hours, mins, secs)
    } else {
        format!("{} min {} s", mins, secs)
    };

    let overall_kbps = if file_size_bytes > 0 && duration_secs > 0.0 {
        (file_size_bytes as f64 * 8.0 / duration_secs / 1000.0).round() as u64
    } else if stats.video_bitrate + stats.audio_bitrate > 0 {
        ((stats.video_bitrate + stats.audio_bitrate) as f64 / 1000.0).round() as u64
    } else {
        0
    };

    let overall_bitrate_str = if overall_kbps > 0 {
        format!("{} kb/s", format_thousands(overall_kbps))
    } else {
        "Variable".to_string()
    };

    let mut out: Vec<String> = Vec::new();

    // ── GENERAL SECTION ───────────────────────────────────────────────────────
    out.push("General".to_string());
    out.push(format!("{:<31}: {}", "Complete name", path));
    out.push(format!("{:<31}: {}", "Format", container_format));
    if let Some(ver) = format_version {
        out.push(format!("{:<31}: {}", "Format version", ver));
    }
    out.push(format!("{:<31}: {}", "File size", file_size_str));
    out.push(format!("{:<31}: {}", "Duration", dur_str));
    out.push(format!("{:<31}: {}", "Overall bit rate mode", "Variable"));
    out.push(format!("{:<31}: {}", "Overall bit rate", overall_bitrate_str));
    if stats.video_fps > 0.0 {
        out.push(format!("{:<31}: {:.3} FPS", "Frame rate", stats.video_fps));
    }
    if !stats.title.is_empty() {
        out.push(format!("{:<31}: {}", "Title", stats.title));
    }

    // ── VIDEO SECTION ─────────────────────────────────────────────────────────
    if stats.video_width > 0 || !stats.video_codec.is_empty() {
        out.push(String::new());
        out.push("Video".to_string());
        out.push(format!("{:<31}: 1", "ID"));

        let raw_codec = if !stats.video_codec.is_empty() {
            &stats.video_codec
        } else {
            &stats.video_format
        };
        let lower = raw_codec.to_lowercase();
        let (fmt_name, fmt_info, codec_id) = if lower.contains("hevc") || lower.contains("h265") {
            ("HEVC", "High Efficiency Video Coding", "V_MPEGH/ISO/HEVC")
        } else if lower.contains("avc") || lower.contains("h264") {
            ("AVC", "Advanced Video Codec", "V_MPEG4/ISO/AVC")
        } else if lower.contains("av1") {
            ("AV1", "AOMedia Video 1", "V_AV1")
        } else if lower.contains("vp9") {
            ("VP9", "Google VP9", "V_VP9")
        } else {
            (raw_codec.as_str(), raw_codec.as_str(), "V_CUSTOM")
        };

        out.push(format!("{:<31}: {}", "Format", fmt_name));
        out.push(format!("{:<31}: {}", "Format/Info", fmt_info));

        let bit_depth = if stats.video_bit_depth > 0 { stats.video_bit_depth } else { 10 };
        if fmt_name == "HEVC" {
            out.push(format!("{:<31}: Main {}@L5@High", "Format profile", bit_depth));
        } else if fmt_name == "AVC" {
            out.push(format!("{:<31}: High@L4.1", "Format profile"));
        }

        out.push(format!("{:<31}: {}", "Codec ID", codec_id));
        out.push(format!("{:<31}: {}", "Duration", dur_str));

        if stats.video_bitrate > 0 {
            let v_kbps = (stats.video_bitrate as f64 / 1000.0).round() as u64;
            out.push(format!("{:<31}: {} kb/s", "Bit rate", format_thousands(v_kbps)));
        }

        out.push(format!("{:<31}: {} pixels", "Width", format_thousands(stats.video_width as u64)));
        out.push(format!("{:<31}: {} pixels", "Height", format_thousands(stats.video_height as u64)));

        let dar = if !stats.aspect_ratio.is_empty() && stats.aspect_ratio != "auto" {
            stats.aspect_ratio.clone()
        } else if stats.video_height > 0 {
            let r = stats.video_width as f64 / stats.video_height as f64;
            if (r - 16.0 / 9.0).abs() < 0.05 { "16:9".to_string() }
            else if (r - 4.0 / 3.0).abs() < 0.05 { "4:3".to_string() }
            else { format!("{:.2}:1", r) }
        } else {
            "16:9".to_string()
        };
        out.push(format!("{:<31}: {}", "Display aspect ratio", dar));
        out.push(format!("{:<31}: Constant", "Frame rate mode"));
        out.push(format!("{:<31}: {:.3} FPS", "Frame rate", stats.video_fps));
        out.push(format!("{:<31}: YUV", "Color space"));
        out.push(format!("{:<31}: 4:2:0", "Chroma subsampling"));
        out.push(format!("{:<31}: {} bits", "Bit depth", bit_depth));

        if stats.video_width > 0 && stats.video_height > 0 && stats.video_fps > 0.0 && stats.video_bitrate > 0 {
            let bpp = stats.video_bitrate as f64 / (stats.video_width as f64 * stats.video_height as f64 * stats.video_fps);
            out.push(format!("{:<31}: {:.3}", "Bits/(Pixel*Frame)", bpp));
        }

        if stats.video_bitrate > 0 && duration_secs > 0.0 {
            let v_bytes = (stats.video_bitrate as f64 * duration_secs / 8.0) as u64;
            let v_mib = v_bytes / (1024 * 1024);
            let pct = if file_size_bytes > 0 { (v_bytes as f64 / file_size_bytes as f64 * 100.0).round() as u64 } else { 55 };
            out.push(format!("{:<31}: {} MiB ({}%)", "Stream size", v_mib, pct));
        }

        out.push(format!("{:<31}: Yes", "Default"));
        out.push(format!("{:<31}: No", "Forced"));

        let range = if !stats.colorlevels.is_empty() { &stats.colorlevels } else { "Limited" };
        let prim = if !stats.primaries.is_empty() { &stats.primaries } else { "BT.709" };
        let trans = if !stats.gamma.is_empty() { &stats.gamma } else { "BT.709" };
        let mat = if !stats.colormatrix.is_empty() { &stats.colormatrix } else { "BT.709" };

        out.push(format!("{:<31}: {}", "Color range", range));
        out.push(format!("{:<31}: {}", "Color primaries", prim));
        out.push(format!("{:<31}: {}", "Transfer characteristics", trans));
        out.push(format!("{:<31}: {}", "Matrix coefficients", mat));
    }

    // ── AUDIO SECTIONS ────────────────────────────────────────────────────────
    if !stats.audio_tracks.is_empty() {
        for (idx, track) in stats.audio_tracks.iter().enumerate() {
            out.push(String::new());
            out.push(format!("Audio #{}", idx + 1));
            out.push(format!("{:<31}: {}", "ID", track.id));

            let c_raw = if !track.codec.is_empty() { &track.codec } else { &stats.audio_codec };
            let c_upper = c_raw.to_uppercase();
            let (fmt_name, fmt_info, codec_id) = if c_upper.contains("FLAC") {
                ("FLAC", "Free Lossless Audio Codec", "A_FLAC")
            } else if c_upper.contains("AAC") {
                ("AAC", "Advanced Audio Coding", "A_AAC-2")
            } else if c_upper.contains("OPUS") {
                ("Opus", "Opus Interactive Audio Codec", "A_OPUS")
            } else if c_upper.contains("AC3") || c_upper.contains("EAC3") {
                ("E-AC-3", "Enhanced AC-3", "A_EAC3")
            } else if c_upper.contains("DTS") {
                ("DTS", "Digital Theater Systems", "A_DTS")
            } else if c_upper.contains("TRUEHD") {
                ("TrueHD", "Dolby TrueHD", "A_TRUEHD")
            } else {
                (c_upper.as_str(), c_upper.as_str(), "A_CUSTOM")
            };

            out.push(format!("{:<31}: {}", "Format", fmt_name));
            out.push(format!("{:<31}: {}", "Format/Info", fmt_info));
            out.push(format!("{:<31}: {}", "Codec ID", codec_id));
            out.push(format!("{:<31}: {}", "Duration", dur_str));
            out.push(format!("{:<31}: Variable", "Bit rate mode"));

            let br = if track.bitrate > 0 { track.bitrate } else { 1_000_000 };
            let br_kbps = (br as f64 / 1000.0).round() as u64;
            out.push(format!("{:<31}: {} kb/s", "Bit rate", format_thousands(br_kbps)));

            let ch_count = if track.channel_count > 0 {
                track.channel_count
            } else if track.channels.contains("5.1") {
                6
            } else if track.channels.contains("7.1") {
                8
            } else if track.channels.contains("2.0") || track.channels.contains("stereo") {
                2
            } else if stats.audio_channels > 0 {
                stats.audio_channels
            } else {
                2
            };

            out.push(format!("{:<31}: {} channels", "Channel(s)", ch_count));

            let ch_layout = match ch_count {
                1 => "C",
                2 => "L R",
                6 => "L R C LFE Ls Rs",
                8 => "L R C LFE Ls Rs Lb Rb",
                _ => "L R",
            };
            out.push(format!("{:<31}: {}", "Channel layout", ch_layout));

            let sr = if track.sample_rate > 0 { track.sample_rate } else { 48000 };
            out.push(format!("{:<31}: {:.1} kHz", "Sampling rate", sr as f64 / 1000.0));

            let bd = if track.bit_depth > 0 { track.bit_depth } else { 16 };
            out.push(format!("{:<31}: {} bits", "Bit depth", bd));

            let is_lossless = fmt_name == "FLAC" || fmt_name == "TrueHD" || fmt_name == "Wave" || fmt_name.contains("PCM");
            out.push(format!("{:<31}: {}", "Compression mode", if is_lossless { "Lossless" } else { "Lossy" }));

            if track.byte_count > 0 {
                let mib = track.byte_count / (1024 * 1024);
                let pct = if file_size_bytes > 0 { (track.byte_count as f64 / file_size_bytes as f64 * 100.0).round() as u64 } else { 20 };
                out.push(format!("{:<31}: {} MiB ({}%)", "Stream size", mib, pct));
            }

            if !track.title.is_empty() {
                out.push(format!("{:<31}: {}", "Title", track.title));
            }
            if !track.lang.is_empty() {
                out.push(format!("{:<31}: {}", "Language", track.lang));
            }

            out.push(format!("{:<31}: {}", "Default", if track.is_default { "Yes" } else { "No" }));
            out.push(format!("{:<31}: No", "Forced"));
        }
    }

    // ── TEXT (SUBTITLE) SECTIONS ──────────────────────────────────────────────
    if !stats.subtitle_tracks.is_empty() {
        for (idx, track) in stats.subtitle_tracks.iter().enumerate() {
            out.push(String::new());
            out.push(format!("Text #{}", idx + 1));
            out.push(format!("{:<31}: {}", "ID", track.id));

            let c_upper = track.codec.to_uppercase();
            let (fmt_name, codec_id, codec_info) = if c_upper.contains("ASS") || c_upper.contains("SSA") {
                ("ASS", "S_TEXT/ASS", "Advanced Sub Station Alpha")
            } else if c_upper.contains("SRT") || c_upper.contains("SUBRIP") {
                ("SubRip", "S_TEXT/UTF8", "SubRip Subtitle")
            } else if c_upper.contains("PGS") || c_upper.contains("HDMV") {
                ("PGS", "S_HDMV/PGS", "Presentation Graphic Stream")
            } else if c_upper.contains("VTT") {
                ("WebVTT", "S_TEXT/WEBVTT", "Web Video Text Tracks")
            } else {
                ("ASS", "S_TEXT/ASS", "Advanced Sub Station Alpha")
            };

            out.push(format!("{:<31}: {}", "Format", fmt_name));
            out.push(format!("{:<31}: {}", "Codec ID", codec_id));
            out.push(format!("{:<31}: {}", "Codec ID/Info", codec_info));
            out.push(format!("{:<31}: {}", "Duration", dur_str));

            if track.bitrate > 0 {
                out.push(format!("{:<31}: {} b/s", "Bit rate", track.bitrate));
            }
            if track.element_count > 0 {
                out.push(format!("{:<31}: {}", "Count of elements", track.element_count));
            }

            out.push(format!("{:<31}: Lossless", "Compression mode"));

            if track.byte_count > 0 {
                let mib = track.byte_count as f64 / (1024.0 * 1024.0);
                out.push(format!("{:<31}: {:.2} MiB (0%)", "Stream size", mib));
            }

            if !track.title.is_empty() {
                out.push(format!("{:<31}: {}", "Title", track.title));
            }
            if !track.lang.is_empty() {
                out.push(format!("{:<31}: {}", "Language", track.lang));
            }

            out.push(format!("{:<31}: {}", "Default", if track.is_default { "Yes" } else { "No" }));
            out.push(format!("{:<31}: No", "Forced"));
        }
    }

    // ── CHAPTERS / MENU SECTION ───────────────────────────────────────────────
    if !stats.chapters.is_empty() {
        out.push(String::new());
        out.push("Menu".to_string());
        for ch in &stats.chapters {
            let total_sec = ch.time_pos;
            let ch_h = (total_sec / 3600.0) as u64;
            let ch_m = ((total_sec % 3600.0) / 60.0) as u64;
            let ch_s = (total_sec % 60.0) as u64;
            let ch_ms = ((total_sec.fract()) * 1000.0) as u64;
            let time_stamp = format!("{:02}:{:02}:{:02}.{:03}", ch_h, ch_m, ch_s, ch_ms);
            out.push(format!("{:<31}: {}", time_stamp, ch.title));
        }
    }

    out.join("\n")
}

fn format_thousands(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(' ');
        }
        result.push(c);
    }
    result
}
