// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PD8, led1 : PC4, led2 : PB8, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : PC13, button1 : PC5, button2 : PB4, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
