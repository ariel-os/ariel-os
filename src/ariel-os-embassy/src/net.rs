//! Provides network access.
//!
//! The network link to use is selected through Cargo features.
//! Additionally, the [`ariel_os::config`](ariel_os_macros::config) attribute macro allows to provide
//! custom network configuration.

#![deny(missing_docs)]
#![allow(unsafe_code)]
#![allow(
    clippy::undocumented_unsafe_blocks,
    reason = "should be addressed eventually"
)]

use embassy_net::{Runner, Stack};
use embassy_sync::once_lock::OnceLock;

use crate::{NetworkDevice, cell::SameExecutorCell};

#[allow(dead_code)]
pub(crate) const ETHERNET_MTU: usize = 1514;

/// A network stack.
///
/// Required to create a UDP or TCP socket.
pub type NetworkStack = Stack<'static>;

pub(crate) static STACK: OnceLock<SameExecutorCell<NetworkStack>> = OnceLock::new();

/// Returns a new [`NetworkStack`].
///
/// Returns [`None`] if networking is not yet initialized.
pub async fn network_stack() -> Option<NetworkStack> {
    STACK.get().await.get_async().await.copied()
}

/// Returns a seed suitable for [`embassy_net::new()`], on a best-effort basis.
///
/// It does not have to be different across reboots, only to be different between devices from the
/// same network.
///
/// # Current implementation
///
/// If support for RNGs is enabled, an RNG is used to obtain a seed.
/// Otherwise, if the device provides a hardware-backed unique ID, it is used for the seed.
/// If none of these is available, a hard-coded, constant seed is returned.
#[allow(dead_code, reason = "conditional compilation")]
#[must_use]
pub(crate) fn unique_seed() -> u64 {
    cfg_if::cfg_if! {
        if #[cfg(feature = "random")] {
            // Even when some using entropy to ensure uniqueness of the seed, the RNG does not need
            // to be cryptographically secure.
            return rand_core::RngCore::next_u64(&mut ariel_os_random::fast_rng());
        } else if #[cfg(capability = "hw/device-identity")] {
            if let Ok(eui48) = ariel_os_identity::interface_eui48(0) {
                // Construct the seed by zero-extending the obtained EUI-48 identifier.
                let mut seed = [0; 8];
                seed[2..].copy_from_slice(&eui48.0);
                // Use a fixed endianness to avoid unfortunate collisions between LE and BE
                // devices; use LE because it is the most common on our supported architectures and
                // avoids the need for reversing instructions on these.
                return u64::from_le_bytes(seed);
            }
        }
    }

    #[allow(unreachable_code, reason = "conditional compilation")]
    1234
}

#[embassy_executor::task]
pub(crate) async fn net_task(mut runner: Runner<'static, NetworkDevice>) -> ! {
    runner.run().await
}

#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) fn config() -> embassy_net::Config {
    cfg_if::cfg_if! {
        if #[cfg(feature = "network-config-override")] {
            unsafe extern "Rust" {
                fn __ariel_os_network_config() -> embassy_net::Config;
            }
            unsafe { __ariel_os_network_config() }
        } else if #[cfg(feature = "dhcpv4")] {
            embassy_net::Config::dhcpv4(embassy_net::DhcpConfig::default())
        } else if #[cfg(not(context = "ariel-os"))] {
            // For platform-independent tooling.
            embassy_net::Config::default()
        }
    }
}

/// Constructor for [`DummyDriver`].
///
/// This is a standalone function instead of an associated method to ease moving [`DummyDriver`]
/// into [`embassy_net`].
#[allow(
    dead_code,
    reason = "constructor is only used in linter / documentation situations"
)]
#[expect(clippy::missing_panics_doc)]
pub(crate) fn new_dummy() -> DummyDriver {
    panic!(
        "DummyDriver must only ever be constructed for documentation and linting, not for running"
    )
}

/// Stand-in for a network driver in documentation and linting.
///
/// It also doubles as the infallible type for its own associated types.
// FIXME: This should be core::convert::Infallible as soon as embassy-net implements the traits on
// that.
pub(crate) struct DummyDriver(core::convert::Infallible);

impl embassy_net::driver::Driver for DummyDriver {
    type RxToken<'a>
        = Self
    where
        Self: 'a;

    type TxToken<'a>
        = Self
    where
        Self: 'a;

    fn receive(
        &mut self,
        _cx: &mut core::task::Context<'_>,
    ) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        match self.0 {}
    }

    fn transmit(&mut self, _cx: &mut core::task::Context<'_>) -> Option<Self::TxToken<'_>> {
        match self.0 {}
    }

    fn link_state(&mut self, _cx: &mut core::task::Context<'_>) -> embassy_net::driver::LinkState {
        match self.0 {}
    }

    fn capabilities(&self) -> embassy_net::driver::Capabilities {
        match self.0 {}
    }

    fn hardware_address(&self) -> embassy_net::driver::HardwareAddress {
        match self.0 {}
    }
}

impl embassy_net::driver::TxToken for DummyDriver {
    fn consume<R, F>(self, _len: usize, _f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        match self.0 {}
    }
}

impl embassy_net::driver::RxToken for DummyDriver {
    fn consume<R, F>(self, _f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        match self.0 {}
    }
}

#[cfg(any(
    feature = "network-config-ipv4-static",
    feature = "network-config-ipv6-static"
))]
// SAFETY: the compiler prevents from defining multiple functions with the same name in the
// same crate; the function signature is checked by the compiler as it is in the same crate as the
// FFI declaration.
#[unsafe(no_mangle)]
fn __ariel_os_network_config() -> embassy_net::Config {
    let mut config = embassy_net::Config::default();

    cfg_if::cfg_if! {
        if #[cfg(feature = "network-config-ipv4-static")] {
            use ariel_os_utils::{ipv4_addr_from_env, u8_from_env};

            let ipaddr = ipv4_addr_from_env!(
                "CONFIG_NET_IPV4_STATIC_ADDRESS",
                "static IPv4 address",
            );

            let gw_addr = ipv4_addr_from_env!(
                "CONFIG_NET_IPV4_STATIC_GATEWAY_ADDRESS",
                "static IPv4 gateway address",
            );

            const PREFIX_LEN: u8 = u8_from_env!(
                "CONFIG_NET_IPV4_STATIC_CIDR_PREFIX_LEN",
                "static IPv4 CIDR prefix length"
            );
            const {
                assert!(
                    PREFIX_LEN <= 32,
                    "`CONFIG_NET_IPV4_STATIC_CIDR_PREFIX_LEN` must be <= 32",
                );
            }

            config.ipv4 = embassy_net::ConfigV4::Static(embassy_net::StaticConfigV4 {
                address: embassy_net::Ipv4Cidr::new(ipaddr, PREFIX_LEN),
                dns_servers: heapless::Vec::new(),
                gateway: Some(gw_addr),
            });
        } else if #[cfg(feature = "dhcpv4")] {
            config.ipv4 = embassy_net::ConfigV4::Dhcp(embassy_net::DhcpConfig::default());
        }
    }

    #[cfg(feature = "network-config-ipv6-static")]
    {
        let ipaddr = ariel_os_utils::ipv6_addr_from_env!(
            "CONFIG_NET_IPV6_STATIC_ADDRESS",
            "static IPv6 address",
        );

        let gw_addr = ariel_os_utils::ipv6_addr_from_env!(
            "CONFIG_NET_IPV6_STATIC_GATEWAY_ADDRESS",
            "static IPv6 gateway address",
        );

        const PREFIX_LEN: u8 = ariel_os_utils::u8_from_env_or!(
            "CONFIG_NET_IPV6_STATIC_CIDR_PREFIX_LEN",
            64,
            "static IPv6 CIDR prefix length"
        );
        const {
            assert!(
                PREFIX_LEN <= 128,
                "`CONFIG_NET_IPV6_STATIC_CIDR_PREFIX_LEN` must be <= 128",
            );
        }

        config.ipv6 = embassy_net::ConfigV6::Static(embassy_net::StaticConfigV6 {
            address: embassy_net::Ipv6Cidr::new(ipaddr, PREFIX_LEN),
            dns_servers: heapless::Vec::new(),
            gateway: Some(gw_addr),
        });
    }

    config
}
