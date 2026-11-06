// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : GPIO7, color : red, active : High,], aliases :
        [D2,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : GPIO9, pull : Up, active : Low, aliases : [IO9_BOOT,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C0, sda : GPIO10, scl : GPIO8, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
