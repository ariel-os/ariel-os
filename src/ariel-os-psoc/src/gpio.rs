//! Provides GPIO access.
#![expect(unsafe_code, reason = "AnyPin::steal builds runtime pins from owned singletons")]

use psoc::gpio::{AnyPin, PinConfig, PinState, mode};

use crate::{PsocPin, peripheral::Peri};

/// A GPIO level.
#[derive(Copy, Clone, PartialEq, Eq)]
#[doc(hidden)]
pub enum Level {
    /// Low.
    Low,
    /// High.
    High,
}

impl From<Level> for PinState {
    fn from(level: Level) -> Self {
        match level {
            Level::Low => PinState::Low,
            Level::High => PinState::High,
        }
    }
}

impl From<ariel_os_embassy_common::gpio::Level> for Level {
    fn from(level: ariel_os_embassy_common::gpio::Level) -> Self {
        match level {
            ariel_os_embassy_common::gpio::Level::Low => Level::Low,
            ariel_os_embassy_common::gpio::Level::High => Level::High,
        }
    }
}

pub mod input {
    //! Input-specific types.

    use super::{AnyPin, Level, PinConfig, PsocPin, Peri, mode};
    use ariel_os_embassy_common::gpio::Pull;

    #[cfg(feature = "external-interrupts")]
    use ariel_os_embassy_common::gpio::input::InterruptError;

    /// A GPIO pin usable as an input.
    #[doc(hidden)]
    pub trait InputPin: PsocPin {}
    impl<P: PsocPin> InputPin for P {}

    /// Whether inputs support configuring whether a Schmitt trigger is enabled.
    pub const SCHMITT_TRIGGER_CONFIGURABLE: bool = false;

    /// A GPIO input.
    #[doc(hidden)]
    pub struct Input<'a> {
        pub(crate) pin: AnyPin<'a>,
    }

    impl Input<'_> {
        #[must_use]
        pub fn is_high(&self) -> bool {
            self.pin.is_high()
        }

        #[must_use]
        pub fn is_low(&self) -> bool {
            self.pin.is_low()
        }

        #[must_use]
        pub fn get_level(&self) -> Level {
            if self.pin.is_high() {
                Level::High
            } else {
                Level::Low
            }
        }
    }

    impl embedded_hal::digital::ErrorType for Input<'_> {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal::digital::InputPin for Input<'_> {
        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok((*self).is_high())
        }

        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok((*self).is_low())
        }
    }

    #[doc(hidden)]
    pub fn new<'a, P: InputPin>(
        _pin: Peri<'a, P>,
        pull: Pull,
        _schmitt_trigger: bool, // Not supported by hardware
    ) -> Result<Input<'a>, core::convert::Infallible> {
        // SAFETY: `_pin` is an owned peripheral singleton, ensuring exclusive
        // access to this pin.
        let mut pin = unsafe { AnyPin::steal(P::PORT, P::PIN) };
        match pull {
            Pull::None => pin.initialize(mode::HighZ::<true>, PinConfig::default()),
            Pull::Up => pin.initialize(mode::PullUp::<true>, PinConfig::default()),
            Pull::Down => pin.initialize(mode::PullDown::<true>, PinConfig::default()),
        }
        Ok(Input { pin })
    }

    #[doc(hidden)]
    #[must_use]
    pub fn into_level(level: Level) -> ariel_os_embassy_common::gpio::Level {
        match level {
            Level::Low => ariel_os_embassy_common::gpio::Level::Low,
            Level::High => ariel_os_embassy_common::gpio::Level::High,
        }
    }

    #[cfg(feature = "external-interrupts")]
    #[doc(hidden)]
    pub struct IntEnabledInput<'a> {
        pub(crate) pin: AnyPin<'a>,
    }

    #[cfg(feature = "external-interrupts")]
    impl embedded_hal::digital::ErrorType for IntEnabledInput<'_> {
        type Error = core::convert::Infallible;
    }

    #[cfg(feature = "external-interrupts")]
    impl embedded_hal::digital::InputPin for IntEnabledInput<'_> {
        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok(self.pin.is_high())
        }

        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok(self.pin.is_low())
        }
    }

    #[cfg(feature = "external-interrupts")]
    impl embedded_hal_async::digital::Wait for IntEnabledInput<'_> {
        async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
            unimplemented!("external interrupts are not supported on PSoC")
        }
        async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
            unimplemented!("external interrupts are not supported on PSoC")
        }
        async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
            unimplemented!("external interrupts are not supported on PSoC")
        }
        async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
            unimplemented!("external interrupts are not supported on PSoC")
        }
        async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
            unimplemented!("external interrupts are not supported on PSoC")
        }
    }

    #[cfg(feature = "external-interrupts")]
    impl IntEnabledInput<'_> {
        #[must_use]
        pub fn is_high(&self) -> bool {
            self.pin.is_high()
        }
        #[must_use]
        pub fn is_low(&self) -> bool {
            self.pin.is_low()
        }
        #[must_use]
        pub fn get_level(&self) -> Level {
            if self.pin.is_high() {
                Level::High
            } else {
                Level::Low
            }
        }
        pub async fn wait_for_high(&mut self) {
            unimplemented!("external interrupts are not supported on PSoC")
        }
        pub async fn wait_for_low(&mut self) {
            unimplemented!("external interrupts are not supported on PSoC")
        }
        pub async fn wait_for_rising_edge(&mut self) {
            unimplemented!("external interrupts are not supported on PSoC")
        }
        pub async fn wait_for_falling_edge(&mut self) {
            unimplemented!("external interrupts are not supported on PSoC")
        }
        pub async fn wait_for_any_edge(&mut self) {
            unimplemented!("external interrupts are not supported on PSoC")
        }
    }

    #[cfg(feature = "external-interrupts")]
    #[doc(hidden)]
    pub fn new_int_enabled<'a, P: InputPin>(
        _pin: Peri<'a, P>,
        _pull: Pull,
        _schmitt_trigger: bool,
    ) -> Result<IntEnabledInput<'a>, InterruptError> {
        // Type-erased async (interrupt) pins are not yet supported by the psoc HAL.
        unimplemented!("external interrupts are not supported on PSoC")
    }
}

pub mod output {
    //! Output-specific types.

    use super::{AnyPin, Level, PinConfig, PsocPin, Peri, mode};

    /// A GPIO pin usable as an output.
    #[doc(hidden)]
    pub trait OutputPin: PsocPin {}
    impl<P: PsocPin> OutputPin for P {}

    /// Whether outputs support configuring their drive strength.
    pub const DRIVE_STRENGTH_CONFIGURABLE: bool = true;
    /// Whether outputs support configuring their speed/slew rate.
    pub const SPEED_CONFIGURABLE: bool = false;

    /// A GPIO output.
    #[doc(hidden)]
    pub struct Output<'a> {
        pin: AnyPin<'a>,
    }

    impl embedded_hal::digital::ErrorType for Output<'_> {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal::digital::OutputPin for Output<'_> {
        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.pin.set_high();
            Ok(())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.pin.set_low();
            Ok(())
        }
    }

    impl embedded_hal::digital::StatefulOutputPin for Output<'_> {
        fn is_set_high(&mut self) -> Result<bool, Self::Error> {
            Ok(self.pin.is_set_high())
        }

        fn is_set_low(&mut self) -> Result<bool, Self::Error> {
            Ok(self.pin.is_set_low())
        }

        fn toggle(&mut self) -> Result<(), Self::Error> {
            self.pin.toggle();
            Ok(())
        }
    }

    #[doc(hidden)]
    pub fn new<'a, P: OutputPin>(
        _pin: Peri<'a, P>,
        initial_level: ariel_os_embassy_common::gpio::Level,
        drive_strength: super::DriveStrength,
        _speed: super::Speed, // Not supported
    ) -> Output<'a> {
        let initial_level = Level::from(initial_level).into();
        let config = PinConfig::default()
            .initial_output(initial_level)
            .drive_strength(drive_strength.into());

        // SAFETY: `_pin` is an owned peripheral singleton, ensuring exclusive
        // access to this pin.
        let mut pin = unsafe { AnyPin::steal(P::PORT, P::PIN) };
        pin.initialize(mode::Strong::<false>, config);
        Output { pin }
    }
}

pub use ariel_os_embassy_common::gpio::UnsupportedSpeed as Speed;

/// Available drive strength settings.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum DriveStrength {
    /// Full drive strength.
    #[default]
    Full,
    /// 1/2 drive strength.
    Half,
    /// 1/4 drive strength.
    Quarter,
    /// 1/8 drive strength.
    Eighth,
}

impl From<DriveStrength> for psoc::gpio::DriveStrength {
    fn from(drive_strength: DriveStrength) -> Self {
        match drive_strength {
            DriveStrength::Full => Self::Full,
            DriveStrength::Half => Self::Half,
            DriveStrength::Quarter => Self::Quarter,
            DriveStrength::Eighth => Self::Eighth,
        }
    }
}

impl ariel_os_embassy_common::gpio::FromDriveStrength for DriveStrength {
    fn from(drive_strength: ariel_os_embassy_common::gpio::DriveStrength<Self>) -> Self {
        use ariel_os_embassy_common::gpio::DriveStrength as CommonDriveStrength;
        match drive_strength {
            CommonDriveStrength::Hal(drive_strength) => drive_strength,
            CommonDriveStrength::Lowest => Self::Eighth,
            CommonDriveStrength::Standard => Self::default(),
            CommonDriveStrength::Medium | CommonDriveStrength::High => Self::Half,
            CommonDriveStrength::Highest => Self::Full,
        }
    }
}
