mod util;
mod commit;

extern crate libc;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn fr_plus(
    s1: *const libc::c_char,
    s2: *const libc::c_char
) -> *const libc::c_char {
    let s1_str = unsafe {
        CStr::from_ptr(s1).to_str().unwrap()
    };
    let s2_str = unsafe {
        CStr::from_ptr(s2).to_str().unwrap()
    };
    let f1 = util::str_to_message(s1_str);
    let f2 = util::str_to_message(s2_str);
    let res = util::message_to_str(f1 + f2);
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn generate_params_1() -> *const libc::c_char {
    let mut rng = rand::thread_rng();
    let srs = commit::gen_params::<rand::rngs::ThreadRng, 4>(&mut rng);
    let res = util::srs_to_str(srs);
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn commit_1(
    srs: *const libc::c_char,
    messages: *const libc::c_char
) -> *const libc::c_char {
    let srs = unsafe {
        CStr::from_ptr(srs).to_str().unwrap()
    };
    let messages = unsafe {
        CStr::from_ptr(messages).to_str().unwrap()
    };
    let srs = util::str_to_srs::<4>(srs);
    let messages = util::str_to_messages(messages);
    let commitment = commit::commit(srs, messages);
    let res = util::g1_to_str(commitment);
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn open_1(
    srs: *const libc::c_char,
    messages: *const libc::c_char,
    pos: libc::c_int
) -> *const libc::c_char {
    let srs = unsafe {
        CStr::from_ptr(srs).to_str().unwrap()
    };
    let messages = unsafe {
        CStr::from_ptr(messages).to_str().unwrap()
    };
    let srs = util::str_to_srs::<4>(srs);
    let messages = util::str_to_messages(messages);
    let witness = commit::open(srs, messages, pos as usize);
    let res = util::g1_to_str(witness);
    CString::new(res).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn verify_1(
    srs: *const libc::c_char,
    commitment: *const libc::c_char,
    message: *const libc::c_char,
    pos: libc::c_int,
    witness: *const libc::c_char
) -> libc::c_int {
    let srs = unsafe {
        CStr::from_ptr(srs).to_str().unwrap()
    };
    let commitment = unsafe {
        CStr::from_ptr(commitment).to_str().unwrap()
    };
    let message = unsafe {
        CStr::from_ptr(message).to_str().unwrap()
    };
    let witness = unsafe {
        CStr::from_ptr(witness).to_str().unwrap()
    };

    let srs = util::str_to_srs::<4>(srs);
    let commitment = util::str_to_g1(commitment);
    let message = util::str_to_message(message);
    let witness = util::str_to_g1(witness);
    let res = commit::verify(srs, commitment, message, pos as usize, witness);
    res as libc::c_int
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::FrParameters;
    use ark_ff::Fp256;
    use ark_ff::UniformRand;
    use rand_chacha::ChaCha20Rng;
    use ark_std::rand::SeedableRng;
    use ark_bls12_381::Bls12_381;
    use ark_ec::PairingEngine;

    #[test]
    fn test_srs_str() {
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        for _ in 0..10 {
            let srs = commit::gen_params::<ChaCha20Rng, 4>(&mut rng);
            let s = util::srs_to_str(srs.clone());
            let srs1 = util::str_to_srs(s.as_str());
            assert_eq!(srs, srs1);
        }
    }

    #[test]
    fn test_message_str() {
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        for _ in 0..10 {
            let v: Vec<Fp256<FrParameters>> = (0..4)
                .map(|_| <Bls12_381 as PairingEngine>::Fr::rand(&mut rng))
                .collect();
            let v1 = util::str_to_messages(&util::messages_to_str(v.clone()));
            assert_eq!(v, v1);
        }
        for _ in 0..10 {
            let v = <Bls12_381 as PairingEngine>::Fr::rand(&mut rng);
            let v1 = util::str_to_message(&util::message_to_str(v));
            assert_eq!(v, v1);
        }
    }

    #[test]
    fn it_works() {
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        for _ in 0..10 {
            let srs = commit::gen_params::<ChaCha20Rng, 4>(&mut rng);
            let srs_str = util::srs_to_str(srs);
            let srs = util::str_to_srs::<4>(&srs_str);
            let messages = (0..4)
                .map(|_| <Bls12_381 as PairingEngine>::Fr::rand(&mut rng))
                .collect::<Vec<Fp256<FrParameters>>>();
            let messages_str = util::messages_to_str(messages.clone());
            let messages = util::str_to_messages(&messages_str);
            let commitment = commit::commit(srs, messages);
            let commmit_str = util::g1_to_str(commitment);

            let srs = util::str_to_srs::<4>(&srs_str);
            let messages = util::str_to_messages(&messages_str);
            let open_value_str = util::message_to_str(messages[2]);
            let witness = commit::open(srs, messages, 2);
            let witness_str = util::g1_to_str(witness);
            let srs = util::str_to_srs::<4>(&srs_str);
            let commitment = util::str_to_g1(&commmit_str);
            let witness = util::str_to_g1(&witness_str);
            let open_value = util::str_to_message(&open_value_str);
            assert!(commit::verify(srs, commitment, open_value, 2, witness))
        }
    }
}
