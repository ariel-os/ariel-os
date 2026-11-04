// @generated

pub mod pins {
    ariel_os_hal::define_peripherals!(
        LedPeripherals { led0 : P0_17, led1 : P0_18, led2 : P0_19, led3 : P0_20, }
    );
    ariel_os_hal::define_buttons![
        { name : Button0, pin : P0_13, pull : None, active : Low, aliases : [SW1,
        BUTTON1,] }, { name : Button1, pin : P0_14, pull : None, active : Low, aliases :
        [SW2, BUTTON2,] }, { name : Button2, pin : P0_15, pull : None, active : Low,
        aliases : [SW3, BUTTON13,] }, { name : Button3, pin : P0_16, pull : None, active
        : Low, aliases : [SW4, BUTTON4,] },
    ];
    ariel_os_hal::define_uarts![
        { name : Uart0, device : UARTE0, tx : P0_06, rx : P0_08, host_facing : true },
    ];
}
#[allow(unused_variables)]
pub fn init(peripherals: &mut ariel_os_hal::hal::OptionalPeripherals) {}
