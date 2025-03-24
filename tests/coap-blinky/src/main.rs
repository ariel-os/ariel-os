#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]

#[path = "../../../examples/blinky/src/pins.rs"]
mod pins;

use ariel_os::gpio::{Level, Output};

#[ariel_os::task(autostart, peripherals)]
async fn coap_run(peripherals: pins::LedPeripherals) {
    use coap_handler_implementations::{new_dispatcher, HandlerBuilder};

    let led = Output::new(peripherals.led, Level::Low);

    // The micro:bit uses an LED matrix; pull the column line low; see blinky example
    #[cfg(context = "bbc-microbit-v2")]
    let _led_col1 = Output::new(peripherals.led_col1, Level::Low);

    let handler = new_dispatcher()
        // We offer a single resource: /led, which CBOR true or false can be PUT into
        .at(
            &["led"],
            riot_coap_handler_demos::gpio::handler_for_output(led),
        );

    ariel_os::coap::coap_run(handler).await;
}
