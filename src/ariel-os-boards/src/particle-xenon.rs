// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P1_12, color : blue, active : Low,], aliases :
        [D7,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P0_11, pull : None, active : Low, aliases : [MODE,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
