use ariel_os_utils::str_from_env;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};

// TODO: this should be factored out in ariel-os-embassy again
pub(crate) const WIFI_NETWORK: &str =
    str_from_env!("CONFIG_WIFI_NETWORK", "Wi-Fi SSID (network name)");
pub(crate) const WIFI_PASSWORD: &str = str_from_env!("CONFIG_WIFI_PASSWORD", "Wi-Fi password");

// State of the wifi interface so it can be controlled from another task.
pub(crate) static WIFI_CONTROL_WANTED_STATE: Watch<CriticalSectionRawMutex, State, 1> =
    Watch::new();

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum State {
    Enabled,
    Disabled,
}

/// Interface controller for the CYW43 driver.
#[derive(Debug, Clone, Copy)]
pub struct Cyw43WifiInterfaceController {}
impl Cyw43WifiInterfaceController {
    /// Create a new interface controller.
    pub fn new() -> Self {
        Self {}
    }
}

impl ariel_os_embassy_common::net::InterfaceController for Cyw43WifiInterfaceController {
    fn disable(&self) {
        WIFI_CONTROL_WANTED_STATE.sender().send(State::Disabled);
    }
    fn enable(&self) {
        WIFI_CONTROL_WANTED_STATE.sender().send(State::Enabled);
    }
}
