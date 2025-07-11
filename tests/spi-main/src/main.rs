//! This example is merely to illustrate and test raw bus usage.
//!
//! Please use [`ariel_os::sensors`] instead for a high-level sensor abstraction that is
//! HAL-agnostic.
//!
//! This example requires a LIS3DH sensor (3-axis accelerometer).
#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    debug::{
        ExitCode, exit,
        log::{debug, info},
    },
    gpio, hal,
    spi::{
        Mode,
        main::{Kilohertz, SpiDevice, highest_freq_in},
    },
};

use embassy_sync::mutex::Mutex;
use embedded_hal_async::spi::{Operation, SpiDevice as _};

// WHO_AM_I register of the sensor
#[cfg(not(any(context = "nordic-thingy-91-x-nrf9151", context = "st-steval-mkboxpro")))]
const WHO_AM_I_REG_ADDR: u8 = 0x0f;
#[cfg(any(context = "nordic-thingy-91-x-nrf9151", context = "st-steval-mkboxpro"))]
use pins::WHO_AM_I_REG_ADDR;

#[cfg(not(any(context = "nordic-thingy-91-x-nrf9151", context = "st-steval-mkboxpro")))]
const DEVICE_ID: u8 = 0x33;
#[cfg(any(context = "nordic-thingy-91-x-nrf9151", context = "st-steval-mkboxpro"))]
use pins::DEVICE_ID;

pub static SPI_BUS: once_cell::sync::OnceCell<
    Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, hal::spi::main::Spi>,
> = once_cell::sync::OnceCell::new();

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    let mut spi_config = hal::spi::main::Config::default();
    spi_config.frequency = const { highest_freq_in(Kilohertz::kHz(1000)..=Kilohertz::kHz(2000)) };
    debug!("Selected frequency: {:?}", spi_config.frequency);
    spi_config.mode = if !cfg!(context = "esp") {
        Mode::Mode3
    } else {
        // FIXME: the sensor datasheet does say SPI mode 3, not mode 0
        Mode::Mode0
    };

    let spi_bus = pins::SensorSpi::new(
        peripherals.spi_sck,
        peripherals.spi_miso,
        peripherals.spi_mosi,
        spi_config,
    );

    let _ = SPI_BUS.set(Mutex::new(spi_bus));

    let cs_output = gpio::Output::new(peripherals.spi_cs, gpio::Level::High);
    let mut spi_device = SpiDevice::new(SPI_BUS.get().unwrap(), cs_output);

    let mut id = [0; 2];
    spi_device
        .transaction(&mut [
            Operation::Write(&[get_spi_read_command(WHO_AM_I_REG_ADDR)]),
            Operation::TransferInPlace(&mut id),
        ])
        .await
        .unwrap();

    #[cfg(not(context = "nordic-thingy-91-x-nrf9151"))]
    let who_am_i = id[0];
    #[cfg(context = "nordic-thingy-91-x-nrf9151")]
    // Skip the leading dummy byte
    let who_am_i = id[1];
    info!("WHO_AM_I_COMMAND register value: 0x{:x}", who_am_i);
    assert_eq!(who_am_i, DEVICE_ID);

    info!("Test passed!");

    exit(ExitCode::SUCCESS);
}

fn get_spi_read_command(addr: u8) -> u8 {
    addr | 0x80
}
