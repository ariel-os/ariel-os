// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P0_14, color : red, active : High,
        aliases : [] }, { field_name : led1, type_name : Led1, pin : P0_26, color :
        green, active : High, aliases : [] }, { field_name : led2, type_name : Led2, pin
        : P0_15, color : blue, active : High, aliases : [] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_24, pull : None, active :
        Low, aliases : [SW4, BUTTON2,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : SERIAL0, sda : P1_03, scl : P1_02, aliases :
        [QwiicI2c,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
