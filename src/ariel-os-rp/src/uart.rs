//! UART configuration.

#![expect(unsafe_code)]

use ariel_os_embassy_common::{impl_async_uart_for_driver_enum, uart::ConfigError};

use embassy_rp::{
    Peripheral, bind_interrupts, peripherals,
    uart::{BufferedInterruptHandler, BufferedUart, RxPin, TxPin},
};

/// UART interface configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// The baud rate at which UART should operate.
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
pub struct Baudrate {
    /// The baud rate at which UART should operate.
    baud: u32,
}

impl From<Baudrate> for u32 {
    fn from(baud: Baudrate) -> u32 {
        baud.baud
    }
}

impl From<u32> for Baudrate {
    fn from(baudrate: u32) -> Baudrate {
        Baudrate { baud: baudrate }
    }
}

impl From<ariel_os_embassy_common::uart::Baudrate<Self>> for Baudrate {
    fn from(baud: ariel_os_embassy_common::uart::Baudrate<Self>) -> Baudrate {
        match baud {
            ariel_os_embassy_common::uart::Baudrate::Hal(baud) => baud,
            ariel_os_embassy_common::uart::Baudrate::_2400 => Baudrate { baud: 2400 },
            ariel_os_embassy_common::uart::Baudrate::_4800 => Baudrate { baud: 4800 },
            ariel_os_embassy_common::uart::Baudrate::_9600 => Baudrate { baud: 9600 },
            ariel_os_embassy_common::uart::Baudrate::_19200 => Baudrate { baud: 19_200 },
            ariel_os_embassy_common::uart::Baudrate::_38400 => Baudrate { baud: 38_400 },
            ariel_os_embassy_common::uart::Baudrate::_57600 => Baudrate { baud: 57_600 },
            ariel_os_embassy_common::uart::Baudrate::_115200 => Baudrate { baud: 115_200 },
        }
    }
}

/// UART number of data bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataBits {
    /// 5 bits per character.
    Data5,
    /// 6 bits per character.
    Data6,
    /// 7 bits per character.
    Data7,
    /// 8 bits per character.
    Data8,
}

fn from_data_bits(databits: DataBits) -> embassy_rp::uart::DataBits {
    match databits {
        DataBits::Data5 => embassy_rp::uart::DataBits::DataBits5,
        DataBits::Data6 => embassy_rp::uart::DataBits::DataBits6,
        DataBits::Data7 => embassy_rp::uart::DataBits::DataBits7,
        DataBits::Data8 => embassy_rp::uart::DataBits::DataBits8,
    }
}

impl From<ariel_os_embassy_common::uart::DataBits<Self>> for DataBits {
    fn from(databits: ariel_os_embassy_common::uart::DataBits<Self>) -> DataBits {
        match databits {
            ariel_os_embassy_common::uart::DataBits::Hal(bits) => bits,
            ariel_os_embassy_common::uart::DataBits::Data8 => DataBits::Data8,
        }
    }
}

/// Parity bit.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    /// No parity bit.
    None,
    /// Even parity bit.
    Even,
    /// Odd parity bit.
    Odd,
}

fn from_parity(parity: Parity) -> embassy_rp::uart::Parity {
    match parity {
        Parity::None => embassy_rp::uart::Parity::ParityNone,
        Parity::Even => embassy_rp::uart::Parity::ParityEven,
        Parity::Odd => embassy_rp::uart::Parity::ParityOdd,
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

/// UART number of stop bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopBits {
    /// One stop bit.
    Stop1,
    /// Two stop bits.
    Stop2,
}

fn from_stop_bits(stop_bits: StopBits) -> embassy_rp::uart::StopBits {
    match stop_bits {
        StopBits::Stop1 => embassy_rp::uart::StopBits::STOP1,
        StopBits::Stop2 => embassy_rp::uart::StopBits::STOP2,
    }
}

impl From<ariel_os_embassy_common::uart::StopBits<Self>> for StopBits {
    fn from(stopbits: ariel_os_embassy_common::uart::StopBits<Self>) -> Self {
        match stopbits {
            ariel_os_embassy_common::uart::StopBits::Hal(stopbits) => stopbits,
            ariel_os_embassy_common::uart::StopBits::Stop1 => StopBits::Stop1,
        }
    }
}

macro_rules! define_uart_drivers {
    ($( $interrupt:ident => $peripheral:ident ),* $(,)?) => {
        $(
            /// Peripheral-specific UART driver.
            pub struct $peripheral<'d> {
                uart: BufferedUart<'d, peripherals::$peripheral>,
            }

            // Make this struct a compile-time-enforced singleton: having multiple statics
            // defined with the same name would result in a compile-time error.
            paste::paste! {
                #[allow(dead_code)]
                static [<PREVENT_MULTIPLE_ $peripheral>]: () = ();
            }

            impl<'d> $peripheral<'d> {
                /// Returns a driver implementing embedded-io traits for this Uart
                /// peripheral.
                ///
                /// # Errors
                ///
                /// This never returns an error.
                #[expect(clippy::new_ret_no_self)]
                pub fn new(
                    rx_pin: impl Peripheral<P: RxPin<peripherals::$peripheral>> + 'd,
                    tx_pin: impl Peripheral<P: TxPin<peripherals::$peripheral>> + 'd,
                    rx_buf: &'d mut [u8],
                    tx_buf: &'d mut [u8],
                    config: Config,
                ) -> Result<Uart<'d>, ConfigError> {
                    let mut uart_config = embassy_rp::uart::Config::default();
                    uart_config.baudrate = Baudrate::from(config.baudrate).into();
                    uart_config.data_bits = from_data_bits(config.data_bits);
                    uart_config.stop_bits = from_stop_bits(config.stop_bits);
                    uart_config.parity = from_parity(config.parity);
                    bind_interrupts!(struct Irqs {
                        $interrupt => BufferedInterruptHandler<peripherals::$peripheral>;
                    });

                    // FIXME(safety): enforce that the init code indeed has run
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // peripheral multiple times.
                    let uart_peripheral = unsafe { peripherals::$peripheral::steal() };

                    let uart = BufferedUart::new(
                        uart_peripheral,
                        Irqs,
                        // Order of TX / RX is swapped compared to other platforms
                        tx_pin,
                        rx_pin,
                        tx_buf,
                        rx_buf,
                        uart_config,
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
            type Error = embassy_rp::uart::Error;
        }

        impl_async_uart_for_driver_enum!(Uart, $( $peripheral ),*);
    }
}

define_uart_drivers!(
   UART0_IRQ => UART0,
   UART1_IRQ => UART1,
);

#[doc(hidden)]
pub fn init(peripherals: &mut crate::OptionalPeripherals) {
    // Take all UART peripherals and do nothing with them.
    cfg_if::cfg_if! {
        if #[cfg(any(context = "rp2040", context = "rp235xa"))] {
            let _ = peripherals.UART0.take().unwrap();
            let _ = peripherals.UART1.take().unwrap();
        } else {
            compile_error!("this RP chip is not supported");
        }
    }
}
