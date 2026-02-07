use ariel_os_debug::log::{debug, info};
use core::ffi::c_void;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_radio::wifi::{
    Config, ModeConfig, WifiController, WifiDevice, WifiEvent, WifiStationState, sta::StationConfig,
};
use esp_radio_rtos_driver::{
    queue::CompatQueue, register_queue_implementation, register_scheduler_implementation,
    register_semaphore_implementation, register_timer_implementation,
    register_wait_queue_implementation, timer::CompatTimer,
};

use crate::scheduler::ArielScheduler;
use crate::semaphore;
use crate::wait_queue::ArielWaitQueue;

pub type NetworkDevice = WifiDevice<'static>;

pub fn init(peripherals: &mut crate::OptionalPeripherals, spawner: Spawner) -> NetworkDevice {
    let config = Config::default();
    let wifi = peripherals.WIFI.take().unwrap();

    let (controller, interfaces) = esp_radio::wifi::new(wifi, config).unwrap();

    spawner.spawn(connection(controller)).ok();

    interfaces.station
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
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

register_scheduler_implementation!(static SCHEDULER: ArielScheduler = ArielScheduler{});
register_wait_queue_implementation!(ArielWaitQueue);
register_semaphore_implementation!(semaphore::CompatSemaphore);
register_timer_implementation!(CompatTimer);
register_queue_implementation!(CompatQueue);
