// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P0_26, color : red, active : Low,
        aliases : [USER_LED_R,] }, { field_name : led1, type_name : Led1, pin : P0_06,
        color : blue, active : Low, aliases : [USER_LED_B,] }, { field_name : led2,
        type_name : Led2, pin : P0_30, color : green, active : Low, aliases :
        [USER_LED_G,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
