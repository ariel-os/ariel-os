use embassy_rp::otp;

pub struct DeviceId(u64);

impl ariel_os_embassy_common::identity::DeviceId for DeviceId {
    type Bytes = [u8; 8];

    fn get() -> Result<Self, impl core::error::Error> {
        // The chip ID is considered to be globally unique.
        let res = embassy_rp::otp::get_chipid();

        match res {
            Ok(id) => Ok(Self(id)),
            Err(err) => match err {
                // OTP page 0, which contains the chip ID, is hard locked (i.e., locked in OTP) to
                // read-only during manufacturing; however it still seems possible to soft lock it
                // to "inaccessible" (see the datasheet's section about permissions on blank
                // devices).
                otp::Error::InvalidPermissions => Err(Error::InvalidPermissions),
                // Not a write operation to OTP.
                otp::Error::UnsupportedModification | otp::Error::Overflow => unreachable!(),
                // OTP indices are hard-coded when reading the chip ID.
                otp::Error::InvalidIndex => unreachable!(),
                // Cannot happen when reading the chip ID.
                otp::Error::UnexpectedFailure(_) => unreachable!(),
            },
        }
    }

    fn bytes(&self) -> Self::Bytes {
        self.0.to_le_bytes()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    InvalidPermissions,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidPermissions => {
                write!(f, "invalid permissions when reading the chip ID in OTP")
            }
        }
    }
}

impl core::error::Error for Error {}
