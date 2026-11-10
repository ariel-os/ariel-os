// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P0_13, color : green, active : Low,
        aliases : [] }, { field_name : led1, type_name : Led1, pin : P0_14, color :
        green, active : Low, aliases : [] }, { field_name : led2, type_name : Led2, pin :
        P0_15, color : green, active : Low, aliases : [] }, { field_name : led3,
        type_name : Led3, pin : P0_16, color : green, active : Low, aliases : [] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_11, pull : None, active :
        Low, aliases : [SW1,] }, { field_name : button1, type_name : Button1, pin :
        P0_12, pull : None, active : Low, aliases : [SW2,] }, { field_name : button2,
        type_name : Button2, pin : P0_24, pull : None, active : Low, aliases : [SW3,] },
        { field_name : button3, type_name : Button3, pin : P0_25, pull : None, active :
        Low, aliases : [SW4,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : TWISPI0, sda : P0_26, scl : P0_27, aliases : [] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_06, rx : P0_08, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
