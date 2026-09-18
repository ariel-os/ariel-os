//! Bootloader backend
use core::cell::RefCell;

use ariel_os_embassy_common::bootloader::{BootLoaderBackend, BootLoaderStorage};
use embassy_nrf::{OptionalPeripherals, nvmc::Nvmc};
use embassy_sync::{
    blocking_mutex::{Mutex, raw::CriticalSectionRawMutex},
    once_lock::OnceLock,
};

// Duplicated from ariel-os-storage, may need to unify this somehow.
#[cfg(not(context = "nrf5340-net"))]
const FLASH_OFFSET: u32 = 0x0;
#[cfg(context = "nrf5340-net")]
const FLASH_OFFSET: u32 = 0x0100_0000;

static NVMC: OnceLock<Mutex<CriticalSectionRawMutex, RefCell<Nvmc<'static>>>> = OnceLock::new();

// Initializes the NVMC needed for operating on the flash.
pub fn init(peripherals: &mut OptionalPeripherals) {
    let nvmc = peripherals.NVMC.take().unwrap();

    let _ = NVMC.init(Mutex::new(RefCell::new(Nvmc::new(nvmc))));
}

pub struct HalBootLoaderBackend;

#[allow(unsafe_code)]
impl BootLoaderBackend for HalBootLoaderBackend {
    type ACTIVE = Nvmc<'static>;
    type DFU = Nvmc<'static>;
    type STATE = Nvmc<'static>;

    fn storage_config<'a>() -> BootLoaderStorage<'a, Self::ACTIVE, Self::DFU, Self::STATE> {
        let nvmc = NVMC.try_get().expect("obtaining initialized NVMC");
        BootLoaderStorage::new_same_flash(nvmc, FLASH_OFFSET)
    }
    fn enable_watchdog() {
        unimplemented!()
    }
    fn pet_watchdog() {
        unimplemented!()
    }
}
