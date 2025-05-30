#![no_main]
#![no_std]

use ariel_os::debug::log::info;

#[ariel_os::task(autostart)]
async fn run_client_operations() {
    use coap_request::Stack;

    let client = ariel_os::coap::coap_client().await;

    // Corresponding to the fixed network setup, we select a fixed server address; this may need to
    // be updated on hosts that are configured differently.
    let addr = "10.42.0.1:5683"; // IPv4 🔔
    let demoserver = addr.parse().unwrap();

    info!("Sending POST to {}...", demoserver);
    let request = coap_request_implementations::Code::post()
        .with_path("/uppercase")
        .with_request_payload_slice(b"This is Ariel OS")
        .processing_response_payload_through(|p| {
            info!(
                "Uppercase response is {:?}",
                core::str::from_utf8(p).map_err(|_| "not Unicode?")
            );
        });
    let response = client.to(demoserver).request(request).await;
    info!("Response {:?}", response.map_err(|_| "TransportError"));
}
