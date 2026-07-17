// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PC13, led1 : PA5, led2 : PB2, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PC2, });
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
