//! Dummy module used to satisfy platform-independent tooling.

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

mod executor;

#[doc(hidden)]
pub mod gpio;

#[doc(hidden)]
pub mod peripheral;

#[doc(hidden)]
#[cfg(feature = "ble")]
pub mod ble;

#[doc(hidden)]
#[cfg(feature = "hwrng")]
pub mod hwrng;

#[doc(hidden)]
#[cfg(feature = "i2c")]
pub mod i2c;

#[doc(hidden)]
pub mod identity;

#[doc(hidden)]
#[cfg(feature = "spi")]
pub mod spi;

#[doc(hidden)]
#[cfg(feature = "storage")]
pub mod storage;

#[doc(hidden)]
#[cfg(feature = "usb")]
pub mod usb;

pub use executor::{Executor, Spawner};
pub use peripheral::{OptionalPeripherals, Peripheral};

#[doc(hidden)]
#[must_use]
pub fn init() -> OptionalPeripherals {
    unimplemented!()
}

#[doc(hidden)]
pub struct SWI;
