// @generated

pub mod pins {
    ariel_os_hal::define_leds![
        { name : Led0, channels : [pin : P0_23, color : red, active : Low,], aliases : []
        }, { name : Led1, channels : [pin : P0_22, color : green, active : Low,], aliases
        : [] }, { name : Led2, channels : [pin : P0_24, color : blue, active : Low,],
        aliases : [] },
    ];
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P1_00, pull : None, active : Low, aliases : [USR_BTN,
        SW2,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_20, rx : P0_19, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
