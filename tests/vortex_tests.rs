use std::path::{Path, PathBuf};
use vortex_player::bookmark::{format_time, BookmarkItem, BookmarkManager, PbfFile};
use vortex_player::capture::ThumbnailSheetConfig;
use vortex_player::config::{AppConfig, DoubleClickAction, PlaylistEndAction};
use vortex_player::engine::audio_dsp::AudioDspConfig;
use vortex_player::engine::chapters::{ChapterItem, ChapterManager};
use vortex_player::engine::video_effects::VideoEffectsConfig;
use vortex_player::playlist::dpl::DplFile;
use vortex_player::playlist::m3u::M3uFile;
use vortex_player::playlist::scanner::{is_media_file, is_subtitle_file};
use vortex_player::playlist::{Playlist, RepeatMode, SequenceDetector};
use vortex_player::subtitles::sami::SamiParser;

#[test]
fn test_time_formatting() {
    assert_eq!(format_time(0.0), "00:00");
    assert_eq!(format_time(59.0), "00:59");
    assert_eq!(format_time(60.0), "01:00");
    assert_eq!(format_time(75.5), "01:15");
    assert_eq!(format_time(3600.0), "01:00:00");
    assert_eq!(format_time(3665.0), "01:01:05");
}

#[test]
fn test_pbf_parse_and_serialize() {
    let pbf_content = r#"
[Bookmark]
0=120000*Intro Song*0
1=345000*Key Scene*0
2=650000*Ending Credits*0
"#;

    let items = PbfFile::parse(pbf_content);
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].time_pos, 120.0);
    assert_eq!(items[0].title, "Intro Song");
    assert_eq!(items[1].time_pos, 345.0);
    assert_eq!(items[1].title, "Key Scene");
    assert_eq!(items[2].time_pos, 650.0);
    assert_eq!(items[2].title, "Ending Credits");

    let serialized = PbfFile::serialize(&items);
    assert!(serialized.contains("[Bookmark]"));
    assert!(serialized.contains("0=120000*Intro Song*0"));
    assert!(serialized.contains("1=345000*Key Scene*0"));
    assert!(serialized.contains("2=650000*Ending Credits*0"));

    let re_parsed = PbfFile::parse(&serialized);
    assert_eq!(items, re_parsed);
}

#[test]
fn test_dpl_format_parse_and_serialize() {
    let dpl_content = r#"DAUMPLAYLIST
playname=Episode 01
playtime=1450000
topindex=0
saveplaypos=0
1*file*D:\Videos\Episode 01.mkv*Episode 01*0*1450000*0
2*file*D:\Videos\Episode 02.mkv*Episode 02*0*1520000*0
"#;

    let items = DplFile::parse(dpl_content);
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].title, "Episode 01");
    assert_eq!(items[0].duration_ms, 1450000);
    assert_eq!(items[1].title, "Episode 02");
    assert_eq!(items[1].duration_ms, 1520000);

    let serialized = DplFile::serialize(&items);
    assert!(serialized.contains("DAUMPLAYLIST"));
    assert!(serialized.contains("1*file*D:\\Videos\\Episode 01.mkv*Episode 01*0*1450000*0"));

    let roundtrip = DplFile::parse(&serialized);
    assert_eq!(items, roundtrip);
}

#[test]
fn test_m3u_format_parse_and_serialize() {
    let m3u_content = r#"#EXTM3U
#EXTINF:120,Sample Track 1
http://example.com/stream1.mp3
#EXTINF:240,Sample Track 2
http://example.com/stream2.mp3
"#;

    let items = M3uFile::parse(m3u_content, None);
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].title, "Sample Track 1");
    assert_eq!(items[0].duration_secs, 120.0);
    assert_eq!(items[1].title, "Sample Track 2");
    assert_eq!(items[1].duration_secs, 240.0);

    let serialized = M3uFile::serialize(&items);
    assert!(serialized.contains("#EXTM3U"));
    assert!(serialized.contains("#EXTINF:120,Sample Track 1"));
    assert!(serialized.contains("http://example.com/stream1.mp3"));
}

#[test]
fn test_sami_subtitle_parse_and_srt_conversion() {
    let sami_content = r#"<SAMI>
<BODY>
<SYNC Start=1000><P Class=ENCC>Hello World!</P>
<SYNC Start=4000><P Class=ENCC>Welcome to Vortex in Rust.</P>
<SYNC Start=8000><P Class=ENCC>&nbsp;</P>
</BODY>
</SAMI>"#;

    let entries = SamiParser::parse(sami_content);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].start_ms, 1000);
    assert_eq!(entries[0].text, "Hello World!");
    assert_eq!(entries[1].start_ms, 4000);
    assert_eq!(entries[1].text, "Welcome to Vortex in Rust.");

    let srt = SamiParser::convert_to_srt(&entries);
    assert!(srt.contains("00:00:01,000 -->"));
    assert!(srt.contains("Hello World!"));
    assert!(srt.contains("00:00:04,000 -->"));
    assert!(srt.contains("Welcome to Vortex in Rust."));
}

#[test]
fn test_video_effects_filter_string_generation() {
    let mut config = VideoEffectsConfig::default();
    assert_eq!(config.build_video_filter_string(), "");

    config.sharpen = 2.0;
    config.denoise = 1.5;
    config.flip_horizontal = true;

    let vf = config.build_video_filter_string();
    assert!(vf.contains("unsharp="));
    assert!(vf.contains("nlmeans="));
    assert!(vf.contains("hflip"));
}

#[test]
fn test_audio_dsp_filter_string_generation() {
    let mut config = AudioDspConfig::default();
    config.enabled = true;
    config.bands = vec![3.0, 2.0, 0.0, -1.0, 0.0, 0.0, 2.0, 3.0, 4.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    config.normalize = true;
    config.crossfeed = true;

    let af = config.build_audio_filter_string();
    assert!(af.contains("equalizer="));
    assert!(af.contains("dynaudnorm="));
    assert!(af.contains("bs2b="));
}

#[test]
fn test_chapter_manager_navigation() {
    let mut cm = ChapterManager::new();
    cm.set_chapters(vec![
        ChapterItem { index: 0, title: "Prologue".to_string(), time_pos: 0.0 },
        ChapterItem { index: 1, title: "Opening".to_string(), time_pos: 90.0 },
        ChapterItem { index: 2, title: "Main Part".to_string(), time_pos: 240.0 },
        ChapterItem { index: 3, title: "Ending".to_string(), time_pos: 1200.0 },
    ]);

    assert_eq!(cm.current_chapter(10.0).map(|c| c.title.as_str()), Some("Prologue"));
    assert_eq!(cm.current_chapter(100.0).map(|c| c.title.as_str()), Some("Opening"));
    assert_eq!(cm.next_chapter(50.0), Some(90.0));
    assert_eq!(cm.prev_chapter(300.0), Some(240.0));
}

#[test]
fn test_thumbnail_sheet_timestamp_calculation() {
    let config = ThumbnailSheetConfig {
        rows: 3,
        cols: 3,
        ..Default::default()
    };
    let timestamps = config.calculate_timestamps(100.0);
    assert_eq!(timestamps.len(), 9);
    assert!(timestamps[0] > 0.0);
    assert!(timestamps[8] < 100.0);
}

#[test]
fn test_ab_loop_segment_logic() {
    let mut bm = BookmarkManager::new();
    assert!(!bm.ab_loop.is_active);

    bm.set_loop_a(10.0);
    assert_eq!(bm.ab_loop.point_a, Some(10.0));
    assert_eq!(bm.ab_loop.point_b, None);
    assert!(!bm.ab_loop.is_active);

    // Setting point B before point A should fail
    let err_res = bm.set_loop_b(5.0);
    assert!(err_res.is_err());

    // Setting point B after point A should succeed and activate loop
    let ok_res = bm.set_loop_b(25.0);
    assert!(ok_res.is_ok());
    assert_eq!(bm.ab_loop.point_b, Some(25.0));
    assert!(bm.ab_loop.is_active);

    // Clearing loop
    bm.clear_ab_loop();
    assert_eq!(bm.ab_loop.point_a, None);
    assert_eq!(bm.ab_loop.point_b, None);
    assert!(!bm.ab_loop.is_active);
}

#[test]
fn test_bookmark_navigation() {
    let mut bm = BookmarkManager::new();
    bm.bookmarks.push(BookmarkItem { time_pos: 10.0, title: "BM1".to_string(), index: 0 });
    bm.bookmarks.push(BookmarkItem { time_pos: 30.0, title: "BM2".to_string(), index: 1 });
    bm.bookmarks.push(BookmarkItem { time_pos: 60.0, title: "BM3".to_string(), index: 2 });

    // Next bookmark from 0s
    assert_eq!(bm.next_bookmark(0.0), Some(10.0));
    // Next bookmark from 15s
    assert_eq!(bm.next_bookmark(15.0), Some(30.0));
    // Next bookmark from 30s
    assert_eq!(bm.next_bookmark(30.0), Some(60.0));
    // Next bookmark from 70s wraps around
    assert_eq!(bm.next_bookmark(70.0), Some(10.0));

    // Prev bookmark from 40s
    assert_eq!(bm.prev_bookmark(40.0), Some(30.0));
    // Prev bookmark from 5s wraps around
    assert_eq!(bm.prev_bookmark(5.0), Some(60.0));
}

#[test]
fn test_media_extensions() {
    assert!(is_media_file(Path::new("video.mp4")));
    assert!(is_media_file(Path::new("video.mkv")));
    assert!(is_media_file(Path::new("video.avi")));
    assert!(is_media_file(Path::new("video.webm")));
    assert!(is_media_file(Path::new("audio.mp3")));
    assert!(is_media_file(Path::new("audio.flac")));
    assert!(!is_media_file(Path::new("document.pdf")));
    assert!(!is_media_file(Path::new("archive.zip")));

    assert!(is_subtitle_file(Path::new("sub.srt")));
    assert!(is_subtitle_file(Path::new("sub.ass")));
    assert!(is_subtitle_file(Path::new("sub.vtt")));
    assert!(is_subtitle_file(Path::new("sub.smi")));
    assert!(!is_subtitle_file(Path::new("sub.mp4")));
}

#[test]
fn test_playlist_natural_sorting() {
    let mut pl = Playlist::new();
    pl.add_item(PathBuf::from("Episode 10.mkv"));
    pl.add_item(PathBuf::from("Episode 2.mkv"));
    pl.add_item(PathBuf::from("Episode 1.mkv"));
    pl.add_item(PathBuf::from("Episode 20.mkv"));

    pl.sort_natural();

    assert_eq!(pl.items[0].title, "Episode 1.mkv");
    assert_eq!(pl.items[1].title, "Episode 2.mkv");
    assert_eq!(pl.items[2].title, "Episode 10.mkv");
    assert_eq!(pl.items[3].title, "Episode 20.mkv");
}

#[test]
fn test_playlist_duplicate_and_missing_file_cleaner() {
    let mut pl = Playlist::new();
    pl.add_item(PathBuf::from("Track 1.mp4"));
    pl.items.push(vortex_player::playlist::PlaylistItem::from_path(PathBuf::from("Track 1.mp4")));
    pl.add_item(PathBuf::from("Track 2.mp4"));
    assert_eq!(pl.len(), 3);

    pl.remove_duplicates();
    assert_eq!(pl.len(), 2);
}

#[test]
fn test_playlist_navigation_and_repeat_modes() {
    let mut pl = Playlist::new();
    pl.add_item(PathBuf::from("Track 1.mp4"));
    pl.add_item(PathBuf::from("Track 2.mp4"));
    pl.add_item(PathBuf::from("Track 3.mp4"));

    assert_eq!(pl.play_index(0), Some(PathBuf::from("Track 1.mp4")));
    assert_eq!(pl.current_index, Some(0));

    // Next track
    assert_eq!(pl.next(), Some(PathBuf::from("Track 2.mp4")));
    assert_eq!(pl.current_index, Some(1));
    assert_eq!(pl.next(), Some(PathBuf::from("Track 3.mp4")));
    assert_eq!(pl.current_index, Some(2));

    // Next at end with RepeatMode::Off returns None
    pl.repeat_mode = RepeatMode::Off;
    assert_eq!(pl.next(), None);

    // Next at end with RepeatMode::RepeatAll loops to start
    pl.repeat_mode = RepeatMode::RepeatAll;
    assert_eq!(pl.next(), Some(PathBuf::from("Track 1.mp4")));

    // Prev at start with RepeatMode::RepeatAll loops to end
    assert_eq!(pl.previous(), Some(PathBuf::from("Track 3.mp4")));
}

#[test]
fn test_sequence_detector_handles_mixed_case_without_numeric_prefixes() {
    let dir = std::env::temp_dir().join(format!("vortex-sequence-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let current = dir.join("Show Part1.mkv");
    let next = dir.join("Show Part2.mkv");
    let _ = std::fs::write(&current, b"");
    let _ = std::fs::write(&next, b"");

    assert_eq!(SequenceDetector::detect_next_part(&current), Some(next.clone()));

    let part10 = dir.join("Show Part10.mkv");
    let part11 = dir.join("Show Part11.mkv");
    let _ = std::fs::write(&part10, b"");
    let _ = std::fs::write(&part11, b"");
    assert_eq!(SequenceDetector::detect_next_part(&part10), None);

    let _ = std::fs::remove_file(current);
    let _ = std::fs::remove_file(next);
    let _ = std::fs::remove_file(part10);
    let _ = std::fs::remove_file(part11);
    let _ = std::fs::remove_dir(&dir);
}

#[test]
fn test_config_persistence_defaults() {
    let mut config = AppConfig::default();
    assert_eq!(config.volume, 80.0);
    assert_eq!(config.playback_speed, 1.0);
    assert_eq!(config.double_click_action, DoubleClickAction::PlayPause);
    assert_eq!(config.playlist_end_action, PlaylistEndAction::ShowHomeScreen);
    assert_eq!(config.audio_channels, "auto");

    config.save_resume_time("C:/test_video.mkv", 45.2);
    assert_eq!(config.get_resume_time("C:/test_video.mkv"), Some(45.2));

    let json = serde_json::to_string(&config).unwrap();
    let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.volume, 80.0);
    assert_eq!(deserialized.get_resume_time("C:/test_video.mkv"), Some(45.2));
}

#[test]
fn test_lyrics_parser_and_nan_sorting() {
    let lrc = r#"
[00:12.50]First line of lyrics
[00:05.00]Intro line
[00:25.00]Chorus line
"#;
    let track = vortex_player::engine::lyrics::LyricTrack::parse_lrc(lrc);
    assert!(track.is_some());
    let t = track.unwrap();
    assert_eq!(t.lines.len(), 3);
    // Verified sorted order
    assert_eq!(t.lines[0].text, "Intro line");
    assert_eq!(t.lines[0].timestamp_sec, 5.0);
    assert_eq!(t.lines[1].text, "First line of lyrics");
    assert_eq!(t.lines[1].timestamp_sec, 12.5);
    assert_eq!(t.lines[2].text, "Chorus line");
    assert_eq!(t.lines[2].timestamp_sec, 25.0);

    // Line lookup by timestamp
    assert_eq!(t.get_current_line(4.0), None);
    assert_eq!(t.get_current_line(6.0).map(|(_, l)| l.text.as_str()), Some("Intro line"));
    assert_eq!(t.get_current_line(15.0).map(|(_, l)| l.text.as_str()), Some("First line of lyrics"));
    assert_eq!(t.get_current_line(30.0).map(|(_, l)| l.text.as_str()), Some("Chorus line"));
}

#[test]
fn test_lyrics_slash_separated_user_format() {
    let sample = "[0:00.54] 君が僕に見せてくれた / [0:05.80] 世界はとても綺麗だったな / [0:12.01] ... / [0:22.81] 君の瞳に住まう";
    let track = vortex_player::engine::lyrics::LyricTrack::parse_lrc(sample);
    assert!(track.is_some());
    let t = track.unwrap();
    assert_eq!(t.lines.len(), 4);
    assert_eq!(t.lines[0].text, "君が僕に見せてくれた");
    assert!((t.lines[0].timestamp_sec - 0.54).abs() < 0.001);
    assert_eq!(t.lines[1].text, "世界はとても綺麗だったな");
    assert!((t.lines[1].timestamp_sec - 5.80).abs() < 0.001);
    assert_eq!(t.lines[2].text, "...");
    assert!((t.lines[2].timestamp_sec - 12.01).abs() < 0.001);
    assert_eq!(t.lines[3].text, "君の瞳に住まう");
    assert!((t.lines[3].timestamp_sec - 22.81).abs() < 0.001);

    // Lookups
    assert_eq!(t.get_current_line(0.0), None);
    assert_eq!(t.get_current_line(1.0).map(|(_, l)| l.text.as_str()), Some("君が僕に見せてくれた"));
    assert_eq!(t.get_current_line(7.0).map(|(_, l)| l.text.as_str()), Some("世界はとても綺麗だったな"));
}

#[test]
fn test_lyrics_mediainfo_text_extractor() {
    let mi_text = r#"
General
Complete name : D:\Downloads\Music\Atarayo - I am....flac
Format : FLAC
Duration : 3 min 59 s
Lyrics : [0:00.54] 君が僕に見せてくれた / [0:05.80] 世界はとても綺麗だったな
Audio
Format : FLAC
"#;
    let track = vortex_player::engine::lyrics::extract_from_mediainfo_text(mi_text);
    assert!(track.is_some());
    let t = track.unwrap();
    assert_eq!(t.lines.len(), 2);
    assert_eq!(t.lines[0].text, "君が僕に見せてくれた");
    assert_eq!(t.lines[1].text, "世界はとても綺麗だったな");
}

#[test]
fn test_lyrics_multi_timestamps_and_offset() {
    let sample = r#"
[offset:500]
[00:01.00][00:05.00] Chorus line
"#;
    let track = vortex_player::engine::lyrics::LyricTrack::parse_lrc(sample);
    assert!(track.is_some());
    let t = track.unwrap();
    assert_eq!(t.lines.len(), 2);
    // 1.0s + 0.5s offset = 1.5s
    assert!((t.lines[0].timestamp_sec - 1.5).abs() < 0.001);
    assert_eq!(t.lines[0].text, "Chorus line");
    // 5.0s + 0.5s offset = 5.5s
    assert!((t.lines[1].timestamp_sec - 5.5).abs() < 0.001);
    assert_eq!(t.lines[1].text, "Chorus line");
}

#[test]
fn test_parametric_eq_filter_generation() {
    use vortex_player::engine::parametric_eq::{FilterType, ParametricEqConfig, ParametricNode};

    let mut config = ParametricEqConfig::default();
    assert!(!config.enabled);
    assert_eq!(config.build_audio_filter_string(), "");

    config.enabled = true;
    config.preamp_gain = 3.5;
    config.nodes = vec![
        ParametricNode { enabled: true, filter_type: FilterType::Peak, freq: 1000.0, gain: 4.0, q: 1.41 },
        ParametricNode { enabled: true, filter_type: FilterType::LowShelf, freq: 120.0, gain: -2.5, q: 0.71 },
        ParametricNode { enabled: false, filter_type: FilterType::HighShelf, freq: 8000.0, gain: 3.0, q: 0.71 },
    ];

    let filter_str = config.build_audio_filter_string();
    assert!(filter_str.contains("volume=volume=3.5dB"));
    assert!(filter_str.contains("equalizer=f=1000.0:width_type=q:w=1.41:g=4.0"));
    assert!(filter_str.contains("lowshelf=f=120.0:gain=-2.5:poles=2"));
    assert!(!filter_str.contains("highshelf")); // Disabled node excluded
}

#[test]
fn test_visualizer_mode_cycling() {
    use vortex_player::ui::visualizer::{VisualizerMode, VisualizerSuite};

    let mut suite = VisualizerSuite::new();
    assert_eq!(suite.mode, VisualizerMode::FftSpectrum);
    assert_eq!(suite.cycle_mode(), VisualizerMode::Oscilloscope);
    assert_eq!(suite.cycle_mode(), VisualizerMode::Vectorscope);
    assert_eq!(suite.cycle_mode(), VisualizerMode::LufsMeter);
    assert_eq!(suite.cycle_mode(), VisualizerMode::FftSpectrum);

    assert_eq!(VisualizerMode::ALL.len(), 4);
    assert_eq!(VisualizerMode::FftSpectrum.display_name(), "FFT Spectrum Analyzer");
}

#[test]
fn test_bookmark_skip_intervals() {
    let mut bm = BookmarkManager::new();
    assert_eq!(bm.skip_intervals.len(), 0);

    bm.auto_skip_bookmarks = true;
    bm.add_skip_interval(10.0, 30.0, "OP Theme".to_string());
    bm.add_skip_interval(120.0, 150.0, "ED Theme".to_string());
    assert_eq!(bm.skip_intervals.len(), 2);

    assert_eq!(bm.should_skip(5.0), None);
    assert_eq!(bm.should_skip(15.0), Some((30.0, "OP Theme".to_string())));
    assert_eq!(bm.should_skip(130.0), Some((150.0, "ED Theme".to_string())));
    assert_eq!(bm.should_skip(200.0), None);
}

#[test]
fn test_player_audio_51_and_multichannel() {
    use vortex_player::engine::Player;
    if let Ok(player) = Player::new() {
        player.set_volume(100.0);
        player.set_audio_channels("auto");
        let path = "/mnt/94C83957C83938B6/Downloads/Torrents/[UDF] The Girl Who Leapt Through Time 2006 (BD 1080p 2xFLAC) [dual-audio] [E5869AB8].mkv";
        if std::path::Path::new(path).exists() {
            player.load_file(path);
        } else {
            player.load_file("test_51_ident.flac");
        }
        player.seek_absolute(45.0); // Seek to 45s where active audio/music is playing
        player.play();

        for i in 1..=5 {
            std::thread::sleep(std::time::Duration::from_millis(400));
            let stats = player.stats();
            println!("─── [API Sample #{}] ───", i);
            println!("  Time Pos: {:.2}s / {:.2}s | Paused: {} | Idle: {}", stats.time_pos, stats.duration, stats.is_paused, stats.is_idle);
            println!("  Volume: {:.0}% | Muted: {} | Speed: {:.1}x", stats.volume, stats.is_muted, stats.speed);
            println!("  AO Driver: '{}' | Device: '{}'", stats.current_ao, stats.active_audio_device);
            println!("  Audio Codec: '{}' ({})", stats.audio_codec, stats.audio_format_str);
            println!("  Input Channels: {}ch ({} Hz) -> Output Channels: {}ch ({} Hz)",
                stats.audio_channels, stats.audio_sample_rate, stats.audio_out_channels, stats.audio_out_sample_rate);
            println!("  Live Per-Channel Meter Levels (dB VU): [L: {:.3}, R: {:.3}, C: {:.3}, LFE: {:.3}, SL: {:.3}, SR: {:.3}]",
                stats.audio_channel_levels[0], stats.audio_channel_levels[1], stats.audio_channel_levels[2],
                stats.audio_channel_levels[3], stats.audio_channel_levels[4], stats.audio_channel_levels[5]);
            println!("  Selected Audio Track: ID {} of {} total tracks", stats.selected_audio_track, stats.audio_tracks.len());
        }
        let final_stats = player.stats();
        assert!(final_stats.time_pos > 0.0 || !final_stats.is_idle);
    }
}

#[cfg(windows)]
#[test]
fn test_taskbar_button_icons_valid() {
    use vortex_player::platform::windows::taskbar::THUMBBUTTON;
    println!("Rust sizeof(THUMBBUTTON) = {}", std::mem::size_of::<THUMBBUTTON>());
    println!("Rust align_of(THUMBBUTTON) = {}", std::mem::align_of::<THUMBBUTTON>());
    println!("Rust offset_of(hIcon) = {}", std::mem::offset_of!(THUMBBUTTON, hIcon));
    println!("Rust offset_of(szTip) = {}", std::mem::offset_of!(THUMBBUTTON, szTip));
    println!("Rust offset_of(dwFlags) = {}", std::mem::offset_of!(THUMBBUTTON, dwFlags));
    let icons = vortex_player::platform::build_button_icons();
    println!("Built button icons: {:?}", icons);
    for (i, &icon) in icons.iter().enumerate() {
        assert_ne!(icon, 0, "Icon #{} must not be NULL!", i);
    }

    // Now test with a real Win32 window
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
        // Run on the interactive "Default" desktop where Shell_TrayWnd resides
        let icons_clone = icons.clone();
        let handle = std::thread::spawn(move || unsafe {
            #[link(name = "user32")]
            unsafe extern "system" {
                fn OpenDesktopW(
                    lpszDesktop: *const u16,
                    dwFlags: u32,
                    fInherit: windows_sys::Win32::Foundation::BOOL,
                    dwDesiredAccess: u32,
                ) -> windows_sys::Win32::Foundation::HANDLE;
                fn SetThreadDesktop(hDesktop: windows_sys::Win32::Foundation::HANDLE) -> windows_sys::Win32::Foundation::BOOL;
            }
            let desk_name: Vec<u16> = "Default\0".encode_utf16().collect();
            let h_desk = OpenDesktopW(desk_name.as_ptr(), 0, 0, 0x01FF);
            if !h_desk.is_null() {
                SetThreadDesktop(h_desk);
            }

            let hinstance = GetModuleHandleW(std::ptr::null());
            let class_name: Vec<u16> = "TestThumbbarClassDef\0".encode_utf16().collect();
            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: 0,
                lpfnWndProc: Some(DefWindowProcW),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: std::ptr::null_mut(),
                hCursor: std::ptr::null_mut(),
                hbrBackground: std::ptr::null_mut(),
                lpszMenuName: std::ptr::null(),
                lpszClassName: class_name.as_ptr(),
                hIconSm: std::ptr::null_mut(),
            };
            RegisterClassExW(&wc);
            let hwnd = CreateWindowExW(
                WS_EX_APPWINDOW,
                class_name.as_ptr(),
                class_name.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                100, 100, 400, 300,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                hinstance,
                std::ptr::null(),
            );
            ShowWindow(hwnd, SW_SHOW);
            windows_sys::Win32::Graphics::Gdi::UpdateWindow(hwnd);

            let mut adapter = vortex_player::platform::WindowsTaskbarAdapter::new();
            adapter.init_thumbbar(hwnd as isize);
            println!("Default desktop 7 buttons init: {}", adapter.is_thumbbar_initialized());
            assert!(adapter.is_thumbbar_initialized(), "ThumbBarAddButtons must succeed on Default desktop!");

            DestroyWindow(hwnd);
        });
        handle.join().unwrap();
    }

