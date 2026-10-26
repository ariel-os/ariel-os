// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P2_09, led1 : P1_10, led2 : P2_07, led3 : P1_14, }
    );
    ariel_os_hal::define_switches![
        { name : Switch0, pins : [pin : P1_13, pull : None, active : Low,], aliases :
        [SW2, Button0,] }, { name : Switch1, pins : [pin : P1_09, pull : None, active :
        Low,], aliases : [SW3, Button1,] }, { name : Switch2, pins : [pin : P1_08, pull :
        None, active : Low,], aliases : [SW5, Button2,] }, { name : Switch3, pins : [pin
        : P0_04, pull : None, active : Low,], aliases : [SW6, Button3,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
