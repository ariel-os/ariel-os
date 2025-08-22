//! Items specific to the "native" implementation

#![cfg_attr(nightly, feature(doc_auto_cfg))]

#[cfg(feature = "hwrng")]
pub mod hwrng;

pub mod identity;
pub mod peripherals {}

pub struct OptionalPeripherals {}

pub fn init() -> OptionalPeripherals {
    OptionalPeripherals {}
}

pub struct SWI {}
