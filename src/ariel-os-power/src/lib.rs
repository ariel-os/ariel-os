//! Provides power management functionality.

#![deny(missing_docs)]
#![cfg_attr(not(context = "native"), no_std)]

/// Reboots the MCU.
///
/// This function initiates a software reset of the microcontroller and never returns.
pub fn reboot() -> ! {
    cfg_if::cfg_if! {
        if #[cfg(context = "cortex-m")] {
            cortex_m::peripheral::SCB::sys_reset()
        } else if #[cfg(context = "esp")] {
            esp_hal::system::software_reset()
        } else if #[cfg(context = "native")] {
            std::process::exit(0)
        } else if #[cfg(context = "ariel-os")] {
            compile_error!("reboot is not yet implemented for this platform")
        } else {
            #[expect(clippy::empty_loop, reason = "for platform-independent tooling only")]
            loop {}
        }
    }
}
