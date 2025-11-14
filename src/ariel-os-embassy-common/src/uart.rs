//! Provides HAL-agnostic UART-related types.

/// UART configuration error.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    /// The requested baud rate is not supported.
    BaudrateNotSupported,
    /// Data bits and parity combination not supported.
    DataParityNotSupported,
    /// Peripheral-specific error.
    ConfigurationNotSupported,
}

/// Common UART baud rates.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Baudrate<A> {
    /// HAL-specific baud rate.
    Hal(A),
    /// 2400 bauds.
    _2400,
    /// 4800 bauds.
    _4800,
    /// 9600 bauds.
    _9600,
    /// 19200 bauds.
    _19200,
    /// 38400 bauds.
    _38400,
    /// 57600 bauds.
    _57600,
    /// 115200 bauds.
    _115200,
}

impl<A> From<Baudrate<A>> for u32
where
    u32: From<A>,
{
    fn from(b: Baudrate<A>) -> u32 {
        match b {
            Baudrate::Hal(hal) => hal.into(),
            Baudrate::_2400 => 2400,
            Baudrate::_4800 => 4800,
            Baudrate::_9600 => 9600,
            Baudrate::_19200 => 19200,
            Baudrate::_38400 => 38400,
            Baudrate::_57600 => 57600,
            Baudrate::_115200 => 115_200,
        }
    }
}

/// Parity bit.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity<A> {
    /// HAL-specific parity configuration.
    Hal(A),
    /// No parity bit.
    None,
    /// Even parity bit.
    Even,
}

/// UART number of stop bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopBits<A> {
    /// HAL-specific stop bit configuration.
    Hal(A),
    /// One stop bit.
    Stop1,
}

/// UART number of data bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataBits<A> {
    /// HAL-specific number of data bits per character.
    Hal(A),
    /// 8 bits per character.
    Data8,
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_async_uart_bufread_for_driver_enum {
    ($driver_enum:ident, $( $peripheral:ident ),*) => {
        impl embedded_io_async::BufRead for $driver_enum<'_> {
            async fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
                match self {
                    $( Self::$peripheral(uart) => embedded_io_async::BufRead::fill_buf(&mut uart.uart).await, )*
                }
            }

            fn consume(&mut self, amt: usize) {
                match self {
                    $( Self::$peripheral(uart) => embedded_io_async::BufRead::consume(&mut uart.uart, amt), )*
                }
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_async_uart_for_driver_enum {
    ($driver_enum:ident, $( $peripheral:ident ),*) => {
        impl embedded_io_async::Read for $driver_enum<'_> {
            async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
                match self {
                    $( Self::$peripheral(uart) => embedded_io_async::Read::read(&mut uart.uart, buf).await, )*
                }
            }
        }


        impl embedded_io_async::ReadReady for $driver_enum<'_> {
            fn read_ready(&mut self) -> Result<bool, Self::Error> {
                match self {
                    $( Self::$peripheral(uart) => embedded_io_async::ReadReady::read_ready(&mut uart.uart), )*
                }
            }
        }

        impl embedded_io_async::Write for $driver_enum<'_> {
            async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
                match self {
                    $( Self::$peripheral(uart) => embedded_io_async::Write::write(&mut uart.uart, buf).await, )*
                }
            }
            async fn flush(&mut self) -> Result<(), Self::Error> {
                match self {
                    $( Self::$peripheral(uart) => embedded_io_async::Write::flush(&mut uart.uart).await, )*
                }
            }
            async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
                match self {
                    $( Self::$peripheral(uart) => embedded_io_async::Write::write_all(&mut uart.uart, buf).await, )*
                }
            }
        }
    }
}
