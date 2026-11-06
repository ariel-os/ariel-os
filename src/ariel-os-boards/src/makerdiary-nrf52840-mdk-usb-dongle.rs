// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P0_23, color : red, active : Low,], aliases : []
        }, { name : Led1, channels : [pin : P0_22, color : green, active : Low,], aliases
        : [] }, { name : Led2, channels : [pin : P0_24, color : blue, active : Low,],
        aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
