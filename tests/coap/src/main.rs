#![no_main]
#![no_std]

#[ariel_os::task(autostart)]
async fn coap_run() {
    use coap_handler_implementations::HandlerBuilder;

    let log = None;
    let buffer = scroll_ring::Buffer::<512>::default();
    // FIXME: Why doesn't scroll_ring provide that?
    struct Stdout<'a>(&'a scroll_ring::Buffer<512>);
    impl core::fmt::Write for Stdout<'_> {
        fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
            self.0.write(s.as_bytes());
            Ok(())
        }
    }
    let mut stdout = Stdout(&buffer);
    use core::fmt::Write;
    writeln!(stdout, "We have our own stdout now.").unwrap();
    writeln!(stdout, "With rings and atomics.").unwrap();

    let handler = coap_message_demos::full_application_tree(log).at(
        &["stdout"],
        coap_scroll_ring_server::BufferHandler::new(&buffer),
    );

    // going with an embassy_futures join instead of Ariel OS's spawn to avoid the need for making
    // stdout static.
    embassy_futures::join::join(
        ariel_os::coap::coap_run(handler),
        run_client_operations(stdout),
    )
    .await;
}

/// In parallel to server operation, this function performs some operations as a client.
///
/// This doubles as an experimentation ground for the client side of embedded_nal_coap and
/// coap-request in general.
async fn run_client_operations(mut stdout: impl core::fmt::Write) {
    let client = ariel_os::coap::coap_client().await;

    // Corresponding to the fixed network setup, we select a fixed server address; this may need to
    // be updated on hosts that are configured differently.
    let addr = "10.42.0.1:1234"; // IPv4 🔔
    let demoserver = addr.parse().unwrap();

    use coap_request::Stack;
    writeln!(stdout, "Sending GET to {}...", addr).unwrap();
    let response = client
        .to(demoserver)
        .request(
            coap_request_implementations::Code::get()
                .with_path("/other/separate")
                .processing_response_payload_through(|p| {
                    writeln!(stdout, "Got payload {:?}", p).unwrap();
                }),
        )
        .await;
    writeln!(
        stdout,
        "Response {:?}",
        response.map_err(|_| "TransportError")
    )
    .unwrap();

    let req = coap_request_implementations::Code::post().with_path("/uppercase");

    writeln!(stdout, "Sending POST...").unwrap();
    let mut response = client.to(demoserver);
    let response = response.request(
        req.with_request_payload_slice(b"Set time to 1955-11-05")
            .processing_response_payload_through(|p| {
                writeln!(stdout, "Uppercase is {}", core::str::from_utf8(p).unwrap()).unwrap();
            }),
    );
    let response = response.await;
    writeln!(
        stdout,
        "Response {:?}",
        response.map_err(|_| "TransportError")
    )
    .unwrap();
}
