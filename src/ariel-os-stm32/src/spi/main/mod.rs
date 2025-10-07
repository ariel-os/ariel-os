//! Provides support for the SPI communication bus in main mode.

#![expect(unsafe_code)]

use ariel_os_embassy_common::{
    impl_async_spibus_for_driver_enum,
    spi::{BitOrder, Mode, main::Kilohertz},
};
use embassy_embedded_hal::adapter::{BlockingAsync, YieldingAsync};
use embassy_stm32::{
    Peripheral, gpio,
    mode::Blocking,
    peripherals,
    spi::{MisoPin, MosiPin, SckPin, Spi as InnerSpi},
    time::Hertz,
};

// TODO: we could consider making this `pub`
// NOTE(hal): values from the datasheets.
// When peripherals support different frequencies, the smallest one is used.
#[cfg(context = "stm32c031c6")]
const MAX_FREQUENCY: Kilohertz = Kilohertz::MHz(24);
#[cfg(context = "stm32f401re")]
const MAX_FREQUENCY: Kilohertz = Kilohertz::MHz(21);
#[cfg(context = "stm32f411re")]
const MAX_FREQUENCY: Kilohertz = Kilohertz::MHz(25);
#[cfg(context = "stm32h755zi")]
const MAX_FREQUENCY: Kilohertz = Kilohertz::MHz(150);
#[cfg(context = "stm32l475vg")]
const MAX_FREQUENCY: Kilohertz = Kilohertz::MHz(40);
#[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))]
const MAX_FREQUENCY: Kilohertz = Kilohertz::MHz(32);
// TODO: verify, datasheet says "Baud rate prescaler up to kernel frequency/2 or bypass from RCC in
// master mode", core freq is 160MHz
#[cfg(context = "stm32u585ai")]
const MAX_FREQUENCY: Kilohertz = Kilohertz::MHz(80);
#[cfg(context = "stm32wb55rg")]
const MAX_FREQUENCY: Kilohertz = Kilohertz::MHz(32);

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
            frequency: Frequency::F(Kilohertz::MHz(1)),
            mode: Mode::Mode0,
            bit_order: BitOrder::default(),
        }
    }
}

/// SPI bus frequency.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Frequency {
    /// Arbitrary frequency.
    F(Kilohertz),
}

impl From<Frequency> for Hertz {
    fn from(freq: Frequency) -> Self {
        match freq {
            Frequency::F(kilohertz) => Hertz::khz(kilohertz.to_kHz()),
        }
    }
}

ariel_os_embassy_common::impl_spi_from_frequency!();
ariel_os_embassy_common::impl_spi_frequency_const_functions!(MAX_FREQUENCY);

macro_rules! define_spi_drivers {
    ($( $interrupt:ident => $peripheral:ident ),* $(,)?) => {
        $(
            /// Peripheral-specific SPI driver.
            pub struct $peripheral {
                spim: YieldingAsync<BlockingAsync<InnerSpi<'static, Blocking>>>,
            }

            impl $peripheral {
                /// Returns a driver implementing [`embedded_hal_async::spi::SpiBus`] for this SPI
                /// peripheral.
                #[expect(clippy::new_ret_no_self)]
                #[must_use]
                pub fn new(
                    sck_pin: impl Peripheral<P: SckPin<peripherals::$peripheral>> + 'static,
                    miso_pin: impl Peripheral<P: MisoPin<peripherals::$peripheral>> + 'static,
                    mosi_pin: impl Peripheral<P: MosiPin<peripherals::$peripheral>> + 'static,
                    config: Config,
                ) -> Spi {
                    // Make this struct a compile-time-enforced singleton: having multiple statics
                    // defined with the same name would result in a compile-time error.
                    paste::paste! {
                        #[allow(dead_code)]
                        static [<PREVENT_MULTIPLE_ $peripheral>]: () = ();
                    }

                    let mut spi_config = embassy_stm32::spi::Config::default();
                    spi_config.frequency = config.frequency.into();
                    spi_config.mode = crate::spi::from_mode(config.mode);
                    spi_config.bit_order = crate::spi::from_bit_order(config.bit_order);
                    spi_config.miso_pull = gpio::Pull::None;

                    // FIXME(safety): enforce that the init code indeed has run
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // peripheral multiple times.
                    let spim_peripheral = unsafe { peripherals::$peripheral::steal() };

                    // The order of MOSI/MISO pins is inverted.
                    let spim = InnerSpi::new_blocking(
                        spim_peripheral,
                        sck_pin,
                        mosi_pin,
                        miso_pin,
                        spi_config,
                    );

                    Spi::$peripheral(Self { spim: YieldingAsync::new(BlockingAsync::new(spim)) })
                }
            }
        )*

        /// Peripheral-agnostic driver.
        pub enum Spi {
            $(
                #[doc = concat!(stringify!($peripheral), " peripheral.")]
                $peripheral($peripheral)
            ),*
        }

        impl embedded_hal_async::spi::ErrorType for Spi {
            type Error = embassy_stm32::spi::Error;
        }

        impl_async_spibus_for_driver_enum!(Spi, $( $peripheral ),*);
    };
}

// Define a driver per peripheral
#[cfg(context = "stm32c031c6")]
define_spi_drivers!(
   SPI1 => SPI1,
);
#[cfg(context = "stm32f401re")]
define_spi_drivers!(
   SPI1 => SPI1,
   SPI2 => SPI2,
   SPI3 => SPI3,
);
#[cfg(context = "stm32f411re")]
define_spi_drivers!(
   SPI1 => SPI1,
   SPI2 => SPI2,
   SPI3 => SPI3,
   SPI4 => SPI4,
   SPI5 => SPI5,
);
#[cfg(context = "stm32h755zi")]
define_spi_drivers!(
   SPI1 => SPI1,
   SPI2 => SPI2,
   SPI3 => SPI3,
   SPI4 => SPI4,
   SPI5 => SPI5,
   SPI6 => SPI6,
);
#[cfg(context = "stm32l475vg")]
define_spi_drivers!(
   SPI1 => SPI1,
   SPI2 => SPI2,
   SPI3 => SPI3,
);
#[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))]
define_spi_drivers!(
   SPI1 => SPI1,
   // FIXME: the other two SPI peripherals share the same interrupt
);
#[cfg(context = "stm32u585ai")]
define_spi_drivers!(
   SPI1 => SPI1,
   SPI2 => SPI2,
   SPI3 => SPI3,
);
#[cfg(context = "stm32wb55rg")]
define_spi_drivers!(
   SPI1 => SPI1,
   SPI2 => SPI2,
);
