//! Provides GPIO access.

pub mod input {
    //! Input-specific types.

    use embassy_stm32::{
        Peri,
        gpio::{Level, Pull},
    };

    #[doc(hidden)]
    pub use embassy_stm32::gpio::{AnyPin, Input, Pin as InputPin};

    #[cfg(feature = "external-interrupts")]
    #[doc(hidden)]
    pub use embassy_stm32::exti::ExtiInput as IntEnabledInput;

    /// Whether inputs support configuring whether a Schmitt trigger is enabled.
    pub const SCHMITT_TRIGGER_CONFIGURABLE: bool = false;

    #[doc(hidden)]
    pub fn new<'a, T: InputPin>(
        pin: Peri<'a, T>,
        pull: ariel_os_embassy_common::gpio::Pull,
        _schmitt_trigger: bool, // Not supported by this hardware
    ) -> Result<Input<'a>, ariel_os_embassy_common::gpio::input::Error> {
        let pull = from_pull(pull);
        Ok(Input::new(pin, pull))
    }

    #[cfg(feature = "external-interrupts")]
    #[doc(hidden)]
    pub fn new_int_enabled<'a, T: InputPin>(
        pin: Peri<'a, T>,
        pull: ariel_os_embassy_common::gpio::Pull,
        _schmitt_trigger: bool, // Not supported by this hardware
    ) -> Result<IntEnabledInput<'a>, ariel_os_embassy_common::gpio::input::Error> {
        let pull = from_pull(pull);
        let ch = crate::extint_registry::EXTINT_REGISTRY.get_interrupt_channel_for_pin(&pin)?;
        let pin: Peri<'_, AnyPin> = pin.into();
        Ok(IntEnabledInput::new(pin, ch, pull))
    }

    ariel_os_embassy_common::define_from_pull!();
    ariel_os_embassy_common::define_into_level!();
}

pub mod output {
    //! Output-specific types.

    use embassy_stm32::{Peri, gpio::Level};

    #[doc(hidden)]
    pub use embassy_stm32::gpio::{Output, Pin as OutputPin};

    /// Whether outputs support configuring their drive strength.
    pub const DRIVE_STRENGTH_CONFIGURABLE: bool = false;
    /// Whether outputs support configuring their speed/slew rate.
    pub const SPEED_CONFIGURABLE: bool = true;

    #[doc(hidden)]
    pub fn new<'a>(
        pin: Peri<'a, impl OutputPin>,
        initial_level: ariel_os_embassy_common::gpio::Level,
        _drive_strength: super::DriveStrength, // Not supported by hardware
        speed: super::Speed,
    ) -> Output<'a> {
        let initial_level = match initial_level {
            ariel_os_embassy_common::gpio::Level::Low => Level::Low,
            ariel_os_embassy_common::gpio::Level::High => Level::High,
        };
        Output::new(pin, initial_level, speed.into())
    }
}

pub use ariel_os_embassy_common::gpio::UnsupportedDriveStrength as DriveStrength;

/// Available output speed/slew rate settings.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Speed {
    /// Low.
    Low,
    /// Medium.
    Medium,
    /// High.
    High,
    /// Very high.
    VeryHigh,
}

impl From<Speed> for embassy_stm32::gpio::Speed {
    fn from(speed: Speed) -> Self {
        match speed {
            Speed::Low => Self::Low,
            Speed::Medium => Self::Medium,
            #[cfg(not(any(gpio_v1, syscfg_f0)))]
            Speed::High => Self::High,
            #[cfg(any(gpio_v1, syscfg_f0))]
            Speed::High => Self::VeryHigh,
            Speed::VeryHigh => Self::VeryHigh,
        }
    }
}

impl ariel_os_embassy_common::gpio::FromSpeed for Speed {
    fn from(speed: ariel_os_embassy_common::gpio::Speed<Self>) -> Self {
        match speed {
            ariel_os_embassy_common::gpio::Speed::Hal(speed) => speed,
            ariel_os_embassy_common::gpio::Speed::Low => Self::Low,
            ariel_os_embassy_common::gpio::Speed::Medium => Self::Medium,
            ariel_os_embassy_common::gpio::Speed::High => Self::High,
            ariel_os_embassy_common::gpio::Speed::VeryHigh => Self::VeryHigh,
        }
    }
}
