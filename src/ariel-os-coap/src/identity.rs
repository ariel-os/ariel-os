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
        encoder: &mut minicbor::Encoder<W>,
        _ctx: &mut (),
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        let identity = ariel_os_identity::device_id_bytes();
        let map_len = if identity.is_ok() {
            3 // OS name, board, identity
        } else {
            2 // only OS name and board
        };
        let encoder = encoder
            // Currently unregistered
            .tag(minicbor::data::Tag::new(3050242967))?
            .map(map_len)?
            .u8(1)?
            .str(ariel_os_buildinfo::OS_NAME)?
            .u8(5)?
            .str(ariel_os_buildinfo::BOARD)?;

        if let Ok(identity) = identity {
            encoder.u8(8)?.bytes(identity.as_ref())?;
        }
        Ok(())
    }
}
