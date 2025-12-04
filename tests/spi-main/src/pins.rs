use ariel_os::hal::{peripherals, spi};

#[cfg(context = "esp")]
pub type SensorSpi = spi::main::SPI2;
#[cfg(context = "esp")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: GPIO0,
    spi_miso: GPIO1,
    spi_mosi: GPIO2,
    spi_cs: GPIO3,
});

#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: P0_13,
    spi_miso: P0_15,
    spi_mosi: P0_14,
    spi_cs: P0_10,
});
#[cfg(context = "nordic-thingy-91-x-nrf9151")]
pub const WHO_AM_I_REG_ADDR: u8 = 0x00;
#[cfg(context = "nordic-thingy-91-x-nrf9151")]
pub const DEVICE_ID: u8 = 0x24;

// Side SPI of Arduino v3 connector
#[cfg(context = "nrf52840")]
pub type SensorSpi = spi::main::SPI3;
#[cfg(context = "nrf52840")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: P1_15,
    spi_miso: P1_14,
    spi_mosi: P1_13,
    spi_cs: P1_12,
});

// Side SPI of Arduino v3 connector
#[cfg(any(context = "nrf5340", context = "nrf91"))]
pub type SensorSpi = spi::main::SERIAL2;
#[cfg(context = "nrf5340")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: P1_15,
    spi_miso: P1_14,
    spi_mosi: P1_13,
    spi_cs: P1_12,
});
#[cfg(context = "nrf9160")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: P0_13,
    spi_miso: P0_12,
    spi_mosi: P0_11,
    spi_cs: P0_10,
});

#[cfg(context = "rp")]
pub type SensorSpi = spi::main::SPI0;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PIN_18,
    spi_miso: PIN_16,
    spi_mosi: PIN_19,
    spi_cs: PIN_17,
});

// Side SPI of Arduino v3 connector
#[cfg(context = "stm32c031c6")]
pub type SensorSpi = spi::main::SPI1;
#[cfg(context = "stm32c031c6")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PA5,
    spi_miso: PA6,
    spi_mosi: PA7,
    spi_cs: PB0,
});

// Side SPI of Arduino v3 connector
#[cfg(context = "stm32h755zi")]
pub type SensorSpi = spi::main::SPI1;
#[cfg(context = "stm32h755zi")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PA5,
    spi_miso: PA6,
    spi_mosi: PB5,
    spi_cs: PD14,
});

// Side SPI of Arduino v3 connector on the st-b-l475e-iot01a board
#[cfg(context = "stm32l475vg")]
pub type SensorSpi = spi::main::SPI1;
#[cfg(context = "stm32l475vg")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PA5,  // Arduino D13
    spi_miso: PA6, // Arduino D12
    spi_mosi: PA7, // Arduino D11
    spi_cs: PA2,   // Arduion D10
});

// Side SPI of Arduino v3 connector
#[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))]
pub type SensorSpi = spi::main::SPI1;
#[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PA5,
    spi_miso: PA6,
    spi_mosi: PA7,
    spi_cs: PA15,
});

// Onboard LIS2DU12
#[cfg(context = "st-steval-mkboxpro")]
pub type SensorSpi = spi::main::SPI2;
#[cfg(context = "st-steval-mkboxpro")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PI1,
    spi_miso: PI2,
    spi_mosi: PI3,
    spi_cs: PI7,
});
#[cfg(context = "st-steval-mkboxpro")]
pub const WHO_AM_I_REG_ADDR: u8 = 0x43;
#[cfg(context = "st-steval-mkboxpro")]
pub const DEVICE_ID: u8 = 0x45;

// Side SPI of Arduino v3 connector
#[cfg(context = "stm32wb55rg")]
pub type SensorSpi = spi::main::SPI1;
#[cfg(context = "stm32wb55rg")]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PA5,
    spi_miso: PA6,
    spi_mosi: PA7,
    spi_cs: PA4,
});

#[cfg(any(context = "stm32f401re", context = "stm32f411re"))]
pub type SensorSpi = spi::main::SPI1;
#[cfg(any(context = "st-nucleo-f401re", context = "st-nucleo-f411re"))]
ariel_os::hal::define_peripherals!(Peripherals {
    spi_sck: PA5,
    spi_miso: PA6,
    spi_mosi: PA7,
    spi_cs: PB6,
});
