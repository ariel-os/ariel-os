//! Push-buttons.

use super::switch::SwitchPin;

/// Represents a push-button.
/// A push-button is typically a monostable tactile switch.
///
/// For generic switches, see [`switch`](super::switch).
pub struct Button {
    switch: SwitchPin,
}

impl Button {
    /// Creates a [`Button`] from a [`SwitchPin`].
    #[must_use]
    pub fn new(switch: SwitchPin) -> Self {
        Self { switch }
    }

    /// Returns whether the push-button is currently in a pressed state.
    #[must_use]
    pub fn is_pressed(&self) -> bool {
        self.switch.is_active()
    }

    /// Returns whether the push-button is currently in a released state.
    #[must_use]
    pub fn is_released(&self) -> bool {
        self.switch.is_inactive()
    }

    /// Waits for the push-button to be in a released state.
    /// Returns immediately if it is already released.
    #[cfg(feature = "external-interrupts")]
    pub fn wait_for_released(&mut self) -> impl Future<Output = ()> {
        self.switch.wait_for_inactive()
    }

    /// Waits for the push-button to be in a pressed state.
    /// Returns immediately if it is already being pressed.
    #[cfg(feature = "external-interrupts")]
    pub fn wait_for_pressed(&mut self) -> impl Future<Output = ()> {
        self.switch.wait_for_active()
    }

    /// Waits for the push-button to be pressed after first being in a released state.
    ///
    /// # Note
    ///
    /// No software debouncing is done.
    #[cfg(feature = "external-interrupts")]
    pub fn wait_for_next_press(&mut self) -> impl Future<Output = ()> {
        self.switch.wait_for_activation_edge()
    }

    /// Waits for the push-button to be released after first being in a pressed state.
    ///
    /// # Note
    ///
    /// No software debouncing is done.
    #[cfg(feature = "external-interrupts")]
    pub fn wait_for_next_release(&mut self) -> impl Future<Output = ()> {
        self.switch.wait_for_deactivation_edge()
    }
}

// Used in the board package.
#[cfg(not(feature = "external-interrupts"))]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_button_inner {
    ($inner:ident) => {
        #[must_use]
        pub fn is_pressed(&self) -> bool {
            self.$inner.is_pressed()
        }

        #[must_use]
        pub fn is_released(&self) -> bool {
            self.$inner.is_released()
        }
    };
}

// Used in the board package.
#[cfg(feature = "external-interrupts")]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_button_inner {
    ($inner:ident) => {
        /// Returns whether the push-button is currently in a pressed state.
        #[must_use]
        pub fn is_pressed(&self) -> bool {
            self.$inner.is_pressed()
        }

        /// Returns whether the push-button is currently released.
        #[must_use]
        pub fn is_released(&self) -> bool {
            self.$inner.is_released()
        }

        /// Waits for the push-button to be in a released state.
        /// Returns immediately if it is already released.
        pub fn wait_for_released(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_released()
        }

        /// Waits for the push-button to be in a pressed state.
        /// Returns immediately if it is already being pressed.
        pub fn wait_for_pressed(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_pressed()
        }

        /// Waits for the push-button to be pressed after first being released.
        ///
        /// # Note
        ///
        /// No software debouncing is done.
        pub fn wait_for_next_press(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_next_press()
        }

        /// Waits for the push-button to be pressed after first being released.
        ///
        /// # Note
        ///
        /// No software debouncing is done.
        pub fn wait_for_next_release(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_next_release()
        }
    };
}
