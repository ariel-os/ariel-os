// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P2_09, led1 : P1_10, led2 : P2_07, led3 : P1_14, }
    );
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P1_13, pull : None, active :
        Low, aliases : [SW2,] }, { field_name : button1, type_name : Button1, pin :
        P1_09, pull : None, active : Low, aliases : [SW3,] }, { field_name : button2,
        type_name : Button2, pin : P1_08, pull : None, active : Low, aliases : [SW5,] },
        { field_name : button3, type_name : Button3, pin : P0_04, pull : None, active :
        Low, aliases : [SW6,] },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
