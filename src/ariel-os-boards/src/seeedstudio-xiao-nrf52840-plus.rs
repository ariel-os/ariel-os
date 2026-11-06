// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P0_26, color : red, active : Low,], aliases :
        [USER_LED_R,] }, { name : Led1, channels : [pin : P0_06, color : blue, active :
        Low,], aliases : [USER_LED_B,] }, { name : Led2, channels : [pin : P0_30, color :
        green, active : Low,], aliases : [USER_LED_G,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
