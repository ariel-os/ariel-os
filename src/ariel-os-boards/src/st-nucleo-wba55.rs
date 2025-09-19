// @generated

pub mod pins {
use ariel_os_hal::hal::peripherals;

#[cfg(context = "st-nucleo-wba55")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PB4,
led1: PA9,
led2: PB8,
});
#[cfg(context = "st-nucleo-wba55")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
button1: PB6,
button2: PB7,
});
}

#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
}
