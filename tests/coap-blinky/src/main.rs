#![no_main]
#![no_std]

use ariel_os::gpio::{Level, Output};
use ariel_os_boards::pins;

#[ariel_os::task(autostart, peripherals)]
async fn coap_run(peripherals: pins::LedPeripherals) {
    use coap_handler_implementations::{HandlerBuilder, new_dispatcher};

    let led = Output::new(peripherals.led0, Level::Low);

    let handler = new_dispatcher()
        // We offer a single resource: /led, which CBOR true or false can be PUT into
        .at(
            &["led"],
            riot_coap_handler_demos::gpio::handler_for_output(led),
        );

    ariel_os::coap::coap_run(handler).await;
}
