//! LoRa transmit (raw PHY) example for the ST B-L072Z-LRWAN1 board.
//!
//! The board embeds a Semtech SX1276 radio (Murata CMWX1ZZABZ module) wired to
//! `SPI1`. This example uses the HAL-agnostic [`lora_phy`] driver to transmit a
//! small LoRa packet every few seconds, carrying a message index and the uptime
//! of the sender (see [`message`]). Pair it with the `lora-rx` example running
//! on a second board, tuned to the same frequency and modulation parameters, to
//! observe the packets being received.
#![no_main]
#![no_std]
// "LoRa", "SX1276" and similar domain terms trip the doc_markdown lint.
#![allow(clippy::doc_markdown)]

mod message;
mod pins;

use ariel_os::{
    gpio::{Input, Level, Output, Pull},
    hal,
    log::{Hex, info},
    spi::main::{Kilohertz, SpiDevice, highest_freq_in},
    time::{Delay, Instant, Timer},
};
use embassy_sync::mutex::Mutex;
use lora_phy::{
    LoRa,
    iv::GenericSx127xInterfaceVariant,
    mod_params::{Bandwidth, CodingRate, SpreadingFactor},
    sx127x::{self, Sx127x, Sx1276},
};

use crate::message::Message;

/// LoRa carrier frequency. Set this to a value legal in your region (this is an
/// EU868 channel).
const LORA_FREQUENCY_IN_HZ: u32 = 868_100_000;

/// Transmit power in dBm. The module routes its antenna through the PA_BOOST pin
/// (see `tx_boost` below); 14 dBm is the EU868 limit.
const TX_POWER_DBM: i8 = 14;

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

    // SX1276 control lines. `lora-phy` drives RESET and waits on DIO0; the
    // module has no external RF switch pins to control (hence `None, None`).
    let reset = Output::new(peripherals.reset, Level::High);
    // Power the module TCXO; the radio is configured with `tcxo_used: true`.
    let _tcxo = Output::new(peripherals.tcxo, Level::High);
    let irq = Input::builder(peripherals.dio0, Pull::None)
        .build_with_interrupt()
        .unwrap();
    let iv = GenericSx127xInterfaceVariant::new(reset, irq, None, None).unwrap();

    let config = sx127x::Config {
        chip: Sx1276,
        tcxo_used: true,
        rx_boost: false,
        tx_boost: true,
    };
    let mut lora = LoRa::new(Sx127x::new(spi, iv, config), false, Delay)
        .await
        .unwrap();

    // With these parameters, a 16 bytes packet is about 47 ms on air: sending
    // one every 5 seconds stays below the 1% duty cycle allowed on the EU868
    // 868.0-868.6 MHz sub-band.
    let modulation = lora
        .create_modulation_params(
            SpreadingFactor::_7,
            Bandwidth::_125KHz,
            CodingRate::_4_5,
            LORA_FREQUENCY_IN_HZ,
        )
        .unwrap();
    let mut tx_params = lora
        .create_tx_packet_params(4, false, true, false, &modulation)
        .unwrap();

    let node_id = message::node_id();
    info!("Node ID: 0x{:x}", node_id);

    let mut index: u32 = 0;
    loop {
        let message = Message {
            node_id,
            index,
            uptime_ms: uptime_ms(),
            tx_power_dbm: TX_POWER_DBM,
        };
        let payload = message.to_bytes();

        lora.prepare_for_tx(
            &modulation,
            &mut tx_params,
            i32::from(TX_POWER_DBM),
            &payload,
        )
        .await
        .unwrap();
        lora.tx().await.unwrap();
        lora.sleep(false).await.unwrap();

        info!(
            "Sent message {} ({} bytes): {}",
            index,
            payload.len(),
            Hex(&payload)
        );
        index = index.wrapping_add(1);

        Timer::after_secs(5).await;
    }
}

/// Returns the uptime in milliseconds, wrapping after about 49 days.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the uptime sent in the messages is allowed to wrap"
)]
fn uptime_ms() -> u32 {
    Instant::now().as_millis() as u32
}
