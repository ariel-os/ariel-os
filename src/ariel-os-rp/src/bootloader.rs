//!Bootloader backend
use core::cell::RefCell;

use ariel_os_embassy_common::bootloader::{BootLoaderBackend, BootLoaderStorage};
use embassy_rp::{OptionalPeripherals, flash::Blocking, peripherals::FLASH};
use embassy_sync::{
    blocking_mutex::{Mutex, raw::CriticalSectionRawMutex},
    once_lock::OnceLock,
};

// Duplicated from ariel-os-storage, may need to unify this somehow.
const FLASH_OFFSET: u32 = 0x1000_0000;
// Duplicated from storage.rs, should this be moved in lib.rs or a common file?.
const FLASH_SIZE: usize = 2 * 1024 * 1024;

type Flash = embassy_rp::flash::Flash<'static, FLASH, Blocking, FLASH_SIZE>;

static FLASH_CONTROLLER: OnceLock<Mutex<CriticalSectionRawMutex, RefCell<Flash>>> = OnceLock::new();

// Initializes the NVMC needed for operating on the flash.
pub fn init(peripherals: &mut OptionalPeripherals) {
    let flash_peri = peripherals.FLASH.take().unwrap();

    let _ = FLASH_CONTROLLER.init(Mutex::new(RefCell::new(Flash::new_blocking(flash_peri))));
}

pub struct HalBootLoaderBackend;

#[allow(unsafe_code)]
impl BootLoaderBackend for HalBootLoaderBackend {
    type ACTIVE = Flash;
    type DFU = Flash;
    type STATE = Flash;

    fn storage_config<'a>() -> BootLoaderStorage<'a, Self::ACTIVE, Self::DFU, Self::STATE> {
        let flash_controller = FLASH_CONTROLLER
            .try_get()
            .expect("obtaining initialized FLASH_CONTROLLER");
        BootLoaderStorage::new_same_flash(flash_controller, FLASH_OFFSET)
    }

    fn enable_watchdog() {
        unimplemented!()
    }
    fn pet_watchdog() {
        unimplemented!()
    }
}
