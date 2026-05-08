//! Provides support for the I2C communication bus.

#[doc(alias = "master")]
pub mod controller;

#[doc(hidden)]
pub fn init(peripherals: &mut crate::OptionalPeripherals) {
    // Take all I2C peripherals and do nothing with them.
    cfg_select! {
        context = "nrf52833" => {
            let _ = peripherals.TWISPI0.take().unwrap();
            let _ = peripherals.TWISPI1.take().unwrap();
        }
        context = "nrf52840" => {
            let _ = peripherals.TWISPI0.take().unwrap();
            let _ = peripherals.TWISPI1.take().unwrap();
        }
        context = "nrf5340-app" => {
            let _ = peripherals.SERIAL0.take().unwrap();
            let _ = peripherals.SERIAL1.take().unwrap();
        }
        context = "nrf91" => {
            let _ = peripherals.SERIAL0.take().unwrap();
            let _ = peripherals.SERIAL1.take().unwrap();
        }
        _ => {
            compile_error!("this nRF chip is not supported");
        }
    }
}
