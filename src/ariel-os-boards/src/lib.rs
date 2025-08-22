#![no_std]

pub mod pins;
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
#[cfg(context = "bbc-microbit-v1")]
{
    // Set the LED matrix column for led0 to low
    let pin = peripherals.P0_04.take().unwrap();
    let output = ariel_os_hal::gpio::Output::new(pin, ariel_os_embassy_common::gpio::Level::Low);
    core::mem::forget(output);
}
#[cfg(context = "bbc-microbit-v2")]
{
    // Set the LED matrix column for led0 to low
    let pin = peripherals.P0_28.take().unwrap();
    let output = ariel_os_hal::gpio::Output::new(pin, ariel_os_embassy_common::gpio::Level::Low);
    core::mem::forget(output);
}
}
