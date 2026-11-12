// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P0_29, color : red, active : High,
        aliases : [] }, { field_name : led1, type_name : Led1, pin : P0_31, color :
        green, active : High, aliases : [] }, { field_name : led2, type_name : Led2, pin
        : P0_30, color : blue, active : High, aliases : [] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_26, pull : None, active :
        Low, aliases : [SW3, BUTTON1,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : SERIAL0, sda : P0_09, scl : P0_08, aliases :
        [QwiicI2c,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : SERIAL3, tx : P0_01, rx : P0_00, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
