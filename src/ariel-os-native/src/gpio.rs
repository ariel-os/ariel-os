
pub use ariel_os_dummy::gpio::input;
pub use ariel_os_dummy::peripheral::Peri;

//impl private::Sealed for Peripheral {}

impl<T> crate::IntoPeripheral<'_, T> for Peri<'static, T> {
    fn into_hal_peripheral(self) -> Self {
        self
    }
}

pub mod output {
    use embedded_hal::digital::StatefulOutputPin;

    pub trait OutputPin {
        const PIN_NUMBER: u8;
    }

    #[derive(Debug, PartialEq)]
    pub enum PinState {
        High,
        Low,
    }

    pub const DRIVE_STRENGTH_CONFIGURABLE: bool = false;
    pub const SPEED_CONFIGURABLE: bool = false;

    pub fn new<T: OutputPin>(
        pin: super::Peri<'static, T>,
        _initial_level: ariel_os_embassy_common::gpio::Level,
        _drive_strength: super::DriveStrength,
        _speed: super::Speed,
    ) -> Output<'static> {
        Output {
            _marker: Default::default(),
            // Don't carry the channel.
            // The channel should survive even if the pin gets destroyed.
            pin_number: T::PIN_NUMBER,
        }
    }

    pub struct Output<'d> {
        _marker: core::marker::PhantomData<&'d ()>,
        pin_number: u8,
    }

    impl embedded_hal::digital::ErrorType for Output<'_> {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal::digital::OutputPin for Output<'_> {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl StatefulOutputPin for Output<'_> {
        fn is_set_high(&mut self) -> Result<bool, Self::Error> {
            unimplemented!();
        }

        fn is_set_low(&mut self) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }
}

    /// Actual type is HAL-specific.
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum DriveStrength {
        #[doc(hidden)]
        Hidden,
    }

    impl ariel_os_embassy_common::gpio::FromDriveStrength for DriveStrength {
        fn from(_drive_strength: ariel_os_embassy_common::gpio::DriveStrength<Self>) -> Self {
            Self::Hidden
        }
    }

    /// Actual type is HAL-specific.
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum Speed {
        #[doc(hidden)]
        Hidden,
    }

    impl ariel_os_embassy_common::gpio::FromSpeed for Speed {
        fn from(_speed: ariel_os_embassy_common::gpio::Speed<Self>) -> Self {
            Self::Hidden
        }
    }
