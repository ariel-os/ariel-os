//! Provides support for the I2C communication bus.

#[doc(alias = "master")]
pub mod controller;

#[doc(hidden)]
pub fn init(peripherals: &mut crate::OptionalPeripherals) {
    // Take all I2C peripherals and do nothing with them.
    cfg_select! {
        context = "rp" => {
            let _ = peripherals.I2C0.take().unwrap();
            let _ = peripherals.I2C1.take().unwrap();
        }
        _ => {
            compile_error!("this RP chip is not supported");
        }
    }
}
