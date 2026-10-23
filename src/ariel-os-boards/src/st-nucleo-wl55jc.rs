// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB15, led1 : PB9, led2 : PB11, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : PA0, button1 : PA1, button2 : PC6, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
