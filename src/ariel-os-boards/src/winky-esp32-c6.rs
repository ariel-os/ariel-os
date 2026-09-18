// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(ButtonPeripherals { button0 : GPIO9, });
    ariel_os_hal::define_uarts![
        { name : uart0, device : UART0, tx : GPIO16, rx : GPIO17, host_facing : true }, {
        name : uart1, device : UART1, tx : GPIO15, rx : GPIO4, host_facing : false },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
