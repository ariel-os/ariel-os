// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : PB5, color : blue, active : High,], aliases :
        [LED1,] }, { name : Led1, channels : [pin : PB0, color : green, active : High,],
        aliases : [LED2,] }, { name : Led2, channels : [pin : PB1, color : red, active :
        High,], aliases : [LED3,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : PC4, pull : None, active : Low, aliases : [SW1,] }, {
        name : Button1, pin : PD0, pull : None, active : Low, aliases : [SW2,] }, { name
        : Button2, pin : PD1, pull : None, active : Low, aliases : [SW3,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB6, rx : PB7, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
