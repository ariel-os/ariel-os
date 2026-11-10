// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : PB5, color : red, active : Low,
        aliases : [LD1, D6,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : PB13, pull : None, active :
        Low, aliases : [BOOT, SCK, K3,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB6, rx : PB7, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
