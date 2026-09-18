//! Bootloader backend
use core::cell::RefCell;

use ariel_os_embassy_common::bootloader::{BootLoaderBackend, BootLoaderStorage};
use embassy_stm32::{OptionalPeripherals, flash::Blocking};
use embassy_sync::{
    blocking_mutex::{Mutex, raw::CriticalSectionRawMutex},
    once_lock::OnceLock,
};

// Duplicated from ariel-os-storage, may need to unify this somehow.
const FLASH_OFFSET: u32 = 0x0800_0000;

type Flash = embassy_stm32::flash::Flash<'static, Blocking>;

static FLASH_CONTROLLER: OnceLock<Mutex<CriticalSectionRawMutex, RefCell<Flash>>> = OnceLock::new();

// Initializes the NVMC needed for operating on the flash.
pub fn init(peripherals: &mut OptionalPeripherals) {
    let flash_peri = peripherals.FLASH.take().unwrap();

    // Accesses all flash banks.
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
