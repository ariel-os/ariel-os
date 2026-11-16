//! Driver for simple LEDs wired directly to MCUs with GPIOs.

use crate::gpio::{Level, Output};

pub struct Led {
    inner: Output<'static>,
    active: Level,
}

impl Led {
    /// Creates new [`Led`].
    #[must_use]
    pub fn new(output: Output<'static>, active: Level) -> Self {
        Self {
            inner: output,
            active,
        }
    }

    /// Toggle the [`Led`].
    pub fn toggle(&mut self) {
        self.inner.toggle();
    }

    /// Turns the [`Led`] on.
    pub fn turn_on(&mut self) {
        if self.active == Level::High {
            self.inner.set_high();
        } else {
            self.inner.set_low();
        }
    }

    /// Turns the [`Led`] off.
    pub fn turn_off(&mut self) {
        if self.active == Level::High {
            self.inner.set_low();
        } else {
            self.inner.set_high();
        }
    }

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
