extern crate x11;
use x11::xlib;

pub fn get_focused_application_xorg() -> Result<(String, String), Box<dyn std::error::Error>> {
    unsafe {
        let display = xlib::XOpenDisplay(std::ptr::null());
        let mut window = 0;
        xlib::XGetInputFocus(display, &mut window, &mut 0);

        let class_hint = xlib::XAllocClassHint();
        xlib::XGetClassHint(display, window, class_hint);

        let res_name = std::ffi::CStr::from_ptr((*class_hint).res_name).to_str().unwrap().to_string();
        let res_class = std::ffi::CStr::from_ptr((*class_hint).res_class).to_str().unwrap().to_string();

        xlib::XFree(class_hint as *mut _);
        xlib::XCloseDisplay(display);

        Ok((res_name, res_class))
    }
}
