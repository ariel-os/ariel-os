// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_14, led1 : P0_30, led2 : P0_22, led3 : P0_31, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : P0_02, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
