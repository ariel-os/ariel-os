// @generated

pub mod pins {
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P0_17, pull : Up, active : Low,], aliases :
        [BTN_A,] }, { name : Switch1, pins : [pin : P0_26, pull : Up, active : Low,],
        aliases : [BTN_B,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
