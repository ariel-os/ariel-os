//! Provides support for the I2C communication bus.
#![deny(missing_docs)]

#[doc(alias = "master")]
pub mod controller;

/// Everything needed to make an I2C Bus.
///
/// This trait relies on the fact that
/// - the I2C Peripheral type from the MCU is also the name of the "driver-creating" type using the same peripheral.
/// - this "device-creating" type has a method `new(sda, scl) -> X` that create something that impl [``embedded_hal_async::i2c::I2c``]
///
/// # Usage
///
/// ```rust
/// use ariel_os_hal::i2c::BusPart;
///
/// use ariel_os_boards::pins::i2c0;
/// use ariel_os::hal;
///
/// use embassy_sync::mutex::Mutex;
/// pub static I2C_BUS: once_cell::sync::OnceCell<
///    Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, hal::i2c::controller::I2c>,
/// > = once_cell::sync::OnceCell::new();
///
/// #[ariel_os::task(autostart, peripherals)]
/// fn main(p: i2c0) {
///     let mut i2c_config = hal::i2c::controller::Config::default();
///     let (sda, scl) = p.into_pins();
///     let i2c = <i2c0 as BusPart>::I2CPeri::new(sda, scl, i2c_config);
///     let _ = I2C_BUS.set(Mutex::new(i2c_bus));
/// }
/// ```
pub trait BusPart {
    /// The dedicated I2C Peripheral from the MCU.
    type I2CPeri;
    /// The pin connected to the I2C Data line.
    type Sda;
    /// The pin connected to the I2C clock line.
    type Scl;

    /// Reclaim the pins.
    fn into_pins(self) -> (Self::Sda, Self::Scl);
}
