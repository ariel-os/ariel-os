// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P0_14, button1 : P0_23, }
    );
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
