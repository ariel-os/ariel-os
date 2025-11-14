// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_28, led1 : P0_29, led2 : P0_30, led3 : P0_31, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P0_23, button1 : P0_24, button2 : P0_08, button3 :
        P0_09, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
