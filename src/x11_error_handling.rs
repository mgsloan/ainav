
/*
// TODO: Idea here is to get a pointer to default xlib error handler, for use
// from the xcb one.

type ErrorCallback = unsafe extern "C" fn(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> c_int;

// https://mgattozzi.com/global-uninitialized
static mut DEFAULT_HANDLER: Option<ErrorCallback> = None;

unsafe fn default_handler() -> Option<&'static mut ErrorCallback> {
match DEFAULT_HANDLER {
Some(ref mut x) => Some(&mut *x),
None => None,
    }
}

unsafe extern "C" fn error_callback(dpy: *mut xlib::Display, event: *mut xlib::XErrorEvent) -> c_int {
    match default_handler() {
        Some(default) => default(dpy, event),
        None => { panic!("No Xlib error callback"); },
    }
} */

/*
unsafe {
match default_handler() {
Some(_) => {},
None => {
if DEFAULT_HANDLER_CELL.is_none() {
DEFAULT_HANDLER_CELL = &xlib::XSetErrorHandler(Some(error_callback));
        }
    }
    */
