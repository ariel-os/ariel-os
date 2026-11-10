// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : PB5, color : blue, active : High,
        aliases : [LED1,] }, { field_name : led1, type_name : Led1, pin : PB0, color :
        green, active : High, aliases : [LED2,] }, { field_name : led2, type_name : Led2,
        pin : PB1, color : red, active : High, aliases : [LED3,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : PC4, pull : None, active :
        Low, aliases : [SW1,] }, { field_name : button1, type_name : Button1, pin : PD0,
        pull : None, active : Low, aliases : [SW2,] }, { field_name : button2, type_name
        : Button2, pin : PD1, pull : None, active : Low, aliases : [SW3,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB6, rx : PB7, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
