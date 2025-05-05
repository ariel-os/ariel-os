//! UART bus configuration.
use ariel_os_embassy_common::{
    impl_async_uart_for_driver_enum,
    uart::{DataBits, Parity, StopBits},
};

use esp_hal::{
    Async,
    gpio::interconnect::{PeripheralInput, PeripheralOutput},
    peripheral::Peripheral,
    peripherals,
    uart::Uart as EspUart,
};

/// UART interface configuration.
#[derive(Clone)]
#[non_exhaustive]
pub struct Config {
    /// The baud rate at which the bus should operate.
    pub baudrate: u32,
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
            baudrate: 9600,
            data_bits: DataBits::Data8,
            stop_bits: StopBits::Stop1,
            parity: Parity::None,
        }
    }
}

fn from_parity(parity: Parity) -> esp_hal::uart::Parity {
    match parity {
        Parity::None => esp_hal::uart::Parity::None,
        Parity::Even => esp_hal::uart::Parity::Even,
        Parity::Odd => esp_hal::uart::Parity::Odd,
    }
}

fn from_stop_bits(stop_bits: StopBits) -> esp_hal::uart::StopBits {
    match stop_bits {
        StopBits::Stop1 => esp_hal::uart::StopBits::_1,
        StopBits::Stop2 => esp_hal::uart::StopBits::_2,
    }
}

fn from_data_bits(data_bits: DataBits) -> esp_hal::uart::DataBits {
    match data_bits {
        DataBits::Data7 => esp_hal::uart::DataBits::_7,
        DataBits::Data8 => esp_hal::uart::DataBits::_8,
    }
}

macro_rules! define_uart_drivers {
    ($( $peripheral:ident ),* $(,)?) => {
        $(
            /// Peripheral-specific UART driver.
            pub struct $peripheral<'d> {
                uart: EspUart<'d, Async>
            }

            impl<'d> $peripheral<'d> {
                #[expect(clippy::new_ret_no_self)]
                #[must_use]
                /// Returns a driver implementing [`embedded-io`] for this Uart
                /// peripheral.
                pub fn new(
                    rx_pin: impl Peripheral<P: PeripheralInput> + 'd,
                    tx_pin: impl Peripheral<P: PeripheralOutput> + 'd,
                    _rx_buf: &'d mut [u8],
                    _tx_buf: &'d mut [u8],
                    config: Config,
                ) -> Uart<'d> {
                    let uart_config = esp_hal::uart::Config::default()
                        .with_baudrate(config.baudrate)
                        .with_data_bits(from_data_bits(config.data_bits))
                        .with_stop_bits(from_stop_bits(config.stop_bits))
                        .with_parity(from_parity(config.parity));
                    // Make this struct a compile-time-enforced singleton: having multiple statics
                    // defined with the same name would result in a compile-time error.
                    paste::paste! {
                        #[allow(dead_code)]
                        static [<PREVENT_MULTIPLE_ $peripheral>]: () = ();
                    }

                    // FIXME(safety): enforce that the init code indeed has run
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // peripheral multiple times.
                    let uart_peripheral = unsafe { peripherals::$peripheral::steal() };

                    let uart = EspUart::new(
                        uart_peripheral,
                        uart_config
                    )
                        .expect("Invalid uart configuration")
                        .with_tx(tx_pin)
                        .with_rx(rx_pin)
                        .into_async();

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
            type Error = esp_hal::uart::Error;
        }

        impl_async_uart_for_driver_enum!(Uart, $( $peripheral ),*);
    }
}

#[cfg(context = "esp32c3")]
define_uart_drivers!(UART0);
#[cfg(context = "esp32c6")]
define_uart_drivers!(UART0, UART1);
