// @generated

pub mod pins {
use ariel_os_hal::hal::peripherals;

#[cfg(context = "stm32u083c-dk")]
ariel_os_hal::define_peripherals!(LedPeripherals {
led0: PA5,
});
#[cfg(context = "stm32u083c-dk")]
ariel_os_hal::define_peripherals!(ButtonPeripherals {
button0: PC2,
});
}

#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
}
