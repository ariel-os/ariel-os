// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_29, led1 : P0_31, led2 : P0_30, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P0_26, pull : None, active : Low,], aliases :
        [SW3, Button_1,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : SERIAL0, sda : P0_09, scl : P0_08, aliases :
        [QwiicI2c,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : SERIAL3, tx : P0_01, rx : P0_00, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
