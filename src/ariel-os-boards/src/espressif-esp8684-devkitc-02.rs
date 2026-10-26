// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : GPIO0, });
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : GPIO9, pull : None, active : Low,], aliases :
        [SW1,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UART0, tx : GPIO20, rx : GPIO19, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
