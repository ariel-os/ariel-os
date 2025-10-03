use ariel_os::hal::peripherals;

#[cfg(context = "bbc-microbit-v2")]
ariel_os::hal::define_peripherals!(LedPeripherals {
    led_col1: P0_28,
    led_col2: P0_11,
    led_col3: P0_31,
    led_col4: P1_05,
    led_col5: P0_30,
    led_row1: P0_21,
    led_row2: P0_22,
    led_row3: P0_15,
    led_row4: P0_24,
    led_row5: P0_19,
});
