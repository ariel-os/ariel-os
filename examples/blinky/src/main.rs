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

    loop {
        led0.toggle();
        Timer::after_millis(500).await;
    }
}
