// extern crate xinerama;
// extern crate xlib;

use std::ptr::null_mut;
use std::slice::from_raw_parts;
use libc::{c_int};
use x11::xinerama::*;
use x11::xlib;
use x11::xlib::*;

// TODO: Support non-xinerama? See query_screens / query_screen_normal in keynav

// Note: Much of the code after this point copy-modified from
// https://github.com/tsurai/xr3wm/blob/78438b3e996ef42d7bbfe4e8da0631d5e4c67f31/src/xlib_window_system.rs#L468
//
// MIT licensed 2014 Cristian Kubis
//
// Changes:
//   * Ported to x11 / libc crates instead of xinerama / xlib crates
//   * Made them into plain functions instead of having a 'self'
//   * Added Window struct which includes root window
//   * Inlined get_display_width / get_display_height

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Screen {
    pub rect: Rect,
    pub screen: *mut xlib::Screen,
    pub root: Window,
}

#[derive(Clone, Debug)]
pub struct DisplayInfo {
    pub dpy: *mut Display,
    pub is_xinerama: bool,
    pub screens: Vec<Screen>,
}

pub fn get_display_info() -> DisplayInfo {
    unsafe {
        let dpy = XOpenDisplay(null_mut());
        let mut dummy = 0;
        if XineramaQueryExtension(dpy, &mut dummy, &mut dummy) != 0
           && XineramaIsActive(dpy) != 0 {
            return DisplayInfo {
                dpy: dpy,
                is_xinerama: true,
                screens: get_xinerama_screens(dpy),
            };
        } else {
            return DisplayInfo {
                dpy: dpy,
                is_xinerama: false,
                screens: get_normal_screens(dpy),
            };
        }
    }
}

fn get_xinerama_screens(display: *mut Display) -> Vec<Screen> {
    unsafe {
        // TODO: xr3wm has fallback for num = 0. Maybe this is just an alternative
        // way to check for xinerama?
        let mut num: c_int = 0;
        let screen_ptr = XineramaQueryScreens(display, &mut num);

        // TODO: Copied from keynav, is 0 correct?  Probably.
        let result_screen = XScreenOfDisplay(display, 0);;
        let result_root = XDefaultRootWindow(display);

        from_raw_parts(screen_ptr, num as usize)
            .iter()
            .map(|screen_info| {
                Screen {
                    rect: Rect {
                        x: screen_info.x_org as u32,
                        y: screen_info.y_org as u32,
                        width: screen_info.width as u32,
                        height: screen_info.height as u32,
                    },
                    screen: result_screen,
                    root: result_root,
                }
            })
            .collect()
    }
}

// TODO: Test this!  Untested because I use xinerama.
fn get_normal_screens(display: *mut Display) -> Vec<Screen> {
    unsafe {
        (0..(XScreenCount(display) - 1))
            .map(|i| {
                let screen = XScreenOfDisplay(display, i);
                return Screen {
                    rect: Rect {
                        x: 0,
                        y: 0,
                        width: (*screen).width as u32,
                        height: (*screen).height as u32,
                    },
                    screen: screen,
                    root: XRootWindowOfScreen(screen),
                };
            })
            .collect()
    }
}

/*
pub fn get_display_rect(display: *mut Display) -> Window{
    unsafe {
        Screen {
            rect: Rect {
                x: 0,
                y: 0,
                width: XDisplayWidth(display, screen as i32) as u32,
                height: XDisplayHeight(display, screen as i32) as u32,
            },
            root:
        }
    }
}

pub fn get_geometry(display: *mut Display, window: Window) -> Rect {
    unsafe {
        let mut root: Window = uninitialized();
        let mut x: c_int = uninitialized();
        let mut y: c_int = uninitialized();
        let mut width: c_uint = uninitialized();
        let mut height: c_uint = uninitialized();
        let mut depth: c_uint = uninitialized();
        let mut border: c_uint = uninitialized();

        XGetGeometry(display,
                     window,
                     &mut root,
                     &mut x,
                     &mut y,
                     &mut width,
                     &mut height,
                     &mut border,
                     &mut depth);

        Rect {
            x: x as u32,
            y: y as u32,
            width: width,
            height: height,
        }
    }
}

pub fn get_screen_infos(display: *mut Display) -> Vec<Rect> {
    unsafe {
        let mut num: c_int = 0;
        let screen_ptr = XineramaQueryScreens(display, &mut num);

        if num == 0 {
            return vec![get_display_rect(display)];
        }

        from_raw_parts(screen_ptr, num as usize)
            .iter()
            .map(|screen_info| {
                Rect {
                    x: screen_info.x_org as u32,
                    y: screen_info.y_org as u32,
                    width: screen_info.width as u32,
                    height: screen_info.height as u32,
                }
            })
            .collect()
    }
}
*/
