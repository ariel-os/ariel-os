//! Provides support for the SPI communication bus.

#[doc(alias = "master")]
pub mod main;

use ariel_os_embassy_common::spi::Mode;
use embassy_rp::spi::{Phase, Polarity};

fn from_mode(mode: Mode) -> (Polarity, Phase) {
    match mode {
        Mode::Mode0 => (Polarity::IdleLow, Phase::CaptureOnFirstTransition),
        Mode::Mode1 => (Polarity::IdleLow, Phase::CaptureOnSecondTransition),
        Mode::Mode2 => (Polarity::IdleHigh, Phase::CaptureOnFirstTransition),
        Mode::Mode3 => (Polarity::IdleHigh, Phase::CaptureOnSecondTransition),
    }
}

#[doc(hidden)]
pub fn init(peripherals: &mut crate::OptionalPeripherals) {
    // Take all SPI peripherals and do nothing with them.
    cfg_if::cfg_if! {
        if #[cfg(context = "rp")] {
            let _ = peripherals.SPI0.take().unwrap();
            let _ = peripherals.SPI1.take().unwrap();
        } else {
            compile_error!("this RP chip is not supported");
        }
    }
}
