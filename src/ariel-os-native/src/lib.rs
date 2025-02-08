//! Items specific to the Nordic Semiconductor nRF MCUs.

#![no_std]
#![feature(doc_auto_cfg)]
//#![deny(missing_docs)]

#[doc(hidden)]
pub mod peripheral {
    //    pub use embassy_nrf::Peripheral;
}

#[doc(hidden)]
pub mod identity {
    use ariel_os_embassy_common::identity;

    pub type DeviceId = identity::NoDeviceId<identity::NotImplemented>;
}

pub struct OptionalPeripherals {}

#[doc(hidden)]
pub fn init() -> OptionalPeripherals {
    OptionalPeripherals {}
}
