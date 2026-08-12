// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : GPIO15, });
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : GPIO9, });
    ariel_os_hal::define_uarts![
        { name : uart0, device : UART0, tx : GPIO16, rx : GPIO17, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
