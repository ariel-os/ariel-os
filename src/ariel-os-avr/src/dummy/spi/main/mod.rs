//! HAL- and MCU-specific types for SPI.
//!
//! This module provides a driver for each SPI peripheral, the driver name being the same as the
//! peripheral; see the tests and examples to learn how to instantiate them.

/// Peripheral-agnostic SPI driver implementing [`embedded_hal_async::spi::SpiBus`].
///
/// This type is not meant to be instantiated directly; instead instantiate a peripheral-specific
/// driver provided by this module.
// NOTE: we keep this type public because it may still required in user-written type signatures.
pub enum Spi {
    // Make the docs show that this enum has variants, but do not show any because they are
    // MCU-specific.
    #[doc(hidden)]
    Hidden,
}

/// MCU-specific SPI bus frequency.
#[expect(clippy::manual_non_exhaustive)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Frequency {
    /// Low speed.
    _1m,
    /// Medium speed.
    _4m,
    #[doc(hidden)]
    Hidden,
}

impl Frequency {
    #[must_use]
    pub const fn first() -> Self {
        Self::_1m
    }

    #[must_use]
    pub const fn last() -> Self {
        Self::_4m
    }

    #[must_use]
    pub const fn next(self) -> Option<Self> {
        match self {
            Self::_1m => Some(Self::_4m),
            Self::_4m => None,
            Self::Hidden => unreachable!(),
        }
    }

    #[must_use]
    pub const fn prev(self) -> Option<Self> {
        match self {
            Self::_1m => None,
            Self::_4m => Some(Self::_1m),
            Self::Hidden => unreachable!(),
        }
    }

    #[must_use]
    pub const fn khz(self) -> u32 {
        match self {
            Self::_1m => 1000,
            Self::_4m => 4000,
            Self::Hidden => unreachable!(),
        }
    }
}
