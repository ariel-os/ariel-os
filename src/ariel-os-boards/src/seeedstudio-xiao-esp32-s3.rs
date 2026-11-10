// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : GPIO21, });
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : GPIO0, pull : Up, active :
        Low, aliases : [BOOT, K2,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
