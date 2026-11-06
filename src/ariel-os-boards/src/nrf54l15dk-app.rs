// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P2_09, color : green, active : High,], aliases :
        [] }, { name : Led1, channels : [pin : P1_10, color : green, active : High,],
        aliases : [] }, { name : Led2, channels : [pin : P2_07, color : green, active :
        High,], aliases : [] }, { name : Led3, channels : [pin : P1_14, color : green,
        active : High,], aliases : [] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P1_13, pull : None, active : Low, aliases : [SW2,
        Button0,] }, { name : Button1, pin : P1_09, pull : None, active : Low, aliases :
        [SW3, Button1,] }, { name : Button2, pin : P1_08, pull : None, active : Low,
        aliases : [SW5, Button2,] }, { name : Button3, pin : P0_04, pull : None, active :
        Low, aliases : [SW6, Button3,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
