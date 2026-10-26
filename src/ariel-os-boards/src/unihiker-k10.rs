// @generated

pub mod pins {
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : GPIO0, pull : None, active : Low,], aliases :
        [BOOT, K1,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C0, sda : GPIO47, scl : GPIO48, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
