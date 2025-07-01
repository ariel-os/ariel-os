use bt_hci::controller::ExternalController;
use embassy_sync::once_lock::OnceLock;
use esp_wifi::ble::controller::BleConnector;

/// Number of command slots for the Bluetooth driver.
pub const SLOTS: usize = 10;

pub(crate) static STACK: OnceLock<
    trouble_host::Stack<'static, ExternalController<BleConnector<'static>, SLOTS>>,
> = OnceLock::new();

pub async fn init(
    peripherals: &mut esp_hal::peripherals::OptionalPeripherals,
    config: &ariel_os_embassy_common::ble::Config,
) {
    let wifi_init = crate::WIFI_INIT.get().unwrap();
    let connector = BleConnector::new(wifi_init, peripherals.BT.take().unwrap());
    let controller: ExternalController<_, SLOTS> = ExternalController::new(connector);
    let resources = ariel_os_embassy_common::ble::get_ble_host_resources();
    let mut rng = ariel_os_random::crypto_rng();
    let stack = trouble_host::new(controller, resources)
        .set_random_generator_seed(&mut rng)
        .set_random_address(config.address);
    let _ = STACK.init(stack);
}

pub async fn ble_stack() -> &'static trouble_host::Stack<'static, impl trouble_host::Controller> {
    STACK.get().await
}
