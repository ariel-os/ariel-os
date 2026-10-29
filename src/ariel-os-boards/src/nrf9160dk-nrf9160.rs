// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_02, led1 : P0_03, led2 : P0_04, led3 : P0_05, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P0_06, pull : None, active : Low,], aliases :
        [SW3, BUTTON1,] }, { name : Switch1, pins : [pin : P0_07, pull : None, active :
        Low,], aliases : [SW4, BUTTON2,] }, { name : Switch2, pins : [pin : P0_08, pull :
        None, active : Low,], aliases : [SW1, SWITCH1,] }, { name : Switch3, pins : [pin
        : P0_09, pull : None, active : Low,], aliases : [SW2, SWITCH2,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : SERIAL0, sda : P0_30, scl : P0_31, aliases : [] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : SERIAL3, tx : P0_29, rx : P0_28, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
