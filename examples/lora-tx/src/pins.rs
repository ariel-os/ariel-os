use ariel_os::hal::spi;

// SPI1 is wired to the on-module Semtech SX1276 radio on the B-L072Z-LRWAN1.
pub type RadioSpi = spi::main::SPI1;

ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PB3,
    spi_miso: PA6,
    spi_mosi: PA7,
    spi_cs: PA15, // Radio NSS
    reset: PC0,   // Radio RESET
    dio0: PB4,    // Radio DIO0, used as the TX/RX-done IRQ line
    tcxo: PA12,   // Powers the module TCXO (enabled at boot)
});
