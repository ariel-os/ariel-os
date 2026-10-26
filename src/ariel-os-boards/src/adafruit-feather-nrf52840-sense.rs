// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : P1_09, led1 : P1_10, });
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P1_02, pull : None, active : Low,], aliases :
        [UserSw, SW2, SWITCH,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : TWISPI0, sda : P0_12, scl : P0_11, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
