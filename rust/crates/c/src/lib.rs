// For more details, please see: https://www.greyblake.com/blog/exposing-rust-library-to-c/

use std::ffi::{c_char, CStr};

#[no_mangle]
pub extern "C" fn print_hello_from_rust() {
    println!("Hello from Rust");
}

#[repr(C)]
pub enum State {
    New,
    Init,
    Running,
    Done,
}

#[no_mangle]
pub extern "C" fn c_str_len(str: *const c_char) -> i32 {
    if str.is_null() {
        return -1;
    }

    let cstr = unsafe { CStr::from_ptr(str) };
    match cstr.to_str() {
        Ok(s) => s.len() as i32,
        Err(_) => -1,
    }
}

#[repr(C)]
pub struct CString {
    str: *const c_char,
    len: u32,
}

#[no_mangle]
pub extern "C" fn cstring_len(cstr: *const CString) -> i32 {
    if cstr.is_null() {
        return -1;
    }

    unsafe { c_str_len((*cstr).str) }
}

