//! Dummy module used to satisfy platform-independent tooling.

mod executor;
pub mod gpio;

pub mod peripheral {
    pub use embassy_hal_internal::Peripheral;
}

#[cfg(feature = "hwrng")]
pub mod hwrng;

#[cfg(feature = "usb")]
pub mod usb;

pub use executor::{Executor, Spawner};

/// Dummy type.
///
/// See the `OptionalPeripherals` type of your Embassy architecture crate instead.
pub struct OptionalPeripherals;

/// Dummy type.
pub struct Peripherals;

impl From<Peripherals> for OptionalPeripherals {
    fn from(_peripherals: Peripherals) -> Self {
        Self {}
    }
}

pub fn init() -> OptionalPeripherals {
    unimplemented!();
}

pub struct SWI;
