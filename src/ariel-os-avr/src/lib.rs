//! Items specific to AVR microcontrollers (ATmega328P / Arduino Uno).

#![no_std]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

/// Dummy module used to satisfy ariel-os-hal's GPIO builder macros on AVR.
///
/// AVR GPIO integration is deferred; this module provides the compile-time
/// surface needed for the rest of ariel-os to compile when `context = "avr"`
/// is set. Concrete GPIO usage should use `embassy_avr::*` directly.
#[doc(hidden)]
pub mod dummy;

/// Time driver integration — delegates to `embassy_avr::time_driver`.
pub mod time_driver;

/// Initialize AVR hardware.
///
/// Starts the Timer0 time driver from `embassy-avr` (1 kHz tick).
#[doc(hidden)]
#[must_use]
pub fn init() -> dummy::OptionalPeripherals {
    embassy_avr::time_driver::init();
    dummy::OptionalPeripherals
}
