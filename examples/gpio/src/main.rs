#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    gpio::{Level, Output},
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
        ariel_os::log::info!("Is Switch0 active ? {}", switch.pins().0.is_active());
        let _ = embassy_futures::select::select(
            switch.pins().0.wait_for_active(),
            Timer::after_millis(300),
        )
        .await;

        led0.toggle();
        Timer::after_millis(100).await;
    }
}
