// @generated

pub mod pins {
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_17, pull : Up, active :
        Low, aliases : [BTN_A,] }, { field_name : button1, type_name : Button1, pin :
        P0_26, pull : Up, active : Low, aliases : [BTN_B,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
