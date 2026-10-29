// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PF6, led1 : PH11, led2 : PH12, led3 : PF9, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : PC13, pull : Down, active : High,], aliases :
        [BT1, SW1,] }, { name : Switch1, pins : [pin : PE0, pull : Down, active : High,],
        aliases : [BT2, SW2, BOOT0,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C1, sda : PB7, scl : PB6, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
