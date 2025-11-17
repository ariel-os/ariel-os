// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_13, led1 : P0_14, led2 : P0_15, led3 : P0_16, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P0_11, button1 : P0_12, button2 : P0_24, button3 :
        P0_25, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
