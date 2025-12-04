// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB5, led1 : PB0, led2 : PB1, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : PC4, button1 : PD0, button2 : PD1, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
