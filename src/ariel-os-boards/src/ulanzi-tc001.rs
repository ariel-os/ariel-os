// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : GPIO26, button1 : GPIO27, button2 : GPIO14, }
    );
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
