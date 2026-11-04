//! Higher Level drivers.

use crate::gpio::{Input, Level};

#[cfg(feature = "external-interrupts")]
use crate::gpio::IntEnabledInput;

/// The combination of an Input and an active Level.
pub struct SwitchPin<'a> {
    input: Input<'a>,
    active: Level,
}

impl<'a> SwitchPin<'a> {
    /// Returns a configured [`SwitchPin`].
    #[must_use]
    pub fn new(input: Input<'a>, active: Level) -> Self {
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
/// The combination of an [`IntEnabledInput`] and an active [`Level`].
pub struct IntEnabledSwitchPin<'a> {
    input: IntEnabledInput<'a>,
    active: Level,
}

#[cfg(feature = "external-interrupts")]
impl<'a> IntEnabledSwitchPin<'a> {
    /// Returns a configured [`IntEnabledSwitchPin`].
    #[must_use]
    pub fn new(input: IntEnabledInput<'a>, active: Level) -> Self {
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

    /// Asynchronously wait for the pin to be active.
    /// Returns immediately if the pin is active.
    pub async fn wait_for_active(&mut self) {
        if self.active == Level::High {
            self.input.wait_for_high().await;
        } else {
            self.input.wait_for_low().await;
        }
    }

    /// Asynchronously wait for the pin to be inactive.
    /// Returns immediately if the pin is inactive.
    pub async fn wait_for_inactive(&mut self) {
        if self.active == Level::High {
            self.input.wait_for_low().await;
        } else {
            self.input.wait_for_high().await;
        }
    }

    /// Asynchronously waits for the input level to transition from inactive to active.
    pub async fn wait_for_activation_edge(&mut self) {
        if self.is_active() {
            self.wait_for_inactive().await;
        }
        self.wait_for_active().await;
    }

    /// Asynchronously waits for the input level to transition from active to inactive.
    pub async fn wait_for_deactivation_edge(&mut self) {
        if self.is_inactive() {
            self.wait_for_active().await;
        }
        self.wait_for_inactive().await;
    }
}

#[macro_export]
macro_rules! impl_intswitch_inner {
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
