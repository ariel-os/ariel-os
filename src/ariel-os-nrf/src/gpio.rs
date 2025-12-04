//! Provides GPIO access.

pub mod input {
    //! Input-specific types.

    use embassy_nrf::{
        Peripheral,
        gpio::{Level, Pull},
    };

    #[doc(hidden)]
    pub use embassy_nrf::gpio::{Input, Pin as InputPin};

    // Re-export `Input` as `IntEnabledInput` as they are interrupt-enabled.
    #[cfg(feature = "external-interrupts")]
    #[doc(hidden)]
    pub use embassy_nrf::gpio::Input as IntEnabledInput;

    /// Whether inputs support configuring whether a Schmitt trigger is enabled.
    pub const SCHMITT_TRIGGER_CONFIGURABLE: bool = false;

    #[doc(hidden)]
    pub fn new(
        pin: impl Peripheral<P: InputPin> + 'static,
        pull: ariel_os_embassy_common::gpio::Pull,
        _schmitt_trigger: bool, // Not supported by hardware
    ) -> Result<Input<'static>, ariel_os_embassy_common::gpio::input::Error> {
        let pull = from_pull(pull);
        Ok(Input::new(pin, pull))
    }

    #[cfg(feature = "external-interrupts")]
    #[doc(hidden)]
    pub fn new_int_enabled(
        pin: impl Peripheral<P: InputPin> + 'static,
        pull: ariel_os_embassy_common::gpio::Pull,
        _schmitt_trigger: bool, // Not supported by hardware
    ) -> Result<IntEnabledInput<'static>, ariel_os_embassy_common::gpio::input::Error> {
        let pull = from_pull(pull);
        let mut pin = pin.into_ref();
        crate::extint_registry::EXTINT_REGISTRY.use_interrupt_for_pin(&mut pin)?;
        Ok(Input::new(pin, pull))
    }

    ariel_os_embassy_common::define_from_pull!();
    ariel_os_embassy_common::define_into_level!();
}

pub mod output {
    //! Output-specific types.

    use embassy_nrf::{
        Peripheral,
        gpio::{Level, OutputDrive},
    };

    use super::DriveStrength;

    #[doc(hidden)]
    pub use embassy_nrf::gpio::{Output, Pin as OutputPin};

    /// Whether outputs support configuring their drive strength.
    pub const DRIVE_STRENGTH_CONFIGURABLE: bool = true;
    /// Whether outputs support configuring their speed/slew rate.
    pub const SPEED_CONFIGURABLE: bool = false;

    #[doc(hidden)]
    pub fn new(
        pin: impl Peripheral<P: OutputPin> + 'static,
        initial_level: ariel_os_embassy_common::gpio::Level,
        drive_strength: DriveStrength,
        _speed: super::Speed, // Not supported by hardware
    ) -> Output<'static> {
        let output_drive = match drive_strength {
            DriveStrength::Standard => OutputDrive::Standard,
            DriveStrength::High => OutputDrive::HighDrive,
        };
        let initial_level = match initial_level {
            ariel_os_embassy_common::gpio::Level::Low => Level::Low,
            ariel_os_embassy_common::gpio::Level::High => Level::High,
        };
        Output::new(pin, initial_level, output_drive)
    }
}

pub use ariel_os_embassy_common::gpio::UnsupportedSpeed as Speed;

/// Available drive strength settings.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum DriveStrength {
    /// Standard.
    #[default]
    Standard,
    /// High.
    High, // Around 10 mA
}

impl ariel_os_embassy_common::gpio::FromDriveStrength for DriveStrength {
    fn from(drive_strength: ariel_os_embassy_common::gpio::DriveStrength<Self>) -> Self {
        // ESPs are able to output up to 40 mA, so we somewhat normalize this.
        match drive_strength {
            ariel_os_embassy_common::gpio::DriveStrength::Hal(drive_strength) => drive_strength,
            ariel_os_embassy_common::gpio::DriveStrength::Lowest
            | ariel_os_embassy_common::gpio::DriveStrength::Medium => Self::Standard,
            ariel_os_embassy_common::gpio::DriveStrength::Standard => Self::default(),
            ariel_os_embassy_common::gpio::DriveStrength::High
            | ariel_os_embassy_common::gpio::DriveStrength::Highest => Self::High,
        }
    }
}
