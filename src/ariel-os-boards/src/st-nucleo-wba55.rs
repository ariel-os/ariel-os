// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB4, led1 : PA9, led2 : PB8, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : PC13, pull : None, active : Low,], aliases :
        [B1,] }, { name : Switch1, pins : [pin : PB6, pull : None, active : Low,],
        aliases : [B2,] }, { name : Switch2, pins : [pin : PB7, pull : None, active :
        Low,], aliases : [B3,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB12, rx : PA8, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
