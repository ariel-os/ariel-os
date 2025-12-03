#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    debug::log::info,
    gpio::{Input, Level, Pull},
    time::Timer,
};

ariel_os::hal::group_peripherals!(Peripherals {
    buttons: pins::ButtonPeripherals,
});

#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: Peripherals) {
    #[allow(unused_variables)]
    let pull = Pull::Up;
    #[cfg(context = "st-nucleo-h755zi-q")]
    let pull = Pull::None;

    let mut btn0 = Input::builder(peripherals.buttons.button0, pull)
        .build_with_interrupt()
        .unwrap();

    loop {
        // Wait for the button being pressed or 300 ms, whichever comes first.
        let _ =
            embassy_futures::select::select(btn0.wait_for_low(), Timer::after_millis(300)).await;
        info!("Button pressed!");
    }
}
