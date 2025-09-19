#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    gpio::{Level, Output},
    time::Timer,
};

#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: pins::LedPeripherals) {
    let mut led0 = Output::new(peripherals.led0, Level::Low);

    // // The micro:bit uses an LED matrix; pull the column line low.
    // #[cfg(any(context = "bbc-microbit-v2", context = "bbc-microbit-v1"))]
    let p = unsafe { ariel_os::hal::peripherals::P0_28::steal() };
    let _led_col1 = Output::new(p, Level::Low);
    core::mem::forget(_led_col1);

    loop {
        led0.toggle();
        Timer::after_millis(500).await;
    }
}
