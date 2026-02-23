// @generated

pub mod pins {
    use ariel_os_hal::hal::peripherals;
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_00, led1 : P0_01, led2 : P0_04, led3 : P0_05, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P0_08, button1 : P0_09, button2 : P0_18, button3 :
        P0_19, }
    );
    ariel_os_hal::define_uarts![
        { name : VCOM0, device : SERIAL3, tx : P0_27, rx : P0_26, host_facing : true }, {
        name : VCOM1, device : SERIAL3, tx : P0_29, rx : P0_28, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
