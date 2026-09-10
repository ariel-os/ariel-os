use ariel_os_log::{debug, info};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_radio::wifi::{
    Config, ModeConfig, WifiController, WifiDevice, WifiEvent, WifiStationState, sta::StationConfig,
};

use ariel_os_embassy_common::wifi::StationConfig as ArielStationConfig;

pub type NetworkDevice = WifiDevice<'static>;

pub fn init(
    peripherals: &mut crate::OptionalPeripherals,
) -> (NetworkDevice, WifiController<'static>) {
    let config = Config::default();
    let wifi = peripherals.WIFI.take().unwrap();

    let (controller, interfaces) = esp_radio::wifi::new(wifi, config).unwrap();

    (interfaces.station, controller)
}

#[embassy_executor::task]
pub async fn join(mut controller: WifiController<'static>, config: ArielStationConfig) {
    debug!("start connection task");

    #[cfg(not(feature = "defmt"))]
    debug!("Device capabilities: {:?}", controller.capabilities());

    loop {
        match esp_radio::wifi::station_state() {
            WifiStationState::Connected => {
                // wait until we're no longer connected
                controller
                    .wait_for_event(WifiEvent::StationDisconnected)
                    .await;
                Timer::after(Duration::from_secs(5)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            debug!("Configuring Wi-Fi");
            let client_config = ModeConfig::Station(
                StationConfig::default()
                    .with_ssid(config.ssid.try_into().unwrap())
                    .with_password(config.password.try_into().unwrap()),
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
