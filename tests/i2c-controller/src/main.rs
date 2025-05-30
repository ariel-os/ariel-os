//! This example is merely to illustrate and test raw bus usage.
//!
//! Please use [`ariel_os::sensors`] instead for a high-level sensor abstraction that is
//! HAL-agnostic.
//!
//! This example requires an onboard sensor or an external LIS3DH/LSM303AGR sensor (3-axis
//! accelerometer).
#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    debug::{
        ExitCode, exit,
        log::{debug, info},
    },
    hal,
    i2c::controller::{I2cDevice, Kilohertz, highest_freq_in},
};
use embassy_sync::mutex::Mutex;
use embedded_hal_async::i2c::I2c as _;

cfg_if::cfg_if! {
    if #[cfg(context = "nordic-thingy-91-x-nrf9151")] {
        // Alternate address
        const TARGET_I2C_ADDR: u8 = 0x1d;
    } else if #[cfg(context = "stm32u083c-dk")] {
        // STTS22H
        const TARGET_I2C_ADDR: u8 = 0x3f;
    } else {
        const TARGET_I2C_ADDR: u8 = 0x19;
    }
}

// WHO_AM_I register of the sensor
cfg_if::cfg_if! {
    if #[cfg(context = "nordic-thingy-91-x-nrf9151")] {
        const WHO_AM_I_REG_ADDR: u8 = 0x02;
    } else if #[cfg(context = "stm32u083c-dk")] {
        const WHO_AM_I_REG_ADDR: u8 = 0x01;
    } else {
        const WHO_AM_I_REG_ADDR: u8 = 0x0f;
    }
}

cfg_if::cfg_if! {
    if #[cfg(context = "nordic-thingy-91-x-nrf9151")] {
        const DEVICE_ID: u8 = 0xf7;
    } else if #[cfg(context = "stm32u083c-dk")] {
        const DEVICE_ID: u8 = 0xa0;
    } else {
        const DEVICE_ID: u8 = 0x33;
    }
}

pub static I2C_BUS: once_cell::sync::OnceCell<
    Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, hal::i2c::controller::I2c>,
> = once_cell::sync::OnceCell::new();

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };
    debug!("Selected frequency: {:?}", i2c_config.frequency);

    let i2c_bus = pins::SensorI2c::new(peripherals.i2c_sda, peripherals.i2c_scl, i2c_config);

    let _ = I2C_BUS.set(Mutex::new(i2c_bus));

    let mut i2c_device = I2cDevice::new(I2C_BUS.get().unwrap());

    let mut id = [0];
    i2c_device
        .write_read(TARGET_I2C_ADDR, &[WHO_AM_I_REG_ADDR], &mut id)
        .await
        .unwrap();

    let who_am_i = id[0];
    info!("WHO_AM_I_COMMAND register value: 0x{:x}", who_am_i);
    assert_eq!(who_am_i, DEVICE_ID);

    info!("Test passed!");

    exit(ExitCode::SUCCESS);
}
