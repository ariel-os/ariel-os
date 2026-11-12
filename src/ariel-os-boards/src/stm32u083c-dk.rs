// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : PC13, color : green, active : High,
        aliases : [LD3, LED_GREEN,] }, { field_name : led1, type_name : Led1, pin : PA5,
        color : blue, active : High, aliases : [LD4, LED_BLUE,] }, { field_name : led2,
        type_name : Led2, pin : PB2, color : red, active : High, aliases : [LD5,
        LED_RED,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : PC2, pull : Up, active : Low,
        aliases : [JOYSTICK_CENTER,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C1, sda : PB7, scl : PB8, aliases : [] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART2, tx : PA2, rx : PA3, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
