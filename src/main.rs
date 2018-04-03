// NOTE: no doubt much more required.
//
// sudo apt-get install libxcb-keysyms1-dev

extern crate xcb;
extern crate x11;
extern crate libc;
extern crate xcb_util;
extern crate image;

mod xcb_extras;

use x11::{xlib, xrandr};
use xcb::*;
use xcb_extras::*;
use xcb_util::keysyms::KeySymbols;
use xcb_util::image::shm;
use std::{slice, ptr};
use image::*;
use std::path::Path;
use std::vec::Vec;

// TODO: Defer some x11 response checking.

// TODO: Update cur_screen

// FIXME: from_raw_parts usage is totally broken in xcb library. Stuff never
// gets deallocated!  Same for usage in xcb_util::image

struct State<'a> {
    pub dpy: *mut xlib::Display,
    pub conn: &'a Connection,
    pub cur_screen: Screen<'a>,
    pub keysyms: KeySymbols<'a>,
    pub window: Window,
}

fn main() {
    let dpy = unsafe { xlib::XOpenDisplay(ptr::null_mut()) };
    let conn = unsafe { Connection::new_from_xlib_display(dpy) };
    let setup = conn.get_setup();
    let mut cur_screen = None;
    for screen in setup.roots() {
        // Ctrl+Space
        check(dpy, grab_key_checked(&conn, false, screen.root(), MOD_MASK_CONTROL, 65, GRAB_MODE_ASYNC, GRAB_MODE_ASYNC));
        cur_screen = Some(screen);
    }
    let window = conn.generate_id();
    let state = State {
        dpy,
        conn: &conn,
        cur_screen: cur_screen.unwrap(),
        keysyms: KeySymbols::new(&conn),
        window
    };
    let (x, y, width, height) = get_screen_dims(&state);
    let values = [
        (CW_OVERRIDE_REDIRECT as u32, 1u32)
    ];
    check(dpy, create_window_checked(
        &conn,
        24,
        window,
        state.cur_screen.root(),
        x as i16, y as i16, width as u16, height as u16, 0,
        WINDOW_CLASS_INPUT_OUTPUT as u16,
        state.cur_screen.root_visual(),
        &values));
    // let img = screenshot(&state);
    loop {
        let event = state.conn.wait_for_event().expect("IO error while waiting for key event");
        match event.response_type() {
            KEY_PRESS => {
                let key_press: &KeyPressEvent = unsafe { cast_event(&event) };
                println!("Key pressed: {}", key_press.detail());
                if key_press.detail() == 65 {
                    start(&state);
                }
            }
            _ => {
            }
        }
    }
}

fn start (state: &State) {
    check(state.dpy, map_window_checked(state.conn, state.window));
    check(state.dpy, grab_keyboard(state.conn, false, state.cur_screen.root(), CURRENT_TIME, GRAB_MODE_ASYNC, GRAB_MODE_ASYNC));
    loop {
        let event = state.conn.wait_for_event().expect("IO error while waiting for key event");
        match event.response_type() {
            KEY_PRESS => {
                let key_press: &KeyPressEvent = unsafe { cast_event(&event) };
                println!("Key pressed: {}", key_press.detail());
                check(state.dpy, ungrab_keyboard_checked(state.conn, CURRENT_TIME));
                let character = key_press_to_ascii(&state.keysyms, key_press);
                println!("{:?}", character);
                let image = screenshot(state);
                save_image("screenshot.png", &image);
                check(state.dpy, unmap_window_checked(state.conn, state.window));
                break;
                // let results = find_chars_list(character);
            }
            _ => {}
        }
    }
}

fn screenshot(state: &State) -> xcb_util::image::Image {
    let planes = unsafe { xlib::XAllPlanes() } as u32;
    xcb_util::image::get(state.conn, state.window, 200, 200, 200, 200, planes, IMAGE_FORMAT_Z_PIXMAP).unwrap()
}

fn save_image(path: &str, image: &xcb_util::image::Image) {
    let buf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(image.width() as u32, image.height() as u32, |x, y| {
        let rgba = image.get(x, y);
        // TODO: more efficient coercion
        return Rgba([
            ((rgba >> 24) & 0xff) as u8,
            ((rgba >> 16) & 0xff) as u8,
            ((rgba >> 8) & 0xff) as u8,
            (rgba & 0xff) as u8,
        ]);
    });
    /*
    println!("{} {} {} {} {} {}", image.width(), image.height(), image.format(), image.depth(), image.bpp(), image.unit());
    let buf: ImageBuffer<Rgba<u8>, &[u8]> =
        ImageBuffer::from_raw(20, 10, image.data()).unwrap();
    */
    buf.save(&Path::new(path)).unwrap();
}

// FIXME
//
// * it needs to have a cookie interface.
//
// * Also, why does it have conn in the image?
//
// * format finding code in 'create' seems wrong too.
//
// * resize function seems iffy.
//
// * Ugly that shm::get returns image.
//
// TODO: Nice to have a higher level API that takes ownership of Image until get
// returns, same for other get operations

/*
sudo apt-get install libxcb-keysyms1-dev libxcb-image0-dev

xcb-util = { version = "0.2.0", features = ["keysyms", "image", "shm"] }
*/

/*
fn screenshot(state: &State) -> shm::Image {
    // TODO: XCB constant for AllPlanes?
    let planes = unsafe { xlib::XAllPlanes() } as u32;
    let depth = 24;
    // let width = state.cur_screen.width_in_pixels();
    // let height = state.cur_screen.height_in_pixels();
    // let (x, y, width, height) = get_screen_dims(state);
    let x = 200;
    let y = 200;
    let width = 200;
    let height = 200;

    let mut image = shm::create(state.conn, depth, width as u16, height as u16).expect("Failed to allocated shared memory for screenshot");
    // shm::get(state.conn, state.cur_screen.root(), &mut image, x as i16, y as i16, planes).unwrap();
    shm::get(state.conn, state.window, &mut image, x as i16, y as i16, 0).unwrap();
    return image;

    /*
    if image.depth() == 24 && image.bpp() == 32 &&
       image.height() == height && image.width() == width &&
       image.scanline_pad() == 0
        // TODO: check image.byte_order
        /* TODO x11-cap crate checks these too
        image.red_mask == 0xFF0000 && image.green_mask == 0xFF00 &&
        image.blue_mask == 0xFF */ {
        return image;
    } else {
        panic!("X Server yielded screenshot with unexpected format");
    } */
}
*/

/*
fn screenshot(state: &State) {
    let screenshot_cookie =
        get_image(state.conn,
                  IMAGE_FORMAT_Z_PIXMAP,
                  state.cur_screen.root(),
                  0,
                  0,
                  state.cur_screen.width_in_pixels(),
                  state.cur_screen.height_in_pixels(),
                  // TODO: XCB constant for AllPlanes?
                  unsafe { xlib::XAllPlanes() } as u32);
    // let screenshot_image = check(state.dpy, screenshot_cookie);
    // FIXME: Check screenshot_image.visual() and .depth(). See
    // https://xcb.freedesktop.org/xlibtoxcbtranslationguide/#defaultvisualdefaultvisualofscreen
}
 */

fn get_screen_dims(state: &State) -> (i32, i32, i32, i32) {
    // TODO: use xcb instead -
    // https://stackoverflow.com/questions/22108822/how-do-i-get-the-resolution-of-randr-outputs-through-the-xcb-randr-extension
    let mon_i = 0;
    let mut n_mons = 0;
    let mons = unsafe {
        xrandr::XRRGetMonitors(state.dpy, state.cur_screen.root() as u64, 1, &mut n_mons)
    };
    let mons = unsafe { slice::from_raw_parts_mut(mons, n_mons as usize) };
    let mon = mons[mon_i];
    let (x, y, width, height) = (mon.x, mon.y, mon.width, mon.height);
    println!("{} {} {} {}", x, y, width, height);
    return (x, y, width, height);
}
