// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P0_28, color : green, active : Low,
        aliases : [LED1,] }, { field_name : led1, type_name : Led1, pin : P0_29, color :
        green, active : Low, aliases : [LED2,] }, { field_name : led2, type_name : Led2,
        pin : P0_30, color : green, active : Low, aliases : [LED3,] }, { field_name :
        led3, type_name : Led3, pin : P0_31, color : green, active : Low, aliases :
        [LED4,] },
    ];
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
