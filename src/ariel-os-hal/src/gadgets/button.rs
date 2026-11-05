//! Push button.

use crate::gadgets::switch::SwitchPin;

pub struct Button {
    switch: SwitchPin,
}

impl Button {
    #[must_use]
    pub fn new(switch: SwitchPin) -> Self {
        Self { switch }
    }

    #[must_use]
    pub fn is_pressed(&self) -> bool {
        self.switch.is_active()
    }

    #[must_use]
    pub fn is_released(&self) -> bool {
        self.switch.is_inactive()
    }

    /// Waits for the button to be clicked.
    ///
    /// This will not return until the button is depressed enough
    /// to register as released and is then pressed again.
    #[cfg(feature = "external-interrupts")]
    pub fn wait_for_click(&mut self) -> impl Future<Output = ()> {
        self.switch.wait_for_activation_edge()
    }

    /// Waits for the button to be pressed.
    ///
    /// Returns immediately if it is already being pressed.
    #[cfg(feature = "external-interrupts")]
    pub fn wait_for_pressed(&mut self) -> impl Future<Output = ()> {
        self.switch.wait_for_active()
    }
}

#[cfg(not(feature = "external-interrupts"))]
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

#[cfg(feature = "external-interrupts")]
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

        /// Waits for the button to be clicked.
        ///
        /// This will not return until the button is depressed enough
        /// to register as released and is then pressed again.
        pub fn wait_for_click(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_click()
        }

        /// Waits for the button to be pressed.
        ///
        /// Returns immediately if it is already being pressed.
        pub fn wait_for_pressed(&mut self) -> impl Future<Output = ()> {
            self.$inner.wait_for_pressed()
        }
    };
}
