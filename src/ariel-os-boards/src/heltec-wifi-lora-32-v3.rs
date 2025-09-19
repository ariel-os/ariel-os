// @generated

pub mod pins {
use ariel_os_hal::hal::peripherals;

#[cfg(context = "heltec-wifi-lora-32-v3")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: GPIO35,
});
#[cfg(context = "heltec-wifi-lora-32-v3")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: GPIO0,
});
}

#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
}
