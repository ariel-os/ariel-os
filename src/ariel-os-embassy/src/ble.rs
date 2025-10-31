use ariel_os_embassy_common::ble::Config;

// Must be async and return &trouble_host::Stack<'static, impl Controller>
pub use crate::hal::ble::ble_stack;

#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) fn config() -> Config {
    let address = get_ble_mac_address();
    let mut fallback_address = get_fallback_mac_address();

    // Scanning apps show that the last byte of the array appears fist, to be more user-friendly we reverse the order
    fallback_address.reverse();
    let address = address.map(|mut addr| {
        addr.reverse();
        trouble_host::Address::random(addr)
    });

    let fallback_address = trouble_host::Address::random(fallback_address);
    Config {
        address,
        fallback_address,
    }
}

/// Checks if we should use a static random address
fn get_ble_mac_address() -> Option<[u8; 6]> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ble-config-static-mac")] {
            use ariel_os_utils::mac_addr_from_env_or;
            let mac: [u8;6] = mac_addr_from_env_or!(
                "CONFIG_BLE_STATIC_MAC",
                "02:00:00:00:00:01",
                "Mac address for BLE stack in format XX:XX:XX:XX:XX:XX",
            );
            Some(mac)
        } else {
            None
        }
    }
}

/// Generate a random static address for the devices that don't have a public address
fn get_fallback_mac_address() -> [u8; 6] {
    cfg_if::cfg_if! {
        if #[cfg(capability = "hw/device-identity")] {
            ariel_os_identity::interface_eui48(2)
                .map(|eui48| eui48.0)
                .unwrap_or([0x02, 0x00, 0x00, 0x00, 0x00, 0x01])
        } else if #[cfg(feature = "random")] {
            let mut addr = [0u8; 6];
            rand_core::RngCore::fill_bytes(&mut ariel_os_random::fast_rng(), &mut addr);

            // Set locally administered and unicast bits
            addr[0] = (addr[0] & 0b1111_0000) | 0b0000_0010;
            addr
        } else {
            // Fallback to a static address
            [0x02, 0x00, 0x00, 0x00, 0x00, 0x01]
        }
    }
}
