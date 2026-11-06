// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P0_14, color : red, active : Low,], aliases :
        [D12, GPIO14,] }, { name : Led1, channels : [pin : P0_30, color : green, active :
        Low,], aliases : [D9,] }, { name : Led2, channels : [pin : P0_22, color : red,
        active : Low,], aliases : [D11,] }, { name : Led3, channels : [pin : P0_31, color
        : blue, active : Low,], aliases : [D10,] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P0_02, pull : None, active : Low, aliases : [BT_WAKE_UP,]
        },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_05, rx : P0_11, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
