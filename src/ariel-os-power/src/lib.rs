//! Provides power management functionality.

#![cfg_attr(not(context = "native"), no_std)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

pub mod reset;

/// Reboots the MCU.
///
/// This function initiates a software reset of the microcontroller and never returns.
pub fn reboot() -> ! {
    cfg_select! {
        context = "cortex-m" => {
            cortex_m::peripheral::SCB::sys_reset()
        }
        context = "esp" => {
            esp_hal::system::software_reset()
        }
        context = "native" => {
            std::process::exit(0)
        }
        context = "ariel-os" => {
            compile_error!("reboot is not yet implemented for this platform")
        }
        _ => {
            #[expect(clippy::empty_loop, reason = "for platform-independent tooling only")]
            loop {}
        }
    }
}
