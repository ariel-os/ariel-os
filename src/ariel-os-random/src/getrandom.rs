//! Glue code to support the `getrandom` crate.
#![expect(
    unsafe_code,
    reason = "providing the getrandom() custom implementation needs unsafe operations"
)]

use rand_core::RngCore as _;

use getrandom::Error;

/// Implements RNG access for the `getrandom` crate
///
/// # Safety
///
/// See the `getrandom` documentation on custom backends.
///
/// # Errors
///
/// See the `getrandom` documentation on custom backends.
///
/// At the time of writing, all Ariel OS RNGs are infallible.
///
/// # Panics
///
/// The function panics if error conversion fails.
#[unsafe(no_mangle)]
#[allow(clippy::unnecessary_wraps)]
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error> {
    // SAFETY: Pointer validity and mutability is provided by the getrandom custom backend
    // conventions.
    let buf = unsafe {
        core::ptr::write_bytes(dest, 0, len);
        core::slice::from_raw_parts_mut(dest, len)
    };
    super::crypto_rng().fill_bytes(buf);
    Ok(())
}
