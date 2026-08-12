//! Connector to custom logging transports.
//!
//! ## For implementors
//!
//!  Custom logging transport implementations end up depending on the HAL, that then depends on
//! `ariel-os-log`, creating a circular dependency.
//! To break this circle, the custom implementations have to register their `write_bytes` and `flush`
//! implementation with [`register_transport_fns`]

use embassy_sync::once_lock::OnceLock;

static TRANSPORT_WRITE_BYTES_FN: OnceLock<fn(&[u8])> = OnceLock::new();
static TRANSPORT_FLUSH_FN: OnceLock<fn()> = OnceLock::new();

/// Register custom transport functions.
pub fn register_transport_functions(write_bytes_fn: fn(&[u8]), flush_fn: fn()) {
    let _ = TRANSPORT_WRITE_BYTES_FN.init(write_bytes_fn);
    let _ = TRANSPORT_FLUSH_FN.init(flush_fn);
}

pub(crate) fn write_bytes(bytes: &[u8]) {
    if let Some(write_fn) = TRANSPORT_WRITE_BYTES_FN.try_get() {
        write_fn(bytes);
    }
}

#[cfg(feature = "defmt")]
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
        $crate::custom_transport::_print(format_args!("{}\n", format_args!($($arg)*)));
    }};
}
