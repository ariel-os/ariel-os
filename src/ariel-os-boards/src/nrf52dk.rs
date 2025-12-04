// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_17, led1 : P0_18, led2 : P0_19, led3 : P0_20, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P0_13, button1 : P0_14, button2 : P0_15, button3 :
        P0_16, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
