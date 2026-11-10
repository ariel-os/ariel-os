// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PD8, led1 : PC4, led2 : PB8, }
    );
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : PC13, pull : None, active :
        Low, aliases : [B1,] }, { field_name : button1, type_name : Button1, pin : PC5,
        pull : None, active : Low, aliases : [B2,] }, { field_name : button2, type_name :
        Button2, pin : PB4, pull : None, active : Low, aliases : [B3,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB12, rx : PA8, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
