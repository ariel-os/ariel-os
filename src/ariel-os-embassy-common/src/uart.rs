//! Provides HAL-agnostic UART-related types.

/// UART parity.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Parity {
    /// No parity bit.
    None,
    /// Even parity bit.
    Even,
    /// Odd parity bit.
    Odd,
}

impl core::fmt::Display for Parity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::None => write!(f, "N"),
            Self::Even => write!(f, "E"),
            Self::Odd => write!(f, "O"),
        }

    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Parity {
    fn format(&self, f: defmt::Formatter<'_>) {
        use defmt::write;
        match self {
            Self::None => write!(f, "N"),
            Self::Even => write!(f, "E"),
            Self::Odd => write!(f, "O"),
        }
    }
}

/// UART number of stop bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StopBits {
    /// One stop bit.
    Stop1,
    /// Two stop bits.
    Stop2,
}

impl core::fmt::Display for StopBits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Stop1 => write!(f, "1"),
            Self::Stop2 => write!(f, "2"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for StopBits {
    fn format(&self, f: defmt::Formatter<'_>) {
        use defmt::write;
        match self {
            Self::Stop1 => write!(f, "1"),
            Self::Stop2 => write!(f, "2"),
        }
    }
}

/// UART number of data bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataBits {
    /// 7 bits per character.
    Data7,
    /// 8 bits per character.
    Data8,
}

impl core::fmt::Display for DataBits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Data7 => write!(f, "7"),
            Self::Data8 => write!(f, "8"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for DataBits {
    fn format(&self, f: defmt::Formatter<'_>) {
        use defmt::write;
        match self {
            Self::Data7 => write!(f, "7"),
            Self::Data8 => write!(f, "8"),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_defmt_display_for_config {
    () => {
        impl core::fmt::Display for Config {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{} {}{}{}", self.baudrate, self.data_bits, self.parity, self.stop_bits)
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter<'_>) {
                use defmt::write;
                write!(f, "{} {}{}{}", self.baudrate, self.data_bits, self.parity, self.stop_bits)
            }
        }
    }
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
