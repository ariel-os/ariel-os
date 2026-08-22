#[cfg(feature = "ethernet-stm32")]
pub(crate) use crate::hal::ethernet::NetworkDevice;

#[cfg(feature = "ethernet-wiznet")]
pub(crate) mod wiznet;
#[cfg(feature = "ethernet-wiznet")]
pub(crate) use wiznet::NetworkDevice;
