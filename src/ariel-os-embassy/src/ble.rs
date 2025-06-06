use ariel_os_embassy_common::ble::Config;

// Must be async and return &trouble_host::Stack<'static, impl Controller>
pub use crate::hal::ble::ble_stack;

#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) fn config() -> Config {
    #[cfg(not(feature = "ble-config-override"))]
    {
        Config::default()
    }
    #[cfg(feature = "ble-config-override")]
    {
        unsafe extern "Rust" {
            fn __ariel_os_ble_config() -> Config;
        }
        unsafe { __ariel_os_ble_config() }
    }
}
