#![allow(unsafe_code)]
#![allow(
    clippy::undocumented_unsafe_blocks,
    reason = "should be addressed eventually"
)]
use ariel_os_embassy_common::ble::Config;
use ariel_os_utils::mac_addr_from_env_or;

// Must be async and return &trouble_host::Stack<'static, impl Controller>
pub use crate::hal::ble::ble_stack;

#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) fn config() -> Config {
    Config {
        address: trouble_host::Address::random(get_ble_mac_address()),
        ..Config::default()
    }
}

fn get_ble_mac_address() -> [u8; 6] {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ble-config-static-mac")] {
           let mac: [u8;6] = mac_addr_from_env_or!(
                "BLE_CONFIG_STATIC_MAC",
                "02:00:00:00:00:01",
                "Mac address for BLE stack in format XX:XX:XX:XX:XX:XX",
            );
            mac
        } else if #[cfg(capability = "hw/device-identity")] {
            ariel_os_identity::interface_eui48(2)
                .map(|eui48| eui48.0)
                .unwrap_or([0x02, 0x00, 0x00, 0x00, 0x00, 0x01])
        } else if #[cfg(feature = "random")] {
            let mut addr = [0u8; 6];
            rand_core::RngCore::fill_bytes(&mut ariel_os_random::fast_rng(), &mut addr);

            // Set locally administered and unicast bits
            addr[0] = (addr[0] & 0b1111_0000) | 0b0000_0010;
        } else {
            // Fallback to a static address
            [0x02, 0x00, 0x00, 0x00, 0x00, 0x01]
        }
    }
}
