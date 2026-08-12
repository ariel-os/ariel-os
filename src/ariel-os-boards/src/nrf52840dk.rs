// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_13, led1 : P0_14, led2 : P0_15, led3 : P0_16, }
    );
    ariel_os_hal::define_peripherals!(
        ButtonPeripherals { button0 : P0_11, button1 : P0_12, button2 : P0_24, button3 :
        P0_25, }
    );
    ariel_os_hal::define_uarts![
        { name : uart0, device : UARTE0, tx : P0_06, rx : P0_08, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
