//! Bootloader backend
use core::cell::RefCell;

use ariel_os_embassy_common::bootloader::{BootLoaderBackend, FlashConfig};
use embassy_boot::BootLoaderConfig;
use embassy_embedded_hal::flash::partition::BlockingPartition;
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

// TODO : find a better name.
pub struct HalBootLoaderBackend;

#[allow(unsafe_code)]
impl BootLoaderBackend for HalBootLoaderBackend {
    type ACTIVE = BlockingPartition<'static, CriticalSectionRawMutex, Nvmc<'static>>;
    type DFU = BlockingPartition<'static, CriticalSectionRawMutex, Nvmc<'static>>;
    type STATE = BlockingPartition<'static, CriticalSectionRawMutex, Nvmc<'static>>;

    fn config(
        flash_config: &FlashConfig,
    ) -> BootLoaderConfig<Self::ACTIVE, Self::DFU, Self::STATE> {
        // TODO: implement flash watchdog to avoid hangs ?

        let nvmc = NVMC.try_get().expect("obtaining initialized NVMC");

        let active_partition = BlockingPartition::new(
            nvmc,
            flash_config.active.start - FLASH_OFFSET,
            flash_config.active.len() as u32,
        );

        let dfu_partition = BlockingPartition::new(
            nvmc,
            flash_config.dfu.start - FLASH_OFFSET,
            flash_config.dfu.len() as u32,
        );
        let state_partition = BlockingPartition::new(
            nvmc,
            flash_config.bootloader_state.start - FLASH_OFFSET,
            flash_config.bootloader_state.len() as u32,
        );

        BootLoaderConfig {
            active: active_partition,
            dfu: dfu_partition,
            state: state_partition,
        }
    }

    // from [embassy-boot-nrf](https://github.com/embassy-rs/embassy/blob/4c9a8998805b95d472b5e8137b16588369c2a8b6/embassy-boot-nrf/src/lib.rs#L48), license MIT OR Apache-2.0
    fn load_active(flash_config: &FlashConfig) {
        let start = flash_config.active.start;

        unsafe {
            let mut p = cortex_m::Peripherals::steal();
            p.SCB.invalidate_icache();
            p.SCB.vtor.write(start);
            cortex_m::asm::bootload(start as *const u32)
        }
    }
    fn enable_watchdog() {
        unimplemented!()
    }
    fn pet_watchdog() {
        unimplemented!()
    }
}
