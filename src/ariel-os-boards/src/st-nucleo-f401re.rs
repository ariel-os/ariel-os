// @generated

pub mod pins {
use ariel_os_hal::hal::peripherals;

#[cfg(context = "st-nucleo-f401re")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PB5,
});
#[cfg(context = "st-nucleo-f401re")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC13,
});
}

#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
}
