// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_14, led1 : P0_26, led2 : P0_15, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P0_24, pull : None, active : Low,], aliases :
        [SW4, Button_2,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : SERIAL0, sda : P1_03, scl : P1_02, aliases :
        [QwiicI2c,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
