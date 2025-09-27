// @generated

pub mod pins {
use ariel_os_hal::hal::peripherals;

#[cfg(context = "nrf9160dk-nrf9160")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: P0_02,
});
#[cfg(context = "nrf9160dk-nrf9160")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: P0_06,
});
}

#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
}
