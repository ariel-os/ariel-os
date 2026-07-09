// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        MonocolorLedPeripherals { led0 : P0_14, led1 : P0_26, led2 : P0_15, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : P0_24, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
