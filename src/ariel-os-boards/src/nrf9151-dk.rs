// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_00, led1 : P0_01, led2 : P0_04, led3 : P0_05, }
    );
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_08, pull : None, active :
        Low, aliases : [BUTTON1, SW1,] }, { field_name : button1, type_name : Button1,
        pin : P0_09, pull : None, active : Low, aliases : [BUTTON2, SW2,] }, { field_name
        : button2, type_name : Button2, pin : P0_18, pull : None, active : Low, aliases :
        [BUTTON3, SW3,] }, { field_name : button3, type_name : Button3, pin : P0_19, pull
        : None, active : Low, aliases : [BUTTON4, SW4,] },
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
