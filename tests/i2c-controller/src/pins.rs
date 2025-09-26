use ariel_os::hal::{i2c, peripherals};

#[cfg(context = "esp")]
pub type SensorI2c = i2c::controller::I2C0;
#[cfg(context = "esp")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: GPIO2,
    i2c_scl: GPIO0,
});

#[cfg(any(context = "nrf52833", context = "nrf52840"))]
pub type SensorI2c = i2c::controller::TWISPI0;
#[cfg(any(context = "nrf5340", context = "nrf91"))]
pub type SensorI2c = i2c::controller::SERIAL0;
#[cfg(all(
    context = "nrf",
    not(any(context = "bbc-microbit-v2", context = "nordic-thingy-91-x-nrf9151"))
))]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: P0_00,
    i2c_scl: P0_01,
});
#[cfg(context = "bbc-microbit-v2")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: P0_16,
    i2c_scl: P0_08,
});
#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: P0_09,
    i2c_scl: P0_08,
});

#[cfg(context = "rp")]
pub type SensorI2c = i2c::controller::I2C0;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PIN_12,
    i2c_scl: PIN_13,
});

#[cfg(context = "stm32c031c6")]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(context = "stm32c031c6")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PB9,
    i2c_scl: PB8,
});

#[cfg(context = "stm32f042k6")]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(context = "stm32f042k6")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PB7,
    i2c_scl: PB6,
});

#[cfg(context = "stm32h755zi")]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(context = "stm32h755zi")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PB9,
    i2c_scl: PB8,
});

#[cfg(context = "stm32l475vg")]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(context = "stm32l475vg")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PB9,
    i2c_scl: PB8,
});

#[cfg(context = "stm32wb55rg")]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(context = "stm32wb55rg")]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PB9,
    i2c_scl: PB8,
});

#[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PB7,
    i2c_scl: PB8,
});

#[cfg(any(context = "stm32f401re", context = "stm32f411re"))]
pub type SensorI2c = i2c::controller::I2C1;
#[cfg(any(context = "st-nucleo-f401re", context = "st-nucleo-f411re"))]
ariel_os::hal::define_peripherals!(Peripherals {
    i2c_sda: PB9,
    i2c_scl: PB8,
});
