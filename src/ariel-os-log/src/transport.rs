//! This module provides facilities for registering a custom logging transport from within
//! an application.

// Internal logging transport implementations use the same [`register_custom_transport`] function
// but uses the `internal-transport-driver` feature to access this module.
//
// Logging transport implementations end up depending on the HAL, that then depends on
// `ariel-os-log`, creating a circular dependency. Connecting log transports like this helps break
// the cycle.

use embassy_sync::once_lock::OnceLock;

static TRANSPORT_WRITE_BYTES_FN: OnceLock<fn(&[u8])> = OnceLock::new();
static TRANSPORT_FLUSH_FN: OnceLock<fn()> = OnceLock::new();

/// Registers a custom transport.
///
/// Registering a custom transport requires providing two functions:
///
/// - `write_bytes_fn` is used to write bytes to the transport. Depending on the logging facade it
///   may be UTF-8 encoded text or raw binary data.
/// - `flush_fn` is used to flush the data in the transport.
///
/// ## Important note
///
/// `write_bytes_fn` and `flush_fn` may be executed inside a critical section, disabling interrupts.
/// They should therefore not wait for interrupts and should not trigger calls to the logging facade.
pub fn register_custom_transport(write_bytes_fn: fn(&[u8]), flush_fn: fn()) {
    let _ = TRANSPORT_WRITE_BYTES_FN.init(write_bytes_fn);
    let _ = TRANSPORT_FLUSH_FN.init(flush_fn);
}

// Write bytes to the transport if available.
pub(crate) fn write_bytes(bytes: &[u8]) {
    if let Some(write_fn) = TRANSPORT_WRITE_BYTES_FN.try_get() {
        write_fn(bytes);
    }
}

#[allow(unused, reason = "conditional compilation")]
#[cfg(feature = "defmt")]
// Flush the data in the transport if available.
pub(crate) fn flush() {
    if let Some(flush_fn) = TRANSPORT_FLUSH_FN.try_get() {
        flush_fn();
    }
}

struct Transport;

impl core::fmt::Write for Transport {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        write_bytes(bytes);
        Ok(())
    }
}

// Based on <https://blog.m-ou.se/format-args/>.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments<'_>) {
    use core::fmt::Write as _;

    Transport.write_fmt(args).unwrap();
}

#[doc(hidden)]
#[macro_export]
macro_rules! transport_println {
    ($($arg:tt)*) => {{
        #[expect(clippy::used_underscore_items, reason = "consistency with std::println")]
        $crate::transport::_print(format_args!("{}\n", format_args!($($arg)*)));
    }};
}
