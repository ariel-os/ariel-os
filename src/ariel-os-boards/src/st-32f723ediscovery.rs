// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : PA5, color : blue, active : High,
        aliases : [LD1,] }, { field_name : led1, type_name : Led1, pin : PA7, color :
        red, active : High, aliases : [LD5,] }, { field_name : led2, type_name : Led2,
        pin : PB1, color : green, active : High, aliases : [LD6,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : PA0, pull : None, active :
        High, aliases : [] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C2, sda : PH5, scl : PH4, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
