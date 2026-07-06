// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB5, led1 : PA5, led2 : PB6, led3 : PB7, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PB2, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
