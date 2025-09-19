//! Provides a seeded random number generator depending on Ariel OS's configuration.
//!
//! The module provides functions for use by applications, [`fast_rng()`] and [`crypto_rng()`],
//! which produce owned types that provide the [`rand_core::RngCore`] and
//! [`rand_core::CryptoRng`] traits, respectively.
//!
//! The crate abstracts over multiple aspects of RNGs:
//! * Where do we take a valid seed for the RNG from?
//! * What's the type of RNG that we take along?
//! * Is RNG state shared across cores, threads, tasks or not at all?
//!
//! No matter the choices taken (eventually through the application's setup), all is hidden behind
//! the [`FastRng`] and [`CryptoRng`] types.
//!
//! Before accessing the RNG, it needs to be initialized through the [`construct_rng()`] function.
//! This is taken care of by the `ariel-os-embassy` initialization functions. Applications can
//! ensure that this has happened by depending on the laze feature `random`.
//!
//! ---
//!
//! Currently, this provides very little choice, and little fanciness: It (more or less
//! arbitrarily) uses the [`rand_chacha::ChaCha20Rng`] generator as a shared global RNG, and
//! [`rand_pcg::Pcg32`] is decided yet for the fast one. Neither the algorithm nor the size of
//! [`FastRng`] or [`CryptoRng`] is guaranteed.
#![no_std]

#[cfg(feature = "csprng")]
mod getrandom;

use core::{cell::RefCell, marker::PhantomData};

use embassy_sync::once_lock::OnceLock;
use rand_core::{RngCore, SeedableRng as _};

/// A global RNG.
// The Mutex<RefCell> can probably be simplified
static RNG: OnceLock<RefCell<SelectedRng>> = OnceLock::new();

/// Type of the global RNG when needing the ability to produce cryptographically secure random
/// numbers.
///
/// If calls to [`rng()`] are rare, it may even make sense to move the HWRNG in here to get a
/// ZST global.
#[cfg(feature = "csprng")]
pub(crate) type SelectedRng = rand_chacha::ChaCha20Rng;

/// Type of the global RNG when cryptographically secure random numbers are not needed.
#[cfg(not(feature = "csprng"))]
pub(crate) type SelectedRng = rand_pcg::Pcg32;

/// Locks the global RNG for a single operation.
///
/// # Panics
///
/// … if initialization did not happen.
///
/// # Deadlocks
///
/// … if the action attempts to lock RNG.
fn with_global<R>(action: impl FnOnce(&mut SelectedRng) -> R) -> R {
    let rng = RNG
        .try_get()
        .expect("Initialization should have populated RNG");
    action(&mut rng.borrow_mut())
}

/// The OS provided fast random number generator.
///
/// This will generally be faster to produce random numbers than [`CryptoRng`].
///
/// Such an RNG can be requested by any component, and will always be seeded appropriately.
pub struct FastRng {
    inner: rand_pcg::Pcg32,
    // Make the type not Send to later allow using thread-locals
    _private: core::marker::PhantomData<*const ()>,
}

// Re-implementing the trait rather than Deref'ing into inner: This avoids leaking implementation
// details to users who might otherwise come to depend on platform specifics of the FastRng.
impl RngCore for FastRng {
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest);
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.inner.try_fill_bytes(dest)
    }
}

/// Same as [`FastRng`], but can be shared across threads and tasks.
/// This is to differentiate from [`FastRng`] that could get optimized in a future version to use thread-local storage.
/// Should be used only when sharing between threads is necessary (e.g. integrating with C code).
#[doc(hidden)]
pub struct FastRngSend {
    inner: rand_pcg::Pcg32,
}

// Re-implementing the trait rather than Deref'ing into inner: This avoids leaking implementation
// details to users who might otherwise come to depend on platform specifics of the FastRng.
impl RngCore for FastRngSend {
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest);
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.inner.try_fill_bytes(dest)
    }
}

/// The OS provided cryptographically secure random number generator.
///
/// Such an RNG can be requested by any component, and will always be seeded appropriately.
#[cfg(feature = "csprng")]
pub struct CryptoRng {
    // Make the type not Send to later allow using thread-locals
    pub(crate) _private: PhantomData<*const ()>,
}

#[cfg(feature = "csprng")]
mod csprng {
    use super::{CryptoRng, RngCore, SelectedRng, with_global};

    // Re-implementing the trait rather than Deref'ing into inner: This avoids leaking implementation
    // details to users who might otherwise come to depend on platform specifics of the CryptoRng.
    impl RngCore for CryptoRng {
        fn next_u32(&mut self) -> u32 {
            with_global(RngCore::next_u32)
        }
        fn next_u64(&mut self) -> u64 {
            with_global(RngCore::next_u64)
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            with_global(|i| i.fill_bytes(dest));
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            with_global(|i| i.try_fill_bytes(dest))
        }
    }

    impl rand_core::CryptoRng for super::CryptoRng {}

    /// Asserts that [`SelectedRng`] is [`CryptoRng`], justifying the implementation above.
    trait _AssertCryptoRng: rand_core::CryptoRng {}
    impl _AssertCryptoRng for SelectedRng {}
}

/// Same as [`CryptoRng`], but can be shared across threads and tasks.
/// This is to differentiate from [`CryptoRng`] that could get optimized in a future version to use thread-local storage.
/// Should be used only when sharing between threads is necessary (e.g. integrating with C code).
#[doc(hidden)]
#[cfg(feature = "csprng")]
pub struct CryptoRngSend {
    inner: rand_chacha::ChaCha20Rng,
}

#[doc(hidden)]
#[cfg(feature = "csprng")]
mod csprng_send {
    use super::{CryptoRngSend, RngCore};

    // Re-implementing the trait rather than Deref'ing into inner: This avoids leaking implementation
    // details to users who might otherwise come to depend on platform specifics of the CryptoRng.
    impl RngCore for CryptoRngSend {
        fn next_u32(&mut self) -> u32 {
            self.inner.next_u32()
        }
        fn next_u64(&mut self) -> u64 {
            self.inner.next_u64()
        }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            self.inner.fill_bytes(dest);
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.inner.try_fill_bytes(dest)
        }
    }

    impl rand_core::CryptoRng for super::CryptoRngSend {}
}

/// Populates the global RNG from a seed value.
///
/// # Panics
///
/// - Panics if the underlying RNG returns an error.
/// - Panics if this function is called multiple times.
#[doc(hidden)]
pub fn construct_rng(hwrng: impl RngCore) {
    RNG.init(RefCell::new(
        SelectedRng::from_rng(hwrng).expect("Hardware RNG failed to provide entropy"),
    ))
    .unwrap();
}

/// Returns a suitably initialized fast random number generator.
#[expect(clippy::missing_panics_doc, reason = "does not panic")]
#[must_use]
#[inline]
pub fn fast_rng() -> FastRng {
    FastRng {
        inner: with_global(|i| rand_pcg::Pcg32::from_rng(i).expect("Global RNG is infallible")),
        _private: PhantomData,
    }
}

/// Same as [`fast_rng()`]. Can be shared across threads and tasks.
/// This is to differentiate from [`fast_rng()`] that could get optimized in a future version to use thread-local storage.
/// Should be used only when sharing between threads is necessary (e.g. integrating with C code).
#[doc(hidden)]
#[must_use]
#[inline]
pub fn fast_rng_send() -> FastRngSend {
    FastRngSend {
        inner: with_global(|i| rand_pcg::Pcg32::from_rng(i).expect("Global RNG is infallible")),
    }
}

/// Returns a suitably initialized cryptographically secure random number generator.
#[must_use]
#[inline]
#[cfg(feature = "csprng")]
pub fn crypto_rng() -> CryptoRng {
    CryptoRng {
        _private: PhantomData,
    }
}

/// Same as [`crypto_rng()`]. Can be shared across threads and tasks.
/// This is to differentiate from [`crypto_rng()`] that could get optimized in a future version to use thread-local storage.
/// Should be used only when sharing between threads is necessary (e.g. integrating with C code).
#[doc(hidden)]
#[must_use]
#[inline]
#[cfg(feature = "csprng")]
pub fn crypto_rng_send() -> CryptoRngSend {
    CryptoRngSend {
        inner: with_global(|i| {
            rand_chacha::ChaCha20Rng::from_rng(i).expect("Global RNG is infallible")
        }),
    }
}
