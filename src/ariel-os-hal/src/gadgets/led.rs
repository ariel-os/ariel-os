//! Driver for simple LEDs wired directly to MCUs with GPIOs.

use crate::gpio::{Output, Level};

pub struct Led {
    inner: Output<'static>,
    active: Level,
    color: &'static str,
}

impl Led {
    /// Creates new [`Led`].
    #[must_use]
    pub fn new(output: Output<'static>, active: Level, color: &'static str) -> Self {
        Self { inner: output, active, color }
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

    /// Returns the color of the [`Led`].
    #[must_use]
    pub fn color(&self) -> &'static str {
        self.color
    }
}
