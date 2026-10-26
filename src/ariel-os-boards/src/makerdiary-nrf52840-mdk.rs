// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_23, led1 : P0_22, led2 : P0_24, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P1_00, pull : None, active : Low,], aliases :
        [USR_BTN, SW2,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_20, rx : P0_19, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
