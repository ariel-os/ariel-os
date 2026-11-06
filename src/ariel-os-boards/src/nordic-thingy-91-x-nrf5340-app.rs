// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P0_14, color : red, active : Low, pin : P0_26,
        color : green, active : Low, pin : P0_15, color : blue, active : Low,], aliases :
        [LED2,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P0_24, pull : None, active : Low, aliases : [SW4,
        BUTTON2,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : SERIAL0, sda : P1_03, scl : P1_02, aliases :
        [QwiicI2c,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
