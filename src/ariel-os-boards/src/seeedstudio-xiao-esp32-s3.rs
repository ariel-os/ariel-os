// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : GPIO21, color : yellow, active : Low,], aliases
        : [USER_LED,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : GPIO0, pull : Up, active : Low, aliases : [BOOT, K2,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
