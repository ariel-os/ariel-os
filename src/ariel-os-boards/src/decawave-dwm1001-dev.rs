// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { field_name : led0, type_name : Led0, pin : P0_14, color : red, active : Low,
        aliases : [D12, GPIO14,] }, { field_name : led1, type_name : Led1, pin : P0_30,
        color : green, active : Low, aliases : [D9,] }, { field_name : led2, type_name :
        Led2, pin : P0_22, color : red, active : Low, aliases : [D11,] }, { field_name :
        led3, type_name : Led3, pin : P0_31, color : blue, active : Low, aliases : [D10,]
        },
    ];
    ariel_os_hal::define_buttons![
        { field_name : button0, type_name : Button0, pin : P0_02, pull : None, active :
        Low, aliases : [BT_WAKE_UP,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_05, rx : P0_11, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
