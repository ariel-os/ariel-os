// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : P1_12, });
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P0_11, pull : None, active : Low,], aliases :
        [MODE,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
