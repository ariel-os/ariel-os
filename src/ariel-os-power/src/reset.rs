#[cfg(context = "nrf")]
use portable_atomic::{AtomicU8, Ordering};

#[cfg(context = "nrf")]
static RESET_REASON: AtomicU8 = AtomicU8::new(ResetReason::PowerOnReset as u8);

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
    /// The reset has been triggered by an external-interrupt wake-up event.
    ExternalInterrupt,
    /// The reset has been triggered by a real-time clock (RTC) wake-up event.
    Rtc,
    /// The reset has been triggered by entering an RF field (e.g., an NFC/RFID field) able to
    /// power the microcontroller.
    Field,
    /// The reset has been triggered by hardware following a power failure, e.g., a brownout.
    PowerFailure,
    /// The reset has been triggered by the watchdog.
    WatchdogReset,
    /// The reset has been triggered by a source for which there is no other suitable variant, but
    /// the microcontroller does allow to distinguish it from a power-on reset.
    Other,
}

#[cfg(context = "nrf")]
impl ResetReason {
    #[must_use]
    fn as_u8(self) -> u8 {
        match self {
            Self::PowerOnReset => 0,
            Self::ResetPin => 1,
            Self::SoftwareReset => 2,
            Self::ExternalInterrupt => 3,
            Self::Rtc => 4,
            Self::Field => 5,
            Self::PowerFailure => 6,
            Self::WatchdogReset => 7,
            Self::Other => 8,
        }
    }

    fn try_from_u8(int: u8) -> Result<Self, ()> {
        match int {
            0 => Ok(Self::PowerOnReset),
            1 => Ok(Self::ResetPin),
            2 => Ok(Self::SoftwareReset),
            3 => Ok(Self::ExternalInterrupt),
            4 => Ok(Self::Rtc),
            5 => Ok(Self::Field),
            6 => Ok(Self::PowerFailure),
            7 => Ok(Self::WatchdogReset),
            8 => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

pub(crate) fn save_reset_reason() {
    cfg_if::cfg_if! {
        if #[cfg(context = "nrf")] {
            // NOTE: this avoids forgetting to update this when adding support for other families.
            #[cfg(not(any(
                context = "nrf51",
                context = "nrf52",
                context = "nrf53",
                context = "nrf91",
            )))]
            compile_error!("unsupported nRF MCU");

            let resetreas;

            cfg_if::cfg_if! {
                if #[cfg(context = "nrf53")] {
                    resetreas = embassy_nrf::pac::RESET.resetreas().read();
                } else {
                    resetreas = embassy_nrf::pac::POWER.resetreas().read();
                }
            }

            let mut reset_reason = ResetReason::default();

            #[cfg(not(any(context = "nrf51", context = "nrf91")))]
            if resetreas.nfc() {
                reset_reason = ResetReason::Field;
            }

            #[cfg(not(context = "nrf53"))]
            if resetreas.dog() {
                reset_reason = ResetReason::WatchdogReset;
            }

            // TODO: it is unclear whether each watchdog timer should be attributed to one of the
            // cores specifically.
            #[cfg(context = "nrf53")]
            if resetreas.dog0() || resetreas.dog1() {
                reset_reason = ResetReason::WatchdogReset;
            }

            #[cfg(not(context = "nrf91"))]
            if resetreas.lpcomp() {
                reset_reason = ResetReason::Other;
            }

            if resetreas.resetpin() {
                reset_reason = ResetReason::ResetPin;
            } else if resetreas.sreq() {
                reset_reason = ResetReason::SoftwareReset;
            } else if resetreas.off() {
                reset_reason = ResetReason::StandbyWakeup;
            } else if resetreas.lockup() | resetreas.dif() {
                reset_reason = ResetReason::Other;
            };

            RESET_REASON.store(reset_reason.as_u8(), Ordering::Release);

            cfg_if::cfg_if! {
                if #[cfg(context = "nrf53")] {
                    let clear_value = embassy_nrf::pac::reset::regs::Resetreas(u32::MAX);
                    embassy_nrf::pac::RESET.resetreas().write_value(clear_value);
                } else {
                    let clear_value = embassy_nrf::pac::power::regs::Resetreas(u32::MAX);
                    embassy_nrf::pac::POWER.resetreas().write_value(clear_value);
                }
            }

        }
    }
}

/// Returns the reason why the microcontroller has reset.
#[cfg(context = "nrf")]
#[must_use]
pub fn reset_reason() -> ResetReason {
    cfg_if::cfg_if! {
        if #[cfg(any(context = "nrf"))] {
            ResetReason::try_from_u8(RESET_REASON.load(Ordering::Acquire)).unwrap()
        } else {
            compile_error!("obtaining the reseat reason is not yet supported on this MCU family");
        }
    }
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
