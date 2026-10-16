// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(LedPeripherals { led0 : GPIO0, });
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : GPIO9, });
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UART0, tx : GPIO20, rx : GPIO19, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
