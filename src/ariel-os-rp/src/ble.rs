use bt_hci::controller::ExternalController;
use cyw43::bluetooth::BtDriver;
use embassy_sync::once_lock::OnceLock;

pub(crate) static STACK: OnceLock<
    trouble_host::Stack<'static, ExternalController<BtDriver<'static>, 10>>,
> = OnceLock::new();

pub async fn ble_stack() -> &'static trouble_host::Stack<'static, impl trouble_host::Controller> {
    STACK.get().await
}
