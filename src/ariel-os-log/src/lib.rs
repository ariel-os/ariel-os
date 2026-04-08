//! Provides logging facilities.
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
#![cfg_attr(nightly, feature(doc_cfg))]
#![cfg_attr(
    all(feature = "log", not(target_has_atomic = "ptr")),
    expect(unsafe_code)
)]
#![deny(missing_docs)]

#[featurecomb::comb]
mod _featurecomb {}

#[allow(unused, reason = "conditional compilation")]
#[doc(hidden)]
#[cfg(feature = "log")]
mod log_logger;

// This module is hidden in the docs, but would still be imported by a wildcard import of this
// crate's items.
#[doc(hidden)]
pub mod hidden_for_logging_macros {
    // Required so the macros can access it.
    #[cfg(feature = "defmt")]
    pub use defmt;
}

// Make sure the `defmt` logger gets linked.
#[cfg(feature = "esp-println")]
use esp_println as _;

#[cfg(feature = "defmt")]
pub use defmt::{Debug2Format, Display2Format};

/// Prints the panic on the logging output in a consistent manner across loggers.
#[doc(hidden)]
pub fn print_panic(info: &core::panic::PanicInfo<'_>) {
    // `location()`'s documentation currently states that it always returns `Some(_)`.
    // It is unclear what the panic formatting would be otherwise, because the std does not
    // currently handle the case where the location cannot be obtained.
    #[allow(
        unused_variables,
        reason = "FP due to macro usage and conditional compilation"
    )]
    let (location, message) = (info.location().unwrap(), info.message());

    // `PanicMessage` does not currently implement `defmt::Format`.
    // We *need* to use the `Display` implementation and cannot use `PanicMessage::as_str()` as
    // that would not work for dynamically formatted messages.
    #[cfg(feature = "defmt")]
    let message = Display2Format(&message);

    // Mimics the `Display` implementation of `core::panic::PanicInfo`.
    println!("panicked at {}:\n{}", location, message);
}

#[cfg(feature = "log")]
#[doc(hidden)]
pub mod log {
    use core::fmt::{Debug, Display, Formatter};

    // Re-export only the minimum set of items to minimize breaking changes in case `log`
    // adds/removes any items.
    pub use log::{debug, error, info, trace, warn};

    /// No-op wrapper that formats the Debug trait (drop-in replacement for the equivalent `defmt`
    /// type).
    pub struct Debug2Format<T: Debug>(pub T);

    impl<T: Debug> Debug for Debug2Format<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
            self.0.fmt(f)
        }
    }

    /// No-op wrapper that formats the Display trait (drop-in replacement for the equivalent
    /// `defmt` type).
    pub struct Display2Format<T: Display>(pub T);

    impl<T: Display> Display for Display2Format<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
            self.0.fmt(f)
        }
    }

    #[cfg(all(
        context = "ariel-os",
        not(any(feature = "esp-println", feature = "uart"))
    ))]
    pub use ariel_os_debug::debug_output_println as println;

    #[cfg(feature = "esp-println")]
    pub use esp_println::println;

    #[cfg(feature = "uart")]
    pub use crate::uart_println as println;

    /// Prints to the logging output, with a newline.
    #[cfg(not(context = "ariel-os"))]
    #[macro_export]
    macro_rules! noop_println {
        ($($arg:tt)*) => {};
    }
    #[cfg(not(context = "ariel-os"))]
    pub use crate::noop_println as println;
}

#[cfg(feature = "log")]
pub use log::{Debug2Format, Display2Format};

#[cfg(not(any(feature = "log", feature = "defmt")))]
#[doc(hidden)]
mod dummy {
    use core::fmt::{Debug, Display};
    /// Dummy struct needed to run clippy on crates that don't select a logging facade.
    pub struct Debug2Format<T: Debug>(pub T);

    /// Dummy struct needed to run clippy on crates that don't select a logging facade.
    pub struct Display2Format<T: Display>(pub T);
}
#[cfg(not(any(feature = "log", feature = "defmt")))]
pub use dummy::{Debug2Format, Display2Format};

// NOTE: log macros are defined within private modules so that `doc_cfg` does not produce
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
            use $crate::hidden_for_logging_macros::defmt;
            if true {
                defmt::trace!($($arg)*);
            } else {
                let _ = format_args!($($arg)*);
            }
        }};
    }

    /// Logs a message at the debug level.
    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {{
            use $crate::hidden_for_logging_macros::defmt;
            if true {
                defmt::debug!($($arg)*);
            } else {
                let _ = format_args!($($arg)*);
            }
        }};
    }

    /// Logs a message at the info level.
    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {{
            use $crate::hidden_for_logging_macros::defmt;
            if true {
                defmt::info!($($arg)*);
            } else {
                let _ = format_args!($($arg)*);
            }
        }};
    }

    /// Logs a message at the warn level.
    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {{
            use $crate::hidden_for_logging_macros::defmt;
            if true {
                defmt::warn!($($arg)*);
            } else {
                let _ = format_args!($($arg)*);
            }
        }};
    }

    /// Logs a message at the error level.
    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {{
            use $crate::hidden_for_logging_macros::defmt;
            if true {
                defmt::error!($($arg)*);
            } else {
                let _ = format_args!($($arg)*);
            }
        }};
    }

    /// Prints to the logging output, with a newline.
    #[macro_export]
    macro_rules! println {
        ($($arg:tt)*) => {{
            use $crate::hidden_for_logging_macros::defmt;
            if true {
                defmt::println!($($arg)*);
            } else {
                let _ = format_args!($($arg)*);
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

    /// Prints to the logging output, with a newline.
    #[macro_export]
    macro_rules! println {
        ($($arg:tt)*) => {{
            $crate::log::println!($($arg)*);
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

    /// Prints to the logging output, with a newline.
    #[macro_export]
    macro_rules! println {
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
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::write!(f, "{=[u8]:02x}", self.0.as_ref());
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
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::write!(f, "{=[u8]:cbor}", self.0.as_ref());
    }
}

#[cfg(feature = "uart")]
#[doc(hidden)]
pub mod backend {
    use embassy_sync::once_lock::OnceLock;

    #[doc(hidden)]
    pub enum Error {
        Writing,
    }

    // Populated by a downstream crate.
    // The function must print to a UART output.
    #[expect(clippy::type_complexity, reason = "not worth it")]
    #[doc(hidden)]
    pub static DEBUG_UART_WRITE_FN: OnceLock<fn(&[u8]) -> Result<(), Error>> = OnceLock::new();

    struct DebugUart;

    impl core::fmt::Write for DebugUart {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let bytes = s.as_bytes();

            if let Some(debug_uart_write_fn) = DEBUG_UART_WRITE_FN.try_get() {
                // Panicking in this case would not be useful as (a) it is recoverable, we would
                // just be dropping some debug output and (b) there would not be a output to print
                // the panic on, as there can currently only be one backend at once.
                let _ = debug_uart_write_fn(bytes);
            }

            Ok(())
        }
    }

    // Based on <https://blog.m-ou.se/format-args/>.
    #[doc(hidden)]
    pub fn _print(args: core::fmt::Arguments<'_>) {
        use core::fmt::Write as _;

        DebugUart.write_fmt(args).unwrap();
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! uart_println {
        ($($arg:tt)*) => {{
            #[expect(clippy::used_underscore_items, reason = "consistency with std::println")]
            $crate::backend::_print(format_args!("{}\n", format_args!($($arg)*)));
        }};
    }
}

#[doc(hidden)]
pub fn init() {
    #[cfg(feature = "log")]
    log_logger::init();
}
