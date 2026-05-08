#![no_main]
#![no_std]
#![expect(unsafe_code)]

use ariel_os::{
    gpio::{Level, Output},
    hal::peripherals,
    log::*,
    thread::{self, CoreAffinity, CoreId},
    time::Timer,
};
use esp_hal::{
    dma::{DmaDescriptor, DmaRxBuf},
    spi::{
        Mode,
        master::{Config, Spi as SpiMaster},
        slave::Spi as SpiSlave,
    },
};

const TAMANHO_PAYLOAD: usize = 32;

#[ariel_os::thread(autostart, affinity = CoreAffinity::one(CoreId::new(0)))]
fn spi_master_thread() {
    info!("SPI Master iniciando no Core 0");

    let spi2 = unsafe { peripherals::SPI2::steal() };
    let sck = unsafe { peripherals::GPIO14::steal() };
    let mosi = unsafe { peripherals::GPIO13::steal() };
    let cs_pin = unsafe { peripherals::GPIO5::steal() };

    let mut spi = SpiMaster::new(
        spi2,
        Config::default()
            .with_frequency(esp_hal::time::Rate::from_khz(100))
            .with_mode(Mode::_1),
    )
    .unwrap()
    .with_sck(sck)
    .with_mosi(mosi);

    let mut cs = Output::new(cs_pin, Level::High);

    thread::block_on(Timer::after_secs(2));

    let mensagem: &[u8] = b"SPI loopback: Hello from Core 0!\n";
    let mut offset = 0;

    loop {
        let fim = core::cmp::min(offset + TAMANHO_PAYLOAD, mensagem.len());
        let dados = &mensagem[offset..fim];

        cs.set_low();
        let resultado = spi.write(dados);
        cs.set_high();

        match resultado {
            Ok(()) => info!("TX enviou {} bytes: {:?}", dados.len(), dados),
            Err(e) => error!("TX erro: {:?}", e),
        }

        offset = fim;
        if offset >= mensagem.len() {
            offset = 0;
            thread::block_on(Timer::after_secs(1));
        } else {
            thread::block_on(Timer::after_millis(100));
        }
    }
}

const RX_BUFFER_WORDS: usize = TAMANHO_PAYLOAD.div_ceil(4);
const RX_DESC_COUNT: usize = 1;

static mut SLAVE_RX_BUFFER: [u32; RX_BUFFER_WORDS] = [0u32; RX_BUFFER_WORDS];
static mut SLAVE_RX_DESCRIPTORS: [DmaDescriptor; RX_DESC_COUNT] =
    [DmaDescriptor::EMPTY; RX_DESC_COUNT];

#[ariel_os::thread(autostart, affinity = CoreAffinity::one(CoreId::new(1)))]
fn spi_slave_thread() {
    info!("SPI Slave iniciando no Core 1");

    let spi3 = unsafe { peripherals::SPI3::steal() };
    let dma_ch = unsafe { peripherals::DMA_SPI3::steal() };
    let sck = unsafe { peripherals::GPIO14::steal() };
    let mosi = unsafe { peripherals::GPIO23::steal() };
    let cs = unsafe { peripherals::GPIO5::steal() };

    let mut spi = SpiSlave::new(spi3, Mode::_1)
        .with_sck(sck)
        .with_mosi(mosi)
        .with_cs(cs)
        .with_dma(dma_ch);

    let rx_buf: &'static mut [u8] =
        unsafe { &mut *(&raw mut SLAVE_RX_BUFFER as *mut _ as *mut [u8; TAMANHO_PAYLOAD]) };
    let rx_desc: &'static mut [DmaDescriptor] = unsafe { &mut *(&raw mut SLAVE_RX_DESCRIPTORS) };

    let mut dma_rx_buf = DmaRxBuf::new(rx_desc, rx_buf).unwrap();

    info!("SPI Slave DMA pronto, aguardando transacoes...");

    loop {
        dma_rx_buf.as_mut_slice().fill(0);

        let transfer = match spi.read(TAMANHO_PAYLOAD, dma_rx_buf) {
            Ok(t) => t,
            Err((e, spi_dev, buf)) => {
                error!("Slave read erro: {:?}, rearmando...", e);
                spi = spi_dev;
                dma_rx_buf = buf;
                continue;
            }
        };

        let (spi_dev, rx_recuperado) = transfer.wait();
        spi = spi_dev;
        dma_rx_buf = rx_recuperado;

        let recebido = dma_rx_buf.as_slice();
        let n = recebido
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(recebido.len());
        info!("RX recebeu {} bytes: {:?}", n, &recebido[..n]);
    }
}
