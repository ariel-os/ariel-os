// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PB5, led1 : PA5, led2 : PB6, led3 : PB7, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PB2, });
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C1, sda : PB9, scl : PB8, aliases : [ArduinoI2c,]
        },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : USART2, tx : PA2, rx : PA3, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
