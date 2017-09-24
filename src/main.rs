#![allow(non_upper_case_globals)]

// TODO: Is it possible to avoid copying screen to a buffer? I.e. raw access?
// Seems like this might be possible if using a screen buffer, but not with GPU.

// TODO: When entering into the keypress mode, do as much pre-processing as
// possible (grayscaling, text region detection). Like byzanz, use XDamage
// https://www.freedesktop.org/wiki/Software/XDamage/ to invalidate parts of
// this cache.  captrs can't do partial screeen capture though..

extern crate bit_vec;
extern crate captrs;
extern crate enigo;
extern crate image;
extern crate freetype;
extern crate log;
extern crate x11;
extern crate libc;

use std::convert::From;
use std::ptr::null_mut;
use captrs::*;
use enigo::*;
use image::*;
use std::mem;
use std::path::Path;
use std::time;
use std::thread;
use freetype::{face};
use bit_vec::*;
use libc::{malloc};
use x11::xlib::*;
use std::ffi::CStr;
use screen::*;

mod images;
mod glyph;
mod screen;
mod keyboard;

// TODO: x11cap (via captrs) also calls XOpenDisplay. Not so pretty.
// https://github.com/bryal/X11Cap/issues/4

#[derive(Debug)]
pub struct State {
    pub displays: DisplayInfo,
}

fn main() {
    // TODO: Instead require and pass in value of DISPLAY environment
    // variable, like in keynav?
    let state = State {
        displays: get_display_info()
    };
    let dpy = state.displays.dpy;

    for screen in &state.displays.screens {
        let keycode = 47;
        let mods = ControlMask;
        println!("{} {} {}", keycode, mods, screen.root);
        unsafe {
            XGrabKey(dpy, keycode, mods, screen.root, false as i32, GrabModeAsync, GrabModeAsync);
        }
    }

    unsafe {
        XSync(dpy, false as i32);
    }

    let mut event = XEvent { pad: [0;24] };
    loop {
        // TODO: what is the int that this returns?
        let _ = unsafe { XNextEvent(dpy, &mut event) };
        match event.get_type() {
            KeyPress => {
                let key_event: XKeyEvent = From::from(event);
                println!("Key press received {}", key_event.keycode);
                if key_event.keycode == 47 {
                    start(&state);
                }
            }
            _ => {
            }
        }
    }
}

fn start(state: &State) {
    // FIXME: Avoid recomputation. Need to learn borrow stuff.
    let dpy = state.displays.dpy;
    // let displays = screen::get_displays(dpy);
    keyboard::grab_keyboard(&state.displays);
    // TODO: what is the int that this returns?
    let mut event = XEvent { pad: [0;24] };
    loop {
        let _ = unsafe { XNextEvent(dpy, &mut event) };
        match event.get_type() {
            KeyPress => {
                let key_event: XKeyEvent = From::from(event);
                let index = if key_event.state & 0x2000 != 0 { 2 } else { 0 } /* ISO Level3 Shift */
                          + if key_event.state & ShiftMask != 0 { 1 } else { 0 };
                // FIXME: Is the cast symptomatic of a bug?
                let str = unsafe {
                    let sym = XkbKeycodeToKeysym(dpy, key_event.keycode as u8, 0, index);
                    let raw_str = XKeysymToString(sym);
                    CStr::from_ptr(raw_str)
                };
                println!("{:?}", str);
                unsafe { XUngrabKeyboard(dpy, CurrentTime); }
                let character = str.to_bytes_with_nul()[0] as usize;
                let results = find_chars_list(character);
                break;
            }
            _ => {
            }
        }
    }

    /*
    unsafe {
        XUngrabKeyboard(dpy, CurrentTime);
    } */
}

fn show_chars(results: Vec<(u32, u32)>) {
    // XUnmapWindow(dpy, zone);
    /*
    for (x, y) in results {
    } */
}

fn create_window() {
}

fn find_chars_list(character: usize) -> Vec<(u32, u32)> {
    let freetype = freetype::Library::init().unwrap();
    // let start = PreciseTime::now();
    let font_face = freetype.new_face("/usr/share/fonts/truetype/hack/Hack-Regular.ttf", 0).unwrap();
    font_face.set_char_size(14 * 64, 0, 72, 0).unwrap();
    // TODO: See if TARGET_MONO is better also compare performance.
    let font_options = face::RENDER;
    font_face.load_char(character, font_options).unwrap();
    // let end = PreciseTime::now();
    let glyph = font_face.glyph();
    let bitmap = glyph.bitmap();
    /*
    let bitvec = BitVec::from_bytes(bitmap.buffer());
    let charWidth = bitmap.width() as u32;
    let charHeight = bitmap.rows() as u32;
    */
    let buffer: ImageBuffer<Luma<u8>, &[u8]> =
        ImageBuffer::from_raw(bitmap.width() as u32, bitmap.rows() as u32, bitmap.buffer()).unwrap();
    /*
    println!("glyph took {} seconds.", start.to(end));
     */
    buffer.save(&Path::new("char.png")).unwrap();

    let mut capturer = Capturer::new(0).unwrap();
    let mut enigo = Enigo::new();
    let (w, h) = capturer.geometry();
    capturer.capture_store_frame().unwrap();
    let frame = capturer.get_stored_frame().unwrap();

    images::to_grayscale_threshold(frame, w, h);
    let results = find_char(bitmap.buffer(), bitmap.width() as u32, bitmap.rows() as u32, frame, w, h);
    return results;
    /*
    for (x, y) in results {
        enigo.mouse_move_to(x as i32, y as i32);
        break;
        // thread::sleep(time::Duration::from_millis(100));
    } */
    /*
    images::to_grayscale_threshold(frame, w, h);
    images::jump_to_emacs_cursor(enigo, frame, w, h);
    */
}

fn find_char(cbuf: &[u8], cw: u32, ch: u32, buf: &[Bgr8], w: u32, h: u32) -> Vec<(u32, u32)> {
    let mut found_chars = Vec::new();
    let thresh = (cw * ch) / 3 * 2;
    for is_light in [true].iter() {
        let result = ImageBuffer::from_fn((w - cw) / 2, (h - ch) / 2, |x, y| {
            let mut acc: u32 = 0;
            for cx in 0..(cw - 1) {
                for cy in 0..(ch - 1) {
                    let screen_val = images::bgr8_to_gray(buf[(x * 2 + cx + (y * 2 + cy) * w) as usize]);
                    let char_val = cbuf[(cx + cy * cw) as usize];
                    let in_char = char_val > images::LIGHT_LOWER;
                    let matches =
                        if *is_light { (screen_val > images::LIGHT_LOWER) == in_char }
                        else { (screen_val < images::DARK_UPPER) == in_char };
                    if matches {
                        acc += 1;
                    }
                }
            }
            // println!("result({}, {}) = {}\n {} {} {}", x, y, acc, cw, ch, cw * ch);
            if acc > thresh {
                found_chars.push((x * 2, y * 2));
            }
            return Luma([if acc > 255 { 255 } else {acc as u8}]);
        });
        result.save(&Path::new(if *is_light { "light.png" } else { "dark.png" })).unwrap();
    }
    return found_chars;
}



/* TODO: Get something like this to work with frame capturing

fn show_time<'a, r>(name: &str, f: &FnOnce() -> &'a r) -> &'a r {
    use time::PreciseTime;
    let start = PreciseTime::now();
    let r = f();
    let end = PreciseTime::now();
    println!("{} took {} seconds.", name, start.to(end));
    return r;
}
*/

/* Bitvec version
NOTE: broken if you switch it to some character other than A

// TODO: See if TARGET_MONO is better also compare performance.
let font_options = face::RENDER | face::NO_HINTING | face::MONOCHROME;
font_face.load_char('A' as usize, font_options).unwrap();
let end = PreciseTime::now();
let glyph = font_face.glyph();
let bitmap = glyph.bitmap();
let bitvec = BitVec::from_bytes(bitmap.buffer());
let charWidth = bitmap.width() as u32;
let charHeight = bitmap.rows() as u32;
let buffer = ImageBuffer::from_fn(charWidth, charHeight,
                                  |x, y| Luma([if bitvec[(y * (charWidth / 8 + 1) * 8 + x) as usize] { 0 } else { 255 }]));
*/
