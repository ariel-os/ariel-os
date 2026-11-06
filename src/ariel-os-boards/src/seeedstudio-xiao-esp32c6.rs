// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : GPIO15, });
    ariel_os_hal::define_buttons![
        { name : Button0, pin : GPIO9, pull : Up, active : Low, aliases : [BOOT,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
