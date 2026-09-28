// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P0_04, color : green, active : Low,
        aliases : [D9,] }, { field_name : led1, type_name : Led1, pin : P0_14, color :
        red, active : Low, aliases : [D12,] }, { field_name : led2, type_name : Led2, pin
        : P0_22, color : red, active : Low, aliases : [D11,] }, { field_name : led3,
        type_name : Led3, pin : P0_05, color : blue, active : Low, aliases : [D10,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_02, pull : None, active :
        Low, aliases : [SW2,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
