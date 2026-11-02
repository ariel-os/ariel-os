// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : PO1, led1 : PO5, });
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PC13, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
