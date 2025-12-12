//! Transport implementation for CoAP-over-UDP.

use core::net::{Ipv6Addr, SocketAddr};

use ariel_os_debug::log::info;
use ariel_os_embassy::cell::SameExecutorCell;

use embassy_net::udp::{PacketMetadata, UdpSocket};
use static_cell::StaticCell;

use super::udp_nal;
use super::{CLIENT_READY, CONCURRENT_REQUESTS};

/// Runs the CoAP handler on CoAP-over-UDP indefinitely.
///
/// # Panics
///
/// This can only be run once, as it sets up a system wide CoAP handler.
pub(crate) async fn coap_run_udp(mut handler: impl coap_handler::Handler) -> ! {
    static COAP: StaticCell<embedded_nal_coap::CoAPShared<CONCURRENT_REQUESTS>> = StaticCell::new();

    let stack = ariel_os_embassy::net::network_stack().await.unwrap();

    // There's no strong need to wait this early (it matters that we wait before populating
    // CLIENT_READY), but this is a convenient place in the code (we have a `stack` now, to populate
    // CLIENT_READY after the server, we'd have to poll the server and `wait_config_up` in parallel), and
    // it's not like we'd expect requests to come in before everything is up. (Not even a loopback
    // request, because we shouldn't hand out a client early).
    stack.wait_config_up().await;

    // FIXME trim to CoAP requirements (those values are just a likely good starting point for "we
    // process any message immediately anyway")
    let mut rx_meta = [PacketMetadata::EMPTY; 2];
    let mut rx_buffer = [0; 1500];
    let mut tx_meta = [PacketMetadata::EMPTY; 2];
    let mut tx_buffer = [0; 1500];

    let socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );

    info!("Starting up CoAP server");

    let local_any = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 5683);
    let mut unconnected = udp_nal::UnconnectedUdp::bind_multiple(socket, local_any)
        .await
        .unwrap();

    info!("Server is ready.");

    let coap = COAP.init_with(embedded_nal_coap::CoAPShared::new);
    let (client, server) = coap.split();
    #[expect(
        clippy::items_after_statements,
        reason = "This is the item's place in the workflow."
    )]
    static CLIENT: StaticCell<embedded_nal_coap::CoAPRuntimeClient<'static, CONCURRENT_REQUESTS>> =
        StaticCell::new();

    CLIENT_READY
        .sender()
        .send(SameExecutorCell::new_async(&*CLIENT.init(client)).await);

    server
        .run(
            &mut unconnected,
            &mut handler,
            &mut ariel_os_random::fast_rng(),
        )
        .await
        .expect("UDP error");
    unreachable!("embassy-net's sockets do not get closed (but embedded-nal-coap can't know that)");
}
