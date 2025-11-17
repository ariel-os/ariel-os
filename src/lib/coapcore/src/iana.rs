//! Constants assigned by IANA.
//!
//! This includes constants that are not yet assigned, but we need some value to work with.
//!
//! Beware that many numbers assigned by IANA also find their way into the [`crate::ace`] module,
//! where the minicbor map keys can use constants.

/// The EDHOC External Authorization Data Registry (EAD Items)
pub(crate) mod edhoc_ead {
    /// ACE-OAuth Access Token
    ///
    /// **Not an official value yet**; described in
    /// [Section 10.8 of the ACE-EDHOC
    /// profile](https://datatracker.ietf.org/doc/html/draft-ietf-ace-edhoc-oscore-profile-06#section-10.8).
    pub(crate) const ACETOKEN: u16 = 20;

    /// (Requesting an) EAD Credential by value
    ///
    /// Value requested but not allocated; described in
    /// [Section 4.14 of the ACE-EDHOC
    /// profile](https://www.ietf.org/archive/id/draft-ietf-ace-edhoc-oscore-profile-09.html#name-requesting-authentication-c).
    pub(crate) const CRED_BY_VALUE: u16 = 15;
}

/// The [COSE Algorithms](https://www.iana.org/assignments/cose/cose.xhtml#algorithms) registry
pub(crate) mod cose_alg {
    /// HMAC 256/256 (from COSE Algorithms)
    pub(crate) const HKDF_HMAC256256: i32 = 5;

    /// AES-CCM-16-64-128
    pub(crate) const AES_CCM_16_64_128: i32 = 10;
}
