//! UART configuration.

#![expect(unsafe_code)]

use ariel_os_embassy_common::{impl_async_uart_for_driver_enum, uart::ConfigError};
use embassy_stm32::{
    Peripheral, bind_interrupts, peripherals,
    usart::{BufferedInterruptHandler, BufferedUart, RxPin, TxPin},
};

/// UART interface configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    baudrate: u32,
}

impl From<Baudrate> for u32 {
    fn from(baudrate: Baudrate) -> u32 {
        baudrate.baudrate
    }
}

impl From<u32> for Baudrate {
    fn from(baudrate: u32) -> Baudrate {
        Baudrate { baudrate }
    }
}

impl From<ariel_os_embassy_common::uart::Baudrate<Self>> for Baudrate {
    fn from(baud: ariel_os_embassy_common::uart::Baudrate<Self>) -> Baudrate {
        match baud {
            ariel_os_embassy_common::uart::Baudrate::Hal(baudrate) => baudrate,
            ariel_os_embassy_common::uart::Baudrate::_2400 => Baudrate { baudrate: 2400 },
            ariel_os_embassy_common::uart::Baudrate::_4800 => Baudrate { baudrate: 4800 },
            ariel_os_embassy_common::uart::Baudrate::_9600 => Baudrate { baudrate: 9600 },
            ariel_os_embassy_common::uart::Baudrate::_19200 => Baudrate { baudrate: 19_200 },
            ariel_os_embassy_common::uart::Baudrate::_38400 => Baudrate { baudrate: 38_400 },
            ariel_os_embassy_common::uart::Baudrate::_57600 => Baudrate { baudrate: 57_600 },
            ariel_os_embassy_common::uart::Baudrate::_115200 => Baudrate { baudrate: 115_200 },
        }
    }
}

/// UART number of data bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataBits {
    /// 7 bits per character.
    Data7,
    /// 8 bits per character.
    Data8,
    /// 9 bits per character.
    Data9,
}

fn from_databits(databits: DataBits) -> embassy_stm32::usart::DataBits {
    match databits {
        DataBits::Data7 => embassy_stm32::usart::DataBits::DataBits7,
        DataBits::Data8 => embassy_stm32::usart::DataBits::DataBits8,
        DataBits::Data9 => embassy_stm32::usart::DataBits::DataBits9,
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

fn from_parity(parity: Parity) -> embassy_stm32::usart::Parity {
    match parity {
        Parity::None => embassy_stm32::usart::Parity::ParityNone,
        Parity::Even => embassy_stm32::usart::Parity::ParityEven,
        Parity::Odd => embassy_stm32::usart::Parity::ParityOdd,
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
    /// 0.5 stop bits.
    Stop0P5,
    /// Two stop bits.
    Stop2,
    /// 1.5 stop bits.
    Stop1P5,
}

fn from_stopbits(stop_bits: StopBits) -> embassy_stm32::usart::StopBits {
    match stop_bits {
        StopBits::Stop1 => embassy_stm32::usart::StopBits::STOP1,
        StopBits::Stop0P5 => embassy_stm32::usart::StopBits::STOP0P5,
        StopBits::Stop2 => embassy_stm32::usart::StopBits::STOP2,
        StopBits::Stop1P5 => embassy_stm32::usart::StopBits::STOP1P5,
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

fn convert_error(err: embassy_stm32::usart::ConfigError) -> ConfigError {
    match err {
        embassy_stm32::usart::ConfigError::BaudrateTooLow
        | embassy_stm32::usart::ConfigError::BaudrateTooHigh => ConfigError::BaudrateNotSupported,
        embassy_stm32::usart::ConfigError::DataParityNotSupported => {
            ConfigError::DataParityNotSupported
        }
        _ => ConfigError::ConfigurationNotSupported,
    }
}

fn from_config(config: &Config) -> embassy_stm32::usart::Config {
    let mut uart_config = embassy_stm32::usart::Config::default();
    uart_config.baudrate = Baudrate::from(config.baudrate).into();
    uart_config.data_bits = from_databits(config.data_bits);
    uart_config.stop_bits = from_stopbits(config.stop_bits);
    uart_config.parity = from_parity(config.parity);
    uart_config
}

macro_rules! define_uart_drivers {
    ($( $interrupt:ident => $peripheral:ident ),* $(,)?) => {
        $(
            /// Peripheral-specific UART driver.
            pub struct $peripheral<'d> {
                uart: BufferedUart<'d>,
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
                /// Returns [`ConfigError::BaudrateNotSupported`] when the baud rate cannot be
                /// applied to the peripheral.
                /// Returns [`ConfigError::DataParityNotSupported`] when the combination of data
                /// bits and parity cannot be applied to the peripheral.
                /// Returns [`ConfigError::ConfigurationNotSupported`] when the requested configuration
                /// cannot be applied to the peripheral.
                #[expect(clippy::new_ret_no_self)]
                pub fn new(
                    rx_pin: impl Peripheral<P: RxPin<peripherals::$peripheral>> + 'd,
                    tx_pin: impl Peripheral<P: TxPin<peripherals::$peripheral>> + 'd,
                    rx_buf: &'d mut [u8],
                    tx_buf: &'d mut [u8],
                    config: Config,
                ) -> Result<Uart<'d>, ConfigError> {

                    let uart_config = from_config(&config);
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
                        rx_pin,
                        tx_pin,
                        tx_buf,
                        rx_buf,
                        uart_config,
                    ).map_err(convert_error)?;

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
            type Error = embassy_stm32::usart::Error;
        }

        impl_async_uart_for_driver_enum!(Uart, $( $peripheral ),*);
    }
}

#[cfg(context = "stm32c031c6")]
define_uart_drivers!(
   USART1 => USART1,
   // USART2 => USART2, // Often used as SWI
);
#[cfg(context = "stm32f042k6")]
define_uart_drivers!(
   USART1 => USART1,
   // USART2 => USART2, // Often used as SWI
);
#[cfg(context = "stm32f401re")]
define_uart_drivers!(
   USART1 => USART1,
   // USART2 => USART2, // Often used as SWI
   USART6 => USART6,
);
#[cfg(context = "stm32f411re")]
define_uart_drivers!(
   USART1 => USART1,
   // USART2 => USART2, // Often used as SWI
   USART6 => USART6,
);
#[cfg(context = "stm32f767zi")]
define_uart_drivers!(
   USART1 => USART1,
   USART2 => USART2,
   USART3 => USART3,
   UART4 => UART4,
   // UART5 => UART5, // Often used as SWI
   USART6 => USART6,
   UART7 => UART7,
   UART8 => UART8,
);
#[cfg(any(context = "stm32h755zi", context = "stm32h753zi"))]
define_uart_drivers!(
   LPUART1 => LPUART1,
   USART1 => USART1,
   USART2 => USART2,
   USART3 => USART3,
   UART4 => UART4,
   // UART5 => UART5, // Often used as SWI
   USART6 => USART6,
   UART7 => UART7,
   UART8 => UART8,
);
#[cfg(context = "stm32l475vg")]
define_uart_drivers!(
   LPUART1 => LPUART1,
   USART1 => USART1,
   USART2 => USART2,
   USART3 => USART3,
   UART4 => UART4,
   // UART5 => UART5, // Often used as SWI
);
#[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))]
define_uart_drivers!(
   USART1 => USART1,
   USART2_LPUART2 => USART2,
   USART3_LPUART1 => USART3,
   // USART4_LPUART3 => USART4, // Often used as SWI
);
#[cfg(context = "stm32u585ai")]
define_uart_drivers!(
   LPUART1 => LPUART1,
   USART1 => USART1,
   // USART2 => USART2, // Often used as SWI
   USART3 => USART3,
   UART4 => UART4,
   UART5 => UART5,
);
#[cfg(context = "stm32wb55rg")]
define_uart_drivers!(
   LPUART1 => LPUART1,
   // USART1 => USART1, // Often used as SWI
);
#[cfg(context = "stm32wba55cg")]
define_uart_drivers!(
   LPUART1 => LPUART1,
   USART1 => USART1,
   // USART2 => USART2, // Often used as SWI
);
#[cfg(context = "stm32wle5jc")]
define_uart_drivers!(
   LPUART1 => LPUART1,
   USART1 => USART1,
   // USART2 => USART2, // Often used as SWI
);

#[doc(hidden)]
pub fn init(peripherals: &mut crate::OptionalPeripherals) {
    // Take all UART peripherals and do nothing with them.
    cfg_if::cfg_if! {
        if #[cfg(context = "stm32c031c6")] {
            let _ = peripherals.USART1.take().unwrap();
        } else if #[cfg(context = "stm32f042k6")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
        } else if #[cfg(context = "stm32f401re")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART6.take().unwrap();
        } else if #[cfg(context = "stm32f411re")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART6.take().unwrap();
        } else if #[cfg(context = "stm32f767zi")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
            let _ = peripherals.UART4.take().unwrap();
            let _ = peripherals.UART5.take().unwrap();
            let _ = peripherals.USART6.take().unwrap();
            let _ = peripherals.UART7.take().unwrap();
            let _ = peripherals.UART8.take().unwrap();
        } else if #[cfg(context = "stm32h755zi")] {
            let _ = peripherals.LPUART1.take().unwrap();
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
            let _ = peripherals.UART4.take().unwrap();
            let _ = peripherals.UART5.take().unwrap();
            let _ = peripherals.USART6.take().unwrap();
            let _ = peripherals.UART7.take().unwrap();
            let _ = peripherals.UART8.take().unwrap();
        } else if #[cfg(context = "stm32h753zi")] {
            let _ = peripherals.LPUART1.take().unwrap();
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
            let _ = peripherals.UART4.take().unwrap();
            let _ = peripherals.UART5.take().unwrap();
            let _ = peripherals.USART6.take().unwrap();
            let _ = peripherals.UART7.take().unwrap();
            let _ = peripherals.UART8.take().unwrap();
        } else if #[cfg(context = "stm32l475vg")] {
            let _ = peripherals.LPUART1.take().unwrap();
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
            let _ = peripherals.UART4.take().unwrap();
            let _ = peripherals.UART5.take().unwrap();
        } else if #[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))] {
            let _ = peripherals.LPUART1.take().unwrap();
            let _ = peripherals.LPUART2.take().unwrap();
            let _ = peripherals.LPUART3.take().unwrap();
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
            let _ = peripherals.USART4.take().unwrap();
        } else if #[cfg(context = "stm32u585ai")] {
            let _ = peripherals.LPUART1.take().unwrap();
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
            let _ = peripherals.UART4.take().unwrap();
            let _ = peripherals.UART5.take().unwrap();
        } else if #[cfg(context = "stm32wb55rg")] {
            let _ = peripherals.LPUART1.take().unwrap();
            let _ = peripherals.USART1.take().unwrap();
        } else if #[cfg(context = "stm32wba55cg")] {
            let _ = peripherals.LPUART1.take().unwrap();
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
        } else if #[cfg(context = "stm32wle5jc")] {
            let _ = peripherals.LPUART1.take().unwrap();
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
        } else {
            compile_error!("this STM32 chip is not supported");
        }
    }
}
