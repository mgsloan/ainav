use libc::{c_int, c_char};
use std::ffi::{CStr};
use std::mem;
use x11::xlib;
use xcb::base::*;
use xcb::xproto::*;
use xcb_util::keysyms::KeySymbols;

pub fn key_press_to_ascii(keysyms: &KeySymbols, key_press: &KeyPressEvent) -> u8 {
    let sym = keysyms.press_lookup_keysym(key_press, 0);
    // TODO: Is there a way to avoid using xlib functions?
    // TODO: Is the cast to u64 indicative of a bug?
    let sym_str = unsafe {
        let raw_str = xlib::XKeysymToString(sym as u64);
        CStr::from_ptr(raw_str)
    };
    // TODO: Remove println
    println!("{:?}", sym_str);
    return sym_str.to_bytes_with_nul()[0];
}

pub fn check<'a, T: Copy>(
    dpy: *mut xlib::Display,
    cookie: Cookie<'a, T>
) -> <Cookie<'a, T> as IsCookie>::Reply
where Cookie<'a, T> : IsCookie {
    check_impl(dpy, None, cookie)
}

pub fn check_ctx<'a, T: Copy>(
    dpy: *mut xlib::Display,
    context: &str,
    cookie: Cookie<'a, T>
) -> <Cookie<'a, T> as IsCookie>::Reply
where Cookie<'a, T> : IsCookie {
    check_impl(dpy, Some(context), cookie)
}

pub fn check_impl<'a, T: Copy>(
    dpy: *mut xlib::Display,
    context: Option<&str>,
    cookie: Cookie<'a, T>
) -> <Cookie<'a, T> as IsCookie>::Reply
where Cookie<'a, T> : IsCookie {
    match cookie.get_reply() {
        Ok(x) => { return x; }
        Err(e) => { panic_xcb(dpy, context, e.error_code()); }
    }
}

fn panic_xcb(dpy: *mut xlib::Display, context: Option<&str>, error_code: u8) -> ! {
    unsafe {
        // TODO: Prettier way to do this?
        let mut buf: [c_char; 256] = mem::uninitialized();
        let result = xlib::XGetErrorText(dpy, error_code as i32, buf.as_mut_ptr(), 256);
        if result != 0 {
            panic!("XCB function failed but the error code could not be rendered. Context: {:?}", context);
        }
        panic!("XCB function failed with {:?}. Context: {:?}", CStr::from_ptr(buf.as_mut_ptr()), context);
    }
}

/*
for font_name in check(dpy, list_fonts(&conn, 1, "*-misc-fixed-*")).names() {
println!("{:?}", font_name.name());
    } */
