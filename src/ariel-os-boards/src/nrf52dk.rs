// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P0_17, color : green, active : Low,
        aliases : [LED1,] }, { field_name : led1, type_name : Led1, pin : P0_18, color :
        green, active : Low, aliases : [LED2,] }, { field_name : led2, type_name : Led2,
        pin : P0_19, color : green, active : Low, aliases : [LED3,] }, { field_name :
        led3, type_name : Led3, pin : P0_20, color : green, active : Low, aliases :
        [LED4,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_13, pull : None, active :
        Low, aliases : [SW1, BUTTON1,] }, { field_name : button1, type_name : Button1,
        pin : P0_14, pull : None, active : Low, aliases : [SW2, BUTTON2,] }, { field_name
        : button2, type_name : Button2, pin : P0_15, pull : None, active : Low, aliases :
        [SW3, BUTTON3,] }, { field_name : button3, type_name : Button3, pin : P0_16, pull
        : None, active : Low, aliases : [SW4, BUTTON4,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_06, rx : P0_08, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
