// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_14, led1 : P0_30, led2 : P0_22, led3 : P0_31, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P0_02, pull : None, active : Low,], aliases :
        [BT_WAKE_UP,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_05, rx : P0_11, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
