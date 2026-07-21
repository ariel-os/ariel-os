// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P0_14, button1 : P0_23, }
    );
    ariel_os_hal::define_uarts![
        { name : uart0, device : UARTE0, tx : P0_06, rx : P1_08, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
