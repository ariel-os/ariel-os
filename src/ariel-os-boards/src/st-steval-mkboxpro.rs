// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : PF6, color : green, active : Low,], aliases :
        [LED1, D5,] }, { name : Led1, channels : [pin : PH11, color : red, active :
        Low,], aliases : [LED2, D6,] }, { name : Led2, channels : [pin : PH12, color :
        orange, active : Low,], aliases : [LED3, D7,] }, { name : Led3, channels : [pin :
        PF9, color : blue, active : High,], aliases : [USB_enum_LED, D10,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : PC13, pull : Down, active : High, aliases : [BT1, SW1,]
        }, { name : Button1, pin : PE0, pull : Down, active : High, aliases : [BT2, SW2,
        BOOT0,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C1, sda : PB7, scl : PB6, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
