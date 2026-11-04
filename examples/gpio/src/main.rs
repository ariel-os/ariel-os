#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    gpio::{Level, Output},
    time::Timer,
};

ariel_os::hal::group_peripherals!(Peripherals {
    leds: pins::LedPeripherals,
    button: pins::Button0<'static>,
});

#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: Peripherals) {
    let mut led0 = Output::new(peripherals.leds.led0, Level::Low);

    let mut button = peripherals.button;

    ariel_os::log::info!("Waiting for a double click within 500 ms");
    button.wait_for_double_click(500).await;

    loop {
        ariel_os::log::info!("Is Button0 active ? {}", button.is_active());
        let _ = embassy_futures::select::select(
            button.wait_for_active(),
            Timer::after_millis(300),
        )
        .await;

        led0.toggle();
        Timer::after_millis(100).await;
    }
}
