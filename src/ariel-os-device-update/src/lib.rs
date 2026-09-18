#![no_main]
#![no_std]

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum DeviceUpdaterError {
    Unknown,
}

pub trait DeviceUpdaterState {
    fn updater(&mut self) -> impl DeviceUpdater + '_;
}

#[allow(async_fn_in_trait)]
pub trait DeviceUpdater {
    const CHUNK_SIZE: usize;
    async fn write_firmware(
        &mut self,
        offset: usize,
        data: &[u8],
    ) -> Result<(), DeviceUpdaterError>;
    async fn mark_updated(&mut self) -> Result<(), DeviceUpdaterError>;
}
