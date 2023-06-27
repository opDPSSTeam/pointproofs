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

const GENERATE_PARAMS: [fn() -> String; 21] = [commit::gen_params::<4>, commit::gen_params::<7>, commit::gen_params::<10>, commit::gen_params::<13>, commit::gen_params::<16>, commit::gen_params::<19>, commit::gen_params::<22>, commit::gen_params::<25>, commit::gen_params::<28>, commit::gen_params::<31>, commit::gen_params::<34>, commit::gen_params::<37>, commit::gen_params::<40>, commit::gen_params::<43>, commit::gen_params::<46>, commit::gen_params::<49>, commit::gen_params::<52>, commit::gen_params::<55>, commit::gen_params::<58>, commit::gen_params::<61>,commit::gen_params::<64>];
const COMMIT: [fn(&str, &str) -> String; 21] = [commit::commit::<4>, commit::commit::<7>, commit::commit::<10>, commit::commit::<13>, commit::commit::<16>, commit::commit::<19>, commit::commit::<22>, commit::commit::<25>, commit::commit::<28>, commit::commit::<31>, commit::commit::<34>, commit::commit::<37>, commit::commit::<40>, commit::commit::<43>, commit::commit::<46>, commit::commit::<49>, commit::commit::<52>, commit::commit::<55>, commit::commit::<58>, commit::commit::<61>,commit::commit::<64>];
const OPEN: [fn(&str, &str, usize) -> String; 21] = [commit::open::<4>, commit::open::<7>, commit::open::<10>, commit::open::<13>, commit::open::<16>, commit::open::<19>, commit::open::<22>, commit::open::<25>, commit::open::<28>, commit::open::<31>, commit::open::<34>, commit::open::<37>, commit::open::<40>, commit::open::<43>, commit::open::<46>, commit::open::<49>, commit::open::<52>, commit::open::<55>, commit::open::<58>, commit::open::<61>,commit::open::<64>];
const VERIFY: [fn(&str, &str, &str, usize, &str) -> bool; 21] =
    [commit::verify::<4>, commit::verify::<7>, commit::verify::<10>, commit::verify::<13>, commit::verify::<16>, commit::verify::<19>, commit::verify::<22>, commit::verify::<25>, commit::verify::<28>, commit::verify::<31>, commit::verify::<34>, commit::verify::<37>, commit::verify::<40>, commit::verify::<43>, commit::verify::<46>, commit::verify::<49>, commit::verify::<52>, commit::verify::<55>, commit::verify::<58>, commit::verify::<61>,commit::verify::<64>];

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
