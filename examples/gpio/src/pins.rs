use ariel_os::hal::peripherals;

#[cfg(context = "nrf52840dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_13,
    btn1: P0_11
});

#[cfg(context = "bbc-microbit-v2")]
ariel_os::hal::define_peripherals!(Peripherals {
    led_col1: P0_28,
    led1: P0_21,
    btn1: P0_14
});

#[cfg(context = "nordic-thingy-91")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_29,
    btn1: P0_26
});

#[cfg(context = "nrf5340dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_28,
    btn1: P0_23
});

#[cfg(context = "nrf9160dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: P0_02,
    btn1: P0_06
});

#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PIN_1,
    btn1: PIN_2
});

#[cfg(context = "esp")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: GPIO0,
    btn1: GPIO1
});

#[cfg(context = "st-nucleo-c031c6")]
ariel_os::hal::define_peripherals!(Peripherals {
    led1: PA5,
    btn1: PC13
});

#[cfg(context = "st-nucleo-f401re")]
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
