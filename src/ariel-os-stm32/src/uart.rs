//! UART bus configuration.
use ariel_os_embassy_common::{
    impl_async_uart_for_driver_enum, impl_defmt_display_for_config,
    uart::{HalBaudRate, HalDataBits, HalParity, HalStopBits},
};
use embassy_stm32::{
    Peripheral, bind_interrupts, peripherals,
    usart::{BufferedInterruptHandler, BufferedUart, RxPin, TxPin},
};

/// UART interface configuration.
#[derive(Clone)]
#[non_exhaustive]
pub struct Config {
    /// The baud rate at which UART should operate.
    pub baudrate: ariel_os_embassy_common::uart::Baud<Baud>,
    /// Number of data bits.
    pub data_bits: ariel_os_embassy_common::uart::DataBits<DataBits>,
    /// Number of stop bits.
    pub stop_bits: ariel_os_embassy_common::uart::StopBits<StopBits>,
    /// Parity mode used for the interface.
    pub parity: ariel_os_embassy_common::uart::Parity<Parity>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baudrate: ariel_os_embassy_common::uart::Baud::_9600,
            data_bits: ariel_os_embassy_common::uart::DataBits::Data8,
            stop_bits: ariel_os_embassy_common::uart::StopBits::Stop1,
            parity: ariel_os_embassy_common::uart::Parity::None,
        }
    }
}

/// UART baud rate.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Baud {
    /// The baud rate at which UART should operate.
    baud: u32,
}

impl HalBaudRate for Baud {}

impl core::fmt::Display for Baud {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.baud)
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Baud {
    fn format(&self, f: defmt::Formatter<'_>) {
        use defmt::write;
        write!(f, "{}", self.baud)
    }
}

impl From<Baud> for u32 {
    fn from(baud: Baud) -> u32 {
        baud.baud
    }
}

impl From<ariel_os_embassy_common::uart::Baud<Self>> for Baud {
    fn from(baud: ariel_os_embassy_common::uart::Baud<Self>) -> Baud {
        match baud {
            ariel_os_embassy_common::uart::Baud::Hal(baud) => baud,
            ariel_os_embassy_common::uart::Baud::_2400 => Baud { baud: 2400 },
            ariel_os_embassy_common::uart::Baud::_4800 => Baud { baud: 4800 },
            ariel_os_embassy_common::uart::Baud::_9600 => Baud { baud: 9600 },
            ariel_os_embassy_common::uart::Baud::_19200 => Baud { baud: 19200 },
            ariel_os_embassy_common::uart::Baud::_38400 => Baud { baud: 38400 },
            ariel_os_embassy_common::uart::Baud::_57600 => Baud { baud: 57600 },
            ariel_os_embassy_common::uart::Baud::_115200 => Baud { baud: 115200 },
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
    /// 9 bits per character.
    Data9,
}

impl HalDataBits for DataBits {}

impl From<DataBits> for embassy_stm32::usart::DataBits {
    fn from(databits: DataBits) -> embassy_stm32::usart::DataBits {
        match databits {
            DataBits::Data7 => embassy_stm32::usart::DataBits::DataBits7,
            DataBits::Data8 => embassy_stm32::usart::DataBits::DataBits8,
            DataBits::Data9 => embassy_stm32::usart::DataBits::DataBits9,
        }
    }
}

impl From<ariel_os_embassy_common::uart::DataBits<Self>> for DataBits {
    fn from(databits: ariel_os_embassy_common::uart::DataBits<Self>) -> DataBits {
        match databits {
            ariel_os_embassy_common::uart::DataBits::Hal(bits) => bits,
            ariel_os_embassy_common::uart::DataBits::Data7 => DataBits::Data7,
            ariel_os_embassy_common::uart::DataBits::Data8 => DataBits::Data8,
        }
    }
}

impl core::fmt::Display for DataBits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Data7 => write!(f, "7"),
            Self::Data8 => write!(f, "8"),
            Self::Data9 => write!(f, "9"),
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
            Self::Data9 => write!(f, "9"),
        }
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

impl HalParity for Parity {}

impl From<Parity> for embassy_stm32::usart::Parity {
    fn from(parity: Parity) -> embassy_stm32::usart::Parity {
        match parity {
            Parity::None => embassy_stm32::usart::Parity::ParityNone,
            Parity::Even => embassy_stm32::usart::Parity::ParityEven,
            Parity::Odd => embassy_stm32::usart::Parity::ParityOdd,
        }
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StopBits {
    /// One stop bit.
    Stop1,
    /// 0.5 stop bits.
    Stop0P5,
    /// Two stop bit.
    Stop2,
    /// 1.5 stop bit.
    Stop1P5,
}

impl HalStopBits for StopBits {}

impl core::fmt::Display for StopBits {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StopBits::Stop1 => write!(f, "1"),
            StopBits::Stop0P5 => write!(f, "0.5"),
            StopBits::Stop2 => write!(f, "2"),
            StopBits::Stop1P5 => write!(f, "1.5"),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for StopBits {
    fn format(&self, f: defmt::Formatter<'_>) {
        use defmt::write;
        match self {
            StopBits::Stop1 => write!(f, "1"),
            StopBits::Stop0P5 => write!(f, "0.5"),
            StopBits::Stop2 => write!(f, "2"),
            StopBits::Stop1P5 => write!(f, "1.5"),
        }
    }
}

impl From<StopBits> for embassy_stm32::usart::StopBits {
    fn from(stop_bits: StopBits) -> embassy_stm32::usart::StopBits {
        match stop_bits {
            StopBits::Stop1 => embassy_stm32::usart::StopBits::STOP1,
            StopBits::Stop0P5 => embassy_stm32::usart::StopBits::STOP0P5,
            StopBits::Stop2 => embassy_stm32::usart::StopBits::STOP2,
            StopBits::Stop1P5 => embassy_stm32::usart::StopBits::STOP1P5,
        }
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

impl_defmt_display_for_config!();

fn convert_error(
    err: embassy_stm32::usart::ConfigError,
) -> ariel_os_embassy_common::uart::ConfigError {
    match err {
        embassy_stm32::usart::ConfigError::BaudrateTooLow => ConfigError::BaudrateNotSupported,
        embassy_stm32::usart::ConfigError::BaudrateTooHigh => ConfigError::BaudrateNotSupported,
        embassy_stm32::usart::ConfigError::DataParityNotSupported => {
            ConfigError::DataParityNotSupported
        }
        _ => ConfigError::ConfigurationNotSupported,
    }
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
                #[expect(clippy::new_ret_no_self)]
                #[must_use]
                pub fn new(
                    rx_pin: impl Peripheral<P: RxPin<peripherals::$peripheral>> + 'd,
                    tx_pin: impl Peripheral<P: TxPin<peripherals::$peripheral>> + 'd,
                    rx_buf: &'d mut [u8],
                    tx_buf: &'d mut [u8],
                    config: Config,
                ) -> Result<Uart<'d>, ConfigError> {

                    let mut uart_config = embassy_stm32::usart::Config::default();
                    uart_config.baudrate = Baud::from(config.baudrate).into();
                    uart_config.data_bits = DataBits::from(config.data_bits).into();
                    uart_config.stop_bits = StopBits::from(config.stop_bits).into();
                    uart_config.parity = Parity::from(config.parity).into();
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
#[cfg(context = "stm32h755zi")]
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
#[cfg(context = "stm32l475vg")]
define_uart_drivers!(
   USART1 => USART1,
   USART2 => USART2,
   USART3 => USART3,
   UART4 => UART4,
   // UART5 => UART5, // Often used as SWI
);
#[cfg(context = "stm32u083mc")]
define_uart_drivers!(
   USART1 => USART1,
   USART2 => USART2,
   USART3 => USART3,
   USART4 => USART4,
);
#[cfg(context = "stm32u585ai")]
define_uart_drivers!(
   USART1 => USART1,
   // USART2 => USART2, // Often used as SWI
   USART3 => USART3,
   UART4 => UART4,
   UART5 => UART5,
);
#[cfg(context = "stm32wb55rg")]
define_uart_drivers!(
   USART1 => USART1,
);
#[cfg(context = "stm32wba55cg")]
define_uart_drivers!(
   USART1 => USART1,
   // USART2 => USART2, // Often used as SWI
   LPUART1 => LPUART1,
);
#[cfg(context = "stm32wle5jc")]
define_uart_drivers!(
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
        } else if #[cfg(context = "stm32h755zi")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
            let _ = peripherals.UART4.take().unwrap();
            let _ = peripherals.UART5.take().unwrap();
            let _ = peripherals.USART6.take().unwrap();
            let _ = peripherals.UART7.take().unwrap();
            let _ = peripherals.UART8.take().unwrap();
        } else if #[cfg(context = "stm32l475vg")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
            let _ = peripherals.UART4.take().unwrap();
            let _ = peripherals.UART5.take().unwrap();
        } else if #[cfg(context = "stm32u083mc")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
            let _ = peripherals.USART4.take().unwrap();
        } else if #[cfg(context = "stm32u585ai")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.USART3.take().unwrap();
        } else if #[cfg(context = "stm32wb55rg")] {
            let _ = peripherals.USART1.take().unwrap();
        } else if #[cfg(context = "stm32wba55cg")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
            let _ = peripherals.LPUART1.take().unwrap();
        } else if #[cfg(context = "stm32wle5jc")] {
            let _ = peripherals.USART1.take().unwrap();
            let _ = peripherals.USART2.take().unwrap();
        } else {
            compile_error!("this STM32 chip is not supported");
        }
    }
}
