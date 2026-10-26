// @generated

pub mod pins {
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : GPIO9, pull : None, active : Low,], aliases :
        [BOOT, SW1,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UART0, tx : GPIO24, rx : GPIO23, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
