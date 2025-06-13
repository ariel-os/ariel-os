//! Common BLE types to be used across different HALs.

use static_cell::StaticCell;
use trouble_host::HostResources;

// Safe value of 27, compatible with all versions.
pub const L2CAP_MTU: usize = 27;

// Safe defaults used in trouble_host examples
pub const MAX_CONNS: usize = 1;
pub const MAX_CHANNELS: usize = 1;

pub type BleHostResource = HostResources<MAX_CONNS, MAX_CHANNELS, L2CAP_MTU>;

static HOST_RESOURCES: StaticCell<BleHostResource> = StaticCell::new();

/// Initialize the BLE host resources.
pub fn get_ble_host_ressources() -> &'static mut BleHostResource {
    HOST_RESOURCES.init(HostResources::new())
}

/// Configuration for the BLE stack.
///
/// You can customize it using the `ble-config-override` feature.
pub struct Config {
    /// The address of the BLE device.
    pub address: trouble_host::Address,
}

/// Default address, this one is used in the trouBLE examples, needs to be changed/randomized in production.
impl Default for Config {
    fn default() -> Self {
        Self {
            address: trouble_host::Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]),
        }
    }
}
