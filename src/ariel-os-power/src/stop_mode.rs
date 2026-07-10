//! Support for Ariel OS low-power stop mode, see [`enter()`].

use ariel_os_hal::gpio::Pull;

pub use ariel_os_embassy_common::power::GpioWakeupTriggerEvent;

/// Defines a GPIO event to trigger a wake-up from [stop mode](enter).
pub struct GpioWakeupTrigger<
    'a,
    T: ariel_os_hal::hal::IntoPeripheral<'a, P>,
    P: ariel_os_hal::hal::power::StopWakeupPin,
> {
    /// GPIO pin on which to expect the event.
    /// On certain MCUs, it is possible that not all GPIOs be usable as a wake-up trigger, even
    /// though this is not typically the case for waking up from stop mode.
    pub gpio: T,
    /// Pull setting to use for the GPIO.
    pub pull: ariel_os_hal::gpio::Pull,
    /// GPIO event upon which to trigger a wake-up.
    pub event: GpioWakeupTriggerEvent,
    _phantom: core::marker::PhantomData<&'a P>,
}

impl<'a, T: ariel_os_hal::hal::IntoPeripheral<'a, P>, P: ariel_os_hal::hal::power::StopWakeupPin>
    GpioWakeupTrigger<'a, T, P>
{
    /// Creates a  to define on which event to wake up from
    /// [stop mode](enter).
    #[must_use]
    pub fn new(gpio: T, pull: Pull, event: GpioWakeupTriggerEvent) -> Self {
        Self {
            gpio,
            pull,
            event,
            _phantom: core::marker::PhantomData,
        }
    }
}

/// Interrupts and events allowed to trigger a wake-up from stop mode.
///
/// If no triggers are set, the application's execution will not resume after calling
/// [`enter()`].
#[non_exhaustive]
pub struct WakeupTriggers<
    'a,
    T: ariel_os_hal::hal::IntoPeripheral<'a, P>,
    P: ariel_os_hal::hal::power::StopWakeupPin,
> {
    /// External interrupts that may trigger a wake-up.
    pub gpio: Option<GpioWakeupTrigger<'a, T, P>>,
    // TODO: an extra field should later be added to allow waking up from an RTC event.
    pub(crate) _phantom: core::marker::PhantomData<&'a P>,
}

impl<'a, T: ariel_os_hal::hal::IntoPeripheral<'a, P>, P: ariel_os_hal::hal::power::StopWakeupPin>
    Default for WakeupTriggers<'a, T, P>
{
    fn default() -> Self {
        Self {
            gpio: None,
            _phantom: core::marker::PhantomData,
        }
    }
}

/// Enters stop mode.
///
/// In this mode, almost every clock of the microcontroller is off, but the RAM contents are
/// retained.
// TODO: enable this doc comment fragment when landing `standby_mode::enter()`.
// Unlike [`standby_mode::enter()`], waking up does not involve rebooting, and execution resumes
// normally after calling this function.
/// In addition, the state of GPIOs, including their pull setting, is maintained.
///
/// The entry into the low-power mode may be delayed by a few cycles, in particular because of
/// outstanding memory writes.
///
/// # Important note
///
/// This is currently implemented on a best-effort basis.
/// Some microcontrollers may not support these low-power settings, they may not be implemented
/// yet, or they may be lacking testing.
/// Do measure the power consumption of your hardware when relevant for your application.
///
/// # Wake-up conditions
///
/// Depending on the microcontroller, waking up from this mode usually requires an RTC interrupt or
/// an external interrupt (sometimes on a limited set of pins).
pub fn enter<
    'a,
    T: ariel_os_hal::hal::IntoPeripheral<'a, P>,
    P: ariel_os_hal::hal::power::StopWakeupPin,
>(
    wakeup: WakeupTriggers<'a, T, P>,
) {
    match wakeup {
        WakeupTriggers {
            gpio: Some(gpio), ..
        } => {
            ariel_os_hal::hal::power::enter_stop_mode(Some((gpio.gpio, gpio.pull, gpio.event)));
        }
        WakeupTriggers { gpio: None, .. } => {
            ariel_os_hal::hal::power::enter_stop_mode::<T, _>(None);
        }
    }
}
