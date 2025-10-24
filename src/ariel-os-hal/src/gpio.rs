//! Provides consistent GPIO access.
//!
//! # Note
//!
//! This API does not currently provide a way of using the same GPIO pin as an input and an output
//! alternatively.
//! If you have a use case for this, especially if this is not regarding bit-banging, please open
//! an issue on our repository.
#![allow(missing_docs)]

use core::marker::PhantomData;

use embedded_hal::digital::StatefulOutputPin;

use crate::hal::{
    self, IntoPeripheral,
    gpio::{
        DriveStrength as HalDriveStrength, Speed as HalSpeed,
        input::{Input as HalInput, InputPin as HalInputPin},
        output::{Output as HalOutput, OutputPin as HalOutputPin},
    },
};

#[cfg(feature = "external-interrupts")]
use crate::hal::gpio::input::IntEnabledInput as HalIntEnabledInput;

use input::InputBuilder;
use output::OutputBuilder;

pub use ariel_os_embassy_common::gpio::{DriveStrength, Level, Pull, Speed};

// We do not provide an `impl` block because it would be grouped separately in the documentation.
macro_rules! inner_impl_input_methods {
    ($inner:ident) => {
        /// Returns whether the input level is high.
        #[must_use]
        pub fn is_high(&self) -> bool {
            self.$inner.is_high()
        }

        /// Returns whether the input level is low.
        #[must_use]
        pub fn is_low(&self) -> bool {
            self.$inner.is_low()
        }

        /// Returns the input level.
        #[must_use]
        pub fn get_level(&self) -> Level {
            #[cfg(context = "esp")]
            let level = hal::gpio::input::into_level(self.$inner.level());
            #[cfg(not(context = "esp"))]
            let level = hal::gpio::input::into_level(self.$inner.get_level());
            level
        }
    };
}

/// A GPIO input.
///
/// If support for external interrupts is needed, use [`InputBuilder::build_with_interrupt()`] to
/// obtain an [`IntEnabledInput`].
pub struct Input<'a> {
    input: HalInput<'a>,
}

impl<'a> Input<'a> {
    /// Returns a configured [`Input`].
    pub fn new<T: IntoPeripheral<'a, P>, P: HalInputPin + 'a>(pin: T, pull: Pull) -> Self {
        Self::builder(pin, pull).build()
    }

    /// Returns an [`InputBuilder`], allowing to configure the GPIO input further.
    pub fn builder<T: IntoPeripheral<'a, P>, P: HalInputPin + 'a>(
        pin: T,
        pull: Pull,
    ) -> InputBuilder<'a, T, P> {
        InputBuilder {
            pin,
            pull,
            schmitt_trigger: false,
            _phantom: PhantomData,
        }
    }

    inner_impl_input_methods!(input);
}

#[doc(hidden)]
impl<'a> embedded_hal::digital::ErrorType for Input<'a> {
    type Error = <HalInput<'a> as embedded_hal::digital::ErrorType>::Error;
}

/// A GPIO input that supports external interrupts.
///
/// Can be obtained with [`InputBuilder::build_with_interrupt()`].
#[cfg(feature = "external-interrupts")]
pub struct IntEnabledInput<'a> {
    input: HalIntEnabledInput<'a>,
}

#[cfg(feature = "external-interrupts")]
impl IntEnabledInput<'_> {
    inner_impl_input_methods!(input);

    /// Asynchronously waits until the input level is high.
    /// Returns immediately if it is already high.
    pub async fn wait_for_high(&mut self) {
        self.input.wait_for_high().await;
    }

    /// Asynchronously waits until the input level is low.
    /// Returns immediately if it is already low.
    pub async fn wait_for_low(&mut self) {
        self.input.wait_for_low().await;
    }

    /// Asynchronously waits for the input level to transition from low to high.
    pub async fn wait_for_rising_edge(&mut self) {
        self.input.wait_for_rising_edge().await;
    }

    /// Asynchronously waits for the input level to transition from high to low.
    pub async fn wait_for_falling_edge(&mut self) {
        self.input.wait_for_falling_edge().await;
    }

    /// Asynchronously waits for the input level to transition from one level to the other.
    pub async fn wait_for_any_edge(&mut self) {
        self.input.wait_for_any_edge().await;
    }
}

#[cfg(feature = "external-interrupts")]
#[doc(hidden)]
impl embedded_hal::digital::ErrorType for IntEnabledInput<'_> {
    type Error = <HalIntEnabledInput<'static> as embedded_hal::digital::ErrorType>::Error;
}

#[cfg(feature = "external-interrupts")]
impl embedded_hal_async::digital::Wait for IntEnabledInput<'_> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        <HalIntEnabledInput<'_> as embedded_hal_async::digital::Wait>::wait_for_high(
            &mut self.input,
        )
        .await
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        <HalIntEnabledInput<'_> as embedded_hal_async::digital::Wait>::wait_for_low(&mut self.input)
            .await
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        <HalIntEnabledInput<'_> as embedded_hal_async::digital::Wait>::wait_for_rising_edge(
            &mut self.input,
        )
        .await
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        <HalIntEnabledInput<'_> as embedded_hal_async::digital::Wait>::wait_for_falling_edge(
            &mut self.input,
        )
        .await
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        <HalIntEnabledInput<'_> as embedded_hal_async::digital::Wait>::wait_for_any_edge(
            &mut self.input,
        )
        .await
    }
}

macro_rules! impl_embedded_hal_input_trait {
    ($type:ty, $hal_type:ty) => {
        impl embedded_hal::digital::InputPin for $type {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                <$hal_type as embedded_hal::digital::InputPin>::is_high(&mut self.input)
            }

            fn is_low(&mut self) -> Result<bool, Self::Error> {
                <$hal_type as embedded_hal::digital::InputPin>::is_low(&mut self.input)
            }
        }
    };
    ($type:ty, $hal_type:ty, $lt:lifetime) => {
        impl<$lt> embedded_hal::digital::InputPin for $type {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                <$hal_type<$lt> as embedded_hal::digital::InputPin>::is_high(&mut self.input)
            }

            fn is_low(&mut self) -> Result<bool, Self::Error> {
                <$hal_type<$lt> as embedded_hal::digital::InputPin>::is_low(&mut self.input)
            }
        }
    };
}

impl_embedded_hal_input_trait!(Input<'_>, HalInput<'_>);
#[cfg(feature = "external-interrupts")]
impl_embedded_hal_input_trait!(IntEnabledInput<'_>, HalIntEnabledInput<'_>);

pub mod input {
    //! Input-specific types.
    use core::marker::PhantomData;

    use ariel_os_embassy_common::gpio::Pull;

    use crate::hal::{self, IntoPeripheral, gpio::input::InputPin as HalInputPin};

    use super::Input;

    #[cfg(feature = "external-interrupts")]
    use super::IntEnabledInput;

    pub use ariel_os_embassy_common::gpio::input::Error;

    #[cfg(feature = "external-interrupts")]
    pub use ariel_os_embassy_common::gpio::input::InterruptError;

    /// Builder type for [`Input`], can be obtained with [`Input::builder()`].
    pub struct InputBuilder<'a, T: IntoPeripheral<'a, P>, P: HalInputPin> {
        pub(crate) pin: T,
        pub(crate) pull: Pull,
        pub(crate) schmitt_trigger: bool,
        pub(crate) _phantom: PhantomData<&'a P>,
    }

    impl<'a, T: IntoPeripheral<'a, P>, P: HalInputPin + 'a> InputBuilder<'a, T, P> {
        /// Configures the input's Schmitt trigger.
        ///
        /// # Note
        ///
        /// Fails to compile if the HAL does not support configuring Schmitt trigger on
        /// inputs.
        #[must_use]
        pub fn schmitt_trigger(self, enable: bool) -> Self {
            #[expect(
                clippy::assertions_on_constants,
                reason = "the constant depends on the HAL"
            )]
            const {
                assert!(
                    hal::gpio::input::SCHMITT_TRIGGER_CONFIGURABLE,
                    "This HAL does not support configuring Schmitt triggers on GPIO inputs."
                );
            }

            Self {
                schmitt_trigger: enable,
                ..self
            }
        }

        // It is unclear whether `opt_*()` functions are actually useful, so we provide them but do not
        // commit to them being part of our API for now.
        // We may remove them in the future if we realize they are never useful.
        #[doc(hidden)]
        #[must_use]
        pub fn opt_schmitt_trigger(self, enable: bool) -> Self {
            if hal::gpio::input::SCHMITT_TRIGGER_CONFIGURABLE {
                // We cannot reuse the non-`opt_*()`, otherwise the const assert inside it would always
                // be triggered.
                Self {
                    schmitt_trigger: enable,
                    ..self
                }
            } else {
                self
            }
        }
    }

    // Split the impl for consistency with outputs.
    impl<'a, T: IntoPeripheral<'a, P>, P: HalInputPin> InputBuilder<'a, T, P> {
        /// Returns an [`Input`] by finalizing the builder.
        pub fn build(self) -> Input<'a> {
            let pin = self.pin.into_hal_peripheral();
            #[allow(irrefutable_let_patterns, reason = "conditional compilation")]
            let Ok(input) = hal::gpio::input::new(pin, self.pull, self.schmitt_trigger) else {
                unreachable!()
            };

            Input { input }
        }

        /// Returns an [`IntEnabledInput`] by finalizing the builder.
        ///
        /// # Errors
        ///
        /// On some MCU families, the number of external interrupts that can simultaneously be
        /// enabled is limited by the number of hardware interrupt channels.
        /// Some MCU families also have other limitations, for instance it may not be possible to
        /// register interrupts on a pin if one is already registered on the pin with the same pin
        /// number of another port (e.g., `PA0` and `PB0`).
        /// In these cases, this returns an [`Error::InterruptChannel`], with a HAL-specific error.
        // FIXME: rename this
        #[cfg(feature = "external-interrupts")]
        pub fn build_with_interrupt(self) -> Result<IntEnabledInput<'a>, Error> {
            let pin = self.pin.into_hal_peripheral();
            let input = hal::gpio::input::new_int_enabled(pin, self.pull, self.schmitt_trigger)?;

            Ok(IntEnabledInput { input })
        }
    }
}

/// A GPIO output.
pub struct Output<'a> {
    output: HalOutput<'a>,
}

impl<'a> Output<'a> {
    /// Returns a configured [`Output`].
    pub fn new<T: IntoPeripheral<'a, P>, P: HalOutputPin + 'a>(
        pin: T,
        initial_level: Level,
    ) -> Self {
        Self::builder(pin, initial_level).build()
    }

    /// Returns an [`OutputBuilder`], allowing to configure the GPIO output further.
    pub fn builder<T: IntoPeripheral<'a, P>, P: HalOutputPin + 'a>(
        pin: T,
        initial_level: Level,
    ) -> OutputBuilder<'a, T, P> {
        OutputBuilder {
            pin,
            initial_level,
            drive_strength: DriveStrength::default(),
            speed: Speed::default(),
            _phantom: PhantomData,
        }
    }

    /// Sets the output as high.
    pub fn set_high(&mut self) {
        // All HALs are infallible.
        let _ = <Self as embedded_hal::digital::OutputPin>::set_high(self);
    }

    /// Sets the output as low.
    pub fn set_low(&mut self) {
        // All HALs are infallible.
        let _ = <Self as embedded_hal::digital::OutputPin>::set_low(self);
    }

    /// Sets the output level.
    pub fn set_level(&mut self, level: Level) {
        let state = level.into();
        // All HALs are infallible.
        let _ = <Self as embedded_hal::digital::OutputPin>::set_state(self, state);
    }

    /// Toggles the output level.
    pub fn toggle(&mut self) {
        // All HALs are infallible.
        let _ = <Self as StatefulOutputPin>::toggle(self);
    }
}

pub mod output {
    //! Output-specific types.
    use core::marker::PhantomData;

    use ariel_os_embassy_common::gpio::{
        DriveStrength, FromDriveStrength, FromSpeed, Level, Speed,
    };

    use crate::hal::{self, IntoPeripheral, gpio::output::OutputPin as HalOutputPin};

    use super::{HalDriveStrength, HalSpeed, Output};

    /// Builder type for [`Output`], can be obtained with [`Output::builder()`].
    pub struct OutputBuilder<'a, T: IntoPeripheral<'a, P>, P: HalOutputPin> {
        pub(crate) pin: T,
        pub(crate) initial_level: Level,
        pub(crate) drive_strength: DriveStrength<HalDriveStrength>,
        pub(crate) speed: Speed<HalSpeed>,
        pub(crate) _phantom: PhantomData<&'a P>,
    }

    // We define this in a macro because it will be useful for open-drain outputs.
    macro_rules! impl_output_builder {
        ($type:ident, $pin_trait:ident) => {
            impl<'a, T: IntoPeripheral<'a, P>, P: $pin_trait> $type<'a, T, P> {
                /// Configures the output's drive strength.
                ///
                /// # Note
                ///
                /// Fails to compile if the HALs does not support configuring drive strength of
                /// outputs.
                #[must_use]
                pub fn drive_strength(
                    self,
                    drive_strength: DriveStrength<HalDriveStrength>,
                ) -> Self {
                    const {
                        assert!(
                            hal::gpio::output::DRIVE_STRENGTH_CONFIGURABLE,
                            "This HAL does not support setting the drive strength of GPIO outputs."
                        );
                    }

                    Self {
                        drive_strength,
                        ..self
                    }
                }

                // It is unclear whether `opt_*()` functions are actually useful, so we provide them but do not
                // commit to them being part of our API for now.
                // We may remove them in the future if we realize they are never useful.
                #[doc(hidden)]
                #[must_use]
                // TODO: or `drive_strength_opt`?
                pub fn opt_drive_strength(
                    self,
                    drive_strength: DriveStrength<HalDriveStrength>,
                ) -> Self {
                    if hal::gpio::output::DRIVE_STRENGTH_CONFIGURABLE {
                        // We cannot reuse the non-`opt_*()`, otherwise the const assert inside it would always
                        // be triggered.
                        Self {
                            drive_strength,
                            ..self
                        }
                    } else {
                        self
                    }
                }

                /// Configures the output's speed.
                ///
                /// # Note
                ///
                /// Fails to compile if the HAL does not support configuring speed of outputs.
                #[must_use]
                pub fn speed(self, speed: Speed<HalSpeed>) -> Self {
                    const {
                        assert!(
                            hal::gpio::output::SPEED_CONFIGURABLE,
                            "This HAL does not support setting the speed of GPIO outputs."
                        );
                    }

                    Self { speed, ..self }
                }

                // It is unclear whether `opt_*()` functions are actually useful, so we provide them but do not
                // commit to them being part of our API for now.
                // We may remove them in the future if we realize they are never useful.
                #[doc(hidden)]
                #[must_use]
                // TODO: or `speed_opt`?
                pub fn opt_speed(self, speed: Speed<HalSpeed>) -> Self {
                    if hal::gpio::output::SPEED_CONFIGURABLE {
                        // We cannot reuse the non-`opt_*()`, otherwise the const assert inside it would always
                        // be triggered.
                        Self { speed, ..self }
                    } else {
                        self
                    }
                }
            }
        };
    }

    impl_output_builder!(OutputBuilder, HalOutputPin);

    impl<'a, T: IntoPeripheral<'a, P>, P: HalOutputPin> OutputBuilder<'a, T, P> {
        /// Returns an [`Output`] by finalizing the builder.
        pub fn build(self) -> Output<'a> {
            // TODO: should we move this into `output::new()`s?
            let drive_strength = <HalDriveStrength as FromDriveStrength>::from(self.drive_strength);
            // TODO: should we move this into `output::new()`s?
            let speed = <HalSpeed as FromSpeed>::from(self.speed);

            let pin = self.pin.into_hal_peripheral();

            let output = hal::gpio::output::new(pin, self.initial_level, drive_strength, speed);

            Output { output }
        }
    }
}

// We define this in a macro because it will be useful for open-drain outputs.
macro_rules! impl_embedded_hal_output_traits {
    ($type:ident, $hal_type:ident) => {
        #[doc(hidden)]
        impl embedded_hal::digital::ErrorType for $type<'_> {
            type Error = <$hal_type<'static> as embedded_hal::digital::ErrorType>::Error;
        }

        impl embedded_hal::digital::OutputPin for $type<'_> {
            fn set_high(&mut self) -> Result<(), Self::Error> {
                <$hal_type<'_> as embedded_hal::digital::OutputPin>::set_high(&mut self.output)
            }

            fn set_low(&mut self) -> Result<(), Self::Error> {
                <$hal_type<'_> as embedded_hal::digital::OutputPin>::set_low(&mut self.output)
            }
        }

        // Outputs are all stateful outputs on:
        // - embassy-nrf
        // - embassy-rp
        // - esp-hal
        // - embassy-stm32
        impl StatefulOutputPin for $type<'_> {
            fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                <$hal_type<'_> as StatefulOutputPin>::is_set_high(&mut self.output)
            }

            fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                <$hal_type<'_> as StatefulOutputPin>::is_set_low(&mut self.output)
            }
        }
    };
}

impl_embedded_hal_output_traits!(Output, HalOutput);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_gpio_type_sizes() {
        // Assert that the GPIO types are zero cost memory-wise.
        assert_eq!(size_of::<Input>(), size_of::<()>());
        assert_eq!(size_of::<IntEnabledInput>(), size_of::<()>());
        assert_eq!(size_of::<Output>(), size_of::<()>());
    }
}
