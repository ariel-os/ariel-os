use embassy_executor::Spawner;
use embassy_nrf::{
    peripherals::{self, RNG},
    rng,
};
use embassy_sync::once_lock::OnceLock;
use nrf_sdc::{
    self as sdc, SoftdeviceController,
    mpsl::{self, MultiprotocolServiceLayer},
};
use static_cell::StaticCell;
use trouble_host::prelude::*;

use ariel_os_debug::log::debug;

use crate::irqs::Irqs;

pub static STACK: OnceLock<Stack<'static, SoftdeviceController<'static>>> = OnceLock::new();

const SDC_MEM_SIZE: usize = 4096;
const MAX_CONNS: usize = 1;
const MAX_CHANNELS: usize = 1;
const L2CAP_MTU: usize = 27;
#[cfg(feature = "ble-central")]
const L2CAP_TXQ: u8 = 20;
#[cfg(feature = "ble-central")]
const L2CAP_RXQ: u8 = 20;

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    debug!("Starting MPSL task");
    mpsl.run().await
}

pub async fn ble_stack() -> &'static Stack<'static, SoftdeviceController<'static>> {
    STACK.get().await
}

pub struct Peripherals {
    pub ppi_ch17: peripherals::PPI_CH17,
    pub ppi_ch18: peripherals::PPI_CH18,
    pub ppi_ch20: peripherals::PPI_CH20,
    pub ppi_ch21: peripherals::PPI_CH21,
    pub ppi_ch22: peripherals::PPI_CH22,
    pub ppi_ch23: peripherals::PPI_CH23,
    pub ppi_ch24: peripherals::PPI_CH24,
    pub ppi_ch25: peripherals::PPI_CH25,
    pub ppi_ch26: peripherals::PPI_CH26,
    pub ppi_ch27: peripherals::PPI_CH27,
    pub ppi_ch28: peripherals::PPI_CH28,
    pub ppi_ch29: peripherals::PPI_CH29,

    pub rtc0: peripherals::RTC0,
    pub timer0: peripherals::TIMER0,
    pub temp: peripherals::TEMP,
    pub ppi_ch19: peripherals::PPI_CH19,
    pub ppi_ch30: peripherals::PPI_CH30,
    pub ppi_ch31: peripherals::PPI_CH31,

    pub rng: peripherals::RNG,
}

impl Peripherals {
    #[must_use]
    pub fn new(peripherals: &mut crate::OptionalPeripherals) -> Self {
        Self {
            ppi_ch17: peripherals.PPI_CH17.take().unwrap(),
            ppi_ch18: peripherals.PPI_CH18.take().unwrap(),
            ppi_ch20: peripherals.PPI_CH20.take().unwrap(),
            ppi_ch21: peripherals.PPI_CH21.take().unwrap(),
            ppi_ch22: peripherals.PPI_CH22.take().unwrap(),
            ppi_ch23: peripherals.PPI_CH23.take().unwrap(),
            ppi_ch24: peripherals.PPI_CH24.take().unwrap(),
            ppi_ch25: peripherals.PPI_CH25.take().unwrap(),
            ppi_ch26: peripherals.PPI_CH26.take().unwrap(),
            ppi_ch27: peripherals.PPI_CH27.take().unwrap(),
            ppi_ch28: peripherals.PPI_CH28.take().unwrap(),
            ppi_ch29: peripherals.PPI_CH29.take().unwrap(),

            rtc0: peripherals.RTC0.take().unwrap(),
            timer0: peripherals.TIMER0.take().unwrap(),
            temp: peripherals.TEMP.take().unwrap(),
            ppi_ch19: peripherals.PPI_CH19.take().unwrap(),
            ppi_ch30: peripherals.PPI_CH30.take().unwrap(),
            ppi_ch31: peripherals.PPI_CH31.take().unwrap(),

            rng: peripherals.RNG.take().unwrap(),
        }
    }
}

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<RNG>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    let builder = sdc::Builder::new()?;

    #[cfg(feature = "ble-peripheral")]
    let builder = builder
        .support_adv()?
        .support_peripheral()?
        .peripheral_count(1)?;

    #[cfg(feature = "ble-central")]
    let builder = builder
        .support_scan()?
        .support_central()?
        .central_count(1)?
        .buffer_cfg(L2CAP_MTU as u8, L2CAP_MTU as u8, L2CAP_TXQ, L2CAP_RXQ)?;

    builder.build(p, rng, mpsl, mem)
}

pub fn driver<'d>(p: Peripherals, spawner: &Spawner, config: ariel_os_embassy_common::ble::Config) {
    debug!("Initializing BLE driver");
    let mpsl_p =
        mpsl::Peripherals::new(p.rtc0, p.timer0, p.temp, p.ppi_ch19, p.ppi_ch30, p.ppi_ch31);
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    let mpsl = MPSL.init(
        mpsl::MultiprotocolServiceLayer::new(mpsl_p, Irqs, lfclk_cfg)
            .expect("Failed to initialize MPSL"),
    );
    spawner.must_spawn(mpsl_task(&*mpsl));

    let sdc_p = sdc::Peripherals::new(
        p.ppi_ch17, p.ppi_ch18, p.ppi_ch20, p.ppi_ch21, p.ppi_ch22, p.ppi_ch23, p.ppi_ch24,
        p.ppi_ch25, p.ppi_ch26, p.ppi_ch27, p.ppi_ch28, p.ppi_ch29,
    );

    static RNG: StaticCell<embassy_nrf::rng::Rng<'static, RNG>> = StaticCell::new();
    let rng = RNG.init(embassy_nrf::rng::Rng::new(p.rng, Irqs));

    // Executor stack should be bigger than SDC memory size
    static SDC_MEM: StaticCell<sdc::Mem<SDC_MEM_SIZE>> = StaticCell::new();
    let sdc_mem = SDC_MEM.init(sdc::Mem::new());

    let sdc = build_sdc(sdc_p, rng, mpsl, sdc_mem).expect("Failed to build SDC");

    let address: Address = config.address;

    static HOST_RESOURCES: StaticCell<HostResources<MAX_CONNS, MAX_CHANNELS, L2CAP_MTU>> =
        StaticCell::new();
    let resources = HOST_RESOURCES.init(HostResources::new());

    debug!("Creating BLE stack");

    if STACK
        .init(trouble_host::new(sdc, resources).set_random_address(address))
        .is_err()
    {
        unreachable!("BLE stack already initialized");
    }

    debug!("BLE stack initialized");
}
