// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB4, led1 : PA9, led2 : PB8, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : PC13, button1 : PB6, button2 : PB7, }
    );
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB12, rx : PA8, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
