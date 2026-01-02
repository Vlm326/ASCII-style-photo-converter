use crate::{Config, letter_pool};
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

fn luma01(r: f32, g: f32, b: f32) -> f32 {
    // 0..1
    let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    (y / 255.0).clamp(0.0, 1.0)
}

fn blend_over(dst: &mut Rgba<u8>, src_rgb: [u8; 3], alpha: f32) {
    let a = alpha.clamp(0.0, 1.0);
    let inv = 1.0 - a;

    dst.0[0] = (dst.0[0] as f32 * inv + src_rgb[0] as f32 * a) as u8;
    dst.0[1] = (dst.0[1] as f32 * inv + src_rgb[1] as f32 * a) as u8;
    dst.0[2] = (dst.0[2] as f32 * inv + src_rgb[2] as f32 * a) as u8;
    dst.0[3] = 255;
}

pub fn convert_simple(
    img: &DynamicImage,
    cfg: &Config,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let (w, h) = img.dimensions();

    // шрифт
    let font_data = std::fs::read(&cfg.font_path)?;
    let font = FontRef::try_from_slice(&font_data)
        .map_err(|_| format!("Failed to load font: {}", cfg.font_path))?;

    let scale = PxScale::from(cfg.font_px);
    let scaled = font.as_scaled(scale);

    // метрики клетки
    let advance_x = scaled.h_advance(scaled.glyph_id('M')).ceil() as u32;
    let ascent = scaled.ascent().ceil() as i32;
    let line_h = (scaled.ascent() - scaled.descent() + scaled.line_gap()).ceil() as u32;

    let cols = cfg.cols.max(1);
    let tile_w = (w as f32 / cols as f32).max(1.0);

    // чтобы не ломать пропорции, оценим rows через метрики шрифта
    let aspect = line_h as f32 / advance_x as f32; // примерно 2.0
    let rows = ((h as f32 / (tile_w * aspect)).max(1.0)) as u32;
    let rows = rows.max(1);
    let tile_h = (h as f32 / rows as f32).max(1.0);

    let charset: Vec<char> = cfg.charset.chars().collect();

    // выход
    let out_w = cols * advance_x;
    let out_h = rows * line_h;
    let mut out = ImageBuffer::from_pixel(out_w, out_h, Rgba([0, 0, 0, 255]));

    for row in 0..rows {
        for col in 0..cols {
            // область в исходнике
            let x0 = (col as f32 * tile_w) as u32;
            let y0 = (row as f32 * tile_h) as u32;
            let x1 = (((col + 1) as f32 * tile_w) as u32).min(w);
            let y1 = (((row + 1) as f32 * tile_h) as u32).min(h);
            if x1 <= x0 || y1 <= y0 {
                continue;
            }

            // средний цвет
            let mut sr = 0.0f32;
            let mut sg = 0.0f32;
            let mut sb = 0.0f32;
            let mut cnt = 0.0f32;

            let bg = Rgba([sr as u8, sg as u8, sb as u8, 255]);

let x_out0 = col * advance_x;
let y_out0 = row * line_h;
let x_out1 = (x_out0 + advance_x).min(out_w);
let y_out1 = (y_out0 + line_h).min(out_h);

for yy in y_out0..y_out1 {
    for xx in x_out0..x_out1 {
        *out.get_pixel_mut(xx, yy) = bg;
    }
}

            for yy in y0..y1 {
                for xx in x0..x1 {
                    let p = img.get_pixel(xx, yy).0;
                    sr += p[0] as f32;
                    sg += p[1] as f32;
                    sb += p[2] as f32;
                    cnt += 1.0;
                }
            }
            sr /= cnt;
            sg /= cnt;
            sb /= cnt;

            let y = luma01(sr, sg, sb);
            let ch = letter_pool::pick_by_luma(&charset, y);

            // позиция глифа (baseline)
            let px = (col * advance_x) as f32;
            let py = (row * line_h) as f32 + ascent as f32;

            let mut glyph = scaled.scaled_glyph(ch);
            glyph.position = ab_glyph::point(px, py);

            let color = [sr as u8, sg as u8, sb as u8];
            let mut glyph = scaled.scaled_glyph(ch);
            glyph.position = ab_glyph::point(px, py);

            if let Some(outlined) = scaled.outline_glyph(glyph) {
                let bounds = outlined.px_bounds(); // где этот глиф реально лежит в пикселях
                outlined.draw(|gx, gy, cov| {
                    // gx,gy идут ОТНОСИТЕЛЬНО bounds.min
                    let ox = bounds.min.x.floor() as i32 + gx as i32;
                    let oy = bounds.min.y.floor() as i32 + gy as i32;

                    if ox >= 0 && oy >= 0 && (ox as u32) < out_w && (oy as u32) < out_h {
                        let dst = out.get_pixel_mut(ox as u32, oy as u32);
                        blend_over(dst, color, cov);
                    }
                });
            }
        }
    }

    Ok(out)
}
