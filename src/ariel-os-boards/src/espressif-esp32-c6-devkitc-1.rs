// @generated

pub mod pins {
    ariel_os_hal::define_buttons![
        { name : Button0, pin : GPIO9, pull : None, active : Low, aliases : [SW1,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UART0, tx : GPIO16, rx : GPIO17, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
