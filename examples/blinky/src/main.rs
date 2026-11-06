#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::time::Timer;

#[ariel_os::task(autostart, peripherals)]
async fn blinky(mut led: pins::Led0) {
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
