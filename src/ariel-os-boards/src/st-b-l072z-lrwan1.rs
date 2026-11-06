// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : PB5, color : green, active : High,], aliases :
        [LD1,] }, { name : Led1, channels : [pin : PA5, color : red, active : High,],
        aliases : [LD2,] }, { name : Led2, channels : [pin : PB6, color : blue, active :
        High,], aliases : [LD3,] }, { name : Led3, channels : [pin : PB7, color : red,
        active : High,], aliases : [LD4,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : PB2, pull : Up, active : Low, aliases : [USER, B1,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C1, sda : PB9, scl : PB8, aliases : [ArduinoI2c,]
        },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART2, tx : PA2, rx : PA3, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
