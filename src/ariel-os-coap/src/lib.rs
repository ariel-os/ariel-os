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
#[cfg(feature = "coap-transport-udp")]
mod udp_nal;

#[cfg(any(
    feature = "coap-server-config-storage",
    feature = "coap-server-config-runtime-identity"
))]
mod stored;

#[cfg(feature = "coap-transport-udp")]
mod transport_udp;

use ariel_os_embassy::cell::SameExecutorCell;
#[cfg(feature = "coap-server")]
use coap_handler_implementations::ReportingHandlerBuilder as _;
use embassy_sync::watch::Watch;

const CONCURRENT_REQUESTS: usize = 3;

static CLIENT_READY: Watch<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    SameExecutorCell<&'static embedded_nal_coap::CoAPRuntimeClient<'static, CONCURRENT_REQUESTS>>,
    1,
> = Watch::new();

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
/// It sets up the security configuration, and ultimately runs the CoAP transport (currently only
/// CoAP-over-UDP) forever.
///
/// # Panics
///
/// This can only be run once, as it sets up a system wide CoAP handler.
async fn coap_run_impl(handler: impl coap_handler::Handler + coap_handler::Reporting) -> ! {
    cfg_select! {
        any(feature = "coap-server-config-storage", feature = "coap-server-config-runtime-identity") => {
            let security_config = stored::server_security_config().await;
        }
        feature = "coap-server-config-demokeys" => {
            let security_config = demo_setup::build_demo_ssc();
        }
        feature = "coap-server-config-unprotected" => {
            // Not setting the config to `coapcore::seccfg::AllowAll` because it won't be taken up
            // by a security context anyway (handler is not wrapped in an OscoreEdhocHandler).
        }
        _ => {
            // Not setting the config to `coapcore::seccfg::DenyAll` because it won't be taken up
            // by a security context anyway -- and as there is no sever configured, there is no
            // resource to which the policy would be applied.

            #[cfg(all(feature = "coap-server", not(feature = "doc")))]
            compile_error!("No CoAP server configuration chosen out of the coap-server-config-* features.");
        }
    }

    // FIXME: Should we allow users to override that? After all, this is just convenience and may
    // be limiting in special applications.
    #[cfg(feature = "coap-server")]
    let handler = handler.with_wkc();
    #[cfg(any(
        feature = "coap-server-config-storage",
        feature = "coap-server-config-runtime-identity",
        feature = "coap-server-config-demokeys"
    ))]
    let handler = coapcore::OscoreEdhocHandler::new(
        handler,
        security_config,
        || lakers_crypto_rustcrypto::Crypto::new(ariel_os_random::crypto_rng()),
        ariel_os_random::crypto_rng(),
        coapcore::time::TimeUnknown,
    );

    cfg_select! {
        feature = "coap-transport-udp" => {
            transport_udp::coap_run_udp(handler).await
        }
        feature = "doc" => {
            loop {}
        }
        _ => {
            compile_error!("No CoAP transport configuration was chosen out of the coap-transport-* features.")
        }
    }
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
