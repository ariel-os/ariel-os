// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_00, led1 : P0_01, led2 : P0_04, led3 : P0_05, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P0_08, pull : None, active : Low,], aliases :
        [Button_1, SW1,] }, { name : Switch1, pins : [pin : P0_09, pull : None, active :
        Low,], aliases : [Button_2, SW2,] }, { name : Switch2, pins : [pin : P0_18, pull
        : None, active : Low,], aliases : [Button_3, SW3,] }, { name : Switch3, pins :
        [pin : P0_19, pull : None, active : Low,], aliases : [Button_4, SW4,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : SERIAL0, sda : P0_30, scl : P0_31, aliases : [] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : SERIAL3, tx : P0_27, rx : P0_26, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
