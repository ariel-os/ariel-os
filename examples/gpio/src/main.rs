#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    gpio::{Level, Output},
    log::info,
    time::Timer,
};

ariel_os::hal::group_peripherals!(Peripherals {
    leds: pins::LedPeripherals,
    switch: pins::Switch0<'static>,
});

#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: Peripherals) {
    let mut led0 = Output::new(peripherals.leds.led0, Level::Low);

    let mut switch = peripherals.switch;

    loop {
        info!("Switch status: {:?}", switch.status());

        // Assuming external interrupts
        let _ = embassy_futures::select::select(switch.wait_for_low().0, Timer::after_millis(300))
            .await;

        led0.toggle();
        Timer::after_millis(100).await;
    }
}
