// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PH10, led1 : PH11, led2 : PH12, led3 : PH13, led4 : PH14,
        led5 : PH15, }
    );
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
