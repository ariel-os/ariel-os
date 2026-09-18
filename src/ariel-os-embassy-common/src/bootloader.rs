//! Common traits for implementin and using the bootloader infrastructure.
use core::ops::Range;

use embassy_boot::BootLoaderConfig;
use embedded_storage::nor_flash::NorFlash;

const fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

/// Flash sections configuration of the board.
// This struct will be returned by ariel-os-rt
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

    /// Creates the bootloader config, use `embassy_embedded_hal::flash::partition::BlockingPartition` if you want to share the flash with different sections.
    // maybe give the flash layout as parameter here ?
    // how would different flashes for active,dfu and state be handled in terms of configurations given to this function ?
    fn config(flash_config: &FlashConfig)
    -> BootLoaderConfig<Self::ACTIVE, Self::DFU, Self::STATE>;

    /// Start execution of the active partition.
    fn load_active(flash_config: &FlashConfig);

    /// Enable the watchdog, starting it.
    // Currently not used.
    fn enable_watchdog();

    /// Pet the watchdog, preventing reset.
    // Currently not used.
    fn pet_watchdog();
}
