//! Provides facilities related to the MCU reset.

use portable_atomic::{AtomicU8, Ordering};

static RESET_REASON: AtomicU8 = AtomicU8::new(ResetReason::PowerOnReset as u8);

/// Indicates why the microcontroller has reset.
///
/// # Note
///
/// Not all microcontrollers allow distinguishing between all variants, and
/// [`ResetReason::PowerOnReset`] acts as the default variant.
/// When a variant other than [`ResetReason::PowerOnReset`] is returned, it does however reflect
/// the actual reset reason.
// NOTE: Marking this as `non_exhaustive` allows to make introducing *new* variants not a breaking
// change, especially on unaffected MCUs. However, returning the newly introduced variant
// instead of an already existing one is still likely to be one.
// NOTE: `cfg` predicates are written differently than in the implementation for documentation clarity.
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
    /// The reset has been triggered by software, e.g., with `reboot()`.
    /// Using `reboot()` however does not guarantee that this variant will be returned, as this
    /// depends on the microcontroller's ability to distinguish this.
    SoftwareReset,
    /// The reset has been triggered by an external-interrupt wake-up event.
    ExternalInterrupt,
    /// The reset has been triggered by a real-time clock (RTC) wake-up event.
    Rtc,
    /// The reset has been triggered by a wake-up event as the USB power became available.
    #[cfg(any(all(context = "nrf52", not(context = "nrf52832")), context = "nrf53"))]
    Usb,
    /// The reset has been triggered by a wake-up event from an analog comparator.
    #[cfg(any(context = "nrf51", context = "nrf52", context = "nrf53"))]
    Comparator,
    /// The reset has been triggered by entering an RF field (e.g., an NFC/RFID field) able to
    /// power the microcontroller.
    #[cfg(any(context = "nrf52", context = "nrf53"))]
    Field,
    /// The reset has been triggered due to a processor lockup.
    CpuLockup,
    /// The reset has been triggered by the watchdog.
    WatchdogReset,
    /// The reset has been triggered by a wake-up event from the debug interface.
    DebugInterface,
}

impl ResetReason {
    #[must_use]
    fn as_u8(self) -> u8 {
        match self {
            Self::PowerOnReset => 0,
            Self::ResetPin => 1,
            Self::SoftwareReset => 2,
            Self::ExternalInterrupt => 3,
            Self::Rtc => 4,
            #[cfg(not(any(context = "nrf51", context = "nrf52832", context = "nrf91")))]
            Self::Usb => 5,
            #[cfg(not(context = "nrf91"))]
            Self::Comparator => 6,
            #[cfg(not(any(context = "nrf51", context = "nrf91")))]
            Self::Field => 7,
            Self::WatchdogReset => 8,
            Self::CpuLockup => 9,
            Self::DebugInterface => 10,
        }
    }

    fn try_from_u8(int: u8) -> Result<Self, ()> {
        match int {
            0 => Ok(Self::PowerOnReset),
            1 => Ok(Self::ResetPin),
            2 => Ok(Self::SoftwareReset),
            3 => Ok(Self::ExternalInterrupt),
            4 => Ok(Self::Rtc),
            #[cfg(not(any(context = "nrf51", context = "nrf52832", context = "nrf91")))]
            5 => Ok(Self::Usb),
            #[cfg(not(context = "nrf91"))]
            6 => Ok(Self::Comparator),
            #[cfg(not(any(context = "nrf51", context = "nrf91")))]
            7 => Ok(Self::Field),
            8 => Ok(Self::WatchdogReset),
            9 => Ok(Self::CpuLockup),
            10 => Ok(Self::DebugInterface),
            _ => Err(()),
        }
    }
}

/// Saves the reset reason.
///
/// *Important*: this needs to be called as early as possible in the boot sequence.
/// In particular, on microcontrollers whose reset reason needs to be cleared manually on each
/// reset, this needs to be called before anything else has the change to clear it.
/// This function may clear these bits.
pub(crate) fn save_reason() {
    // NOTE: this avoids forgetting to update this when adding support for other families.
    #[cfg(not(any(
        context = "nrf51",
        context = "nrf52",
        context = "nrf53",
        context = "nrf91",
        not(context = "ariel-os"),
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

    // NOTE: `cfg` predicates is written differently than on `ResetReason` to avoid forgetting to
    // update the list of reasons when adding support for new families.

    #[cfg(not(any(context = "nrf51", context = "nrf91")))]
    if resetreas.nfc() {
        reset_reason = ResetReason::Field;
    }

    #[cfg(not(any(context = "nrf51", context = "nrf52832", context = "nrf91")))]
    if resetreas.vbus() {
        reset_reason = ResetReason::Usb;
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
        reset_reason = ResetReason::Comparator;
    }

    if resetreas.resetpin() {
        reset_reason = ResetReason::ResetPin;
    } else if resetreas.sreq() {
        reset_reason = ResetReason::SoftwareReset;
    } else if resetreas.off() {
        reset_reason = ResetReason::ExternalInterrupt;
    } else if resetreas.lockup() {
        reset_reason = ResetReason::CpuLockup;
    } else if resetreas.dif() {
        reset_reason = ResetReason::DebugInterface;
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

#[doc(hidden)]
pub fn reason() -> ResetReason {
    ResetReason::try_from_u8(RESET_REASON.load(Ordering::Acquire)).unwrap()
}
