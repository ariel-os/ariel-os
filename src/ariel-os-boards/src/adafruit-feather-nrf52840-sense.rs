// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P1_09, color : red, active : High,], aliases :
        [D13, D2,] }, { name : Led1, channels : [pin : P1_10, color : blue, active :
        High,], aliases : [D4, LED2, Conn,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P1_02, pull : None, active : Low, aliases : [UserSw, SW2,
        SWITCH,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : TWISPI0, sda : P0_12, scl : P0_11, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
