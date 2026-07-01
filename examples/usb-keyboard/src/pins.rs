#[cfg(context = "nrf52840dk")]
ariel_os::hal::define_peripherals!(Buttons {
    btn1: P0_11,
    btn2: P0_12,
    btn3: P0_24,
    btn4: P0_25,
});

#[cfg(context = "nrf5340dk-app")]
ariel_os::hal::define_peripherals!(Buttons {
    btn1: P0_23,
    btn2: P0_24,
    btn3: P0_08,
    btn4: P0_09,
});
