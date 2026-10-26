// @generated

pub mod pins {
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P0_14, pull : Up, active : Low,], aliases :
        [BTN_A,] }, { name : Switch1, pins : [pin : P0_23, pull : Up, active : Low,],
        aliases : [BTN_B,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : TWISPI0, sda : P0_16, scl : P0_08, aliases : [] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_06, rx : P1_08, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
