//! This module is intended to be @generated.
use ariel_os::{
    debug::log::debug,
    hal,
    i2c::controller::{Kilohertz, highest_freq_in},
};
use embassy_sync::mutex::Mutex;

pub static I2C_BUS: once_cell::sync::OnceCell<
    Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, hal::i2c::controller::I2c>,
> = once_cell::sync::OnceCell::new();

pub fn init(peripherals: crate::pins::I2CPins) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };
    debug!("Selected frequency: {:?}", i2c_config.frequency);

    let i2c_bus = crate::pins::SensorI2c::new(peripherals.i2c_sda, peripherals.i2c_scl, i2c_config);
    let _ = I2C_BUS.set(Mutex::new(i2c_bus));
}
