//! Provides power management functionality.

use ariel_os_embassy_common::power::GpioWakeupTriggerEvent;

pub trait StopWakeupPin {}

/// Interrupts to configure to trigger a wake-up from standby mode.
#[derive(Debug, Default)]
pub struct WakeupInterrupts {
    _private: (),
}

pub fn enter_stop_mode<'a, T: crate::hal::IntoPeripheral<'a, P>, P: StopWakeupPin>(
    gpio_wakeup: Option<(
        T,
        ariel_os_embassy_common::gpio::Pull,
        GpioWakeupTriggerEvent,
    )>,
) {
    unimplemented!();
}

pub fn enter_standby_mode(interrupts: WakeupInterrupts) {
    unimplemented!();
}
