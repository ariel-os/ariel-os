// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PH10, led1 : PH11, led2 : PH12, led3 : PH13, led4 : PH14,
        led5 : PH15, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
