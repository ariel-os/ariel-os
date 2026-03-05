/// Indicates why the microcontroller has reset.
///
/// # Note
///
/// Not all microcontrollers allow distinguishing between all variants, and
/// [`ResetReason::PowerOnReset`] acts as the default variant.
/// When a variant other than [`ResetReason::PowerOnReset`] is returned, it does however reflect
/// the actual reset reason.
/// The [`ResetReason::Other`] variant is used when the reset reason can be determined not to be a
/// power-on reset but there is no other suitable variant.
// NOTE: Marking this as `non_exhaustive` allows to make introducing *new* variants not a breaking
// change, especially on unaffected MCU families. However, returning the newly introduced variant
// instead of an already existing one is still likely to be one.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ResetReason {
    /// The reset has been triggered by a power cycle.
    /// This variant also acts as a default when the reset reason cannot be determined or
    /// distinguished from a power-on reset.
    /// In particular, many microcontrollers do not allow distinguishing brownout resets from
    /// power-on resets.
    #[default]
    PowerOnReset,
    /// The reset has been triggered through the dedicated reset pin.
    ResetPin,
    /// The reset has been triggered by software, e.g., with [`reboot()`].
    /// Using [`reboot()`] however does not guarantee that this variant will be returned, as this
    /// depends on the microcontroller's ability to distinguish this.
    SoftwareReset,
    /// The reset has been triggered through a wake-up event which woke up the microcontroller from a
    /// low-power standby state.
    StandbyWakeup,
    /// The reset has been triggered by hardware following a power failure, e.g., a brownout.
    PowerFailure,
    /// The reset has been triggered by the watchdog.
    WatchdogReset,
    /// The reset has been triggered by a source for which there is no other suitable variant, but
    /// the microcontroller does allow to distinguish it from a power-on reset.
    Other,
}

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
