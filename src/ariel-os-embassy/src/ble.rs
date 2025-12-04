use trouble_host::{
    Address,
    prelude::{AddrKind, BdAddr},
};

use ariel_os_debug::log::debug;
use ariel_os_embassy_common::ble::Config;

// Must be async and return &trouble_host::Stack<'static, impl Controller>
pub use crate::hal::ble::ble_stack;

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

    debug!("Setting random address: {:?}", address);

    Config { address }
}

/// Generates a random address.
fn get_random_addr() -> [u8; 6] {
    let mut addr = [0u8; 6];
    rand_core::RngCore::fill_bytes(&mut ariel_os_random::crypto_rng(), &mut addr);
    addr
}
