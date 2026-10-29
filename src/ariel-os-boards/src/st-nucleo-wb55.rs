// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB5, led1 : PB0, led2 : PB1, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : PC4, pull : None, active : Low,], aliases :
        [SW1,] }, { name : Switch1, pins : [pin : PD0, pull : None, active : Low,],
        aliases : [SW2,] }, { name : Switch2, pins : [pin : PD1, pull : None, active :
        Low,], aliases : [SW3,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB6, rx : PB7, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
