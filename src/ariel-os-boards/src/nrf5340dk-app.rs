// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_28, led1 : P0_29, led2 : P0_30, led3 : P0_31, }
    );
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_23, pull : None, active :
        Low, aliases : [SW1,] }, { field_name : button1, type_name : Button1, pin :
        P0_24, pull : None, active : Low, aliases : [SW2,] }, { field_name : button2,
        type_name : Button2, pin : P0_08, pull : None, active : Low, aliases : [SW3,] },
        { field_name : button3, type_name : Button3, pin : P0_09, pull : None, active :
        Low, aliases : [SW4,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : SERIAL0, sda : P1_02, scl : P1_03, aliases : [] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : SERIAL3, tx : P1_01, rx : P1_00, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
