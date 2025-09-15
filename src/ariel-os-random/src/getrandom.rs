//! Implementation of the getrandom ABI.
//! Enable it by using the custom backend from getrandom. See [Opt-in Backends](https://github.com/rust-random/getrandom?tab=readme-ov-file#opt-in-backends)

use crate::crypto_rng;
use getrandom::Error;
use rand_core::RngCore;

#[unsafe(no_mangle)]
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error> {
    let buf = unsafe {
        core::ptr::write_bytes(dest, 0, len);
        core::slice::from_raw_parts_mut(dest, len)
    };
    crypto_rng()
        .try_fill_bytes(buf)
        .map_err(|e| Error::new_custom(e.raw_os_error().unwrap() as u16))
}
