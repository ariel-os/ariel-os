// @generated

pub mod pins {
use ariel_os_hal::hal::peripherals;

#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_29,
});
#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_26,
});
}

#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
}
