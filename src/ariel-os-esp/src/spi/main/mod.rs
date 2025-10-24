//! Provides support for the SPI communication bus in main mode.

#![expect(unsafe_code)]

use ariel_os_embassy_common::{
    impl_async_spibus_for_driver_enum,
    spi::{BitOrder, Mode, main::Kilohertz},
};
use embassy_embedded_hal::adapter::{BlockingAsync, YieldingAsync};
use esp_hal::{
    gpio::{
        self,
        interconnect::{PeripheralInput, PeripheralOutput},
    },
    peripherals,
    spi::master::Spi as InnerSpi,
};

// TODO: we could consider making this `pub`
// NOTE(hal): values from the datasheets.
#[cfg(any(
    context = "esp32",
    context = "esp32c3",
    context = "esp32c6",
    context = "esp32s2",
    context = "esp32s3"
))]
const MAX_FREQUENCY: Kilohertz = Kilohertz::MHz(80);

/// SPI bus configuration.
#[derive(Clone)]
#[non_exhaustive]
pub struct Config {
    /// The frequency at which the bus should operate.
    pub frequency: Frequency,
    /// The SPI mode to use.
    pub mode: Mode,
    #[doc(hidden)]
    pub bit_order: BitOrder,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: Frequency::F(Kilohertz::MHz(80)),
            mode: Mode::Mode0,
            bit_order: BitOrder::default(),
        }
    }
}

/// SPI bus frequency.
// Possible values are copied from embassy-nrf
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Frequency {
    /// Arbitrary frequency.
    F(Kilohertz),
}

ariel_os_embassy_common::impl_spi_from_frequency!();
ariel_os_embassy_common::impl_spi_frequency_const_functions!(MAX_FREQUENCY);

impl From<Frequency> for esp_hal::time::Rate {
    fn from(freq: Frequency) -> Self {
        match freq {
            Frequency::F(kilohertz) => esp_hal::time::Rate::from_khz(kilohertz.raw()),
        }
    }
}

macro_rules! define_spi_drivers {
    ($( $peripheral:ident ),* $(,)?) => {
        $(
            /// Peripheral-specific SPI driver.
            pub struct $peripheral<'a> {
                spim: YieldingAsync<BlockingAsync<InnerSpi<'a, esp_hal::Blocking>>>,
            }

            impl<'a> $peripheral<'a> {
                /// Returns a driver implementing [`embedded_hal_async::spi::SpiBus`] for this SPI
                /// peripheral.
                #[expect(clippy::new_ret_no_self)]
                #[must_use]
                pub fn new(
                    sck_pin: impl PeripheralOutput<'a>,
                    miso_pin: impl PeripheralInput<'a>,
                    mosi_pin: impl PeripheralOutput<'a>,
                    config: Config,
                ) -> Spi<'a> {
                    // Make this struct a compile-time-enforced singleton: having multiple statics
                    // defined with the same name would result in a compile-time error.
                    paste::paste! {
                        #[allow(dead_code)]
                        static [<PREVENT_MULTIPLE_ $peripheral>]: () = ();
                    }

                    let mut spi_config = esp_hal::spi::master::Config::default()
                        .with_frequency(config.frequency.into())
                        .with_mode(crate::spi::from_mode(config.mode))
                        .with_read_bit_order(crate::spi::from_bit_order(config.bit_order))
                        .with_write_bit_order(crate::spi::from_bit_order(config.bit_order));

                    // FIXME(safety): enforce that the init code indeed has run
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // peripheral multiple times.
                    let spi_peripheral = unsafe { peripherals::$peripheral::steal() };

                    let spi = esp_hal::spi::master::Spi::new(
                        spi_peripheral,
                        spi_config,
                    )
                        .unwrap()
                        .with_sck(sck_pin)
                        .with_mosi(mosi_pin)
                        .with_miso(miso_pin)
                        .with_cs(gpio::NoPin); // The CS pin is managed separately

                    Spi::$peripheral(Self { spim: YieldingAsync::new(BlockingAsync::new(spi)) })
                }
            }
        )*

        /// Peripheral-agnostic driver.
        pub enum Spi<'a> {
            $(
                #[doc = concat!(stringify!($peripheral), " peripheral.")]
                $peripheral($peripheral<'a>)
            ),*
        }

        impl<'a> embedded_hal_async::spi::ErrorType for Spi<'a> {
            type Error = esp_hal::spi::Error;
        }

        impl_async_spibus_for_driver_enum!(Spi, $( $peripheral ),*);
    };
}

// Define a driver per peripheral
// SPI0 and SPI1 exist but are not general-purpose SPI peripherals.
#[cfg(context = "esp32")]
define_spi_drivers!(SPI2, SPI3);
#[cfg(context = "esp32c3")]
define_spi_drivers!(SPI2);
#[cfg(context = "esp32c6")]
define_spi_drivers!(SPI2);
#[cfg(context = "esp32s2")]
define_spi_drivers!(SPI2, SPI3);
#[cfg(context = "esp32s3")]
define_spi_drivers!(SPI2, SPI3);
