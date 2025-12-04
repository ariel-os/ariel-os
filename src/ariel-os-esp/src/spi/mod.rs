//! Provides support for the SPI communication bus.

#[doc(alias = "master")]
pub mod main;

use ariel_os_embassy_common::spi::{BitOrder, Mode};

fn from_mode(mode: Mode) -> esp_hal::spi::Mode {
    match mode {
        Mode::Mode0 => esp_hal::spi::Mode::_0,
        Mode::Mode1 => esp_hal::spi::Mode::_1,
        Mode::Mode2 => esp_hal::spi::Mode::_2,
        Mode::Mode3 => esp_hal::spi::Mode::_3,
    }
}

fn from_bit_order(bit_order: BitOrder) -> esp_hal::spi::BitOrder {
    match bit_order {
        BitOrder::MsbFirst => esp_hal::spi::BitOrder::MsbFirst,
        BitOrder::LsbFirst => esp_hal::spi::BitOrder::LsbFirst,
    }
}

#[doc(hidden)]
pub fn init(peripherals: &mut crate::OptionalPeripherals) {
    // Take all SPI peripherals and do nothing with them.
    cfg_if::cfg_if! {
        if #[cfg(context = "esp32")] {
            let _ = peripherals.SPI2.take().unwrap();
            let _ = peripherals.SPI3.take().unwrap();
        } else if #[cfg(context = "esp32c3")] {
            let _ = peripherals.SPI2.take().unwrap();
        } else if #[cfg(context = "esp32c6")] {
            let _ = peripherals.SPI2.take().unwrap();
        } else if #[cfg(context = "esp32s2")] {
            let _ = peripherals.SPI2.take().unwrap();
            let _ = peripherals.SPI3.take().unwrap();
        } else if #[cfg(context = "esp32s3")] {
            let _ = peripherals.SPI2.take().unwrap();
            let _ = peripherals.SPI3.take().unwrap();
        } else {
            compile_error!("this ESP32 chip is not supported");
        }
    }
}
