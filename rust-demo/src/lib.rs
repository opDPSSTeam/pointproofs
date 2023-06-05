extern crate libc;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn rust_demo(
    name: *const libc::c_char,
    num: libc::c_int
) -> *const libc::c_char {
    let cstr_name = unsafe {
        CStr::from_ptr(name)
    };
    let mut str_name = cstr_name.to_str().unwrap().to_string();
    str_name.push_str(&num.to_string());
    CString::new(str_name).unwrap().into_raw()
}
