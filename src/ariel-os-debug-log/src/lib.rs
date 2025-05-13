//! Provides debug logging facilities.
//!
//! # Syntax of formatting strings
//!
//! The behavior of the provided logging macros depends on which Cargo feature is enabled:
//! - When the `defmt` feature is enabled, `defmt` is used for logging.
//! - When the `log` feature is enabled, `log` is used for logging.
//! - Otherwise, the logging macros are no-ops.
//!
//! This means that the syntax of the formatting strings differs depending on the enabled Cargo
//! feature; please refer to the documentation of those crates for details on the supported syntax.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(missing_docs)]
#![deny(clippy::pedantic)]

#[featurecomb::comb]
mod _featurecomb {}

/// These re-exports can be used to satisfy the requirements of the `info!` and similar macros,
/// either by deriving [`Format`] on your own data structures, or by falling back to [`Debug2Format`]
/// or [`Display2Format`] to show third party data structures that do not have [`Format`] derived.
pub use defmt::{Debug2Format, Display2Format, Format};

/// This pub reexport is needed in addition to [`Format`] above because the generated
/// implementation needs to have access to a `defmt` module.
pub use defmt;

#[cfg(feature = "log")]
#[doc(hidden)]
pub mod log {
    // Re-export only the minimum set of items to minimize breaking changes in case `log`
    // adds/removes any items.
    pub use log::{debug, error, info, trace, warn};
}

// NOTE: log macros are defined within private modules so that `doc_auto_cfg` does not produce
// "feature flairs" on them.
// The macros are still exported even though they are defined "within" private modules.
//
// The `if true` conditionals ensure that the arguments are also valid under format_args!; this is
// requires because a laze switch can just make the back-end switch over. The actual and the unused
// formatting need to be in different branches to avoid trouble due to arguments being moved.
#[cfg(feature = "defmt")]
mod log_macros {
    /// Logs a message at the trace level.
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {{
            use $crate::defmt;
            if true {
                defmt::trace!($($arg)*);
            } else {
                drop(format_args!($($arg)*));
            }
        }};
    }

    /// Logs a message at the debug level.
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {{
            use $crate::defmt;
            if true {
                defmt::debug!($($arg)*);
            } else {
                drop(format_args!($($arg)*));
            }
        }};
    }

    /// Logs a message at the info level.
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {{
            use $crate::defmt;
            if true {
                defmt::info!($($arg)*);
            } else {
                drop(format_args!($($arg)*));
            }
        }};
    }

    /// Logs a message at the warn level.
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {{
            use $crate::defmt;
            if true {
                defmt::warn!($($arg)*);
            } else {
                drop(format_args!($($arg)*));
            }
        }};
    }

    /// Logs a message at the error level.
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {{
            use $crate::defmt;
            if true {
                defmt::error!($($arg)*);
            } else {
                drop(format_args!($($arg)*));
            }
        }};
    }
}

#[cfg(feature = "log")]
mod log_macros {
    /// Logs a message at the trace level.
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {{
            $crate::log::trace!($($arg)*);
        }};
    }

    /// Logs a message at the debug level.
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {{
            $crate::log::debug!($($arg)*);
        }};
    }

    /// Logs a message at the info level.
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {{
            $crate::log::info!($($arg)*);
        }};
    }

    /// Logs a message at the warn level.
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {{
            $crate::log::warn!($($arg)*);
        }};
    }

    /// Logs a message at the error level.
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {{
            $crate::log::error!($($arg)*);
        }};
    }
}

// Define no-op macros in case no facade is enabled.
#[cfg(not(any(feature = "defmt", feature = "log")))]
mod log_macros {
    /// Logs a message at the trace level.
    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {};
    }

    /// Logs a message at the debug level.
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {};
    }

    /// Logs a message at the info level.
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {};
    }

    /// Logs a message at the warn level.
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {};
    }

    /// Logs a message at the error level.
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {};
    }
}

/// A newtype around byte slices used for all of Ariel OS's logging facades that turns the bytes
/// into some hex output.
///
/// Its preferred output is `00 11 22 33`, but log facades may also produce something like `[00,
/// 11, 22, 33]` (eg. while that is cheaper on defmt).
///
/// Instead of writing some variation of `info!("Found bytes {:02x}", data)`, you can write
/// `info!("Found bytes {}", Hex(data))`.
pub struct Hex<T: AsRef<[u8]>>(pub T);

impl<T: AsRef<[u8]>> core::fmt::Display for Hex<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02x?}", self.0.as_ref())
    }
}

#[cfg(feature = "defmt")]
impl<T: AsRef<[u8]>> defmt::Format for Hex<T> {
    fn format(&self, f: defmt::Formatter) {
        ::defmt::write!(f, "{=[u8]:02x}", self.0.as_ref())
    }
}

/// A newtype around byte slices used for all of Ariel OS's logging facades that prefers
/// interpreting the data as CBOR.
///
/// Its preferred output is CBOR Diagnostic Notation (EDN), but showing hex is also acceptable.
///
/// Instead of writing some variation of `info!("Found bytes {:cbor}", item)`, you can write
/// `info!("Found bytes {}", Cbor(item))`.
///
/// Note that using this wrapper is not necessary when using a
/// [`cboritem::CborItem`](https://docs.rs/cboritem/latest/cboritem/struct.CborItem.html) as it
/// already does something similar on its own.
pub struct Cbor<T: AsRef<[u8]>>(pub T);

impl<T: AsRef<[u8]>> core::fmt::Display for Cbor<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Hex(self.0.as_ref()).fmt(f)
    }
}

#[cfg(feature = "defmt")]
impl<T: AsRef<[u8]>> defmt::Format for Cbor<T> {
    fn format(&self, f: defmt::Formatter) {
        ::defmt::write!(f, "{=[u8]:cbor}", self.0.as_ref())
    }
}
