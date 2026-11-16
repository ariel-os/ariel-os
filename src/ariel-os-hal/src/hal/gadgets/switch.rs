//! Switches.
//!
//! A switch is conceptually composed of one or multiple [`SwitchPin`].
//! If the switch is a push-button, [`Button`](super::button::Button) can be used instead.

use crate::gpio::Level;

#[cfg(feature = "external-interrupts")]
use crate::gpio::IntEnabledInput;

#[cfg(not(feature = "external-interrupts"))]
use crate::gpio::Input;

/// A pin of a switch.
/// Combines an input and an active [`Level`].
///
/// If the `external-interrupts` feature is enabled, additionally provides `wait_for_X` methods.
pub struct SwitchPin {
    #[cfg(feature = "external-interrupts")]
    input: IntEnabledInput<'static>,

    #[cfg(not(feature = "external-interrupts"))]
    input: Input<'static>,
    active: Level,
}

impl SwitchPin {
    /// Returns a configured [`SwitchPin`].
    #[must_use]
    #[cfg(not(feature = "external-interrupts"))]
    pub fn new(input: Input<'static>, active: Level) -> Self {
        Self { input, active }
    }

    /// Returns a configured [`SwitchPin`].
    #[must_use]
    #[cfg(feature = "external-interrupts")]
    pub fn new(input: IntEnabledInput<'static>, active: Level) -> Self {
        Self { input, active }
    }

    /// Returns whether the pin is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.input.get_level() == self.active
    }

    /// Returns whether the pin is inactive.
    #[must_use]
    pub fn is_inactive(&self) -> bool {
        self.input.get_level() != self.active
    }
}

#[cfg(feature = "external-interrupts")]
impl SwitchPin {
    /// Waits for the pin to be active.
    /// Returns immediately if the pin is already active.
    pub async fn wait_for_active(&mut self) {
        if self.active == Level::High {
            self.input.wait_for_high().await;
        } else {
            self.input.wait_for_low().await;
        }
    }

    /// Waits for the pin to be inactive.
    /// Returns immediately if the pin is already inactive.
    pub async fn wait_for_inactive(&mut self) {
        if self.active == Level::High {
            self.input.wait_for_low().await;
        } else {
            self.input.wait_for_high().await;
        }
    }

    /// Waits for the input level to transition from inactive to active.
    pub async fn wait_for_activation_edge(&mut self) {
        if self.active == Level::High {
            self.input.wait_for_rising_edge().await;
        } else {
            self.input.wait_for_falling_edge().await;
        }
    }

    /// Waits for the input level to transition from active to inactive.
    pub async fn wait_for_deactivation_edge(&mut self) {
        if self.active == Level::High {
            self.input.wait_for_falling_edge().await;
        } else {
            self.input.wait_for_rising_edge().await;
        }
    }
}

// Used in the board package.
#[cfg(feature = "external-interrupts")]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_switch_inner {
    ($inner:ident) => {
        /// Returns whether the pin is active.
        pub fn is_active(&self) -> bool {
            self.$inner.is_active()
        }

        /// Returns whether the pin is inactive.
        #[must_use]
        pub fn is_inactive(&self) -> bool {
            self.$inner.is_inactive()
        }

        /// Asynchronously wait for the pin to be active.
        /// Returns immediately if the pin is active.
        pub fn wait_for_active(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_active()
        }

        /// Asynchronously wait for the pin to be inactive.
        /// Returns immediately if the pin is inactive.
        pub fn wait_for_inactive(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_inactive()
        }

        /// Asynchronously waits for the input level to transition from inactive to active.
        pub fn wait_for_activation_edge(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_activation_edge()
        }

        /// Asynchronously waits for the input level to transition from active to inactive.
        pub fn wait_for_deactivation_edge(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_deactivation_edge()
        }
    };
}

// Used in the board package.
#[cfg(not(feature = "external-interrupts"))]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_switch_inner {
    ($inner:ident) => {
        /// Returns whether the pin is active.
        pub fn is_active(&self) -> bool {
            self.$inner.is_active()
        }

        /// Returns whether the pin is inactive.
        #[must_use]
        pub fn is_inactive(&self) -> bool {
            self.$inner.is_inactive()
        }
    };
}
