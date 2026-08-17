//! AVR time driver integration.
//!
//! The actual Timer0-based time driver lives in the `embassy-avr` crate.
//! `ariel-os-avr::init()` calls `embassy_avr::time_driver::init()` to
//! register the driver with `embassy-time`.
