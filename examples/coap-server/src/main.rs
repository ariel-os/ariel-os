#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]

#[ariel_os::task(autostart)]
async fn coap_run() {
    use coap_handler_implementations::{new_dispatcher, HandlerBuilder, SimpleRendered};

    let handler = new_dispatcher()
        // We offer a single resource: /hello, which responds just with a text string.
        .at(&["hello"], SimpleRendered("Hello from Ariel OS"));

    ariel_os::coap::coap_run(handler).await;
}
