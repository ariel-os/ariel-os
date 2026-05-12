#[doc(hidden)]
pub mod storage {
    pub use ariel_os_esp::storage::{Flash, FlashError, init as flash_init};
}