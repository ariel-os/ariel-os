// @generated

pub mod pins {
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : GPIO26, pull : None, active : Low,], aliases :
        [left,] }, { name : Switch1, pins : [pin : GPIO27, pull : None, active : Low,],
        aliases : [middle,] }, { name : Switch2, pins : [pin : GPIO14, pull : None,
        active : Low,], aliases : [right,] },
    ];
    ariel_os_hal::define_i2c_buses![
        { name : I2c0, peripheral : I2C0, sda : GPIO21, scl : GPIO22, aliases : [] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UART0, tx : GPIO1, rx : GPIO3, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {
    {
        let pin = peripherals.GPIO15.take().unwrap();
        let output = ariel_os_hal::gpio::Output::new(
            pin,
            ariel_os_embassy_common::gpio::Level::Low,
        );
        core::mem::forget(output);
    }
}
