// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB5, led1 : PB0, led2 : PB1, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : PC4, button1 : PD0, button2 : PD1, }
    );
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB6, rx : PB7, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
