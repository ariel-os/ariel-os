#[cfg(feature = "ethernet-stm32")]
pub(crate) use crate::hal::ethernet::NetworkDevice;

#[cfg(feature = "ethernet-wiznet")]
pub(crate) mod wiznet;
#[cfg(feature = "ethernet-wiznet")]
pub(crate) use wiznet::NetworkDevice;

/// Returns a stable MAC address based on the device identity.
///
/// # Panics
///
/// Panics in case of transient issue when obtaining the device identity.
/// This function only used when enforcing that an implementation of `ariel-os-identity` exists, so
/// this will not unconditionally panic.
// NOTE: Keep in sync with the implementation for the STM32 Ethernet MAC.
// NOTE: This cannot be moved to `ariel-os-embassy-common` because a concrete implementation of
// `DeviceId` is needed.
#[allow(dead_code, reason = "conditional compilation")]
fn get_mac_address() -> [u8; 6] {
    use ariel_os_embassy_common::{
        ethernet::MAC_ADDRESS_DEVICE_ID_IF_INDEX, identity::DeviceId as _,
    };

    let device_id =
        crate::hal::identity::DeviceId::get().expect("the device identity can be obtained");
    device_id.interface_eui48(MAC_ADDRESS_DEVICE_ID_IF_INDEX).0
}
