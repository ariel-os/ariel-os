//! UART configuration.

#![expect(unsafe_code)]

use ariel_os_embassy_common::{impl_async_uart_for_driver_enum, uart::ConfigError};
use embassy_nrf::{
    Peri, bind_interrupts,
    buffered_uarte::{BufferedUarte, InterruptHandler},
    gpio::Pin as GpioPin,
    peripherals,
};

/// UART interface configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// The baud rate at which UART operates.
    pub baudrate: ariel_os_embassy_common::uart::Baudrate<Baudrate>,
    /// Number of data bits.
    pub data_bits: DataBits,
    /// Number of stop bits.
    pub stop_bits: StopBits,
    /// Parity mode used for the interface.
    pub parity: Parity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baudrate: ariel_os_embassy_common::uart::Baudrate::_115200,
            data_bits: DataBits::Data8,
            stop_bits: StopBits::Stop1,
            parity: Parity::None,
        }
    }
}

/// UART baud rate.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Baudrate {
    /// 1200 baud.
    _1200,
    /// 2400 baud.
    _2400,
    /// 4800 baud.
    _4800,
    /// 9600 baud.
    _9600,
    /// 14400 baud.
    _14400,
    /// 19200 baud.
    _19200,
    /// 28800 baud.
    _28800,
    /// 31250 baud.
    _31250,
    /// 38400 baud.
    _38400,
    /// 56000 baud.
    _56000,
    /// 57600 baud.
    _57600,
    /// 76800 baud.
    _76800,
    /// 115200 baud.
    _115200,
    /// 230400 baud.
    _230400,
    /// 250000 baud.
    _250000,
    /// 460800 baud.
    _460800,
    /// 921600 baud.
    _921600,
    /// 1 Megabaud.
    _1000000,
}

impl From<Baudrate> for u32 {
    fn from(baud: Baudrate) -> u32 {
        match baud {
            Baudrate::_1200 => 1200,
            Baudrate::_2400 => 2400,
            Baudrate::_4800 => 4800,
            Baudrate::_9600 => 9600,
            Baudrate::_14400 => 14_400,
            Baudrate::_19200 => 19_200,
            Baudrate::_28800 => 28_800,
            Baudrate::_31250 => 31_250,
            Baudrate::_38400 => 38_400,
            Baudrate::_56000 => 56_000,
            Baudrate::_57600 => 57_600,
            Baudrate::_76800 => 76_800,
            Baudrate::_115200 => 11_5200,
            Baudrate::_230400 => 23_0400,
            Baudrate::_250000 => 25_0000,
            Baudrate::_460800 => 46_0800,
            Baudrate::_921600 => 92_1600,
            Baudrate::_1000000 => 1_000_000,
        }
    }
}

fn from_baudrate(baud: Baudrate) -> embassy_nrf::buffered_uarte::Baudrate {
    match baud {
        Baudrate::_1200 => embassy_nrf::uarte::Baudrate::BAUD1200,
        Baudrate::_2400 => embassy_nrf::uarte::Baudrate::BAUD2400,
        Baudrate::_4800 => embassy_nrf::uarte::Baudrate::BAUD4800,
        Baudrate::_9600 => embassy_nrf::uarte::Baudrate::BAUD9600,
        Baudrate::_14400 => embassy_nrf::uarte::Baudrate::BAUD14400,
        Baudrate::_19200 => embassy_nrf::uarte::Baudrate::BAUD19200,
        Baudrate::_28800 => embassy_nrf::uarte::Baudrate::BAUD28800,
        Baudrate::_31250 => embassy_nrf::uarte::Baudrate::BAUD31250,
        Baudrate::_38400 => embassy_nrf::uarte::Baudrate::BAUD38400,
        Baudrate::_56000 => embassy_nrf::uarte::Baudrate::BAUD56000,
        Baudrate::_57600 => embassy_nrf::uarte::Baudrate::BAUD57600,
        Baudrate::_76800 => embassy_nrf::uarte::Baudrate::BAUD76800,
        Baudrate::_115200 => embassy_nrf::uarte::Baudrate::BAUD115200,
        Baudrate::_230400 => embassy_nrf::uarte::Baudrate::BAUD230400,
        Baudrate::_250000 => embassy_nrf::uarte::Baudrate::BAUD250000,
        Baudrate::_460800 => embassy_nrf::uarte::Baudrate::BAUD460800,
        Baudrate::_921600 => embassy_nrf::uarte::Baudrate::BAUD921600,
        Baudrate::_1000000 => embassy_nrf::uarte::Baudrate::BAUD1M,
    }
}

impl From<ariel_os_embassy_common::uart::Baudrate<Self>> for Baudrate {
    fn from(baud: ariel_os_embassy_common::uart::Baudrate<Self>) -> Baudrate {
        match baud {
            ariel_os_embassy_common::uart::Baudrate::Hal(baud) => baud,
            ariel_os_embassy_common::uart::Baudrate::_2400 => Baudrate::_2400,
            ariel_os_embassy_common::uart::Baudrate::_4800 => Baudrate::_4800,
            ariel_os_embassy_common::uart::Baudrate::_9600 => Baudrate::_9600,
            ariel_os_embassy_common::uart::Baudrate::_19200 => Baudrate::_19200,
            ariel_os_embassy_common::uart::Baudrate::_38400 => Baudrate::_38400,
            ariel_os_embassy_common::uart::Baudrate::_57600 => Baudrate::_57600,
            ariel_os_embassy_common::uart::Baudrate::_115200 => Baudrate::_115200,
        }
    }
}

/// UART number of data bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum DataBits {
    /// 8 bits per character.
    Data8,
}

impl From<ariel_os_embassy_common::uart::DataBits<Self>> for DataBits {
    fn from(databits: ariel_os_embassy_common::uart::DataBits<Self>) -> DataBits {
        match databits {
            ariel_os_embassy_common::uart::DataBits::Hal(bits) => bits,
            ariel_os_embassy_common::uart::DataBits::Data8 => DataBits::Data8,
        }
    }
}
/// UART number of stop bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum StopBits {
    /// One stop bit.
    Stop1,
}

impl From<ariel_os_embassy_common::uart::StopBits<Self>> for StopBits {
    fn from(stopbits: ariel_os_embassy_common::uart::StopBits<Self>) -> Self {
        match stopbits {
            ariel_os_embassy_common::uart::StopBits::Hal(stopbits) => stopbits,
            ariel_os_embassy_common::uart::StopBits::Stop1 => StopBits::Stop1,
        }
    }
}

/// Parity bit.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Parity {
    /// No parity bit.
    None,
    /// Even parity bit.
    Even,
}

fn from_parity(parity: Parity) -> embassy_nrf::uarte::Parity {
    match parity {
        Parity::None => embassy_nrf::uarte::Parity::EXCLUDED,
        Parity::Even => embassy_nrf::uarte::Parity::INCLUDED,
    }
}

impl From<ariel_os_embassy_common::uart::Parity<Self>> for Parity {
    fn from(parity: ariel_os_embassy_common::uart::Parity<Self>) -> Self {
        match parity {
            ariel_os_embassy_common::uart::Parity::Hal(parity) => parity,
            ariel_os_embassy_common::uart::Parity::None => Self::None,
            ariel_os_embassy_common::uart::Parity::Even => Self::Even,
        }
    }
}

macro_rules! define_uart_drivers {
    ($( $interrupt:ident => $peripheral:ident + $timer:ident + $ppi_ch1:ident + $ppi_ch2:ident + $ppi_group:ident),* $(,)?) => {
        $(
            /// Peripheral-specific UART driver.
            pub struct $peripheral<'d> {
                uart: BufferedUarte<'d>,
            }

            // Make this struct a compile-time-enforced singleton: having multiple statics
            // defined with the same name would result in a compile-time error.
            paste::paste! {
                #[allow(dead_code)]
                static [<PREVENT_MULTIPLE_ $peripheral>]: () = ();
                #[allow(dead_code)]
                static [<PREVENT_MULTIPLE_ $timer>]: () = ();
                #[allow(dead_code)]
                static [<PREVENT_MULTIPLE_ $ppi_ch1>]: () = ();
                #[allow(dead_code)]
                static [<PREVENT_MULTIPLE_ $ppi_ch2>]: () = ();
                #[allow(dead_code)]
                static [<PREVENT_MULTIPLE_ $ppi_group>]: () = ();
            }

            impl<'d> $peripheral<'d> {
                /// Returns a driver implementing [`embedded_io_async`] for this Uart
                /// peripheral.
                ///
                /// # Errors
                ///
                /// This never returns an error.
                #[expect(clippy::new_ret_no_self)]
                pub fn new<T: GpioPin, U: GpioPin>(
                    rx_pin: Peri<'d, T>,
                    tx_pin: Peri<'d, U>,
                    rx_buffer: &'d mut [u8],
                    tx_buffer: &'d mut [u8],
                    config: Config,
                ) -> Result<Uart<'d>, ConfigError> {
                    let mut uart_config = embassy_nrf::uarte::Config::default();
                    uart_config.baudrate = from_baudrate(Baudrate::from(config.baudrate));
                    uart_config.parity = from_parity(config.parity);
                    bind_interrupts!(struct Irqs {
                        $interrupt => InterruptHandler<peripherals::$peripheral>;
                    });

                    // FIXME(safety): enforce that the init code indeed has run
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // peripheral multiple times.
                    let uart_peripheral = unsafe { peripherals::$peripheral::steal() };
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // required timer multiple times.
                    let timer_peripheral = unsafe { peripherals::$timer::steal() };
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // required ppi channel multiple times.
                    let ppi_ch1_peripheral = unsafe { peripherals::$ppi_ch1::steal() };
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // required ppi channel multiple times.
                    let ppi_ch2_peripheral = unsafe { peripherals::$ppi_ch2::steal() };
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // required ppi group multiple times.
                    let ppi_group_peripheral = unsafe { peripherals::$ppi_group::steal() };

                    let uart = BufferedUarte::new(
                        uart_peripheral,
                        timer_peripheral,
                        ppi_ch1_peripheral,
                        ppi_ch2_peripheral,
                        ppi_group_peripheral,
                        rx_pin,
                        tx_pin,
                        Irqs,
                        uart_config,
                        rx_buffer,
                        tx_buffer
                    );

                    Ok(Uart::$peripheral(Self { uart }))
                }
            }
        )*

        /// Peripheral-agnostic UART driver.
        pub enum Uart<'d> {
            $(
                #[doc = concat!(stringify!($peripheral), " peripheral.")]
                $peripheral($peripheral<'d>)
            ),*
        }

        impl embedded_io_async::ErrorType for Uart<'_> {
            type Error = embassy_nrf::buffered_uarte::Error;
        }

        impl_async_uart_for_driver_enum!(Uart, $( $peripheral ),*);
    }
}

// Define a driver per peripheral
#[cfg(context = "nrf52832")]
define_uart_drivers!(
   UARTE0 => UARTE0 + TIMER4 + PPI_CH14 + PPI_CH15 + PPI_GROUP5,
);
#[cfg(context = "nrf52833")]
define_uart_drivers!(
   UARTE0 => UARTE0 + TIMER3 + PPI_CH13 + PPI_CH14 + PPI_GROUP4,
   UARTE1 => UARTE1 + TIMER4 + PPI_CH15 + PPI_CH16 + PPI_GROUP5,
);
#[cfg(context = "nrf52840")]
define_uart_drivers!(
   UARTE0 => UARTE0 + TIMER3 + PPI_CH13 + PPI_CH14 + PPI_GROUP4,
   UARTE1 => UARTE1 + TIMER4 + PPI_CH15 + PPI_CH16 + PPI_GROUP5,
);
#[cfg(context = "nrf5340")]
define_uart_drivers!(
   SERIAL3 => SERIAL3 + TIMER2 + PPI_CH18 + PPI_CH19 + PPI_GROUP5,
);
#[cfg(any(context = "nrf9151", context = "nrf9160"))]
define_uart_drivers!(
   SERIAL3 => SERIAL3 + TIMER2 + PPI_CH14 + PPI_CH15 + PPI_GROUP5,
);

#[doc(hidden)]
pub fn init(peripherals: &mut crate::OptionalPeripherals) {
    // Take all UART peripherals and do nothing with them.
    cfg_if::cfg_if! {
        if #[cfg(context = "nrf52832")] {
            let _ = peripherals.UARTE0.take().unwrap();
            let _ = peripherals.TIMER4.take().unwrap();
            let _ = peripherals.PPI_CH14.take().unwrap();
            let _ = peripherals.PPI_CH15.take().unwrap();
            let _ = peripherals.PPI_GROUP5.take().unwrap();
        } else if #[cfg(context = "nrf52833")] {
            let _ = peripherals.UARTE0.take().unwrap();
            let _ = peripherals.TIMER3.take().unwrap();
            let _ = peripherals.PPI_CH13.take().unwrap();
            let _ = peripherals.PPI_CH14.take().unwrap();
            let _ = peripherals.PPI_GROUP4.take().unwrap();

            let _ = peripherals.UARTE1.take().unwrap();
            let _ = peripherals.TIMER4.take().unwrap();
            let _ = peripherals.PPI_CH15.take().unwrap();
            let _ = peripherals.PPI_CH16.take().unwrap();
            let _ = peripherals.PPI_GROUP5.take().unwrap();
        } else if #[cfg(context = "nrf52840")] {
            let _ = peripherals.UARTE0.take().unwrap();
            let _ = peripherals.TIMER3.take().unwrap();
            let _ = peripherals.PPI_CH13.take().unwrap();
            let _ = peripherals.PPI_CH14.take().unwrap();
            let _ = peripherals.PPI_GROUP4.take().unwrap();

            let _ = peripherals.UARTE1.take().unwrap();
            let _ = peripherals.TIMER4.take().unwrap();
            let _ = peripherals.PPI_CH15.take().unwrap();
            let _ = peripherals.PPI_CH16.take().unwrap();
            let _ = peripherals.PPI_GROUP5.take().unwrap();
        } else if #[cfg(context = "nrf5340")] {
            let _ = peripherals.SERIAL3.take().unwrap();
            let _ = peripherals.TIMER2.take().unwrap();
            let _ = peripherals.PPI_CH18.take().unwrap();
            let _ = peripherals.PPI_CH19.take().unwrap();
            let _ = peripherals.PPI_GROUP5.take().unwrap();
        } else if #[cfg(any(context = "nrf9151", context = "nrf9160"))] {
            let _ = peripherals.SERIAL3.take().unwrap();
            let _ = peripherals.TIMER2.take().unwrap();
            let _ = peripherals.PPI_CH14.take().unwrap();
            let _ = peripherals.PPI_CH15.take().unwrap();
            let _ = peripherals.PPI_GROUP5.take().unwrap();
        } else {
            compile_error!("this nRF chip is not supported");
        }
    }
}
