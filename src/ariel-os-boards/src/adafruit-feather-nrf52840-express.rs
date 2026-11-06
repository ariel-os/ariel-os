// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P1_15, color : red, active : High,], aliases :
        [LED1, D2, D3,] }, { name : Led1, channels : [pin : P1_10, color : blue, active :
        High,], aliases : [LED2, D4, Conn,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P1_02, pull : None, active : Low, aliases : [UserSw, SW2,
        SWITCH,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
