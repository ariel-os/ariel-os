use embassy_executor::Spawner;
use embassy_nrf::peripherals;
use embassy_sync::once_lock::OnceLock;
use nrf_sdc::{
    self as sdc, SoftdeviceController,
    mpsl::{self, MultiprotocolServiceLayer},
};
use static_cell::StaticCell;
use trouble_host::Stack;

use ariel_os_debug::log::debug;
use ariel_os_embassy_common::ble::MTU;

use crate::irqs::Irqs;

static STACK: OnceLock<Stack<'static, SoftdeviceController<'static>>> = OnceLock::new();
static MPSL: StaticCell<MultiprotocolServiceLayer<'_>> = StaticCell::new();
static SDC_MEM: StaticCell<sdc::Mem<SDC_MEM_SIZE>> = StaticCell::new();
static RNG: StaticCell<ariel_os_random::CryptoRngSend> = StaticCell::new();

// Memory to allocate to the SoftDevice Controller (SDC).
//
// During testing central mode needed 2912 bytes, peripheral mode needed 1448 bytes.
// Multirole (central + peripheral) needed 6080 bytes. Allocate more here if using extended features.

#[cfg(all(
    context = "nrf52",
    feature = "ble-peripheral",
    not(feature = "ble-central")
))]
const SDC_MEM_SIZE: usize = 2880;
#[cfg(all(
    context = "nrf52",
    feature = "ble-central",
    not(feature = "ble-peripheral")
))]
const SDC_MEM_SIZE: usize = 2912;
#[cfg(all(context = "nrf52", feature = "ble-peripheral", feature = "ble-central"))]
const SDC_MEM_SIZE: usize = 6080;

#[cfg(all(
    context = "nrf53",
    feature = "ble-peripheral",
    not(feature = "ble-central")
))]
const SDC_MEM_SIZE: usize = 4768;
#[cfg(all(
    context = "nrf53",
    feature = "ble-central",
    not(feature = "ble-peripheral")
))]
const SDC_MEM_SIZE: usize = 4904;
#[cfg(all(context = "nrf53", feature = "ble-peripheral", feature = "ble-central"))]
const SDC_MEM_SIZE: usize = 6080;

// Size of the TX buffer (number of packets), minimum is 1, SoftDevice default is 3 (SDC_DEFAULT_TX_PACKET_COUNT).
const L2CAP_TXQ: u8 = 3;
// Size of the RX buffer (number of packets), minimum is 1, SoftDevice default is 2 (SDC_DEFAULT_RX_PACKET_COUNT).
const L2CAP_RXQ: u8 = 2;

pub async fn ble_stack() -> &'static Stack<'static, SoftdeviceController<'static>> {
    STACK.get().await
}

#[cfg(context = "nrf52")]
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
}

#[cfg(context = "nrf52")]
impl Peripherals {
    /// Reserves the necessary peripherals for the BLE stack.
    ///
    /// # Panics
    /// Panics if any of the required peripherals are not available.
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
        }
    }
}

#[cfg(context = "nrf53")]
pub struct Peripherals {
    pub ppi_ch3: peripherals::PPI_CH3,
    pub ppi_ch4: peripherals::PPI_CH4,
    pub ppi_ch5: peripherals::PPI_CH5,
    pub ppi_ch6: peripherals::PPI_CH6,
    pub ppi_ch7: peripherals::PPI_CH7,
    pub ppi_ch8: peripherals::PPI_CH8,
    pub ppi_ch9: peripherals::PPI_CH9,
    pub ppi_ch10: peripherals::PPI_CH10,
    pub ppi_ch11: peripherals::PPI_CH11,
    pub ppi_ch12: peripherals::PPI_CH12,

    pub rtc0: peripherals::RTC0,
    pub timer0: peripherals::TIMER0,
    pub timer1: peripherals::TIMER1,
    pub ppi_ch0: peripherals::PPI_CH0,
    pub ppi_ch1: peripherals::PPI_CH1,
    pub ppi_ch2: peripherals::PPI_CH2,
}

#[cfg(context = "nrf53")]
impl Peripherals {
    /// Reserves the necessary peripherals for the BLE stack.
    ///
    /// # Panics
    /// Panics if any of the required peripherals are not available.
    #[must_use]
    pub fn new(peripherals: &mut crate::OptionalPeripherals) -> Self {
        Self {
            ppi_ch3: peripherals.PPI_CH3.take().unwrap(),
            ppi_ch4: peripherals.PPI_CH4.take().unwrap(),
            ppi_ch5: peripherals.PPI_CH5.take().unwrap(),
            ppi_ch6: peripherals.PPI_CH6.take().unwrap(),
            ppi_ch7: peripherals.PPI_CH7.take().unwrap(),
            ppi_ch8: peripherals.PPI_CH8.take().unwrap(),
            ppi_ch9: peripherals.PPI_CH9.take().unwrap(),
            ppi_ch10: peripherals.PPI_CH10.take().unwrap(),
            ppi_ch11: peripherals.PPI_CH11.take().unwrap(),
            ppi_ch12: peripherals.PPI_CH12.take().unwrap(),

            rtc0: peripherals.RTC0.take().unwrap(),
            timer0: peripherals.TIMER0.take().unwrap(),
            timer1: peripherals.TIMER1.take().unwrap(),
            ppi_ch0: peripherals.PPI_CH0.take().unwrap(),
            ppi_ch1: peripherals.PPI_CH1.take().unwrap(),
            ppi_ch2: peripherals.PPI_CH2.take().unwrap(),
        }
    }
}

/// Configures and initializes the nRF BLE driver.
///
/// # Panics
/// Panics if initialization fails on one of the components, such as MPSL or SDC.
#[expect(
    clippy::needless_pass_by_value,
    reason = "keeping consistency with other initialization functions"
)]
pub fn driver(p: Peripherals, spawner: Spawner, config: ariel_os_embassy_common::ble::Config) {
    debug!("Initializing nRF BLE driver");
    #[cfg(context = "nrf52")]
    let mpsl_p =
        mpsl::Peripherals::new(p.rtc0, p.timer0, p.temp, p.ppi_ch19, p.ppi_ch30, p.ppi_ch31);
    #[cfg(context = "nrf53")]
    let mpsl_p =
        mpsl::Peripherals::new(p.rtc0, p.timer0, p.timer1, p.ppi_ch0, p.ppi_ch1, p.ppi_ch2);
    #[allow(clippy::cast_possible_truncation)]
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    let mpsl = MPSL.init(
        mpsl::MultiprotocolServiceLayer::new(mpsl_p, Irqs, lfclk_cfg)
            .expect("Failed to initialize MPSL"),
    );
    spawner.must_spawn(mpsl_task(mpsl));

    let rng = RNG.init(ariel_os_random::crypto_rng_send());

    #[cfg(context = "nrf52")]
    let sdc_p = sdc::Peripherals::new(
        p.ppi_ch17, p.ppi_ch18, p.ppi_ch20, p.ppi_ch21, p.ppi_ch22, p.ppi_ch23, p.ppi_ch24,
        p.ppi_ch25, p.ppi_ch26, p.ppi_ch27, p.ppi_ch28, p.ppi_ch29,
    );
    #[cfg(context = "nrf53")]
    let sdc_p = sdc::Peripherals::new(
        p.ppi_ch3, p.ppi_ch4, p.ppi_ch5, p.ppi_ch6, p.ppi_ch7, p.ppi_ch8, p.ppi_ch9, p.ppi_ch10,
        p.ppi_ch11, p.ppi_ch12,
    );

    let sdc_mem = SDC_MEM.init(sdc::Mem::new());

    let sdc = build_sdc(sdc_p, rng, mpsl, sdc_mem).expect("Failed to build SDC");

    let resources = ariel_os_embassy_common::ble::get_ble_host_resources();

    let stack = trouble_host::new(sdc, resources).set_random_address(config.address);
    let _ = STACK.init(stack);

    debug!("nRF BLE driver initialized");
}

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    debug!("Starting nRF MPSL task");
    mpsl.run().await
}

/// Builds the SoftDevice Controller (SDC) with the given peripherals and memory.
///
/// # Errors
///
/// An error is returned if the SDC cannot be built with the provided configuration.
/// The meaning of the errors code can be found in [nrfxlib](https://github.com/nrfconnect/sdk-nrfxlib/blob/3a14dbc326c385a0161fc122f72b6d9be308f7d6/softdevice_controller/include/sdc.h)
#[expect(
    clippy::doc_markdown,
    reason = "gets wrongly triggered for 'SoftDevice'"
)]
fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut ariel_os_random::CryptoRngSend,
    mpsl: &'d MultiprotocolServiceLayer<'_>,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    let builder = sdc::Builder::new()?;

    // Order matters here if we want multirole to work.

    #[cfg(feature = "ble-peripheral")]
    let builder = builder.support_adv()?;
    #[cfg(feature = "ble-central")]
    let builder = builder.support_scan()?;
    #[cfg(feature = "ble-peripheral")]
    let builder = builder.support_peripheral()?;
    #[cfg(feature = "ble-central")]
    let builder = builder.support_central()?;
    #[cfg(feature = "ble-peripheral")]
    let builder = builder.peripheral_count(1)?;
    #[cfg(feature = "ble-central")]
    let builder = builder.central_count(1)?;

    #[allow(clippy::cast_possible_truncation)]
    let builder = builder.buffer_cfg(MTU as u8, MTU as u8, L2CAP_TXQ, L2CAP_RXQ)?;

    builder.build(p, rng, mpsl, mem)
}
