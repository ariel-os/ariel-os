// @generated

pub mod pins {
use ariel_os_hal::hal::peripherals;

ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_21,
});
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_14,
button1: P0_23,
});
}

#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
{
    // Set the LED matrix column for led0 to low
    let pin = peripherals.P0_28.take().unwrap();
    let output = ariel_os_hal::gpio::Output::new(pin, ariel_os_embassy_common::gpio::Level::Low);
    core::mem::forget(output);
}
}
