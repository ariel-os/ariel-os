//! LEDs.
//!
//! LEDs can be specifically monocolor LEDs or individual color channels from bicolor or RGB LEDs.
//! Addressable LEDs (e.g., "smart pixels") are not part of this module.

use crate::gpio::{Level, Output};

/// An LED.
pub struct Led {
    inner: Output<'static>,
    active: Level,
}

impl Led {
    /// Creates a new [`Led`] from an output and an active [`Level`].
    #[must_use]
    pub fn new(output: Output<'static>, active: Level) -> Self {
        Self {
            inner: output,
            active,
        }
    }

    /// Toggles the LED.
    pub fn toggle(&mut self) {
        self.inner.toggle();
    }

    /// Turns the LED on.
    pub fn turn_on(&mut self) {
        if self.active == Level::High {
            self.inner.set_high();
        } else {
            self.inner.set_low();
        }
    }

    /// Turns the LED off.
    pub fn turn_off(&mut self) {
        if self.active == Level::High {
            self.inner.set_low();
        } else {
            self.inner.set_high();
        }
    }

    /// Returns the underlying [`Output`], consuming the [`Led`].
    #[must_use]
    pub fn into_output(self) -> Output<'static> {
        self.inner
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_led_inner {
    ($color:ident) => {
        /// Turns the LED on.
        pub fn turn_on(&mut self) {
            self.$color.turn_on()
        }

        /// Turns the LED off.
        pub fn turn_off(&mut self) {
            self.$color.turn_off()
        }

        /// Toggles the LED.
        pub fn toggle(&mut self) {
            self.$color.toggle()
        }

        /// Returns the underlying output, consuming the type.
        pub fn into_output(self) -> $crate::gpio::Output<'static> {
            self.$color.into_output()
        }
    };
}
