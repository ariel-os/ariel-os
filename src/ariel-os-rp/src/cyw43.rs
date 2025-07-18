#[cfg_attr(
    any(context = "rpi-pico-w", context = "rpi-pico2-w"),
    path = "cyw43/rpi_pico_w.rs"
)]
mod rpi_pico_w;

use cyw43::{Control, Runner};
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Level, Output},
    pio::Pio,
};
use rpi_pico_w::{CywSpi, DEFAULT_CLOCK_DIVIDER, Irqs};
use static_cell::StaticCell;

#[cfg(feature = "ble-cyw43")]
use bt_hci::controller::ExternalController;
#[cfg(feature = "wifi")]
use cyw43::JoinOptions;

#[cfg(feature = "ble-cyw43")]
use crate::ble::{self, SLOTS};

pub type NetworkDevice = cyw43::NetDriver<'static>;

static STATE: StaticCell<cyw43::State> = StaticCell::new();

#[cfg(feature = "wifi")]
pub async fn join(mut control: cyw43::Control<'static>) {
    use ariel_os_debug::log::info;
    loop {
        //control.join_open(WIFI_NETWORK).await;
        match control
            .join(
                crate::wifi::WIFI_NETWORK,
                JoinOptions::new(crate::wifi::WIFI_PASSWORD.as_bytes()),
            )
            .await
        {
            Ok(_) => {
                info!("Wifi connected!");
                break;
            }
            Err(err) => {
                info!(" Wifi join failed with status={}", err.status);
            }
        }
    }
}

#[embassy_executor::task]
async fn wifi_cyw43_task(runner: Runner<'static, Output<'static>, CywSpi>) -> ! {
    runner.run().await
}

/// # Panics
///
/// Panics if we fail to launch the cyw43 runner task.
pub async fn device<'a, 'b: 'a>(
    peripherals: &'a mut crate::OptionalPeripherals,
    spawner: &Spawner,
    #[cfg(feature = "ble-cyw43")] config: ariel_os_embassy_common::ble::Config,
) -> (embassy_net_driver_channel::Device<'b, 1514>, Control<'b>) {
    let pins = rpi_pico_w::take_pins(peripherals);

    let fw = include_bytes!("cyw43/firmware/43439A0.bin");
    let clm = include_bytes!("cyw43/firmware/43439A0_clm.bin");
    #[cfg(feature = "ble-cyw43")]
    let btfw = include_bytes!("cyw43/firmware/43439A0_btfw.bin");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs download 43439A0.bin --format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs download 43439A0_clm.bin --format bin --chip RP2040 --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    let pwr = Output::new(pins.pwr, Level::Low);
    let cs = Output::new(pins.cs, Level::High);
    let mut pio = Pio::new(pins.pio, Irqs);
    let spi = CywSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        pins.dio,
        pins.clk,
        pins.dma,
    );

    #[cfg(not(feature = "ble-cyw43"))]
    let (net_device, mut control, runner) =
        cyw43::new(STATE.init_with(cyw43::State::new), pwr, spi, fw).await;

    #[cfg(feature = "ble-cyw43")]
    let (net_device, mut control, runner) = {
        let (net_device, bt_device, control, runner) =
            cyw43::new_with_bluetooth(STATE.init_with(cyw43::State::new), pwr, spi, fw, btfw).await;
        let controller: ExternalController<_, SLOTS> = ExternalController::new(bt_device);
        let resources = ariel_os_embassy_common::ble::get_ble_host_resources();
        let mut rng = ariel_os_random::crypto_rng();
        let stack = trouble_host::new(controller, resources)
            .set_random_generator_seed(&mut rng)
            .set_random_address(config.address);
        let _ = ble::STACK.init(stack);

        (net_device, control, runner)
    };

    // control
    //     .set_power_management(cyw43::PowerManagementMode::PowerSave)
    //     .await;

    // this needs to be spawned here (before using `control`)
    spawner.spawn(wifi_cyw43_task(runner)).unwrap();

    control.init(clm).await;

    (net_device, control)
}
