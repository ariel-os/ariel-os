/// Error type returned from various functions that ingest any input to an authentication or
/// authorization step.
#[derive(Debug)]
pub struct CredentialError {
    #[expect(
        dead_code,
        reason = "This is deliberately unused: The kind is merely a debug helper, and when no logging is active, code should not behave any different no matter in which way the credential processing failed"
    )]
    detail: CredentialErrorDetail,
    pub(crate) position: Option<usize>,
}

/// Classification of a [`CredentialError`].
///
/// The variants in here are mainly used for debug output, and all signify *that*
/// the processing of the token was not successful. This type can be used to
/// construct a [`CredentialError`].
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CredentialErrorDetail {
    /// Input data contains items that violate a protocol.
    ///
    /// This is somewhat fuzzy towards
    /// [`UnsupportedExtension`][CredentialErrorDetail::UnsupportedExtension] if extension points
    /// are not clearly documented or understood in a protocol.
    ProtocolViolation,
    /// Input data contains items that are understood in principle, but not supported by the
    /// implementation.
    ///
    /// In the fuzziness towards [`ProtocolViolation`][CredentialErrorDetail::ProtocolViolation],
    /// it is preferred to err on the side of `UnsupportedExtension`.
    UnsupportedExtension,
    /// Input data uses a COSE algorithm that is not supported by the implementation.
    UnsupportedAlgorithm,
    /// The data looks fine to a point, but exceeds the capacity of the implementation (data or
    /// identifier too long).
    ConstraintExceeded,
    /// Input data is understood, but self-contradictory.
    ///
    /// Example: A COSE encrypted item where the nonce length does not match the algorithm's
    /// requirements.
    InconsistentDetails,
    /// The peer expects to use key material which is not known to this system.
    KeyNotPresent,
    /// Data could be processed and keys were found, but cryptographic verification was
    /// unsuccessful.
    VerifyFailed,
}

impl From<CredentialErrorDetail> for CredentialError {
    fn from(value: CredentialErrorDetail) -> Self {
        Self {
            detail: value,
            position: None,
        }
    }
}

impl From<minicbor::decode::Error> for CredentialError {
    fn from(_: minicbor::decode::Error) -> Self {
        // FIXME: We could try to distinguish "extra field" (that tends to be UnsupportedExtension) from
        // "wrong type" (that tends to be ProtocolViolation).
        Self {
            detail: CredentialErrorDetail::UnsupportedExtension,
            position: None,
        }
    }
}

impl From<minicbor::encode::Error<minicbor_adapters::OutOfSpace>> for CredentialError {
    fn from(_: minicbor::encode::Error<minicbor_adapters::OutOfSpace>) -> Self {
        Self {
            detail: CredentialErrorDetail::ConstraintExceeded,
            position: None,
        }
    }
}
