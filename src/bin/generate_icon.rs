use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let assets_dir = Path::new("assets");
    if !assets_dir.exists() {
        std::fs::create_dir_all(assets_dir).unwrap();
    }

    let sizes = [16, 24, 32, 48, 64, 128, 256];
    let mut png_blobs: Vec<(u32, Vec<u8>)> = Vec::new();

    for &size in &sizes {
        let img = generate_vortex_icon_image(size);
        let mut png_bytes = Vec::new();
        {
            let mut cursor = std::io::Cursor::new(&mut png_bytes);
            img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
        }
        if size == 256 {
            img.save("assets/icon.png").unwrap();
        }
        png_blobs.push((size, png_bytes));
    }

    // Pack into .ico file
    let ico_path = "assets/icon.ico";
    let ico_file = File::create(ico_path).unwrap();
    let mut writer = BufWriter::new(ico_file);

    let count = png_blobs.len() as u16;
    // ICONDIR header
    writer.write_all(&0u16.to_le_bytes()).unwrap(); // Reserved (0)
    writer.write_all(&1u16.to_le_bytes()).unwrap(); // Type (1 for .ico)
    writer.write_all(&count.to_le_bytes()).unwrap(); // Image count

    // Header is 6 bytes + count * 16 bytes directory entries
    let mut offset = 6 + (count as u32) * 16;

    for (size, data) in &png_blobs {
        let w = if *size >= 256 { 0u8 } else { *size as u8 };
        let h = if *size >= 256 { 0u8 } else { *size as u8 };
        let color_count = 0u8;
        let reserved = 0u8;
        let planes = 1u16;
        let bit_count = 32u16;
        let bytes_in_res = data.len() as u32;

        writer.write_all(&[w, h, color_count, reserved]).unwrap();
        writer.write_all(&planes.to_le_bytes()).unwrap();
        writer.write_all(&bit_count.to_le_bytes()).unwrap();
        writer.write_all(&bytes_in_res.to_le_bytes()).unwrap();
        writer.write_all(&offset.to_le_bytes()).unwrap();

        offset += bytes_in_res;
    }

    // Write image data
    for (_, data) in &png_blobs {
        writer.write_all(data).unwrap();
    }

    println!("Successfully generated assets/icon.ico and assets/icon.png");
}

fn generate_vortex_icon_image(size: u32) -> image::RgbaImage {
    let mut img = image::RgbaImage::new(size, size);
    let s = size as f32;
    let center = s * 0.5;
    let r_outer = s * 0.46;

    for y in 0..size {
        for x in 0..size {
            // 4x Supersampling for ultra-crisp anti-aliasing
            let mut r_acc = 0.0;
            let mut g_acc = 0.0;
            let mut b_acc = 0.0;
            let mut a_acc = 0.0;

            for sy in 0..4 {
                for sx in 0..4 {
                    let px = (x as f32) + (sx as f32 + 0.5) / 4.0;
                    let py = (y as f32) + (sy as f32 + 0.5) / 4.0;

                    let dx = px - center;
                    let dy = py - center;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist <= r_outer {
                        let rel_dist = dist / r_outer; // 0.0 (center) to 1.0 (outer border)

                        let (mut r, mut g, mut b, mut a) = if rel_dist > 0.92 {
                            // Outer dark gold/bronze rim
                            (180.0, 110.0, 15.0, 1.0)
                        } else if rel_dist > 0.84 {
                            // Bright highlight gold ring
                            (255.0, 220.0, 80.0, 1.0)
                        } else if rel_dist > 0.80 {
                            // Inner bronze groove
                            (120.0, 70.0, 10.0, 1.0)
                        } else {
                            // Obsidian body with subtle radial gradient
                            let factor = rel_dist / 0.80;
                            let dark_r = 18.0 + (1.0 - factor) * 22.0;
                            let dark_g = 20.0 + (1.0 - factor) * 25.0;
                            let dark_b = 28.0 + (1.0 - factor) * 32.0;
                            (dark_r, dark_g, dark_b, 1.0)
                        };

                        // Concentric decorative tracks
                        if (rel_dist > 0.65 && rel_dist < 0.67) || (rel_dist > 0.50 && rel_dist < 0.52) {
                            r += 30.0;
                            g += 30.0;
                            b += 40.0;
                        }

                        // Glass dome reflection across top half
                        if py < center && rel_dist < 0.80 {
                            let glass_factor = (1.0 - py / center).clamp(0.0, 1.0) * (1.0 - rel_dist);
                            r += glass_factor * 80.0;
                            g += glass_factor * 85.0;
                            b += glass_factor * 110.0;
                        }

                        // Vibrant Golden Play Triangle with Vortex Flair
                        let tri_center_x = center + s * 0.02;
                        let tri_center_y = center;
                        let tri_size = s * 0.46;

                        let p_x = px - tri_center_x;
                        let p_y = py - tri_center_y;

                        let half_h = tri_size * 0.50;
                        let left_x = -tri_size * 0.35;
                        let right_x = tri_size * 0.55;

                        // Triangle bounding check
                        if p_x >= left_x && p_x <= right_x {
                            let slope = half_h / (right_x - left_x);
                            let max_y = (right_x - p_x) * slope;

                            if p_y.abs() <= max_y {
                                let norm_x = (p_x - left_x) / (right_x - left_x);
                                let norm_y = p_y.abs() / max_y.max(0.001);

                                // Rich metallic gold & amber gradient with bright core
                                let gold_r: f32 = 255.0;
                                let gold_g: f32 = 180.0 + (1.0 - norm_x) * 60.0 - norm_y * 30.0;
                                let gold_b: f32 = 20.0 + (1.0 - norm_x) * 40.0;

                                // Core swirl highlight
                                if p_y < 0.0 && norm_y < 0.6 {
                                    r = (gold_r + 40.0f32).min(255.0f32);
                                    g = (gold_g + 50.0f32).min(255.0f32);
                                    b = (gold_b + 80.0f32).min(255.0f32);
                                } else {
                                    r = gold_r;
                                    g = gold_g.clamp(0.0f32, 255.0f32);
                                    b = gold_b.clamp(0.0f32, 255.0f32);
                                }

                                // Golden outer rim glow
                                if p_x - left_x < 3.0 || right_x - p_x < 3.0 || (max_y - p_y.abs()) < 3.0 {
                                    r = 255.0;
                                    g = 245.0;
                                    b = 180.0;
                                }
                            }
                        }

                        // Anti-aliased outer edge softness
                        let edge_dist = r_outer - dist;
                        if edge_dist < 1.0 {
                            a *= edge_dist.clamp(0.0, 1.0);
                        }

                        r_acc += r;
                        g_acc += g;
                        b_acc += b;
                        a_acc += a;
                    }
                }
            }

            let samples = 16.0;
            let final_a = (a_acc / samples).clamp(0.0, 1.0);
            if final_a > 0.001 {
                let final_r = ((r_acc / samples) * final_a).clamp(0.0, 255.0) as u8;
                let final_g = ((g_acc / samples) * final_a).clamp(0.0, 255.0) as u8;
                let final_b = ((b_acc / samples) * final_a).clamp(0.0, 255.0) as u8;
                let final_a_u8 = (final_a * 255.0) as u8;
                img.put_pixel(x, y, image::Rgba([final_r, final_g, final_b, final_a_u8]));
            }
        }
    }

    img
}
