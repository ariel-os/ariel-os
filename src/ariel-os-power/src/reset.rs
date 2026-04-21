#![allow(unsafe_code)]

#[cfg(any(context = "nrf", context = "stm32"))]
use portable_atomic::{AtomicU8, Ordering};

#[cfg(any(context = "nrf", context = "stm32"))]
static RESET_REASON: AtomicU8 = AtomicU8::new(ResetReason::PowerOnReset as u8);

/// Indicates why the microcontroller has reset.
///
/// # Note
///
/// Not all architectures allow distinguish between all variants, and [`ResetReason::PowerOnReset`]
/// acts as the default variant.
/// When a variant other than [`ResetReason::PowerOnReset`] is returned, it can however be
/// decisively concluded that it does reflect the actual reset reason.
/// The [`ResetReason::Other`] variant is used when the reset reason can be determined not to be a
/// power-on reset but there is no other suitable variant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ResetReason {
    #[default]
    PowerOnReset,
    ResetPin,
    SoftwareReset,
    StandbyWakeup,
    PowerFailure,
    WatchdogReset,
    Other,
}

#[cfg(any(context = "nrf", context = "stm32"))]
impl ResetReason {
    #[must_use]
    fn as_u8(self) -> u8 {
        match self {
            Self::PowerOnReset => 0,
            Self::ResetPin => 1,
            Self::SoftwareReset => 2,
            Self::StandbyWakeup => 3,
            Self::PowerFailure => 4,
            Self::WatchdogReset => 5,
            Self::Other => 6,
        }
    }

    fn try_from_u8(int: u8) -> Result<Self, ()> {
        match int {
            0 => Ok(Self::PowerOnReset),
            1 => Ok(Self::ResetPin),
            2 => Ok(Self::SoftwareReset),
            3 => Ok(Self::StandbyWakeup),
            4 => Ok(Self::PowerFailure),
            5 => Ok(Self::WatchdogReset),
            6 => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

pub(crate) fn save_reset_reason() {
    cfg_if::cfg_if! {
        if #[cfg(context = "nrf")] {
            // TODO: handle nRF5340.

            let mut reset_reason = ResetReason::default();

            let resetreas = embassy_nrf::pac::POWER.resetreas().read();

            #[cfg(not(any(context = "nrf51", context = "nrf91")))]
            if resetreas.nfc() {
                reset_reason = ResetReason::Other;
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
            } else if resetreas.dog() {
                reset_reason = ResetReason::WatchdogReset;
            } else if resetreas.lockup() | resetreas.dif() {
                reset_reason = ResetReason::Other;
            };

            RESET_REASON.store(reset_reason.as_u8(), Ordering::Release);

            let clear_value = embassy_nrf::pac::power::regs::Resetreas(u32::MAX);
            embassy_nrf::pac::POWER.resetreas().write_value(clear_value);
        } else if #[cfg(context = "stm32")] {
            let mut reset_reason = ResetReason::default();

            cfg_if::cfg_if! {
                if #[cfg(context = "stm32wle5jc")] {
                    if embassy_stm32::pac::PWR.extscr().read().c1sbf() {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else if #[cfg(any(
                    context = "stm32c031c6",
                    context = "stm32u073kc",
                    context = "stm32u083mc",
                ))] {
                    if embassy_stm32::pac::PWR.sr1().read().sbf() {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else if #[cfg(any(
                    context = "stm32f042k6",
                    context = "stm32f303cb",
                    context = "stm32f303re",
                ))] {
                    if embassy_stm32::pac::PWR.csr().read().sbf() {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else if #[cfg(any(
                    context = "stm32f401re",
                    context = "stm32f411re",
                ))] {
                    // The datasheet's name *is* CSR.
                    if embassy_stm32::pac::PWR.csr1().read().sbf() {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else if #[cfg(context = "stm32f767zi")] {
                    if embassy_stm32::pac::PWR.csr1().read().sbf() {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else if #[cfg(any(
                    context = "stm32h753zi",
                    context = "stm32h755zi",
                ))] {
                    // Currently we only support CPU1 (i.e., the Cortex-M7).
                    if embassy_stm32::pac::PWR.cpucr().read().sbf() {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else if #[cfg(any(
                    context = "stm32l475vg",
                ))] {
                    // The datasheet's name *is* SBF.
                    if embassy_stm32::pac::PWR.sr1().read().csbf() {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else if #[cfg(any(
                    context = "stm32wb55rg",
                ))] {
                    // Currently we only support CPU1 (i.e., the Cortex-M4).
                    if embassy_stm32::pac::PWR.extscr().read().c1sbf() {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else if #[cfg(any(
                    context = "stm32u585ai",
                ))] {
                    if embassy_stm32::pac::PWR.sr().read().sbf() {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else if #[cfg(any(
                    context = "stm32wba55cg",
                    context = "stm32wba65ri",
                ))] {
                    if embassy_stm32::pac::PWR.sr().read().sbf().to_bits() == 1 {
                        reset_reason = ResetReason::StandbyWakeup;
                    }
                } else {
                    const { panic!("currently unsupported MCU") }
                }
            }

            RESET_REASON.store(reset_reason.as_u8(), Ordering::Release);

            // The bits indicating the reset reason are only cleared by hardware on *power-on*
            // reset. This clears them manually here so they do not "accumulate" over non-power-on
            // resets.
            // NOTE: this is a "clear register" so we can simply use `write()`.
            cfg_if::cfg_if! {
                if #[cfg(context = "stm32wle5jc")] {
                    embassy_stm32::pac::PWR.extscr().write(|w| w.set_c1cssf(true));
                } else if #[cfg(any(
                    context = "stm32c031c6",
                    context = "stm32u073kc",
                    context = "stm32u083mc",
                ))] {
                    embassy_stm32::pac::PWR.scr().write(|w| w.set_csbf(true));
                } else if #[cfg(any(
                    context = "stm32f042k6",
                    context = "stm32f303cb",
                    context = "stm32f303re",
                ))] {
                    embassy_stm32::pac::PWR.cr().write(|w| w.set_csbf(true));
                } else if #[cfg(any(
                    context = "stm32f401re",
                    context = "stm32f411re",
                ))] {
                    // The datasheet's name *is* CR.
                    embassy_stm32::pac::PWR.cr1().write(|w| w.set_csbf(true));
                } else if #[cfg(context = "stm32f767zi")] {
                    embassy_stm32::pac::PWR.cr1().write(|w| w.set_csbf(true));
                } else if #[cfg(any(
                    context = "stm32h753zi",
                    context = "stm32h755zi",
                ))] {
                    // Currently we only support CPU1 (i.e., the Cortex-M7).
                    embassy_stm32::pac::PWR.cpucr().write(|w| w.set_cssf(true));
                } else if #[cfg(any(
                    context = "stm32l475vg",
                ))] {
                    // The datasheet's name *is* CSBF.
                    embassy_stm32::pac::PWR.scr().write(|w| w.set_sbf(true));
                } else if #[cfg(any(
                    context = "stm32wb55rg",
                ))] {
                    // Currently we only support CPU1 (i.e., the Cortex-M4).
                    embassy_stm32::pac::PWR.extscr().write(|w| w.set_c1cssf(true));
                } else if #[cfg(any(
                    context = "stm32u585ai",
                    context = "stm32wba55cg",
                    context = "stm32wba65ri",
                ))] {
                    embassy_stm32::pac::PWR.sr().write(|w| w.set_cssf(true));
                } else {
                    const { panic!("currently unsupported MCU") }
                }
            }
        }
    }
}

/// Returns the reason why the microcontroller has reset.
#[must_use]
pub fn reset_reason() -> ResetReason {
    cfg_if::cfg_if! {
        if #[cfg(context = "esp")] {
            // TODO: use esp_hal::rtc_cntl::reset_reason()
            // TODO: use esp_hal::rtc_cntl::wakeup_reason()
        } else if #[cfg(any(context = "nrf", context = "stm32"))] {
            ResetReason::try_from_u8(RESET_REASON.load(Ordering::Acquire)).unwrap()
        } else if #[cfg(context = "rp")] {
            // NOTE: these MCUs do not need the reset reason to be manually cleared.

            // TODO: do RP235x.

            let chip_reset = embassy_rp::pac::VREG_AND_CHIP_RESET.chip_reset().read();

            if chip_reset.had_run() {
                // The reset pin is called RUN on RP MCUs.
                return ResetReason::ResetPin;
            }

            if chip_reset.had_psm_restart() {
                return ResetReason::Other;
            }

            let watchdog_reset_reason = critical_section::with(|_| {
                // SAFETY: peripheral usage is entirely wrapped in a critical section and only
                // reads registers.
                let p = unsafe { embassy_rp::peripherals::WATCHDOG::steal() };
                let watchdog = embassy_rp::watchdog::Watchdog::new(p);

                watchdog.reset_reason()
            });

            if watchdog_reset_reason.is_some() {
                return ResetReason::WatchdogReset;
            }

            ResetReason::default()
        } else {
            todo!();
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
