// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P1_22, led1 : P1_25, led2 : P1_27, led3 : P1_28, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P1_26, button1 : P1_09, button2 : P1_08, button3 :
        P0_05, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
