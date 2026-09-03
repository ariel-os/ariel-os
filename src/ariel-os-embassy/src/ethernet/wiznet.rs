//! Provides support for WIZnet SPI Ethernet chips (W5500, W5100S, W6100, W6300).
//!
//! This backend is HAL-agnostic: it only relies on generic SPI/GPIO abstractions, so it works
//! identically on any MCU family. Only the board-specific pin mapping needs to be added per board.

cfg_select! {
    context = "waveshare-esp32-s3-eth" => {
        #[path = "wiznet/waveshare_esp32_s3_eth.rs"]
        mod board;
    }
    // Add a `#[path = "wiznet/<board>.rs"] mod board;` arm here, gated on
    // `context = "<board>"`, for each additional board that wires up a WIZnet chip.
    context = "ariel-os" => {
        compile_error!(
            "no pin mapping for the WIZnet Ethernet driver is known for this board; add one to \
             ariel-os-embassy/src/ethernet/wiznet.rs"
        );
    }
    _ => {}
}

use embassy_executor::Spawner;
use embassy_net_wiznet::{Runner, State};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use static_cell::StaticCell;

use ariel_os_hal::gpio;

use crate::{
    hal,
    spi::main::{Kilohertz, SpiDevice, highest_freq_in},
};

/// Maximum SPI clock frequency to communicate with the WIZnet chip.
///
/// WIZnet chips are specified to support faster SPI clocks, but using the MCU's maximum
/// available frequency (as [`hal::spi::main::Config::default()`] does) has been observed to
/// corrupt reads (e.g. the chip version check failing) on at least the ESP32-S3. 20 MHz matches
/// the clock speed used by Espressif's own W5500 examples and is a safe, well-tested choice.
const MAX_WIZNET_SPI_FREQUENCY: Kilohertz = Kilohertz::MHz(20);

cfg_select! {
    feature = "wiznet-w5500" => {
        pub use embassy_net_wiznet::chip::W5500 as WiznetChip;
    }
    feature = "wiznet-w5100s" => {
        pub use embassy_net_wiznet::chip::W5100S as WiznetChip;
    }
    feature = "wiznet-w6100" => {
        pub use embassy_net_wiznet::chip::W6100 as WiznetChip;
    }
    feature = "wiznet-w6300" => {
        pub use embassy_net_wiznet::chip::W6300 as WiznetChip;
    }
    context = "ariel-os" => {
        compile_error!(
            "no WIZnet chip selected for the `ethernet-wiznet` backend; select exactly one of \
             `wiznet-w5500`, `wiznet-w5100s`, `wiznet-w6100`, `wiznet-w6300`"
        );
    }
    _ => {}
}

/// Number of packet buffers held by the driver in each direction.
const N_RX: usize = 8;
const N_TX: usize = 8;

/// The [`embassy_net`](https://docs.rs/embassy-net) driver for this Ethernet chip.
pub type NetworkDevice = embassy_net_wiznet::Device<'static>;

#[embassy_executor::task]
async fn ethernet_wiznet_task(
    runner: Runner<
        'static,
        WiznetChip,
        SpiDevice<'static>,
        gpio::IntEnabledInput<'static>,
        gpio::Output<'static>,
    >,
) -> ! {
    runner.run().await
}

/// Initializes the WIZnet Ethernet chip and returns the [`NetworkDevice`] to be used by
/// `embassy-net`.
///
/// # Panics
///
/// Panics if the chip does not respond as expected over SPI (e.g., because of a wiring issue),
/// or if launching the driver task fails.
pub(crate) async fn device(
    peripherals: &mut hal::OptionalPeripherals,
    spawner: Spawner,
) -> NetworkDevice {
    let pins = board::take_pins(peripherals);

    let mut spi_config = hal::spi::main::Config::default();
    spi_config.frequency = const { highest_freq_in(Kilohertz::kHz(1)..=MAX_WIZNET_SPI_FREQUENCY) };

    let spi = board::WiznetSpi::new(pins.spi_sck, pins.spi_miso, pins.spi_mosi, spi_config);

    static SPI_BUS: StaticCell<Mutex<CriticalSectionRawMutex, hal::spi::main::Spi>> =
        StaticCell::new();
    let spi_bus = SPI_BUS.init(Mutex::new(spi));
    let cs = gpio::Output::new(pins.cs, gpio::Level::High);
    let spi_dev = SpiDevice::new(spi_bus, cs);

    let reset = gpio::Output::new(pins.reset, gpio::Level::High);
    let int = gpio::Input::builder(pins.int, gpio::Pull::Up)
        .build_with_interrupt()
        .unwrap();

    static STATE: StaticCell<State<N_RX, N_TX>> = StaticCell::new();
    let state = STATE.init(State::new());

    let mac_addr = ariel_os_identity::interface_eui48(0)
        .expect("Should provide a valid MAC address")
        .0;

    let (device, runner) =
        embassy_net_wiznet::new::<_, _, WiznetChip, _, _, _>(mac_addr, state, spi_dev, int, reset)
            .await
            .unwrap();

    spawner.spawn(ethernet_wiznet_task(runner)).unwrap();

    device
}
