#![no_main]
#![no_std]

#[ariel_os::task(autostart)]
async fn coap_run() {
    use coap_handler_implementations::{HandlerBuilder, SimpleRendered, new_dispatcher};

    let handler = new_dispatcher()
        // We offer two resources, /hello and /admin, which both respond with a plain text string.
        //
        // Who is allowed to access them is configured in ../peers.yml.
        .at(&["hello"], SimpleRendered("Hello from Ariel OS"))
        .at(
            &["admin"],
            SimpleRendered("Congratulations, you are authorized."),
        );

    ariel_os::coap::coap_run(handler).await;
}
