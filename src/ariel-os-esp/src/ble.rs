use bt_hci::controller::ExternalController;
use core::ffi::c_void;
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, once_lock::OnceLock,
};
use esp_radio::ble::controller::BleConnector;
use esp_radio_rtos_driver::{
    queue::CompatQueue, register_queue_implementation, register_scheduler_implementation,
    register_semaphore_implementation, register_timer_implementation,
    register_wait_queue_implementation, semaphore::CompatSemaphore, timer::CompatTimer,
};
use static_cell::StaticCell;
use trouble_host::prelude::DefaultPacketPool;

use ariel_os_embassy_common::cell::SameExecutorCell;

use crate::scheduler::ArielScheduler;
use crate::wait_queue::ArielWaitQueue;

/// Number of command slots for the Bluetooth driver.
pub const SLOTS: usize = 10;

pub type BleStack = trouble_host::Stack<
    'static,
    ExternalController<BleConnector<'static>, SLOTS>,
    DefaultPacketPool,
>;

pub(crate) static STACK: StaticCell<SameExecutorCell<BleStack>> = StaticCell::new();
// The stack can effectively only be taken by a single application; once taken, the Option is None.
pub(crate) static STACKREF: OnceLock<
    Mutex<CriticalSectionRawMutex, Option<&'static mut SameExecutorCell<BleStack>>>,
> = OnceLock::new();

pub async fn init(
    peripherals: &mut esp_hal::peripherals::OptionalPeripherals,
    config: &ariel_os_embassy_common::ble::Config,
    spawner: Spawner,
) {
    let connector = BleConnector::new(peripherals.BT.take().unwrap(), Default::default()).unwrap();
    let controller: ExternalController<_, SLOTS> = ExternalController::new(connector);
    let resources = ariel_os_embassy_common::ble::get_ble_host_resources();
    let mut rng = ariel_os_random::crypto_rng();
    let stack = trouble_host::new(controller, resources)
        .set_random_generator_seed(&mut rng)
        .set_random_address(config.address);
    let stackref = STACK.init(SameExecutorCell::new(stack, spawner));
    // Error case is unreachable: just init'ed another once item.
    let _ = STACKREF.init(Some(stackref).into());
}

/// Returns the system ble stack.
///
/// # Panics
/// - panics if the stack was already taken
/// - panics when not called from the main executor
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

register_scheduler_implementation!(static SCHEDULER: ArielScheduler = ArielScheduler{});
register_wait_queue_implementation!(ArielWaitQueue);
register_semaphore_implementation!(CompatSemaphore);
register_timer_implementation!(CompatTimer);
register_queue_implementation!(CompatQueue);
