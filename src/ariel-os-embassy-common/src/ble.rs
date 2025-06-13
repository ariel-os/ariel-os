//! Common BLE types to be used across different HALs.

/// Configuration for the BLE stack.
///
/// You can customize it using the `ble-config-override` feature.
pub struct Config {
    /// The address of the BLE device.
    pub address: trouble_host::Address,
    /// The Maximum Transmission Unit for the L2CAP channels
    pub mtu: usize,
}

/// Default address, this one is used in the trouBLE examples, needs to be changed/randomized in production.
impl Default for Config {
    fn default() -> Self {
        Self {
            address: trouble_host::Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]),
            mtu: 27,
        }
    }
}
