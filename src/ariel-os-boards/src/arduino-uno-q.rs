// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : PH10, color : red, active : Low,
        aliases : [] }, { field_name : led1, type_name : Led1, pin : PH11, color : green,
        active : Low, aliases : [] }, { field_name : led2, type_name : Led2, pin : PH12,
        color : blue, active : Low, aliases : [] }, { field_name : led3, type_name :
        Led3, pin : PH13, color : red, active : Low, aliases : [] }, { field_name : led4,
        type_name : Led4, pin : PH14, color : green, active : Low, aliases : [] }, {
        field_name : led5, type_name : Led5, pin : PH15, color : blue, active : Low,
        aliases : [] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C2, sda : PB11, scl : PB10, aliases : [ArduinoI2c,]
        }, { name : I2c1, peripheral : I2C4, sda : PD13, scl : PD12, aliases :
        [QwiicI2c,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : LPUART1, tx : PG7, rx : PG8, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
