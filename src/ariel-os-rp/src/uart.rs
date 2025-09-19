//! UART bus configuration.
use ariel_os_embassy_common::{
    impl_async_uart_for_driver_enum, impl_defmt_display_for_config,
    uart::{Baud, DataBits, Parity, StopBits},
};

use embassy_rp::{
    Peripheral, bind_interrupts, peripherals,
    uart::{BufferedInterruptHandler, BufferedUart, RxPin, TxPin},
};

/// UART interface configuration.
#[derive(Clone)]
#[non_exhaustive]
pub struct Config {
    /// The baud rate at which UART should operate.
    pub baudrate: Baud,
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
            baudrate: Baud::_9600,
            data_bits: DataBits::Data8,
            stop_bits: StopBits::Stop1,
            parity: Parity::None,
        }
    }
}

fn from_parity(parity: Parity) -> embassy_rp::uart::Parity {
    match parity {
        Parity::None => embassy_rp::uart::Parity::ParityNone,
        Parity::Even => embassy_rp::uart::Parity::ParityEven,
        Parity::Odd => embassy_rp::uart::Parity::ParityOdd,
    }
}

fn from_stop_bits(stop_bits: StopBits) -> embassy_rp::uart::StopBits {
    match stop_bits {
        StopBits::Stop1 => embassy_rp::uart::StopBits::STOP1,
        StopBits::Stop2 => embassy_rp::uart::StopBits::STOP2,
    }
}

fn from_data_bits(data_bits: DataBits) -> embassy_rp::uart::DataBits {
    match data_bits {
        DataBits::Data7 => embassy_rp::uart::DataBits::DataBits7,
        DataBits::Data8 => embassy_rp::uart::DataBits::DataBits8,
    }
}

impl_defmt_display_for_config!();

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
                #[expect(clippy::new_ret_no_self)]
                #[must_use]
                pub fn new(
                    rx_pin: impl Peripheral<P: RxPin<peripherals::$peripheral>> + 'd,
                    tx_pin: impl Peripheral<P: TxPin<peripherals::$peripheral>> + 'd,
                    rx_buf: &'d mut [u8],
                    tx_buf: &'d mut [u8],
                    config: Config,
                ) -> Uart<'d> {
                    let mut uart_config = embassy_rp::uart::Config::default();
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
                        // Order of TX / RX is swapped compared to other platforms
                        tx_pin,
                        rx_pin,
                        tx_buf,
                        rx_buf,
                        uart_config,
                    );

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
