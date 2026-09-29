// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PA5, led1 : PA7, led2 : PB1, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PA0, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
