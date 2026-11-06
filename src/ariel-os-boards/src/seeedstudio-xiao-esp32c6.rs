// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : GPIO15, color : yellow, active : Low,], aliases
        : [D4, USER_LED,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : GPIO9, pull : Up, active : Low, aliases : [BOOT,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
