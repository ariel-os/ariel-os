#![no_std]
pub mod gpio;

#[cfg(feature = "i2c")]
pub mod i2c;

pub mod hal;

#[cfg(feature = "uart")]
pub mod uart;

// All items of this module are re-exported at the root of `ariel_os`.
#[doc(hidden)]
pub mod api {
    pub use crate::gpio;
    pub use crate::hal;

    // #[cfg(feature = "ble")]
    // pub use crate::ble;
    #[cfg(feature = "i2c")]
    pub use crate::i2c;
    // #[cfg(feature = "net")]
    // pub use crate::net;
    // #[cfg(feature = "spi")]
    // pub use crate::spi;
    #[cfg(feature = "uart")]
    pub use crate::uart;
    // #[cfg(feature = "usb")]
    // pub use crate::usb;
}
