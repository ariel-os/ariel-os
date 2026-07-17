// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : P1_09, led1 : P1_10, });
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : P1_02, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
