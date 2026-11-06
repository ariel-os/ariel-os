// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : P1_15, led1 : P1_10, });
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P1_02, pull : None, active : Low, aliases : [UserSw, SW2,
        SWITCH,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
