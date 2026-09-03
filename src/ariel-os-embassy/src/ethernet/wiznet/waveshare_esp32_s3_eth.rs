//! Pin wiring for the [Waveshare ESP32-S3-ETH](https://www.waveshare.com/wiki/ESP32-S3-ETH), an
//! ESP32-S3 board with an onboard W5500 Ethernet chip.

use crate::hal::peripherals;

/// The SPI peripheral the W5500 is wired to.
pub type WiznetSpi = crate::hal::spi::main::SPI2;

/// Pins used to communicate with the onboard W5500.
pub struct WiznetPins {
    /// SPI clock pin.
    pub spi_sck: peripherals::GPIO13<'static>,
    /// SPI MOSI pin.
    pub spi_mosi: peripherals::GPIO11<'static>,
    /// SPI MISO pin.
    pub spi_miso: peripherals::GPIO12<'static>,
    /// SPI chip-select pin.
    pub cs: peripherals::GPIO14<'static>,
    /// Reset pin.
    pub reset: peripherals::GPIO9<'static>,
    /// Interrupt pin.
    pub int: peripherals::GPIO10<'static>,
}

/// # Panics
///
/// Panics if at least one of these pins is already taken:
/// - `GPIO9`
/// - `GPIO10`
/// - `GPIO11`
/// - `GPIO12`
/// - `GPIO13`
/// - `GPIO14`
pub fn take_pins(peripherals: &mut crate::hal::OptionalPeripherals) -> WiznetPins {
    WiznetPins {
        spi_sck: peripherals.GPIO13.take().unwrap(),
        spi_mosi: peripherals.GPIO11.take().unwrap(),
        spi_miso: peripherals.GPIO12.take().unwrap(),
        cs: peripherals.GPIO14.take().unwrap(),
        reset: peripherals.GPIO9.take().unwrap(),
        int: peripherals.GPIO10.take().unwrap(),
    }
}
