extern crate libc;

use std::ffi:: { CStr };
use std::os::raw:: { c_char };
use std::ptr;

#[repr(C)] pub struct Display { _private: [u8; 0] }

#[link(name = "X11")]
extern {
        fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
        fn XDisplayString(display: *mut Display) -> *const c_char;
}

fn main() {
    let mut display = unsafe { XOpenDisplay(ptr::null()) };
    let display_string : &CStr = unsafe { CStr::from_ptr(XDisplayString(display)) };
    println!("Hello, rustedmenu @Display {0}!", display_string.to_string_lossy()); 
}
