// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_29, led1 : P0_31, led2 : P0_30, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : P0_26, });
    ariel_os_hal::define_uarts![
        { name : uart0, device : SERIAL3, tx : P0_01, rx : P0_00, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
