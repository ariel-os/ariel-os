// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : P0_25, });
    ariel_os_hal::define_uarts![
        { name : uart0, device : SERIAL3, tx : P0_11, rx : P0_12, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
