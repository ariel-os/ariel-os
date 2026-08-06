// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P0_14, button1 : P0_23, }
    );
    ariel_os_hal::define_i2c_buses![
        { name : i2c0, peripheral : TWISPI0, sda : P0_16, scl : P0_08 },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
