//! Provides control over the system BLE (Bluetooth Low Energy) stack.
//!
//! All interactions happen through the [`trouble_host::Stack`] struct that can be obtained using
//! [`ble_stack()`].
//!
//! The address of the device is randomly generated at boot and may be rotated during execution.
//!
//! # Current implementation
//!
//! The address is not currently rotated during execution; however this behavior may not be relied upon.

use embassy_sync::once_lock::OnceLock;
use futures_util::FutureExt as _;
use trouble_host::{
    Address,
    prelude::{AddrKind, BdAddr},
};

use ariel_os_embassy_common::ble::Config;
use ariel_os_log::debug;

// Must be async and return &trouble_host::Stack<'static, impl Controller>
pub use crate::hal::ble::ble_stack;

static CURRENT_ADDRESS: OnceLock<Address> = OnceLock::new();

#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) fn config() -> Config {
    // Scanning apps show that the last byte of the array appears fist.
    let mut raw_address = get_random_addr();

    // Set the two most significant bits to 1 to indicate a static random address https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/low-energy-controller/link-layer-specification.html#UUID-7edea27a-a47f-8436-4bd7-aedc1945c366_figure-idm4497995733171233616486354268
    raw_address[5] |= 0b1100_0000;

    let address = Address {
        addr: BdAddr::new(raw_address),
        kind: AddrKind::RANDOM,
    };

    let _ = CURRENT_ADDRESS.init(address);

    debug!("Setting random address: {:?}", address);

    Config { address }
}

/// Returns the BLE address currently in use.
///
/// Note that the BLE address may be rotated over time.
pub fn current_address() -> impl Future<Output = Address> {
    // Using map() to avoid creating a new state machine.
    CURRENT_ADDRESS.get().map(|addr| *addr)
}

/// Generates a random address.
#[cfg(not(feature = "ble-config-static-address"))]
fn get_random_addr() -> [u8; 6] {
    let mut addr = [0u8; 6];
    rand_core::RngCore::fill_bytes(&mut ariel_os_random::crypto_rng(), &mut addr);
    addr
}

/// Get random address from env.
#[cfg(feature = "ble-config-static-address")]
fn get_random_addr() -> [u8; 6] {
    use ariel_os_utils::eui48_from_env;
    eui48_from_env!(
        "CONFIG_BLE_STATIC_ADDRESS",
        "static address for BLE in format XX:XX:XX:XX:XX:XX",
    )
}
