#![no_main]
#![no_std]

use ariel_os_boards::pins;

#[ariel_os::task(autostart, peripherals)]
async fn coap_run(led: pins::Led0) {
    use coap_handler_implementations::{HandlerBuilder, new_dispatcher};

    let led = led.into_outputs().0;

    let handler = new_dispatcher()
        // We offer a single resource: /led, which CBOR true or false can be PUT into
        .at(
            &["led"],
            riot_coap_handler_demos::gpio::handler_for_output(led),
        );

    ariel_os::coap::coap_run(handler).await;
}
