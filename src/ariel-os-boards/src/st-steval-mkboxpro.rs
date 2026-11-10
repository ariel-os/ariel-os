// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : PF6, color : green, active : High,
        aliases : [LED1, D5,] }, { field_name : led1, type_name : Led1, pin : PH11, color
        : red, active : High, aliases : [LED2, D6,] }, { field_name : led2, type_name :
        Led2, pin : PH12, color : orange, active : High, aliases : [LED3, D7,] }, {
        field_name : led3, type_name : Led3, pin : PF9, color : blue, active : High,
        aliases : [USB_enum_LED, D10,] },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : PC13, pull : Down, active :
        High, aliases : [BT1, SW1,] }, { field_name : button1, type_name : Button1, pin :
        PE0, pull : Down, active : High, aliases : [BT2, SW2, BOOT0,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C1, sda : PB7, scl : PB6, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
