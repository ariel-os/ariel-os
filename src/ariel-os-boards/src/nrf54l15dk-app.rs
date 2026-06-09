// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P2_09, led1 : P1_10, led2 : P2_07, led3 : P1_14, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P1_13, button1 : P1_09, button2 : P1_08, button3 :
        P0_04, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
