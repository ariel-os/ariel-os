// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : PA5, color : green, active : Low,], aliases :
        [LD2,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : PC13, pull : Up, active : Low, aliases : [B1, USER,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART2, tx : PA2, rx : PA3, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
