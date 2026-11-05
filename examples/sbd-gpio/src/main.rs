#![no_main]
#![no_std]

use ariel_os_boards::pins;

use ariel_os::{
    gpio::{Level, Output}, time::Timer,
};

#[cfg(not(feature = "use-switches"))]
ariel_os::hal::group_peripherals!(Peripherals {
    leds: pins::LedPeripherals,
    button: pins::Button0,
});


#[cfg(feature = "use-switches")]
ariel_os::hal::group_peripherals!(Peripherals {
    leds: pins::LedPeripherals,
    button: pins::Button0,
    switch: pins::Switch0,
});



#[ariel_os::task(autostart, peripherals)]
async fn blinky(peripherals: Peripherals) {
    let mut led0 = Output::new(peripherals.leds.led0, Level::Low);

    let mut button = peripherals.button;

    #[cfg(feature = "use-switches")]
    let mut switch = peripherals.switch;
    #[cfg(feature = "use-switches")]
    {
        ariel_os::log::info!("Waiting for the switch to be active");
        switch.wait_for_active().await;
        ariel_os::log::info!("Turning the switch off will end the program");
    }


    loop {
        #[cfg(feature = "use-switches")]
        if switch.is_inactive() { break; }

        let _ = embassy_futures::select::select(button.wait_for_pressed(), Timer::after_millis(300))
            .await;

        led0.toggle();
        Timer::after_millis(100).await;
    }
}
