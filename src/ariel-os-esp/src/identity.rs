use ariel_os_embassy_common::identity;

/// The device's factory-programmed base MAC address, read from eFuse.
pub struct DeviceId([u8; 6]);

impl identity::DeviceId for DeviceId {
    type Bytes = [u8; 6];

    #[expect(
        refining_impl_trait_reachable,
        reason = "making this fallible would be a breaking API change for Ariel OS"
    )]
    fn get() -> Result<Self, core::convert::Infallible> {
        Ok(Self(esp_hal::efuse::Efuse::read_base_mac_address()))
    }

    fn bytes(&self) -> Self::Bytes {
        self.0
    }
}
