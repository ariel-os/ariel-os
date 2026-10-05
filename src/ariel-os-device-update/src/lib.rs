#![no_main]
#![no_std]

#[derive(Debug)]
pub enum DeviceUpdaterError<InnerError: core::fmt::Debug> {
    /// Operation Was attempted while in a bad state.
    BadState,
    /// An error happened when interacting with the storage medium.
    Inner(InnerError),
}

pub trait DeviceUpdaterState {
    fn updater(&mut self) -> impl DeviceUpdater + '_;
}

#[allow(async_fn_in_trait)]
pub trait DeviceUpdater {
    type InnerError: core::fmt::Debug;
    const CHUNK_SIZE: usize;
    /// Write to the update storage area.
    ///
    /// # Errors
    ///
    /// This function returns an error when the bootloader is in a bad state
    /// (current firmware didnt call [`Self::mark_booted()`]) or a write error happened
    /// (unaligned or out of bounds).
    async fn write_firmware(
        &mut self,
        offset: u32,
        data: &[u8],
    ) -> Result<(), DeviceUpdaterError<Self::InnerError>>;

    /// Read from the update storage area.
    ///
    /// # Errors
    ///
    /// This function returns an error when failing to read from the storage.
    async fn read_firmware(
        &mut self,
        offset: u32,
        buffer: &mut [u8],
    ) -> Result<(), DeviceUpdaterError<Self::InnerError>>;

    /// Mark current firmware as successfully booted.
    /// Preventing the bootloader from rolling back the update.
    async fn mark_booted(&mut self) -> Result<(), DeviceUpdaterError<Self::InnerError>>;
    /// Indicate that the new firmware has been written to the update slot, it will applied next boot.
    async fn mark_updated(&mut self) -> Result<(), DeviceUpdaterError<Self::InnerError>>;
}
