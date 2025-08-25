//! UART bus configuration.
use ariel_os_embassy_common::{
    impl_async_uart_for_driver_enum, impl_defmt_display_for_config,
    uart::{Baud, DataBits, Parity, StopBits},
};
use embassy_stm32::{
    Peripheral, bind_interrupts, peripherals,
    usart::{BufferedInterruptHandler, BufferedUart, RxPin, TxPin},
};

/// UART interface configuration.
#[derive(Clone)]
#[non_exhaustive]
pub struct Config {
    /// The baud rate at which the bus should operate.
    pub baudrate: Baud,
    /// Number of data bits
    pub data_bits: DataBits,
    /// Number of stop bits
    pub stop_bits: StopBits,
    /// Parity mode used for the interface.
    pub parity: Parity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baudrate: Baud::_9600,
            data_bits: DataBits::Data8,
            stop_bits: StopBits::Stop1,
            parity: Parity::None,
        }
    }
}

fn from_parity(parity: Parity) -> embassy_stm32::usart::Parity {
    match parity {
        Parity::None => embassy_stm32::usart::Parity::ParityNone,
        Parity::Even => embassy_stm32::usart::Parity::ParityEven,
        Parity::Odd => embassy_stm32::usart::Parity::ParityOdd,
    }
}

fn from_stop_bits(stop_bits: StopBits) -> embassy_stm32::usart::StopBits {
    match stop_bits {
        StopBits::Stop1 => embassy_stm32::usart::StopBits::STOP1,
        StopBits::Stop2 => embassy_stm32::usart::StopBits::STOP2,
    }
}

fn from_data_bits(data_bits: DataBits) -> embassy_stm32::usart::DataBits {
    match data_bits {
        DataBits::Data7 => embassy_stm32::usart::DataBits::DataBits7,
        DataBits::Data8 => embassy_stm32::usart::DataBits::DataBits8,
    }
}

impl_defmt_display_for_config!();

macro_rules! define_uart_drivers {
    ($( $interrupt:ident => $peripheral:ident ),* $(,)?) => {
        $(
            /// Peripheral-specific UART driver.
            pub struct $peripheral<'d> {
                uart: BufferedUart<'d>,
            }

            impl<'d> $peripheral<'d> {
                #[expect(clippy::new_ret_no_self)]
                #[must_use]
                /// Returns a driver implementing [`embedded-io`] for this Uart
                /// peripheral.
                pub fn new(
                    rx_pin: impl Peripheral<P: RxPin<peripherals::$peripheral>> + 'd,
                    tx_pin: impl Peripheral<P: TxPin<peripherals::$peripheral>> + 'd,
                    rx_buf: &'d mut [u8],
                    tx_buf: &'d mut [u8],
                    config: Config,
                ) -> Uart<'d> {
                    // Make this struct a compile-time-enforced singleton: having multiple statics
                    // defined with the same name would result in a compile-time error.
                    paste::paste! {
                        #[allow(dead_code)]
                        static [<PREVENT_MULTIPLE_ $peripheral>]: () = ();
                    }

                    let mut uart_config = embassy_stm32::usart::Config::default();
                    uart_config.baudrate = config.baudrate.into();
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
                        rx_pin,
                        tx_pin,
                        tx_buf,
                        rx_buf,
                        uart_config,
                    ).expect("Invalid config for UART");

                    Uart::$peripheral(Self { uart })
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

#[doc(hidden)]
pub fn init(peripherals: &mut crate::OptionalPeripherals) {
    // Take all SPI peripherals and do nothing with them.
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
        } else if #[cfg(context = "stm32wb55rg")] {
            let _ = peripherals.USART1.take().unwrap();
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
        } else {
            compile_error!("this STM32 chip is not supported");
        }
    }
}
