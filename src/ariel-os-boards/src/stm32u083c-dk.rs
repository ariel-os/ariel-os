// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : PC13, color : green, active : Low,], aliases :
        [LD3, LED_GREEN,] }, { name : Led1, channels : [pin : PA5, color : blue, active :
        Low,], aliases : [LD4, LED_BLUE,] }, { name : Led2, channels : [pin : PB2, color
        : red, active : Low,], aliases : [LD5, LED_RED,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : PC2, pull : Up, active : Low, aliases :
        [JOYSTICK_CENTER,] },
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
