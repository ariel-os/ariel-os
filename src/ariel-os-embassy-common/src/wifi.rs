//! Common Wi-Fi configuration for Ariel OS.
//!
//! This module contains the types and [`WIFI_CONFIG`] constant containing the build-time configuration.

use ariel_os_utils::str_from_env;

/// Current Wi-Fi configuration.
// TODO: Feature-gate this when AP mode is implemented.
pub const WIFI_CONFIG: StationConfig = {
    const WIFI_NETWORK: &str = str_from_env!("CONFIG_WIFI_NETWORK", "Wi-Fi SSID (network name)");
    const WIFI_PASSWORD: &str = str_from_env!("CONFIG_WIFI_PASSWORD", "Wi-Fi password");
    StationConfig {
        ssid: WIFI_NETWORK,
        password: WIFI_PASSWORD,
    }
};

/// Wi-Fi configuration in station mode.
pub struct StationConfig {
    /// Wi-Fi access point SSID.
    pub ssid: &'static str,
    /// Wi-Fi access point password.
    pub password: &'static str,
}
