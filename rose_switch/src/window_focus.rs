extern crate x11;
use x11::xlib;

fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(std::ptr::null());
        let mut window = 0;
        xlib::XGetInputFocus(display, &mut window, &mut 0);

        let class_hint = xlib::XAllocClassHint();
        xlib::XGetClassHint(display, window, class_hint);

        println!("res_name: {}", std::ffi::CStr::from_ptr((*class_hint).res_name).to_str().unwrap());
        println!("res_class: {}", std::ffi::CStr::from_ptr((*class_hint).res_class).to_str().unwrap());

        xlib::XFree(class_hint as *mut _);
        xlib::XCloseDisplay(display);
    }
}