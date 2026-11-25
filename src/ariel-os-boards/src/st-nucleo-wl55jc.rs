// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : PB15, color : blue, active : High,
        aliases : [LED1,] }, { field_name : led1, type_name : Led1, pin : PB9, color :
        green, active : High, aliases : [LED2,] }, { field_name : led2, type_name : Led2,
        pin : PB11, color : red, active : High, aliases : [LED3,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : PA0, pull : None, active :
        Low, aliases : [B1,] }, { field_name : button1, type_name : Button1, pin : PA1,
        pull : None, active : Low, aliases : [B2,] }, { field_name : button2, type_name :
        Button2, pin : PC6, pull : None, active : Low, aliases : [B3,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
