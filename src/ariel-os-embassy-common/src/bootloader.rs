//! Common traits for implementing and using the ariel-os-bootloader infrastructure.
use core::{cell::RefCell, ops::Range};

use embassy_boot::BootLoaderConfig;
use embassy_embedded_hal::flash::partition::BlockingPartition;
use embassy_sync::blocking_mutex::{Mutex, raw::CriticalSectionRawMutex};
use embedded_storage::nor_flash::NorFlash;

const fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

/// Flash sections configuration of the board.
#[derive(Clone)]
pub struct FlashConfig {
    /// Active section range, as seen from the MCU's memory model.
    // We need the absolute range visible by the MCU to boot into it.
    pub active: Range<u32>,
    /// DFU section range, as seen from the MCU's memory model.
    pub dfu: Range<u32>,
    /// Bootloader state section range, as seen from the MCU's memory model.
    pub bootloader_state: Range<u32>,
}

/// Storage configuration of the bootloader.
///
/// Generic `<ACTIVE|DFU|STATE>_FLASH_OFFSET` corresponds to where the flash is mapped in the memory.
/// Set this to 0 if the flash controller already takes into account this offset.
pub struct BootLoaderStorage<'a, ACTIVE: NorFlash, DFU: NorFlash, STATE: NorFlash> {
    active_flash_controller: &'a Mutex<CriticalSectionRawMutex, RefCell<ACTIVE>>,
    active_flash_offset: u32,
    dfu_flash_controller: &'a Mutex<CriticalSectionRawMutex, RefCell<DFU>>,
    dfu_flash_offset: u32,
    state_flash_controller: &'a Mutex<CriticalSectionRawMutex, RefCell<STATE>>,
    state_flash_offset: u32,
}

impl<'a, CONTROLLER: NorFlash> BootLoaderStorage<'a, CONTROLLER, CONTROLLER, CONTROLLER> {
    /// Create a storage configuration where all the partitions are on the same flash (controller).
    pub fn new_same_flash(
        flash_controller: &'a Mutex<CriticalSectionRawMutex, RefCell<CONTROLLER>>,
        offset: u32,
    ) -> Self {
        Self::new(
            flash_controller,
            offset,
            flash_controller,
            offset,
            flash_controller,
            offset,
        )
    }
}

impl<'a, ACTIVE: NorFlash, DFU: NorFlash, STATE: NorFlash>
    BootLoaderStorage<'a, ACTIVE, DFU, STATE>
{
    /// Create a storage configuration with separate flash controllers for each section.
    pub fn new(
        active_flash_controller: &'a Mutex<CriticalSectionRawMutex, RefCell<ACTIVE>>,
        active_flash_offset: u32,
        dfu_flash_controller: &'a Mutex<CriticalSectionRawMutex, RefCell<DFU>>,
        dfu_flash_offset: u32,
        state_flash_controller: &'a Mutex<CriticalSectionRawMutex, RefCell<STATE>>,
        state_flash_offset: u32,
    ) -> Self {
        Self {
            active_flash_controller,
            active_flash_offset,
            dfu_flash_controller,
            dfu_flash_offset,
            state_flash_controller,
            state_flash_offset,
        }
    }
}

/// Backend to be implemented by the HAL to allow the bootloader to operate on the chip.
pub trait BootLoaderBackend {
    /// Active section flash type.
    type ACTIVE: NorFlash;
    /// DFU section flash type.
    type DFU: NorFlash;
    /// Bootloader state section flash type.
    type STATE: NorFlash;

    /// The buffer size that should be used to copy from a flash section to another.
    // Try to guess the (biggest) page size: the biggest read or erase size across flashes.
    const ALIGNED_BUFFER_SIZE: usize = max(
        Self::ACTIVE::ERASE_SIZE,
        max(
            Self::ACTIVE::WRITE_SIZE,
            max(
                Self::DFU::ERASE_SIZE,
                max(
                    Self::DFU::WRITE_SIZE,
                    max(Self::STATE::ERASE_SIZE, Self::STATE::WRITE_SIZE),
                ),
            ),
        ),
    );

    /// Creates the bootloader config.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::type_complexity)]
    fn config(
        flash_config: &FlashConfig,
    ) -> BootLoaderConfig<
        BlockingPartition<'_, CriticalSectionRawMutex, Self::ACTIVE>,
        BlockingPartition<'_, CriticalSectionRawMutex, Self::DFU>,
        BlockingPartition<'_, CriticalSectionRawMutex, Self::STATE>,
    > {
        let storage = Self::storage_config();

        let active_partition = BlockingPartition::new(
            storage.active_flash_controller,
            flash_config.active.start - storage.active_flash_offset,
            flash_config.active.len() as u32,
        );

        let dfu_partition = BlockingPartition::new(
            storage.dfu_flash_controller,
            flash_config.dfu.start - storage.dfu_flash_offset,
            flash_config.dfu.len() as u32,
        );
        let state_partition = BlockingPartition::new(
            storage.state_flash_controller,
            flash_config.bootloader_state.start - storage.state_flash_offset,
            flash_config.bootloader_state.len() as u32,
        );

        BootLoaderConfig {
            active: active_partition,
            dfu: dfu_partition,
            state: state_partition,
        }
    }

    /// Return the storage configuration of the platform.
    fn storage_config<'a>() -> BootLoaderStorage<'a, Self::ACTIVE, Self::DFU, Self::STATE>;

    /// Enable the watchdog, starting it.
    // Currently not used.
    fn enable_watchdog();

    /// Pet the watchdog, preventing reset.
    // Currently not used.
    fn pet_watchdog();
}
