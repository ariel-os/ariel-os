// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PF6, led1 : PH11, led2 : PH12, led3 : PF9, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : PC13, button1 : PE0, }
    );
    ariel_os_hal::define_i2c_buses![
        { name : i2c0, peripheral : I2C1, sda : PB7, scl : PB6, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
