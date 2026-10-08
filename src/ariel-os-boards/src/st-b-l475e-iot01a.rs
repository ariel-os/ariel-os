// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : PA5, led1 : PB14, });
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PC13, });
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C1, sda : PB9, scl : PB8, aliases : [ArduinoI2c,]
        }, { name : I2c1, peripheral : I2C2, sda : PB11, scl : PB10, aliases : [] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART1, tx : PB6, rx : PB7, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
