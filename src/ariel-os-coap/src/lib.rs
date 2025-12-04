//! A CoAP stack preconfigured for Ariel OS.
//!
//! This crate mainly provides easy-to-use wrappers around the [`coapcore`] crate, with presets
//! tailored towards Ariel OS: It utilizes [`embassy_net`] to open a network accessible CoAP socket
//! and selects [`embedded_nal_coap`] for CoAP over UDP, it selects [`ariel_os_random`] as a source
//! of randomness, and [`lakers_crypto_rustcrypto`] for the cryptographic algorithm
//! implementations.
#![no_std]
#![deny(missing_docs)]

// Moving work from https://github.com/embassy-rs/embassy/pull/2519 in here for the time being
mod udp_nal;

#[cfg(feature = "coap-server-config-storage")]
mod stored;

use core::net::{Ipv6Addr, SocketAddr};

use ariel_os_debug::log::info;
use ariel_os_embassy::cell::SameExecutorCell;
use coap_handler_implementations::ReportingHandlerBuilder as _;
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_sync::watch::Watch;
use static_cell::StaticCell;

const CONCURRENT_REQUESTS: usize = 3;

static CLIENT_READY: Watch<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    SameExecutorCell<&'static embedded_nal_coap::CoAPRuntimeClient<'static, CONCURRENT_REQUESTS>>,
    1,
> = Watch::new();

#[cfg(feature = "coap-server-config-demokeys")]
mod demo_setup {
    use cbor_macro::cbor;
    use hexlit::hex;

    /// Credential presented by any demo device.
    const DEVICE_CREDENTIAL: &[u8] = &hex!(
        "A2026008A101A5010202410A2001215820BBC34960526EA4D32E940CAD2A234148DDC21791A12AFBCBAC93622046DD44F02258204519E257236B2A0CE2023F0931F1F386CA7AFDA64FCDE0108C224C51EABF6072"
    );
    /// Private key for `DEVICE_CREDENTIAL`.
    const DEVICE_KEY: [u8; 32] =
        hex!("72cc4761dbd4c78f758931aa589d348d1ef874a7e303ede2f140dcf3e6aa4aac");

    /// Scope usable by any client inside any demo device.
    const UNAUTHENTICATED_SCOPE: cboritem::CborItem<'_> = cbor!([
            ["/.well-known/core", 1],
            ["/poem", 1],
            ["/hello", 1],
            / any operation /
            ["/led", 63]
    ]);

    /// Scope usable by the the administrator of the demo device.
    const ADMIN_SCOPE: cboritem::CborItem<'_> = cbor!([
            ["/stdout", 17 / GET and FETCH /],
            ["/.well-known/core", 1],
            ["/poem", 1]
    ]);
    /// Credential by which the administrator of any demo device is recognized.
    ///
    /// The corresponding private key is shipped in `tests/coap/client.cosekey`.
    const ADMIN_CREDENTIAL: &[u8] = &hex!(
        "A2027734322D35302D33312D46462D45462D33372D33322D333908A101A5010202412B2001215820AC75E9ECE3E50BFC8ED60399889522405C47BF16DF96660A41298CB4307F7EB62258206E5DE611388A4B8A8211334AC7D37ECB52A387D257E6DB3C2A93DF21FF3AFFC8"
    );

    /// Assembles this module's components into a server security configuration.
    pub(super) fn build_demo_ssc() -> coapcore::seccfg::ConfigBuilder {
        let own_key = DEVICE_KEY;
        let own_credential = lakers::Credential::parse_ccs(DEVICE_CREDENTIAL)
            .expect("Credential should be processable");

        let unauthenticated_scope = coapcore::scope::AifValue::parse(&UNAUTHENTICATED_SCOPE)
            .expect("hard-coded scope fits this type")
            .into();
        let admin_key = lakers::Credential::parse_ccs(ADMIN_CREDENTIAL)
            .expect("hard-coded credential fits this type");
        let admin_scope = coapcore::scope::AifValue::parse(&ADMIN_SCOPE)
            .expect("hard-coded scope fits this type")
            .into();

        coapcore::seccfg::ConfigBuilder::new()
            .allow_unauthenticated(unauthenticated_scope)
            .with_own_edhoc_credential(own_credential, own_key)
            .with_known_edhoc_credential(admin_key, admin_scope)
    }
}

/// Runs a CoAP server with the given handler on the system's CoAP transports.
///
/// # Note
///
/// The application needs to run this in a task; otherwise, other components (e.g., system
/// components that also run on the CoAP server, or the CoAP client that depends on the server
/// loop to run) get stalled.
///
/// As the CoAP stack gets ready (which may take some time if the network is not ready yet), it also
/// unblocks [`coap_client()`].
///
/// # Panics
///
/// This can only be run once, as it sets up a system wide CoAP handler.
#[cfg(feature = "coap-server")]
pub async fn coap_run(handler: impl coap_handler::Handler + coap_handler::Reporting) -> ! {
    coap_run_impl(handler).await
}

/// Workhorse of [`coap_run`], see there for details.
///
/// This is a separate function because if that function is not exposed publicly (i.e. when the
/// laze feature `coap-server` is not active), it is called automatically in a separate task.
///
/// # Panics
///
/// This can only be run once, as it sets up a system wide CoAP handler.
async fn coap_run_impl(handler: impl coap_handler::Handler + coap_handler::Reporting) -> ! {
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

    cfg_if::cfg_if! {
        if #[cfg(feature = "coap-server-config-storage")] {
            let security_config = stored::server_security_config().await;
        } else if #[cfg(feature = "coap-server-config-demokeys")] {
            let security_config = demo_setup::build_demo_ssc();
        } else if #[cfg(feature = "coap-server-config-unprotected")] {
            let security_config = coapcore::seccfg::AllowAll;
        } else {
            // We could pick another policy too to get 4.04 errors, but "there may be something but
            // I won't tell you" is just as good an answer, and may prune some more branches even.
            let security_config = coapcore::seccfg::DenyAll;

            #[cfg(all(feature = "coap-server", not(feature = "doc")))]
            compile_error!("No CoAP server configuration chosen out of the coap-server-config-* features.");
        }
    }

    // FIXME: Should we allow users to override that? After all, this is just convenience and may
    // be limiting in special applications.
    let handler = handler.with_wkc();
    let mut handler = coapcore::OscoreEdhocHandler::new(
        handler,
        security_config,
        || lakers_crypto_rustcrypto::Crypto::new(ariel_os_random::crypto_rng()),
        ariel_os_random::crypto_rng(),
        coapcore::time::TimeUnknown,
    );

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

/// Returns a CoAP client requester.
///
/// This asynchronously blocks until [`coap_run()`] has been called (which happens at startup
/// when the corresponding feature `coap-server` is not active), and the CoAP stack is operational.
///
/// # Panics
///
/// This is currently only available from the thread that hosts the network stack, and panics
/// otherwise. This restriction will be lifted in the future (by generalization in
/// [`embedded_nal_coap`] to allow different mutexes).
pub async fn coap_client()
-> &'static embedded_nal_coap::CoAPRuntimeClient<'static, CONCURRENT_REQUESTS> {
    let mut receiver = CLIENT_READY
        .receiver()
        .expect("Too many CoAP clients are waiting for the network to come up.");
    receiver
        .get()
        .await
        .get_async()
        .await // Not an actual await, just a convenient way to see which executor is running
        .expect("CoAP client can currently only be used from the thread the network is bound to")
}

/// Auto-started CoAP server that serves two purposes:
///
/// * It provides the backend for the CoAP client operation (which leaves message sending to that
///   task).
/// * It runs any CoAP server components provided by the OS (none yet).
#[cfg(not(feature = "coap-server"))]
#[ariel_os_macros::task(autostart)]
async fn coap_run() {
    use coap_handler_implementations::new_dispatcher;

    // FIXME: Provide an "all system components" constructor in this crate.
    let handler = new_dispatcher();
    coap_run_impl(handler).await;
}
