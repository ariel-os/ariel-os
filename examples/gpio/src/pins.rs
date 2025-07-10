use ariel_os::hal::peripherals;

#[cfg(context = "nrf52840dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_13,
    btn1: P0_11
});

#[cfg(context = "bbc-microbit-v1")]
ariel_os::hal::define_peripherals!(Peripherals {
    led_col1: P0_04,
    led1: P0_13,
    btn1: P0_17
});

#[cfg(context = "bbc-microbit-v2")]
ariel_os::hal::define_peripherals!(Peripherals {
    led_col1: P0_28,
    led1: P0_21,
    btn1: P0_14
});

#[cfg(context = "dfrobot-firebeetle2-esp32-c6")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: GPIO15,
    btn1: GPIO1
});

#[cfg(context = "heltec-wifi-lora-32-v3")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: GPIO35,
    btn1: GPIO0,
});

#[cfg(context = "nordic-thingy-91-x-nrf9151")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_29,
    btn1: P0_26
});

#[cfg(context = "nrf5340dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_28,
    btn1: P0_23
});

#[cfg(context = "nrf9160dk-nrf9160")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_02,
    btn1: P0_06
});

#[cfg(context = "nrf52dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_17,
    btn1: P0_13
});

#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PIN_1,
    btn1: PIN_2
});

#[cfg(all(
    context = "esp",
    not(any(
        context = "dfrobot-firebeetle2-esp32-c6",
        context = "heltec-wifi-lora-32-v3",
    ))
))]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: GPIO0,
    btn1: GPIO1
});

#[cfg(context = "st-b-l475e-iot01a")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PA5,
    btn1: PC13
});

#[cfg(context = "st-nucleo-c031c6")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PA5,
    btn1: PC13
});

#[cfg(context = "st-nucleo-f042k6")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PB3,
    // Note: the board only has one physical button which is hardwired to NRST so this uses D2
    btn1: PA12
});

#[cfg(any(context = "st-nucleo-f401re", context = "st-nucleo-f411re"))]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PA5,
    btn1: PC13
});

#[cfg(context = "st-nucleo-h755zi-q")]
ariel_os::hal::define_peripherals!(Peripherals {
    btn1: PC13,
    led1: PB0
});

#[cfg(context = "st-nucleo-wb55")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PB5,
    btn1: PC4
});

#[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PA5,
    btn1: PC2
});

#[cfg(context = "st-steval-mkboxpro")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PF6,
    btn1: PC13
});

#[cfg(context = "seeedstudio-lora-e5-mini")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PB5,
    btn1: PB13
});
