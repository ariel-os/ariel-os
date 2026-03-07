//! Provides power management functionality.

// #![deny(missing_docs)]
#![cfg_attr(not(context = "native"), no_std)]

mod reset;

pub use reset::*;

/// Initializes power management.
///
/// *Important*: this needs to be called as early as possible in the boot sequence.
/// In particular, on microcontrollers whose reset reason needs to be cleared manually on each
/// reset, this needs to be called before anything else has the change to clear it.
/// This function may clear these bits.
#[doc(hidden)]
pub fn init() {
    reset::save_reset_reason();
}
