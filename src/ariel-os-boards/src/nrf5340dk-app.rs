// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P0_28, color : green, active : Low,], aliases :
        [LED1,] }, { name : Led1, channels : [pin : P0_29, color : green, active : Low,],
        aliases : [LED2,] }, { name : Led2, channels : [pin : P0_30, color : green,
        active : Low,], aliases : [LED3,] }, { name : Led3, channels : [pin : P0_31,
        color : green, active : Low,], aliases : [LED4,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P0_23, pull : None, active : Low, aliases : [SW1,] }, {
        name : Button1, pin : P0_24, pull : None, active : Low, aliases : [SW2,] }, {
        name : Button2, pin : P0_08, pull : None, active : Low, aliases : [SW3,] }, {
        name : Button3, pin : P0_09, pull : None, active : Low, aliases : [SW4,] },
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
