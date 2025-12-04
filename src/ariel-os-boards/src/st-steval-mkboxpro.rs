// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PF6, led1 : PH11, led2 : PH12, led3 : PF9, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : PC13, button1 : PE0, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
