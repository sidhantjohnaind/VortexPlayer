use crate::engine::{MediaStats, Player};
use eframe::egui::{
    self, Color32, CornerRadius, Pos2, Rect, Stroke, TextureHandle, Vec2,
};
use std::path::{Path, PathBuf};

pub struct MusicBackgroundView {
    pub cached_cover_path: Option<PathBuf>,
    pub cover_texture: Option<TextureHandle>,
    pub blurred_texture: Option<TextureHandle>,
    pub last_file_path: String,
    pub extracted_temp_cover: PathBuf,
    cover_receiver: Option<std::sync::mpsc::Receiver<(egui::ColorImage, egui::ColorImage, PathBuf)>>,
}

impl Default for MusicBackgroundView {
    fn default() -> Self {
        Self {
            cached_cover_path: None,
            cover_texture: None,
            blurred_texture: None,
            last_file_path: String::new(),
            extracted_temp_cover: std::env::temp_dir().join(format!("vortex_music_cover_cache_{}.png", std::process::id())),
            cover_receiver: None,
        }
    }
}

impl MusicBackgroundView {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if currently playing file is an audio song
    pub fn is_song(stats: &MediaStats) -> bool {
        if stats.is_idle || stats.file_path.is_empty() {
            return false;
        }
        let p = Path::new(&stats.file_path);
        if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            // Video file extensions must NEVER be treated as songs (prevents album art flash during video load)
            if crate::playlist::scanner::SUPPORTED_VIDEO_EXTENSIONS.contains(&ext_lower.as_str()) {
                return false;
            }
            if crate::playlist::scanner::SUPPORTED_AUDIO_EXTENSIONS.contains(&ext_lower.as_str()) {
                return true;
            }
        }
        // For extensionless files: only treat as song if explicitly confirmed NO video stream and has audio
        !stats.video_codec.is_empty() && stats.video_codec == "none" && stats.video_width == 0 && stats.audio_channels > 0
    }

    /// Locate cover art in song folder
    fn find_cover_art(file_path: &str) -> Option<PathBuf> {
        let p = Path::new(file_path);
        let parent = p.parent()?;

        // 1. Check exact song stem (e.g. "Song.flac" -> "Song.jpg", "Song.png")
        if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
            for ext in &["jpg", "jpeg", "png", "webp"] {
                let song_img = parent.join(format!("{}.{}", stem, ext));
                if song_img.exists() && song_img.is_file() {
                    return Some(song_img);
                }
            }
        }

        // 2. Standard album folder artwork filenames
        let candidates = [
            "cover.jpg", "cover.png", "folder.jpg", "folder.png",
            "album.jpg", "album.png", "front.jpg", "front.png",
            "artwork.jpg", "artwork.png", "AlbumArtSmall.jpg",
            "Folder.jpg", "Cover.jpg", "Front.jpg", "Album.jpg",
            "cover.jpeg", "folder.jpeg", "front.jpeg",
        ];

        for name in &candidates {
            let path = parent.join(name);
            if path.exists() && path.is_file() {
                return Some(path);
            }
        }
        None
    }

    /// Decode standard base64 byte stream
    fn decode_base64(s: &[u8]) -> Option<Vec<u8>> {
        let mut out = Vec::with_capacity(s.len() * 3 / 4);
        let mut buf = 0u32;
        let mut bits = 0;
        for &b in s {
            let val = match b {
                b'A'..=b'Z' => b - b'A',
                b'a'..=b'z' => b - b'a' + 26,
                b'0'..=b'9' => b - b'0' + 52,
                b'+' | b'-' => 62,
                b'/' | b'_' => 63,
                b'=' | b'\r' | b'\n' | b' ' => continue,
                _ => break,
            };
            buf = (buf << 6) | (val as u32);
            bits += 6;
            if bits >= 8 {
                bits -= 8;
                out.push((buf >> bits) as u8);
                buf &= (1 << bits) - 1;
            }
        }
        if out.is_empty() { None } else { Some(out) }
    }

    /// Parse FLAC PICTURE metadata block structure
    fn parse_flac_picture_block(block_data: &[u8]) -> Option<Vec<u8>> {
        if block_data.len() < 32 {
            return None;
        }
        let mime_len = u32::from_be_bytes([block_data[4], block_data[5], block_data[6], block_data[7]]) as usize;
        let mut offset = 8usize.checked_add(mime_len)?;
        if offset + 4 > block_data.len() {
            return None;
        }
        let desc_len = u32::from_be_bytes([block_data[offset], block_data[offset + 1], block_data[offset + 2], block_data[offset + 3]]) as usize;
        offset = offset.checked_add(4)?.checked_add(desc_len)?.checked_add(16)?;
        if offset + 4 > block_data.len() {
            return None;
        }
        let data_len = u32::from_be_bytes([block_data[offset], block_data[offset + 1], block_data[offset + 2], block_data[offset + 3]]) as usize;
        offset += 4;
        if offset.checked_add(data_len)? <= block_data.len() {
            Some(block_data[offset..offset + data_len].to_vec())
        } else {
            None
        }
    }

    /// Extract Vorbis comment base64 picture (METADATA_BLOCK_PICTURE or COVERART)
    fn extract_vorbis_picture(slice: &[u8]) -> Option<Vec<u8>> {
        let keys = [
            b"METADATA_BLOCK_PICTURE=" as &[u8],
            b"metadata_block_picture=",
            b"COVERART=",
            b"coverart=",
        ];
        for key in &keys {
            if let Some(pos) = slice.windows(key.len()).position(|w| w == *key) {
                let b64_data = &slice[pos + key.len()..];
                let end = b64_data.iter().position(|&b| !b.is_ascii_alphanumeric() && b != b'+' && b != b'/' && b != b'-' && b != b'_' && b != b'=').unwrap_or(b64_data.len());
                if let Some(decoded) = Self::decode_base64(&b64_data[..end]) {
                    if let Some(pic) = Self::parse_flac_picture_block(&decoded) {
                        return Some(pic);
                    }
                    return Some(decoded);
                }
            }
        }
        None
    }

    /// Pure-Rust instant universal embedded album art extractor (FLAC, Vorbis Base64, MP3 APIC, M4A covr)
    fn extract_album_art_image(file_path: &str) -> Option<image::DynamicImage> {
        use std::io::Read;
        let mut file = std::fs::File::open(file_path).ok()?;
        // Read first 4MB (where all audio headers and cover art reside)
        let mut buf = vec![0u8; 4 * 1024 * 1024];
        let bytes_read = file.read(&mut buf).unwrap_or(0);
        if bytes_read < 32 {
            return None;
        }
        let data = &buf[..bytes_read];

        // 1. FLAC PICTURE & VORBIS_COMMENT METADATA PARSER
        if &data[0..4] == b"fLaC" {
            let mut is_last = (data[4] & 0x80) != 0;
            let mut block_type = data[4] & 0x7F;
            let mut length = ((data[5] as usize) << 16) | ((data[6] as usize) << 8) | (data[7] as usize);
            let mut offset = 8usize;

            loop {
                if offset + length > data.len() {
                    break;
                }
                let block_slice = &data[offset..offset + length];

                if block_type == 6 {
                    // Method A: Exact FLAC Picture block parser
                    if let Some(pic_bytes) = Self::parse_flac_picture_block(block_slice) {
                        if let Ok(img) = image::load_from_memory(&pic_bytes) {
                            return Some(img);
                        }
                    }
                    // Method B: Direct JPEG/PNG signature search inside Picture block slice
                    if let Some(pos) = block_slice.windows(3).position(|w| w == [0xFF, 0xD8, 0xFF]) {
                        if let Ok(img) = image::load_from_memory(&block_slice[pos..]) {
                            return Some(img);
                        }
                    }
                    if let Some(pos) = block_slice.windows(4).position(|w| w == [0x89, 0x50, 0x4E, 0x47]) {
                        if let Ok(img) = image::load_from_memory(&block_slice[pos..]) {
                            return Some(img);
                        }
                    }
                } else if block_type == 4 {
                    // Vorbis Comment Base64 Picture (METADATA_BLOCK_PICTURE / COVERART)
                    if let Some(pic_bytes) = Self::extract_vorbis_picture(block_slice) {
                        if let Ok(img) = image::load_from_memory(&pic_bytes) {
                            return Some(img);
                        }
                    }
                }

                if is_last {
                    break;
                }

                offset += length;
                if offset + 4 > data.len() {
                    break;
                }
                let hdr = &data[offset..offset + 4];
                offset += 4;
                is_last = (hdr[0] & 0x80) != 0;
                block_type = hdr[0] & 0x7F;
                length = ((hdr[1] as usize) << 16) | ((hdr[2] as usize) << 8) | (hdr[3] as usize);
            }
        }

        // 2. MP3 ID3V2 APIC ATTACHED PICTURE
        if &data[0..3] == b"ID3" {
            let tag_size = ((data[6] as usize & 0x7F) << 21)
                | ((data[7] as usize & 0x7F) << 14)
                | ((data[8] as usize & 0x7F) << 7)
                | (data[9] as usize & 0x7F);
            let id3_end = (10 + tag_size).min(data.len());
            let id3_data = &data[10..id3_end];

            if let Some(pos) = id3_data.windows(3).position(|w| w == [0xFF, 0xD8, 0xFF]) {
                if let Ok(img) = image::load_from_memory(&id3_data[pos..]) {
                    return Some(img);
                }
            }
            if let Some(pos) = id3_data.windows(4).position(|w| w == [0x89, 0x50, 0x4E, 0x47]) {
                if let Ok(img) = image::load_from_memory(&id3_data[pos..]) {
                    return Some(img);
                }
            }
        }

        // 3. MP4 / M4A covr ATOM
        if let Some(pos) = data.windows(4).position(|w| w == b"covr") {
            let after_covr = &data[pos + 4..];
            if let Some(data_pos) = after_covr.windows(4).position(|w| w == b"data") {
                let payload = &after_covr[data_pos + 12..];
                if let Ok(img) = image::load_from_memory(payload) {
                    return Some(img);
                }
            }
        }

        // 4. Header-only fast JPEG search with valid 4th marker byte in first 512KB
        let scan_limit = data.len().min(512 * 1024);
        let header_slice = &data[..scan_limit];
        for (i, w) in header_slice.windows(4).enumerate() {
            if w[0] == 0xFF && w[1] == 0xD8 && w[2] == 0xFF && (w[3] & 0xF0 == 0xE0 || w[3] == 0xDB || w[3] == 0xC0 || w[3] == 0xEE) {
                if let Ok(img) = image::load_from_memory(&data[i..]) {
                    return Some(img);
                }
            }
        }

        // 5. Header-only fast PNG search
        for (i, w) in header_slice.windows(8).enumerate() {
            if w == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
                if let Ok(img) = image::load_from_memory(&data[i..]) {
                    return Some(img);
                }
            }
        }

        None
    }

    /// Process color images from raw image bytes off the main thread in < 1ms
    fn process_image_to_color_images(img: image::DynamicImage) -> (egui::ColorImage, egui::ColorImage) {
        let rgba = img.to_rgba8();
        let (w, h) = (rgba.width() as usize, rgba.height() as usize);
        let sharp_ci = egui::ColorImage::from_rgba_unmultiplied([w, h], rgba.as_raw());

        // Heavy atmospheric diffusion blur matching PotPlayer's creamy, smooth color backdrop
        let downsampled = image::imageops::resize(&img, 48, 48, image::imageops::FilterType::Triangle);
        let blurred = image::imageops::blur(&downsampled, 14.0);
        let blur_ci = egui::ColorImage::from_rgba_unmultiplied([48, 48], blurred.as_raw());

        (sharp_ci, blur_ci)
    }


    /// Generate fallback blurred texture with procedural gradient
    fn generate_fallback_textures(ctx: &egui::Context, name: &str) -> (TextureHandle, TextureHandle, image::RgbaImage) {
        let mut hash: u32 = 5381;
        for b in name.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as u32);
        }

        let r1 = ((hash & 0xFF) % 140 + 40) as u8;
        let g1 = (((hash >> 8) & 0xFF) % 140 + 60) as u8;
        let b1 = (((hash >> 16) & 0xFF) % 140 + 40) as u8;

        let r2 = (((hash >> 4) & 0xFF) % 130 + 30) as u8;
        let g2 = (((hash >> 12) & 0xFF) % 150 + 50) as u8;
        let b2 = (((hash >> 20) & 0xFF) % 150 + 60) as u8;

        // Generate procedural 32x32 gradient image instantly
        let mut img_buf = image::RgbaImage::new(32, 32);
        for y in 0..32 {
            for x in 0..32 {
                let fx = x as f32 / 32.0;
                let fy = y as f32 / 32.0;
                let r = (r1 as f32 * (1.0 - fx) + r2 as f32 * fx) as u8;
                let g = (g1 as f32 * (1.0 - fy) + g2 as f32 * fy) as u8;
                let b = (b1 as f32 * fx + b2 as f32 * (1.0 - fy)) as u8;
                img_buf.put_pixel(x, y, image::Rgba([r, g, b, 255]));
            }
        }

        let blur_ci = egui::ColorImage::from_rgba_unmultiplied([32, 32], img_buf.as_raw());
        let blur_tex = ctx.load_texture("music_fallback_blur", blur_ci, egui::TextureOptions::LINEAR);

        // Sharp stylized vinyl cover
        let mut sharp_buf = image::RgbaImage::new(64, 64);
        for y in 0..64 {
            for x in 0..64 {
                let dx = x as f32 - 32.0;
                let dy = y as f32 - 32.0;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < 30.0 {
                    let ring = ((dist * 1.5) as u32 % 6) as u8;
                    let v = 25 + ring * 5;
                    sharp_buf.put_pixel(x, y, image::Rgba([v, v + 2, v + 6, 255]));
                } else {
                    sharp_buf.put_pixel(x, y, image::Rgba([0, 0, 0, 0]));
                }
            }
        }

        let sharp_ci = egui::ColorImage::from_rgba_unmultiplied([64, 64], sharp_buf.as_raw());
        let sharp_tex = ctx.load_texture("music_fallback_sharp", sharp_ci, egui::TextureOptions::LINEAR);

        (sharp_tex, blur_tex, sharp_buf)
    }

    /// Update current track cover textures asynchronously without blocking the UI thread
    pub fn update_track(&mut self, ctx: &egui::Context, _player: &Player, stats: &MediaStats) {
        if stats.file_path.is_empty() || self.last_file_path == stats.file_path {
            return;
        }

        self.last_file_path = stats.file_path.clone();
        self.cached_cover_path = None;

        // Instant placeholder gradient so UI is immediately responsive
        let (sharp, blur, sharp_img) = Self::generate_fallback_textures(ctx, &stats.file_path);
        let _ = sharp_img.save(&self.extracted_temp_cover);
        self.cached_cover_path = Some(self.extracted_temp_cover.clone());
        self.cover_texture = Some(sharp);
        self.blurred_texture = Some(blur);

        // Spawn background worker thread to extract and decode album art with ZERO UI stutter
        let file_path = stats.file_path.clone();
        let temp_cover = self.extracted_temp_cover.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        self.cover_receiver = Some(rx);

        let ctx_clone = ctx.clone();
        std::thread::spawn(move || {
            // 1. Look for folder artwork
            if let Some(folder_cover) = Self::find_cover_art(&file_path) {
                if let Ok(img) = image::open(&folder_cover) {
                    let (sharp_ci, blur_ci) = Self::process_image_to_color_images(img);
                    let _ = tx.send((sharp_ci, blur_ci, folder_cover));
                    ctx_clone.request_repaint();
                    return;
                }
            }

            // 2. Extract embedded album art (FLAC, Vorbis Base64, MP3 APIC, M4A covr, raw streams)
            if let Some(img) = Self::extract_album_art_image(&file_path) {
                let _ = img.save(&temp_cover);
                let (sharp_ci, blur_ci) = Self::process_image_to_color_images(img);
                let _ = tx.send((sharp_ci, blur_ci, temp_cover));
                ctx_clone.request_repaint();
            }
        });
    }



    pub fn render(&mut self, ui: &mut egui::Ui, player: &Player, rect: Rect, stats: &MediaStats) {
        if !Self::is_song(stats) {
            if self.cover_texture.is_some() || self.blurred_texture.is_some() {
                self.cover_texture = None;
                self.blurred_texture = None;
                self.cached_cover_path = None;
                self.last_file_path.clear();
            }
            return;
        }

        self.update_track(ui.ctx(), player, stats);

        // Receive completed asynchronous album art textures from background thread
        if let Some(ref rx) = self.cover_receiver {
            if let Ok((sharp_ci, blur_ci, cover_path)) = rx.try_recv() {
                self.cover_texture = Some(ui.ctx().load_texture("music_cover_sharp", sharp_ci, egui::TextureOptions::LINEAR));
                self.blurred_texture = Some(ui.ctx().load_texture("music_cover_blurred", blur_ci, egui::TextureOptions::LINEAR));
                self.cached_cover_path = Some(cover_path);
                self.cover_receiver = None;
            }
        }


        let painter = ui.painter().with_clip_rect(rect);

        // ── 1. FULL-CANVAS ATMOSPHERIC DIFFUSE GAUSSIAN BLURRED BACKDROP ────────
        // In compact mode (height < 180px), use seamless matte dark fill to match PotPlayer mini player.
        // In normal / expanded mode (height >= 180px), render atmospheric blurred color wash.
        if rect.height() >= 180.0 {
            if let Some(ref blur_tex) = self.blurred_texture {
                painter.image(
                    blur_tex.id(),
                    rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );

                // Subtle depth vignette to prevent overly washed-out tones
                painter.rect_filled(
                    rect,
                    CornerRadius::ZERO,
                    Color32::from_rgba_unmultiplied(0, 0, 0, 18),
                );
            } else {
                painter.rect_filled(rect, CornerRadius::ZERO, Color32::from_rgb(20, 21, 25));
            }
        } else {
            painter.rect_filled(rect, CornerRadius::ZERO, Color32::from_rgb(20, 21, 25));
        }

        // ── 2. CENTERED CRISP SHARP ALBUM COVER ART ─────────────────────────
        if rect.height() >= 120.0 && rect.width() >= 180.0 {
            if let Some(ref sharp_tex) = self.cover_texture {
                let center = rect.center();
                // Compute native aspect ratio of album art to avoid distortion
                let s = sharp_tex.size_vec2();
                let (tex_w, tex_h) = (s.x.max(1.0), s.y.max(1.0));
                let aspect = tex_w / tex_h;

                // Proportional album art scaling matching PotPlayer's ~58% viewport coverage
                let max_w = (rect.width() * 0.58).clamp(80.0, 1400.0);
                let max_h = (rect.height() * 0.58).clamp(80.0, 1400.0);

                let (card_w, card_h) = if aspect > (max_w / max_h) {
                    (max_w, max_w / aspect)
                } else {
                    (max_h * aspect, max_h)
                };

                let card_rect = Rect::from_center_size(center, Vec2::new(card_w, card_h));

                // Deep, soft multi-pass atmospheric drop shadow matching PotPlayer's floating album jacket
                let shadow_passes = [
                    (22.0, 12.0, 14),
                    (16.0, 8.0, 26),
                    (11.0, 6.0, 42),
                    (7.0, 4.0, 65),
                    (4.0, 2.0, 95),
                    (1.5, 1.0, 130),
                ];
                for (expand, y_off, alpha) in shadow_passes {
                    painter.rect_filled(
                        card_rect.expand(expand).translate(Vec2::new(0.0, y_off)),
                        CornerRadius::ZERO,
                        Color32::from_rgba_unmultiplied(0, 0, 0, alpha),
                    );
                }

                painter.image(
                    sharp_tex.id(),
                    card_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );

                // Crisp thin 1px border around the album art matching PotPlayer
                painter.rect_stroke(
                    card_rect,
                    CornerRadius::ZERO,
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 45)),
                    egui::StrokeKind::Inside,
                );
                painter.rect_stroke(
                    card_rect.expand(1.0),
                    CornerRadius::ZERO,
                    Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 90)),
                    egui::StrokeKind::Outside,
                );
            }
        }
    }
}
