//! Items specific to the "native" implementation

#![no_std]
#![cfg_attr(nightly, feature(doc_auto_cfg))]

#[cfg(feature = "hwrng")]
pub mod hwrng;

pub struct OptionalPeripherals {}

pub fn init() -> OptionalPeripherals {
    OptionalPeripherals {}
}
