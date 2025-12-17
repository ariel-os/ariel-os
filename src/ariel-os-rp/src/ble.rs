use bt_hci::controller::ExternalController;
use cyw43::bluetooth::BtDriver;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, once_lock::OnceLock,
};
use static_cell::StaticCell;
use trouble_host::{Stack, prelude::DefaultPacketPool};

use ariel_os_embassy_common::cell::SameExecutorCell;

pub type BleStack = Stack<'static, ExternalController<BtDriver<'static>, SLOTS>, DefaultPacketPool>;

/// Number of command slots for the Bluetooth driver.
pub const SLOTS: usize = 10;

pub(crate) static STACK: StaticCell<SameExecutorCell<BleStack>> = StaticCell::new();
// The stack can effectively only be taken by a single application; once taken, the Option is None.
pub(crate) static STACKREF: OnceLock<
    Mutex<CriticalSectionRawMutex, Option<&'static mut SameExecutorCell<BleStack>>>,
> = OnceLock::new();

pub async fn ble_stack() -> &'static mut BleStack {
    STACKREF
        .get()
        .await
        .try_lock()
        .expect("Two tasks racing for lock, one would fail the main-executor check")
        .take()
        .expect("Stack was already taken")
        .get_mut_async()
        .await
        .expect("Stack needs to be taken from main executor")
}
