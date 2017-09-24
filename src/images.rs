use std::path::Path;
use bit_vec::*;
use captrs::*;
use enigo::*;
use image::*;

// TODO: optimize grayscale
//
// * https://stackoverflow.com/questions/3705320/fast-threshold-and-bit-packing-algorithm-possible-improvements
// * http://www.songho.ca/dsp/luminance/luminance.html

pub const LIGHT_LOWER: u8 = 75;
pub const DARK_UPPER: u8 = 150;

pub fn to_grayscale_threshold(frame: &[Bgr8], w: u32, h: u32) {
    let lightTextBitmap = BitVec::from_fn((w * h) as usize, |ix| bgr8_to_gray(frame[ix]) > 75);
    let darkTextBitmap = BitVec::from_fn((w * h) as usize, |ix| bgr8_to_gray(frame[ix]) < 150);
    save_grayscale("grayscale.png", frame, w, h);
    save_bitmap("light-text.png", lightTextBitmap, w, h);
    save_bitmap("dark-text.png", darkTextBitmap, w, h);
}

pub fn save_grayscale(path: &str, frame: &[Bgr8], w: u32, h: u32) {
    let image = ImageBuffer::from_fn(w, h, |x, y| Luma([bgr8_to_gray(frame[(y * w + x) as usize])]));
    image.save(&Path::new(path)).unwrap();
}

pub fn save_bitmap(path: &str, bitmap: BitVec, w: u32, h: u32) {
    // TODO: What happens here if w * h is larger than the bitmap?
    let image = ImageBuffer::from_fn(w, h, |x, y| Luma([if bitmap[(y * w + x) as usize] { 0 } else { 255 }]));
    image.save(&Path::new(path)).unwrap();
}

pub fn bgr8_to_gray(pixel: Bgr8) -> u8 {
    let Bgr8 {r, g, b, .. } = pixel;
    return (((r as u32 * 77) + (g as u32 * 151) + (b as u32 * 28)) >> 8) as u8;
}

pub fn jump_to_emacs_cursor(mut enigo: Enigo, frame: &[Bgr8], w: u32, h: u32) {
    'outer: for x in 0..w as u32 {
        for y in 0..h as u32 {
            let Bgr8 {r, g, b, .. } = frame[(y * w + x) as usize];
            if r == 238 && g == 173 && b == 14 {
                enigo.mouse_move_to(x as i32, y as i32);
                break 'outer;
            }
        }
    }
}
