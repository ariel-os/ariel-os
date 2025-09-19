//! UART bus configuration.
use ariel_os_embassy_common::{
    impl_async_uart_for_driver_enum,
    uart::{Baud, Parity},
};
use embassy_nrf::{
    Peripheral, bind_interrupts,
    buffered_uarte::{Baudrate, BufferedUarte, InterruptHandler},
    gpio::Pin as GpioPin,
    peripherals,
};

/// Converts a Parity into nRF UART specific settings.
///
/// # Panics
///
/// Panics when parity setting is not supported by the nRF.
fn from_parity(parity: Parity) -> embassy_nrf::uarte::Parity {
    match parity {
        Parity::None => embassy_nrf::uarte::Parity::EXCLUDED,
        Parity::Even => embassy_nrf::uarte::Parity::INCLUDED,
        Parity::Odd => panic!("Odd parity not supported"),
    }
}

/// Converts a baud into nRF UART specific settings.
///
/// # Panics
///
/// Panics when the baud rate setting is not supported by the nRF.
fn from_baudrate(baudrate: u32) -> Baudrate {
    match baudrate {
        1200 => Baudrate::BAUD1200,
        2400 => Baudrate::BAUD2400,
        4800 => Baudrate::BAUD4800,
        9600 => Baudrate::BAUD9600,
        14_400 => Baudrate::BAUD14400,
        19_200 => Baudrate::BAUD19200,
        28_800 => Baudrate::BAUD28800,
        31_250 => Baudrate::BAUD31250,
        38_400 => Baudrate::BAUD38400,
        56_000 => Baudrate::BAUD56000,
        57_600 => Baudrate::BAUD57600,
        76_800 => Baudrate::BAUD76800,
        115_200 => Baudrate::BAUD115200,
        230_400 => Baudrate::BAUD230400,
        250_000 => Baudrate::BAUD250000,
        460_800 => Baudrate::BAUD460800,
        921_600 => Baudrate::BAUD921600,
        1_000_000 => Baudrate::BAUD1M,
        _ => panic!("Baud rate not supported"),
    }
}

/// UART interface configuration.
#[derive(Clone)]
#[non_exhaustive]
pub struct Config {
    /// The baud rate at which UART should operate.
    pub baudrate: Baud,
    /// Parity mode used for the interface.
    pub parity: Parity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baudrate: Baud::_9600,
            parity: Parity::None,
        }
    }
}

impl core::fmt::Display for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} 8{}1", self.baudrate, self.parity)
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter<'_>) {
        use defmt::write;
        write!(f, "{} 8{}1", self.baudrate, self.parity)
    }
}

macro_rules! define_uart_drivers {
    ($( $interrupt:ident => $peripheral:ident + $timer:ident + $ppi_ch1:ident + $ppi_ch2:ident + $ppi_group:ident),* $(,)?) => {
        $(
            /// Peripheral-specific UART driver.
            pub struct $peripheral<'d> {
                uart: BufferedUarte<'d, peripherals::$peripheral, peripherals::$timer>,
            }

            // Make this struct a compile-time-enforced singleton: having multiple statics
            // defined with the same name would result in a compile-time error.
            paste::paste! {
                #[allow(dead_code)]
                static [<PREVENT_MULTIPLE_ $peripheral>]: () = ();
            }

            impl<'d> $peripheral<'d> {
                /// Returns a driver implementing [`embedded-io`] for this Uart
                /// peripheral.
                #[expect(clippy::new_ret_no_self)]
                #[must_use]
                pub fn new(
                    rx_pin: impl Peripheral<P: GpioPin> + 'd,
                    tx_pin: impl Peripheral<P: GpioPin> + 'd,
                    rx_buffer: &'d mut [u8],
                    tx_buffer: &'d mut [u8],
                    config: Config,
                ) -> Uart<'d> {

                    let mut uart_config = embassy_nrf::uarte::Config::default();
                    let baudrate: u32 = config.baudrate.into();
                    uart_config.baudrate = from_baudrate(baudrate);
                    uart_config.parity = from_parity(config.parity);

                    bind_interrupts!(struct Irqs {
                        $interrupt => InterruptHandler<peripherals::$peripheral>;
                    });

                    // FIXME(safety): enforce that the init code indeed has run
                    // SAFETY: this struct being a singleton prevents us from stealing the
                    // peripheral multiple times.
                    let uart_peripheral = unsafe { peripherals::$peripheral::steal() };
                    let timer_peripheral = unsafe { peripherals::$timer::steal() };
                    let ppi_ch1_peripheral = unsafe { peripherals::$ppi_ch1::steal() };
                    let ppi_ch2_peripheral = unsafe { peripherals::$ppi_ch2::steal() };
                    let ppi_group_peripheral = unsafe { peripherals::$ppi_group::steal() };

                    let mut uart = BufferedUarte::new(
                        uart_peripheral,
                        timer_peripheral,
                        ppi_ch1_peripheral,
                        ppi_ch2_peripheral,
                        ppi_group_peripheral,
                        Irqs,
                        rx_pin,
                        tx_pin,
                        uart_config,
                        rx_buffer,
                        tx_buffer
                    );
                    uart.set_baudrate(baudrate.into());

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
            type Error = embassy_nrf::buffered_uarte::Error;
        }

        impl_async_uart_for_driver_enum!(Uart, $( $peripheral ),*);
    }
}

// Define a driver per peripheral
#[cfg(context = "nrf52832")]
define_uart_drivers!(
   UARTE0 => UARTE0 + TIMER4 + PPI_CH18 + PPI_CH19 + PPI_GROUP5,
);
#[cfg(context = "nrf52833")]
define_uart_drivers!(
   UARTE0 => UARTE0 + TIMER3 + PPI_CH16 + PPI_CH17 + PPI_GROUP4,
   UARTE1 => UARTE1 + TIMER4 + PPI_CH18 + PPI_CH19 + PPI_GROUP5,
);
#[cfg(context = "nrf52840")]
define_uart_drivers!(
   UARTE0 => UARTE0 + TIMER3 + PPI_CH16 + PPI_CH17 + PPI_GROUP4,
   UARTE1 => UARTE1 + TIMER4 + PPI_CH18 + PPI_CH19 + PPI_GROUP5,
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
            let _ = peripherals.PPI_CH18.take().unwrap();
            let _ = peripherals.PPI_CH19.take().unwrap();
            let _ = peripherals.PPI_GROUP5.take().unwrap();
        } else if #[cfg(context = "nrf52833")] {
            let _ = peripherals.UARTE0.take().unwrap();
            let _ = peripherals.TIMER3.take().unwrap();
            let _ = peripherals.PPI_CH16.take().unwrap();
            let _ = peripherals.PPI_CH17.take().unwrap();
            let _ = peripherals.PPI_GROUP4.take().unwrap();

            let _ = peripherals.UARTE1.take().unwrap();
            let _ = peripherals.TIMER4.take().unwrap();
            let _ = peripherals.PPI_CH18.take().unwrap();
            let _ = peripherals.PPI_CH19.take().unwrap();
            let _ = peripherals.PPI_GROUP5.take().unwrap();
        } else if #[cfg(context = "nrf52840")] {
            let _ = peripherals.UARTE0.take().unwrap();
            let _ = peripherals.TIMER3.take().unwrap();
            let _ = peripherals.PPI_CH16.take().unwrap();
            let _ = peripherals.PPI_CH17.take().unwrap();
            let _ = peripherals.PPI_GROUP4.take().unwrap();

            let _ = peripherals.UARTE1.take().unwrap();
            let _ = peripherals.TIMER4.take().unwrap();
            let _ = peripherals.PPI_CH18.take().unwrap();
            let _ = peripherals.PPI_CH19.take().unwrap();
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
