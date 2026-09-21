use ariel_os_hal::hal::{
    OptionalPeripherals,
    storage::{Flash, init as flash_init},
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, once_lock::OnceLock,
};

pub(crate) static STORAGE: OnceLock<Mutex<CriticalSectionRawMutex, Storage<Flash>>> =
    OnceLock::new();

pub fn init(p: &mut OptionalPeripherals) {
    let flash = flash_init(p);

    let _ = STORAGE.init(Mutex::new(Storage { flash }));
}

pub struct Storage<F> {
    pub flash: F,
}
