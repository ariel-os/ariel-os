// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        MonocolorLedPeripherals { led0 : P0_29, led1 : P0_31, led2 : P0_30, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : P0_26, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
