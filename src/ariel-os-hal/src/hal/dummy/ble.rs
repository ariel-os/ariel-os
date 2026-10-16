use bt_hci::{
    cmd,
    controller::{ControllerCmdAsync, ControllerCmdSync},
};
use embassy_executor::Spawner;

// Export the BLE controller type for this device.
pub type BleController = DummyController;

// This is the function called by default by `ariel-os-embassy` to initialize the BLE driver,
// if BLE initialization is tied to WiFi initialization, make a special case in `ariel-os-embassy`.
/// Build a BLE controller for this device.
#[must_use]
pub fn build_controller(
    _p: &mut crate::hal::OptionalPeripherals,
    _spawner: Spawner,
) -> BleController {
    unimplemented!();
}

pub struct DummyController {}

#[derive(Debug)]
pub struct DummyError {}

impl core::fmt::Display for DummyError {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unimplemented!()
    }
}

impl core::error::Error for DummyError {}

impl embedded_io::Error for DummyError {
    fn kind(&self) -> embedded_io::ErrorKind {
        unimplemented!()
    }
}

impl embedded_io::ErrorType for DummyController {
    type Error = DummyError;
}
impl bt_hci::controller::Controller for DummyController {
    async fn write_acl_data(
        &self,
        _packet: &bt_hci::data::AclPacket<'_>,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn write_sync_data(
        &self,
        _packet: &bt_hci::data::SyncPacket<'_>,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn write_iso_data(
        &self,
        _packet: &bt_hci::data::IsoPacket<'_>,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn read<'a>(
        &self,
        _buf: &'a mut [u8],
    ) -> Result<bt_hci::ControllerToHostPacket<'a>, Self::Error> {
        unimplemented!()
    }
}

impl<C: cmd::AsyncCmd + ?Sized> ControllerCmdAsync<C> for DummyController {
    async fn exec(&self, _cmd: &C) -> Result<(), cmd::Error<Self::Error>> {
        unimplemented!();
    }
}

impl<C: cmd::SyncCmd + ?Sized> ControllerCmdSync<C> for DummyController {
    async fn exec(&self, _cmd: &C) -> Result<C::Return, cmd::Error<Self::Error>> {
        unimplemented!();
    }
}
