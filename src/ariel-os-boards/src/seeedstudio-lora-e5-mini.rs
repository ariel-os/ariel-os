// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : PB5, });
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : PB13, pull : None, active : Low,], aliases :
        [BOOT, SCK, K3,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB6, rx : PB7, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
