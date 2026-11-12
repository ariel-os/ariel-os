// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P0_02, color : green, active : High,
        aliases : [LED1,] }, { field_name : led1, type_name : Led1, pin : P0_03, color :
        green, active : High, aliases : [LED2,] }, { field_name : led2, type_name : Led2,
        pin : P0_04, color : green, active : High, aliases : [LED3,] }, { field_name :
        led3, type_name : Led3, pin : P0_05, color : green, active : High, aliases :
        [LED4,] },
    ];
    ariel_os_hal::define_switches![
        { field_name : switch0, type_name : Switch0, pins : [pin : P0_08, pull : None,
        active : Low,], aliases : [SW1, SWITCH1,] }, { field_name : switch1, type_name :
        Switch1, pins : [pin : P0_09, pull : None, active : Low,], aliases : [SW2,
        SWITCH2,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_06, pull : None, active :
        Low, aliases : [SW3, BUTTON1,] }, { field_name : button1, type_name : Button1,
        pin : P0_07, pull : None, active : Low, aliases : [SW4, BUTTON2,] },
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
