// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P0_00, color : green, active : High,], aliases :
        [LED1,] }, { name : Led1, channels : [pin : P0_01, color : green, active :
        High,], aliases : [LED2,] }, { name : Led2, channels : [pin : P0_04, color :
        green, active : High,], aliases : [LED3,] }, { name : Led3, channels : [pin :
        P0_05, color : green, active : High,], aliases : [LED4,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P0_08, pull : None, active : Low, aliases : [BUTTON1,
        SW1,] }, { name : Button1, pin : P0_09, pull : None, active : Low, aliases :
        [BUTTON2, SW2,] }, { name : Button2, pin : P0_18, pull : None, active : Low,
        aliases : [BUTTON3, SW3,] }, { name : Button3, pin : P0_19, pull : None, active :
        Low, aliases : [BUTTON4, SW4,] },
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
