/// A common resource that allows querying the device type and identity.
///
/// See the [book](https://ariel-os.github.io/ariel-os/dev/docs/book/tooling/coap.html) for
/// representation details and privacy implications.
pub(super) struct Identity;

impl coap_handler_implementations::GetRenderable for Identity {
    type Get = Self;

    fn get(&mut self) -> Result<Self::Get, coap_handler_implementations::Error> {
        Ok(Self)
    }
}

// Doing this manually because the device_id_bytes() return value can't be named until TAIT
// lands.
impl minicbor::Encode<()> for Identity {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut (),
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        let identity = ariel_os_identity::device_id_bytes();
        let e = e
            // Currently unregistered
            .tag(minicbor::data::Tag::new(971465540))?
            .map(2 + identity.is_ok() as u64)?
            .u8(1)?
            .str(ariel_os_buildinfo::OS_NAME)?
            .u8(3)?
            .str(ariel_os_buildinfo::BOARD)?;
        if let Ok(identity) = identity {
            e.u8(5)?.bytes(identity.as_ref())?;
        }
        Ok(())
    }
}
