//! HAL-agnostic types shared between HALs.

#![no_std]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(missing_docs)]

pub mod gpio;

#[cfg(context = "cortex-m")]
pub mod executor_swi;

#[cfg(feature = "executor-thread")]
pub mod executor_thread;

#[cfg(feature = "i2c")]
pub mod i2c;

#[cfg(feature = "ble")]
pub mod ble;

pub mod identity;

#[cfg(feature = "spi")]
pub mod spi;

pub mod reexports {
    //! Crate re-exports.

    // Used by macros provided by this crate.
    pub use embassy_futures;
    pub use embassy_time;
    pub use embedded_hal_async;
}
