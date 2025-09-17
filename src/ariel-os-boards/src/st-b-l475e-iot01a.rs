// @generated

pub mod pins {
use ariel_os_hal::hal::peripherals;

#[cfg(context = "st-b-l475e-iot01a")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PA5,
led1: PB14,
});
#[cfg(context = "st-b-l475e-iot01a")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
});
}

#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
}
