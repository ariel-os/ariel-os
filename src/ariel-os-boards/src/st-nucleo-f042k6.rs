// @generated

pub mod pins {
use ariel_os_hal::hal::peripherals;

#[cfg(context = "st-nucleo-f042k6")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PA5,
});
}

#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
}
