//! Provides HAL-agnostic UART-related types.

/// Common UART baud rates.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Baud {
    /// Custom baud rate.
    Baud(u32),
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

impl From<Baud> for u32 {
    fn from(b: Baud) -> u32 {
        match b {
            Baud::Baud(b) => b,
            Baud::_2400 => 2400,
            Baud::_4800 => 4800,
            Baud::_9600 => 9600,
            Baud::_19200 => 19200,
            Baud::_38400 => 38400,
            Baud::_57600 => 57600,
            Baud::_115200 => 115_200,
        }
    }
}

impl From<u32> for Baud {
    fn from(b: u32) -> Self {
        match b {
            2400 => Baud::_2400,
            4800 => Baud::_4800,
            9600 => Baud::_9600,
            19200 => Baud::_19200,
            38400 => Baud::_38400,
            57600 => Baud::_57600,
            115_200 => Baud::_115200,
            b => Baud::Baud(b),
        }
    }
}

impl core::fmt::Display for Baud {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Into::<u32>::into(*self))
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Baud {
    fn format(&self, f: defmt::Formatter<'_>) {
        use defmt::write;
        write!(f, "{=u32}", Into::<u32>::into(*self));
    }
}

/// Parity bit.
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
                write!(
                    f,
                    "{} {}{}{}",
                    self.baudrate, self.data_bits, self.parity, self.stop_bits
                )
            }
        }
        #[cfg(feature = "defmt")]
        impl defmt::Format for Config {
            fn format(&self, f: defmt::Formatter<'_>) {
                use defmt::write;
                write!(
                    f,
                    "{} {}{}{}",
                    self.baudrate, self.data_bits, self.parity, self.stop_bits
                )
            }
        }
    };
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
