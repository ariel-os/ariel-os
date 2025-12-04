// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB0, led1 : PE1, led2 : PE1, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PC13, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
