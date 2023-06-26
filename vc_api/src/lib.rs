mod commit;
mod util;

extern crate libc;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn fr_plus(s1: *const libc::c_char, s2: *const libc::c_char) -> *const libc::c_char {
    let s1_str = unsafe { CStr::from_ptr(s1).to_str().unwrap() };
    let s2_str = unsafe { CStr::from_ptr(s2).to_str().unwrap() };
    let f1 = util::str_to_message(s1_str);
    let f2 = util::str_to_message(s2_str);
    let res = util::message_to_str(f1 + f2);
    CString::new(res).unwrap().into_raw()
}

const GENERATE_PARAMS: [fn() -> String; 2] = [commit::gen_params::<4>, commit::gen_params::<7>];
const COMMIT: [fn(&str, &str) -> String; 2] = [commit::commit::<4>, commit::commit::<7>];
const OPEN: [fn(&str, &str, usize) -> String; 2] = [commit::open::<4>, commit::open::<7>];
const VERIFY: [fn(&str, &str, &str, usize, &str) -> bool; 2] =
    [commit::verify::<4>, commit::verify::<7>];

#[no_mangle]
pub extern "C" fn generate_params(number: libc::c_int) -> *const libc::c_char {
    let res = GENERATE_PARAMS[number as usize]();
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn commit(
    number: libc::c_int,
    srs: *const libc::c_char,
    messages: *const libc::c_char,
) -> *const libc::c_char {
    let srs = unsafe { CStr::from_ptr(srs).to_str().unwrap() };
    let messages = unsafe { CStr::from_ptr(messages).to_str().unwrap() };
    let res = COMMIT[number as usize](srs, messages);
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn open_(
    number: libc::c_int,
    srs: *const libc::c_char,
    messages: *const libc::c_char,
    pos: libc::c_int,
) -> *const libc::c_char {
    let srs = unsafe { CStr::from_ptr(srs).to_str().unwrap() };
    let messages = unsafe { CStr::from_ptr(messages).to_str().unwrap() };
    let res = OPEN[number as usize](srs, messages, pos as usize);
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn verify(
    number: libc::c_int,
    srs: *const libc::c_char,
    commitment: *const libc::c_char,
    message: *const libc::c_char,
    pos: libc::c_int,
    witness: *const libc::c_char,
) -> libc::c_int {
    let srs = unsafe { CStr::from_ptr(srs).to_str().unwrap() };
    let commitment = unsafe { CStr::from_ptr(commitment).to_str().unwrap() };
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };
    let witness = unsafe { CStr::from_ptr(witness).to_str().unwrap() };

    let res = VERIFY[number as usize](srs, commitment, message, pos as usize, witness);
    res as libc::c_int
}
