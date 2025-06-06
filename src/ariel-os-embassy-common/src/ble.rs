//! Common BLE types to be used across different HALs.

/// Configuration for the BLE stack
/// You can customize it using the `ble-config-override` feature.
pub struct Config {
    /// The address of the BLE device.
    pub address: trouble_host::Address,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: trouble_host::Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xfc]),
        }
    }
}
