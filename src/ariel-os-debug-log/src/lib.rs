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
// Required for nested macros with repetitions in the inner macro.
#![feature(macro_metavar_expr)]
#![feature(doc_auto_cfg)]
#![deny(missing_docs)]
#![deny(clippy::pedantic)]

#[featurecomb::comb]
mod _featurecomb {}

#[cfg(feature = "defmt")]
pub mod defmt {
    //! Selected [`defmt`] items.

    // This module is hidden in the docs, but would still be imported by a wildcard import of this
    // crate's items.
    #[doc(hidden)]
    pub mod hidden {
        // Required so the macros can access it.
        #[doc(hidden)]
        pub use defmt;
    }

    pub use defmt::{Debug2Format, Display2Format, Format};

    // These are required "internally" by `defmt`.
    pub use defmt::{Formatter, Str, export, unreachable};
}

#[cfg(feature = "log")]
#[doc(hidden)]
pub mod log {
    // Re-export only the minimum set of items to minimize breaking changes in case `log`
    // adds/removes any items.
    pub use log::{debug, error, info, trace, warn};
}

#[cfg(feature = "defmt")]
macro_rules! define_logging_macros {
    () => {
        /// Logs a message at the trace level.
        #[macro_export]
        macro_rules! trace {
            ($$($$arg:tt)*) => {{
                use $crate::defmt::hidden::defmt;
                defmt::trace!($$($$arg)*);
            }};
        }

        /// Logs a message at the debug level.
        #[macro_export]
        macro_rules! debug {
            ($$($$arg:tt)*) => {{
                use $crate::defmt::hidden::defmt;
                defmt::debug!($$($$arg)*);
            }};
        }

        /// Logs a message at the info level.
        #[macro_export]
        macro_rules! info {
            ($$($$arg:tt)*) => {{
                use $crate::defmt::hidden::defmt;
                defmt::info!($$($$arg)*);
            }};
        }

        /// Logs a message at the warn level.
        #[macro_export]
        macro_rules! warn {
            ($$($$arg:tt)*) => {{
                use $crate::defmt::hidden::defmt;
                defmt::warn!($$($$arg)*);
            }};
        }

        /// Logs a message at the error level.
        #[macro_export]
        macro_rules! error {
            ($$($$arg:tt)*) => {{
                use $crate::defmt::hidden::defmt;
                defmt::error!($$($$arg)*);
            }};
        }
    }
}

#[cfg(feature = "log")]
macro_rules! define_logging_macros {
    () => {
        /// Logs a message at the trace level.
        #[macro_export]
        macro_rules! trace {
            ($$($$arg:tt)*) => {{
                $crate::log::trace!($$($$arg)*);
            }};
        }

        /// Logs a message at the debug level.
        #[macro_export]
        macro_rules! debug {
            ($$($$arg:tt)*) => {{
                $crate::log::debug!($$($$arg)*);
            }};
        }

        /// Logs a message at the info level.
        #[macro_export]
        macro_rules! info {
            ($$($$arg:tt)*) => {{
                $crate::log::info!($$($$arg)*);
            }};
        }

        /// Logs a message at the warn level.
        #[macro_export]
        macro_rules! warn {
            ($$($$arg:tt)*) => {{
                $crate::log::warn!($$($$arg)*);
            }};
        }

        /// Logs a message at the error level.
        #[macro_export]
        macro_rules! error {
            ($$($$arg:tt)*) => {{
                $crate::log::error!($$($$arg)*);
            }};
        }
    }
}

// Define no-op macros in case no facade is enabled.
#[cfg(not(any(feature = "defmt", feature = "log")))]
macro_rules! define_logging_macros {
    () => {
        /// Logs a message at the trace level.
        #[macro_export]
        macro_rules! trace {
            ($$($$arg:tt)*) => {};
        }

        /// Logs a message at the debug level.
        #[macro_export]
        macro_rules! debug {
            ($$($$arg:tt)*) => {};
        }

        /// Logs a message at the info level.
        #[macro_export]
        macro_rules! info {
            ($$($$arg:tt)*) => {};
        }

        /// Logs a message at the warn level.
        #[macro_export]
        macro_rules! warn {
            ($$($$arg:tt)*) => {};
        }

        /// Logs a message at the error level.
        #[macro_export]
        macro_rules! error {
            ($$($$arg:tt)*) => {};
        }
    };
}

// NOTE: these nested macros are used so `doc_auto_cfg` doesn't produce "feature flairs" on the
// logging macros.
define_logging_macros!();
