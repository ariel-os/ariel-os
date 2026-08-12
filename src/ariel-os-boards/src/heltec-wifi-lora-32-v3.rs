// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : GPIO35, });
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : GPIO0, });
    ariel_os_hal::define_i2c_buses![
        { name : i2c0, peripheral : I2C1, sda : GPIO17, scl : GPIO18, aliases : [] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
