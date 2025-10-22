// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led1 : P0_00, led2 : P0_01, led3 : P0_04, led4 : P0_05, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button1 : P0_08, button2 : P0_09, button3 : P0_18, button4 :
        P0_19, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
