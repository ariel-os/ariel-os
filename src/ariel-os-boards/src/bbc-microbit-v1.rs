// @generated

pub mod pins {
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P0_17, pull : Up, active : Low, aliases : [BTN_A,] }, {
        name : Button1, pin : P0_26, pull : Up, active : Low, aliases : [BTN_B,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
