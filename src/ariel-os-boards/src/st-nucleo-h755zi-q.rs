// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB0, led1 : PE1, led2 : PB14, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PC13, });
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART3, tx : PD8, rx : PD9, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
