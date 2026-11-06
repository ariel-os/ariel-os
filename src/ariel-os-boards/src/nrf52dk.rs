// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P0_17, color : green, active : Low,], aliases :
        [LED1,] }, { name : Led1, channels : [pin : P0_18, color : green, active : Low,],
        aliases : [LED2,] }, { name : Led2, channels : [pin : P0_19, color : green,
        active : Low,], aliases : [LED3,] }, { name : Led3, channels : [pin : P0_20,
        color : green, active : Low,], aliases : [LED4,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P0_13, pull : None, active : Low, aliases : [SW1,
        BUTTON1,] }, { name : Button1, pin : P0_14, pull : None, active : Low, aliases :
        [SW2, BUTTON2,] }, { name : Button2, pin : P0_15, pull : None, active : Low,
        aliases : [SW3, BUTTON3,] }, { name : Button3, pin : P0_16, pull : None, active :
        Low, aliases : [SW4, BUTTON4,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_06, rx : P0_08, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
