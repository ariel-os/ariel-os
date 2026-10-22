#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    gpio::{Level, Output},
    time::Timer,
    log::info,
};

ariel_os::hal::group_peripherals!(Peripherals {
    leds: pins::LedPeripherals,
    joystick: pins::Joystick<'static>,
    // buttons: pins::ButtonPeripherals,
});

#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: Peripherals) {
    let mut led0 = Output::new(peripherals.leds.led0, Level::Low);

    let joystick = peripherals.joystick;
    loop {
        info!("Is the button High {}", joystick.PC2.is_high());

        led0.toggle();
        Timer::after_millis(100).await;
    }
}
