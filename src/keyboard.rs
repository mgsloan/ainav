use screen;
use x11::xlib;

pub fn grab_keyboard(displays: &screen::DisplayInfo) {
    // FIXME: Instead this should be current screen.
    let root =
        if displays.screens.len() < 1 {
            // TODO: better message
            panic!("No screens")
        } else {
            displays.screens[0].root
        };

    unsafe {
        // TODO: Retry keyboard grabbing as in keynav?
        let grabstate = xlib::XGrabKeyboard(displays.dpy, root, false as i32,
                                            xlib::GrabModeAsync, xlib::GrabModeAsync, xlib::CurrentTime);
        if grabstate != xlib::GrabSuccess {
            panic!("Couldn't grab key events");
        }
    }
}
