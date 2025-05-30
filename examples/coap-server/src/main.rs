#![no_main]
#![no_std]

#[ariel_os::task(autostart)]
async fn coap_run() {
    use coap_handler_implementations::{HandlerBuilder, SimpleRendered, new_dispatcher};

    let handler = new_dispatcher()
        // We offer a single resource: /hello, which responds just with a text string.
        .at(&["hello"], SimpleRendered("Hello from Ariel OS"));

    ariel_os::coap::coap_run(handler).await;
}
