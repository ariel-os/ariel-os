
pub use ariel_os_dummy::gpio::input;
pub use ariel_os_dummy::peripheral::Peri;

//impl private::Sealed for Peripheral {}

impl<T> crate::IntoPeripheral<'_, T> for Peri<'static, T> {
    fn into_hal_peripheral(self) -> Self {
        self
    }
}

pub mod output {
    use std::sync::{Arc, LazyLock, Mutex, mpsc};
    use ariel_os_embassy_common::gpio::Level;
    use embedded_hal::digital::StatefulOutputPin;

    use crate::peripherals::OutStream;

    pub static STREAMS: LazyLock<[OutStream<PinState>; 1]> = LazyLock::new(|| {
        let init = || {
            let (sender, recv) = mpsc::channel();
            OutStream {
                recv: Arc::new(Mutex::new(recv)),
                sender,
            }
        };
        [init()]
    });

    pub trait OutputPin {
        /// Index to the peripherals::OUT_STREAMS array
        const PIN_NUMBER: usize;
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct PinState {
        level: ariel_os_embassy_common::gpio::Level,
    }

    pub const DRIVE_STRENGTH_CONFIGURABLE: bool = false;
    pub const SPEED_CONFIGURABLE: bool = false;

    pub fn new<T: OutputPin>(
        pin: super::Peri<'static, T>,
        initial_level: ariel_os_embassy_common::gpio::Level,
        _drive_strength: super::DriveStrength,
        _speed: super::Speed,
    ) -> Output<'static> {
        let state = PinState { level: initial_level };
        let ret = Output {
            _marker: Default::default(),
            // Don't carry the channel.
            // The channel should survive even if the pin gets destroyed.
            pin_number: T::PIN_NUMBER,
            state: state.clone(),
        };
        ret.send_update(state);
        ret
    }

    pub struct Output<'d> {
        _marker: core::marker::PhantomData<&'d ()>,
        pin_number: usize,
        state: PinState,
    }

    impl<'d> Output<'d> {
        fn send_update(&self, event: PinState) {
            STREAMS[self.pin_number].sender.send(event).unwrap();
        }
    }

    impl embedded_hal::digital::ErrorType for Output<'_> {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal::digital::OutputPin for Output<'_> {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.state.level = Level::Low;
            self.send_update(self.state.clone());
            Ok(())
        }

        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.state.level = Level::High;
            self.send_update(self.state.clone());
            Ok(())
        }
    }

    impl StatefulOutputPin for Output<'_> {
        fn is_set_high(&mut self) -> Result<bool, Self::Error> {
            Ok(self.state.level == Level::High)
        }

        fn is_set_low(&mut self) -> Result<bool, Self::Error> {
            Ok(self.state.level == Level::Low)
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
