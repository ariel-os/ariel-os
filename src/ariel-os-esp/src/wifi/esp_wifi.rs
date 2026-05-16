use ariel_os_log::{debug, info, warn};
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use esp_radio::wifi::{
    Config, ModeConfig, WifiController, WifiDevice, WifiEvent, WifiStationState, sta::StationConfig,
};

pub type NetworkDevice = WifiDevice<'static>;

// State of the wifi interface so it can be controlled from another task.
static WIFI_CONTROL_WANTED_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Enabled,
    Disabled,
}

/// Interface controller for the esp wifi.
#[derive(Debug, Clone, Copy)]
pub struct EspWifiInterfaceController {}
impl EspWifiInterfaceController {
    /// Create a new interface controller.
    pub fn new() -> Self {
        Self {}
    }
}

impl ariel_os_embassy_common::net::InterfaceController for EspWifiInterfaceController {
    fn disable(&self) {
        WIFI_CONTROL_WANTED_STATE.sender().send(State::Disabled);
    }
    fn enable(&self) {
        WIFI_CONTROL_WANTED_STATE.sender().send(State::Enabled);
    }
}

pub fn init(peripherals: &mut crate::OptionalPeripherals, spawner: Spawner) -> NetworkDevice {
    let config = Config::default();
    let wifi = peripherals.WIFI.take().unwrap();

    WIFI_CONTROL_WANTED_STATE.sender().send(State::Enabled);

    let (controller, interfaces) = esp_radio::wifi::new(wifi, config).unwrap();

    spawner.spawn(connection(controller)).ok();

    interfaces.station
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    let mut control_receiver = WIFI_CONTROL_WANTED_STATE.receiver().unwrap();

    debug!("start connection task");

    #[cfg(not(feature = "defmt"))]
    debug!("Device capabilities: {:?}", controller.capabilities());

    loop {
        if control_receiver.get().await == State::Disabled {
            // Turn off the wifi modem.
            if let Err(err) = controller.stop_async().await {
                warn!("Error when stopping wifi: {}", err);
            }

            control_receiver.changed().await;
            continue;
        }

        match esp_radio::wifi::station_state() {
            WifiStationState::Connected => {
                // wait until we're no longer connected or we receive a command to stop.
                match select(
                    controller.wait_for_event(WifiEvent::StationDisconnected),
                    control_receiver.changed(),
                )
                .await
                {
                    Either::First(_) => {}
                    // We want to change the state, this is handled at the start of the loop.
                    Either::Second(_) => {
                        continue;
                    }
                }
                Timer::after(Duration::from_secs(5)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            debug!("Configuring Wi-Fi");
            let client_config = ModeConfig::Station(
                StationConfig::default()
                    .with_ssid(crate::wifi::WIFI_NETWORK.try_into().unwrap())
                    .with_password(crate::wifi::WIFI_PASSWORD.try_into().unwrap()),
            );
            controller.set_config(&client_config).unwrap();
            debug!("Starting Wi-Fi");
            controller.start_async().await.unwrap();
            debug!("Wi-Fi started!");
        }
        debug!("About to connect...");

        match controller.connect_async().await {
            Ok(_) => info!("Wifi connected!"),
            Err(e) => {
                info!("Failed to connect to Wi-Fi: {:?}", e);
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}
