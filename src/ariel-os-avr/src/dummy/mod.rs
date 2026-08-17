//! Dummy module used to satisfy ariel-os-hal's GPIO builder macros on AVR.
//!
//! AVR GPIO integration is deferred; this module provides the compile-time
//! surface needed for the rest of ariel-os to compile when `context = "avr"`
//! is set. Concrete GPIO usage should use `embassy_avr::*` directly.

#![allow(
    clippy::missing_errors_doc,
    reason = "this module's items are hidden in the docs"
)]
#![allow(
    clippy::module_name_repetitions,
    reason = "this dummy module mimics manufacturer-specific crates"
)]
#![allow(
    clippy::needless_pass_by_value,
    reason = "this dummy module mimics manufacturer-specific crates"
)]
#![allow(unused, reason = "used by documentation only")]

mod executor;

#[doc(hidden)]
pub mod gpio;

#[doc(hidden)]
pub mod peripheral;

#[doc(hidden)]
pub mod identity;

#[doc(hidden)]
#[cfg(feature = "i2c")]
pub mod i2c;

#[doc(hidden)]
#[cfg(feature = "spi")]
pub mod spi;

#[doc(hidden)]
#[cfg(feature = "storage")]
pub mod storage;

#[doc(hidden)]
#[cfg(feature = "uart")]
pub mod uart;

pub use executor::{Executor, Spawner};
pub use peripheral::{IntoPeripheral, OptionalPeripherals};

#[doc(hidden)]
#[must_use]
pub fn init() -> OptionalPeripherals {
    embassy_avr::time_driver::init();
    OptionalPeripherals
}

#[doc(hidden)]
pub struct SWI;
