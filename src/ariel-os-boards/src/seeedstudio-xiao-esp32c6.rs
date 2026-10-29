// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : GPIO15, });
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : GPIO9, pull : Up, active : Low,], aliases :
        [BOOT,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
