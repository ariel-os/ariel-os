// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : PO1, color : red, active : High,
        aliases : [LED1,] }, { field_name : led1, type_name : Led1, pin : PO5, color :
        blue, active : Low, aliases : [LED2,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : PC13, pull : Up, active : Low,
        aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
