// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : PA5, });
    ariel_os_hal::define_i2c_buses![
        { name : i2c0, peripheral : I2C1, sda : PB7, scl : PB6, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
