// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : PC13, });
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PA0, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
