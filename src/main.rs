extern crate libc;

use std::ffi:: { CStr };
use std::os::raw:: { c_char, c_int , c_ulong };
use std::ptr;

#[repr(C)] pub struct Display { _private: [u8; 0] }
pub type Window = c_ulong;

#[link(name = "X11")]
extern {
        fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
        fn XDisplayString(display: *mut Display) -> *const c_char;
        fn XDefaultScreen(display: *mut Display) -> c_int;
        fn XRootWindow(display: *mut Display,  screen_no: c_int) -> Window;
        // extern int XDefaultScreen(
        //     Display*		/* display */
        // );
        // extern Window XRootWindow(
        //     Display*		/* display */,
        //     int			/* screen_number */
        // );
}

fn main() {
    let display : *mut Display = unsafe { XOpenDisplay(ptr::null()) };
    let display_string : &CStr = unsafe { CStr::from_ptr(XDisplayString(display)) };
    let screen : c_int = unsafe { XDefaultScreen(display) };
    let root : Window = unsafe { XRootWindow(display, screen) };

    println!("Hello, rustedmenu @Display {0} @Screen {1} @Window {2}!", display_string.to_string_lossy(), screen, root); 
}
