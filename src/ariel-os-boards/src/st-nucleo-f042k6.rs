// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : PB3, color : green, active : High,], aliases :
        [LD3,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART2, tx : PA2, rx : PA15, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
