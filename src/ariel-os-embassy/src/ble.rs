use ariel_os_embassy_common::ble::Config;

// Must be async and return &trouble_host::Stack<'static, impl Controller>
pub use crate::hal::ble::ble_stack;

#[cfg(all(feature = "random", feature = "storage"))]
const BLE_ADDR_STORAGE_KEY: &str = "ble-addr";

#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) async fn config() -> Config {
    #[cfg(feature = "ble-config-public-mac")]
    let address = None;
    #[cfg(not(feature = "ble-config-public-mac"))]
    let address = Some(get_ble_mac_address());

    let mut fallback_address = get_fallback_mac_address().await;

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

/// Checks if we should use a static random address, for debugging/testing only
#[allow(clippy::unnecessary_wraps, reason = "Returns none ")]
fn get_ble_mac_address() -> [u8; 6] {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ble-config-static-mac")] {
            use ariel_os_utils::mac_addr_from_env_or;
            let mac: [u8;6] = mac_addr_from_env_or!(
                "CONFIG_BLE_STATIC_MAC",
                "02:00:00:00:00:01",
                "Mac address for BLE stack in format XX:XX:XX:XX:XX:XX",
            );
            mac
        } else {
            // Random address generated at boot
            get_random_addr()
        }
    }
}

/// Generate a random local address
fn get_random_addr() -> [u8; 6] {
    let mut addr = [0u8; 6];
    rand_core::RngCore::fill_bytes(&mut ariel_os_random::fast_rng(), &mut addr);

    // Set locally administered and unicast bits
    addr[0] = (addr[0] & 0b1111_0000) | 0b0000_0010;
    addr
}

/// Generate a random static address for the devices that don't have a public address
///
/// # Panics
/// - when storage is not initialized
#[cfg_attr(
    not(all(feature = "random", feature = "storage")),
    expect(clippy::unused_async)
)]
async fn get_fallback_mac_address() -> [u8; 6] {
    cfg_if::cfg_if! {
        if #[cfg(capability = "hw/device-identity")] {
            ariel_os_identity::interface_eui48(2)
                .map(|eui48| eui48.0)
                .unwrap_or([0x02, 0x00, 0x00, 0x00, 0x00, 0x01])
        } else if #[cfg(all(feature = "random", feature = "storage"))] {

            // check if we already generated an address
            let stored_addr: Option<[u8;6]> = ariel_os_storage::get(BLE_ADDR_STORAGE_KEY).await.unwrap();
            if let Some(addr) = stored_addr {
                return addr
            }

            let addr = get_random_addr();

            // store in storage
            ariel_os_storage::insert(BLE_ADDR_STORAGE_KEY, addr).await.unwrap();

            addr
        } else {
            // Fallback to a static, locally administered address
            [0x02, 0x00, 0x00, 0x00, 0x00, 0x01]
        }
    }
}
