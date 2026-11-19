// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : PC13, color : blue, active : Low,
        aliases : [] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : PA0, pull : Down, active :
        High, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
