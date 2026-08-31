//! This module is intended to be @generated.
use ariel_os::{
    log::debug,
    hal,
    i2c::controller::{Kilohertz, highest_freq_in},
};
use embassy_sync::mutex::Mutex;

pub static I2C_BUS: once_cell::sync::OnceCell<
    Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, hal::i2c::controller::I2c>,
> = once_cell::sync::OnceCell::new();

use ariel_os::i2c::BusPart;
use ariel_os_boards::pins::I2c0;

pub fn init(peripherals: I2c0) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };
    debug!("Selected frequency: {:?}", i2c_config.frequency);

    let (sda, scl) = peripherals.into_pins();
    let i2c_bus = <I2c0 as BusPart>::I2cPeri::new(sda, scl, i2c_config);
    let _ = I2C_BUS.set(Mutex::new(i2c_bus));
}
