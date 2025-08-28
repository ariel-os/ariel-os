use bt_hci::{
    cmd,
    controller::{ControllerCmdAsync, ControllerCmdSync},
};
use embassy_executor::Spawner;

pub struct Peripherals {}

impl Peripherals {
    pub fn new(_peripherals: &mut crate::OptionalPeripherals) -> Self {
        unimplemented!();
    }
}

pub async fn ble_stack() -> &'static trouble_host::Stack<'static, DummyController> {
    async { unimplemented!() }.await
}

pub fn driver(_p: Peripherals, _spawner: Spawner, _config: ariel_os_embassy_common::ble::Config) {
    unimplemented!();
}

pub struct DummyController {}

#[derive(Debug)]
pub struct DummyError {}

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
