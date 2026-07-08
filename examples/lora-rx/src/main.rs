//! LoRa receive (raw PHY) example for the ST B-L072Z-LRWAN1 board.
//!
//! The board embeds a Semtech SX1276 radio (Murata CMWX1ZZABZ module) wired to
//! `SPI1`. This example uses the HAL-agnostic [`lora_phy`] driver to listen for
//! LoRa packets continuously and log each one it receives. It is the companion
//! of the `lora-tx` example: run `lora-tx` on a second board tuned to the same
//! frequency and modulation parameters to see packets arrive here.
#![no_main]
#![no_std]
// "LoRa", "SX1276" and similar domain terms trip the doc_markdown lint.
#![allow(clippy::doc_markdown)]

mod pins;

use ariel_os::{
    gpio::{Input, Level, Output, Pull},
    hal,
    log::info,
    spi::main::{Kilohertz, SpiDevice, highest_freq_in},
    time::Delay,
};
use embassy_sync::mutex::Mutex;
use lora_phy::{
    LoRa, RxMode,
    iv::GenericSx127xInterfaceVariant,
    mod_params::{Bandwidth, CodingRate, SpreadingFactor},
    sx127x::{self, Sx127x, Sx1276},
};

/// LoRa carrier frequency. Must match the transmitter (this is an EU868
/// channel).
const LORA_FREQUENCY_IN_HZ: u32 = 868_100_000;

/// Largest payload the receiver will accept, in bytes.
const MAX_PAYLOAD_LEN: u8 = 64;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    // Set up the SPI1 bus. The `SpiDevice` wrapper needs the bus behind a mutex.
    let mut spi_config = hal::spi::main::Config::default();
    spi_config.frequency = const { highest_freq_in(Kilohertz::kHz(200)..=Kilohertz::kHz(1000)) };
    let spi_bus = pins::RadioSpi::new(
        peripherals.spi_sck,
        peripherals.spi_miso,
        peripherals.spi_mosi,
        spi_config,
    );
    let spi_bus = Mutex::new(spi_bus);

    let nss = Output::new(peripherals.spi_cs, Level::High);
    let spi = SpiDevice::new(&spi_bus, nss);

    // SX1276 control lines (see the lora-tx example for details).
    let reset = Output::new(peripherals.reset, Level::High);
    let _tcxo = Output::new(peripherals.tcxo, Level::High);
    let irq = Input::builder(peripherals.dio0, Pull::None)
        .build_with_interrupt()
        .unwrap();
    let iv = GenericSx127xInterfaceVariant::new(reset, irq, None, None).unwrap();

    let config = sx127x::Config {
        chip: Sx1276,
        tcxo_used: true,
        rx_boost: false,
        tx_boost: false,
    };
    let mut lora = LoRa::new(Sx127x::new(spi, iv, config), false, Delay)
        .await
        .unwrap();

    // These must match the transmitter's parameters exactly.
    let modulation = lora
        .create_modulation_params(
            SpreadingFactor::_10,
            Bandwidth::_125KHz,
            CodingRate::_4_8,
            LORA_FREQUENCY_IN_HZ,
        )
        .unwrap();
    let rx_params = lora
        .create_rx_packet_params(4, false, MAX_PAYLOAD_LEN, true, false, &modulation)
        .unwrap();

    lora.prepare_for_rx(RxMode::Continuous, &modulation, &rx_params)
        .await
        .unwrap();

    info!("Listening for LoRa packets...");
    let mut buffer = [0u8; MAX_PAYLOAD_LEN as usize];
    loop {
        if let Ok((len, status)) = lora.rx(&rx_params, &mut buffer).await {
            info!(
                "Received {} bytes (rssi = {} dBm, snr = {})",
                len, status.rssi, status.snr
            );
        } else {
            info!("Receive error");
        }
    }
}
