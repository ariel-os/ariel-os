// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P2_09, color : green, active : High,
        aliases : [] }, { field_name : led1, type_name : Led1, pin : P1_10, color :
        green, active : High, aliases : [] }, { field_name : led2, type_name : Led2, pin
        : P2_07, color : green, active : High, aliases : [] }, { field_name : led3,
        type_name : Led3, pin : P1_14, color : green, active : High, aliases : [] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P1_13, pull : None, active :
        Low, aliases : [SW2,] }, { field_name : button1, type_name : Button1, pin :
        P1_09, pull : None, active : Low, aliases : [SW3,] }, { field_name : button2,
        type_name : Button2, pin : P1_08, pull : None, active : Low, aliases : [SW5,] },
        { field_name : button3, type_name : Button3, pin : P0_04, pull : None, active :
        Low, aliases : [SW6,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
