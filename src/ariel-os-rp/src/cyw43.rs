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
use ariel_os_embassy_common::wifi::StationConfig;
#[cfg(feature = "wifi")]
use cyw43::JoinOptions;

pub type NetworkDevice = cyw43::NetDriver<'static>;

static STATE: StaticCell<cyw43::State> = StaticCell::new();

#[cfg(feature = "wifi")]
pub async fn join(mut control: cyw43::Control<'static>, config: StationConfig) {
    use ariel_os_log::info;
    loop {
        //control.join_open(WIFI_NETWORK).await;
        match control
            .join(config.ssid, JoinOptions::new(config.password.as_bytes()))
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

/// Data structures returned by [`device()`].
pub struct Cyw43Device<'b> {
    pub net_device: embassy_net_driver_channel::Device<'b, 1514>,
    pub net_control: Control<'b>,
    #[cfg(feature = "ble-cyw43")]
    pub ble_controller: crate::ble::BleController,
}

/// # Panics
///
/// Panics if we fail to launch the cyw43 runner task.
pub async fn device<'a, 'b: 'a>(
    peripherals: &'a mut crate::OptionalPeripherals,
    spawner: &Spawner,
) -> Cyw43Device<'b> {
    let pins = rpi_pico_w::take_pins(peripherals);

    let fw = cyw43_firmware::CYW43_43439A0;
    let clm = cyw43_firmware::CYW43_43439A0_CLM;
    #[cfg(feature = "ble-cyw43")]
    let btfw = cyw43_firmware::CYW43_43439A0_BTFW;

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
    let (net_device, mut net_control, runner) =
        cyw43::new(STATE.init_with(cyw43::State::new), pwr, spi, fw).await;

    #[cfg(feature = "ble-cyw43")]
    let (net_device, mut net_control, runner, ble_controller) = {
        let (net_device, bt_device, control, runner) =
            cyw43::new_with_bluetooth(STATE.init_with(cyw43::State::new), pwr, spi, fw, btfw).await;
        let ble_controller = ExternalController::new(bt_device);

        (net_device, control, runner, ble_controller)
    };

    // control
    //     .set_power_management(cyw43::PowerManagementMode::PowerSave)
    //     .await;

    // this needs to be spawned here (before using `control`)
    spawner.spawn(wifi_cyw43_task(runner)).unwrap();

    net_control.init(clm).await;

    Cyw43Device {
        net_device,
        net_control,
        #[cfg(feature = "ble-cyw43")]
        ble_controller,
    }
}
