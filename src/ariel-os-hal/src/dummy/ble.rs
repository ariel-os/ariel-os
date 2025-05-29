use bt_hci::{
    cmd,
    controller::{ControllerCmdAsync, ControllerCmdSync},
};
use embassy_executor::Spawner;
use embassy_sync::once_lock::OnceLock;
use static_cell::StaticCell;

static STACK: OnceLock<trouble_host::Stack<'static, DummyController>> = OnceLock::new();
pub struct Peripherals {}

impl Peripherals {
    pub fn new(_peripherals: &mut crate::OptionalPeripherals) -> Self {
        unimplemented!();
    }
}

pub async fn ble_stack() -> &'static trouble_host::Stack<'static, impl trouble_host::Controller> {
    STACK.get().await
}

pub fn driver(_p: Peripherals, _spawner: &Spawner, config: ariel_os_embassy_common::ble::Config) {
    static HOST_RESOURCES: StaticCell<trouble_host::HostResources<1, 1, 27>> = StaticCell::new();

    let resources = HOST_RESOURCES.init(trouble_host::HostResources::new());

    let stack = trouble_host::new(DummyController {}, resources).set_random_address(config.address);

    if STACK.init(stack).is_err() {
        unreachable!();
    }
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
