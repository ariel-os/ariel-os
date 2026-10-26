// @generated

pub mod pins {
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P0_25, pull : None, active : Low,], aliases :
        [USR,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : SERIAL3, tx : P0_11, rx : P0_12, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
