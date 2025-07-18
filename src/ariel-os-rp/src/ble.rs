use bt_hci::controller::ExternalController;
use cyw43::bluetooth::BtDriver;
use embassy_sync::once_lock::OnceLock;

/// Number of command slots for the Bluetooth driver.
pub const SLOTS: usize = 10;

pub(crate) static STACK: OnceLock<
    trouble_host::Stack<'static, ExternalController<BtDriver<'static>, SLOTS>>,
> = OnceLock::new();

pub async fn ble_stack()
-> &'static trouble_host::Stack<'static, ExternalController<BtDriver<'static>, SLOTS>> {
    STACK.get().await
}
