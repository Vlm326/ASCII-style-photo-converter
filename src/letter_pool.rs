pub fn pick_by_luma(charset: &[char], luma01: f32) -> char {
    let t = luma01.clamp(0.0, 1.0);
    let idx = (t * (charset.len() as f32 - 1.0)).round() as usize;
    charset[idx]
}
