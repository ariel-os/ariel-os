// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : PA5, led1 : PA7, led2 : PB1, }
    );
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : PA0, });
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C2, sda : PH5, scl : PH4, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
