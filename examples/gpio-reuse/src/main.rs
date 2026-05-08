#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    gpio::{Input, Level, Output, Pull},
    log::info,
    time::Timer,
};

#[ariel_os::task(autostart, peripherals)]
async fn blinky(mut peripherals: pins::LedPeripherals) {
    loop {
        // First, set the GPIO as input and read its level.
        // Some HALs require dropping both the driver itself and the peripheral ZST before using it
        // again for another driver.
        {
            let input = Input::new(peripherals.led0.reborrow(), Pull::Down);

            let level = input.get_level();
            info!("GPIO level: {:?}", level);
        }

        // Then, make the LED blink.
        let mut led = Output::new(peripherals.led0.reborrow(), Level::Low);

        for _ in 0..(2 * 3) {
            led.toggle();
            Timer::after_millis(500).await;
        }
    }
}
