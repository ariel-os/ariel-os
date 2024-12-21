//! Descriptions of ACE Authorization Servers (AS) and other trust anchors, as viewed from the
//! Resource Server (RS) which coapcore runs on.

use crate::ace::HeaderMap;

/// The error type for [`ServerSecurityConfig::decrypt_symmetric_token`] and future similar
/// methods.
#[derive(Debug)]
pub enum DecryptionError {
    /// A key was indicated that is not available.
    NoKeyFound,
    /// Details of the encrypted message msimatch.
    ///
    /// For example, the nonce size could not match the nonce size expected by the indicated key's
    /// algorithm.
    InconsistentDetails,
    /// The decryption itself failed, indicating mismatch of the keys.
    DecryptionError,
}

/// A single or collection of authorization servers that a handler trusts to create ACE tokens.
pub trait ServerSecurityConfig: crate::Sealed {
    /// True if the type will at any time need to process tokens at /authz-info
    ///
    /// This is used by the handler implementation to shortcut through some message processing
    /// paths.
    const PARSES_TOKENS: bool;

    /// The way scopes issued with this system as audience by this AS are expressed here.
    type Scope: crate::scope::Scope;
    // Can't `-> Result<impl ..., _>` here because that would capture lifetimes we don't want
    // captured
    type ScopeGenerator: crate::scope::ScopeGenerator<Scope = Self::Scope>;

    /// Unprotect a symmetriclly encrypted token.
    ///
    /// It would be preferable to return a decryption key and let the `ace` module do the
    /// decryption, but the key is not dyn safe, and [`aead::AeadInPlace`] can not be enum'd around
    /// different potential key types because the associated types are fixed length. (Returning a
    /// key in some COSE crypto abstraction may work better).
    ///
    /// Note that the full AAD (COSE's AAD including the external AAD) is built by the caller; the
    /// headers are only passed in to enable the AS to select the right key.
    ///
    /// The buffer is given as heapless buffer rather than an an [`aead::Buffer`] because the
    /// latter is not on the latest heaples version in its released version.
    ///
    /// On success, the ciphertext_buffer contains the decrypted and verified plaintext.
    #[allow(
        unused_variables,
        reason = "Names are human visible part of API description"
    )]
    // The method is already sealed by the use of a HeaderMap, but that may become more public over
    // time, and that should not impct this method's publicness.
    fn decrypt_symmetric_token<const N: usize>(
        &self,
        headers: &HeaderMap,
        aad: &[u8],
        ciphertext_buffer: &mut heapless::Vec<u8, N>,
        _: crate::PrivateMethod,
    ) -> Result<Self::ScopeGenerator, DecryptionError> {
        Err(DecryptionError::NoKeyFound)
    }

    fn own_edhoc_credential(&self) -> Option<(lakers::Credential, lakers::BytesP256ElemLen)> {
        None
    }

    /// Expands an EDHOC `ID_CRED_x` into a parsed `CRED_x` along with the associated
    /// authorizations.
    #[allow(
        unused_variables,
        reason = "Names are human visible part of API description"
    )]
    fn expand_id_cred_x(
        &self,
        id_cred_x: lakers::IdCred,
    ) -> Option<(lakers::Credential, Self::Scope)> {
        None
    }

    /// Generates the scope representing unauthenticated access.
    fn nosec_authorization(&self) -> Option<Self::Scope> {
        None
    }

    /// Render the "not allowed" message in this scenario.
    ///
    /// The default (or any error) renderer produces a generic 4.01 Unauthorized in the handler;
    /// specifics can be useful in ACE scenarios to return a Request Creation Hint.
    #[allow(
        unused_variables,
        reason = "Names are human visible part of API description"
    )]
    fn render_not_allowed<M: coap_message::MutableWritableMessage>(
        &self,
        message: &mut M,
    ) -> Result<(), ()> {
        Err(())
    }
}

/// The default empty configuration that denies all access.
pub struct DenyAll;

impl crate::Sealed for DenyAll {}

impl ServerSecurityConfig for DenyAll {
    const PARSES_TOKENS: bool = false;

    type Scope = core::convert::Infallible;
    type ScopeGenerator = core::convert::Infallible;
}

/// A ScopeGenerator that can be used on [`ServerSecurityConfig`] types that don't process tokens
///
/// Unlike [`core::convert::Infallible`], this produces none of any scope, rather tha none of
/// [`Infallible`][core::convert::Infallible].
pub enum NullGenerator<Scope> {
    _Phantom(core::convert::Infallible, core::marker::PhantomData<Scope>),
}

impl<Scope: crate::scope::Scope> crate::scope::ScopeGenerator for NullGenerator<Scope> {
    type Scope = Scope;

    fn from_token_scope(self, _bytes: &[u8]) -> Result<Self::Scope, crate::scope::InvalidScope> {
        match self {
            NullGenerator::_Phantom(infallible, _) => match infallible {},
        }
    }
}

/// An SSC representing unconditionally allowed access, including unencrypted.
pub struct AllowAll;

impl crate::Sealed for AllowAll {}

impl ServerSecurityConfig for AllowAll {
    const PARSES_TOKENS: bool = false;

    type Scope = crate::scope::AllowAll;
    type ScopeGenerator = NullGenerator<Self::Scope>;

    fn nosec_authorization(&self) -> Option<Self::Scope> {
        Some(crate::scope::AllowAll)
    }
}

/// An implementation of [`ServerSecurityConfig`] that can be extended using builder methods.
///
/// This is very much in flux, and will need further exploration as to inhowmuch this can be
/// type-composed from components.
pub struct ConfigBuilder {
    as_key_31: Option<[u8; 32]>,
    unauthenticated_scope: Option<crate::scope::UnionScope>,
    own_edhoc_credential: Option<(lakers::Credential, lakers::BytesP256ElemLen)>,
    known_edhoc_clients: Option<(lakers::Credential, crate::scope::UnionScope)>,
    request_creation_hints: &'static [u8],
}

impl crate::Sealed for ConfigBuilder {}

impl ServerSecurityConfig for ConfigBuilder {
    // We can't know at build time, assume yes
    const PARSES_TOKENS: bool = true;

    type Scope = crate::scope::UnionScope;
    type ScopeGenerator = crate::scope::ParsingAif<crate::scope::UnionScope>;

    fn decrypt_symmetric_token<const N: usize>(
        &self,
        headers: &HeaderMap,
        aad: &[u8],
        ciphertext_buffer: &mut heapless::Vec<u8, N>,
        _: crate::PrivateMethod,
    ) -> Result<Self::ScopeGenerator, DecryptionError> {
        use ccm::aead::AeadInPlace;
        use ccm::KeyInit;

        let key = self.as_key_31.ok_or_else(|| {
            defmt_or_log::error!("ConfigBuilder is not configured with a symmetric key.");
            DecryptionError::NoKeyFound
        })?;

        // FIXME: should be something Aes256Ccm::TagLength
        const TAG_SIZE: usize = 16;
        const NONCE_SIZE: usize = 13;

        pub type Aes256Ccm = ccm::Ccm<aes::Aes256, ccm::consts::U16, ccm::consts::U13>;
        let cipher = Aes256Ccm::new((&key).into());

        let nonce: &[u8; NONCE_SIZE] = headers
            .iv
            .ok_or_else(|| {
                defmt_or_log::error!("Decryption IV");
                DecryptionError::InconsistentDetails
            })?
            .try_into()
            .map_err(|_| {
                defmt_or_log::error!("IV length mismatch");
                DecryptionError::InconsistentDetails
            })?;

        let ciphertext_len = ciphertext_buffer
            .len()
            .checked_sub(TAG_SIZE)
            .ok_or_else(|| {
                defmt_or_log::error!("Ciphertext too short for tag");
                DecryptionError::InconsistentDetails
            })?;

        let (ciphertext, tag) = ciphertext_buffer.split_at_mut(ciphertext_len);

        cipher
            .decrypt_in_place_detached(nonce.into(), aad, ciphertext, ccm::Tag::from_slice(tag))
            .map_err(|_| {
                defmt_or_log::error!("Decryption failed");
                DecryptionError::DecryptionError
            })?;

        ciphertext_buffer.truncate(ciphertext_len);

        Ok(crate::scope::ParsingAif::new())
    }

    fn nosec_authorization(&self) -> Option<Self::Scope> {
        self.unauthenticated_scope.clone()
    }

    fn own_edhoc_credential(&self) -> Option<(lakers::Credential, lakers::BytesP256ElemLen)> {
        self.own_edhoc_credential
    }

    fn expand_id_cred_x(
        &self,
        id_cred_x: lakers::IdCred,
    ) -> Option<(lakers::Credential, Self::Scope)> {
        use defmt_or_log::{debug, info};

        debug!("Evaluating peer's credenital {}", id_cred_x.as_full_value());

        for (credential, scope) in &[self.known_edhoc_clients.as_ref()?] {
            debug!("Comparing to {}", credential.bytes.as_slice());
            if id_cred_x.reference_only() {
                // ad Ok: If our credential has no KID, it can't be recognized in this branch
                if credential.by_kid() == Ok(id_cred_x) {
                    info!("Peer indicates use of the one preconfigured key");
                    return Some((credential.clone(), scope.clone()));
                }
            } else {
                // ad Ok: This is always the case for CCSs, but inapplicable eg. for PSKs.
                if credential.by_value() == Ok(id_cred_x) {
                    return Some((credential.clone(), scope.clone()));
                }
            }
        }

        debug!("Fell through");
        if let Some(small_scope) = self.nosec_authorization() {
            debug!("There is an unauthenticated scope");
            if let Some(credential_by_value) = id_cred_x.get_ccs() {
                debug!("and get_ccs worked");
                return Some((credential_by_value.clone(), small_scope.clone()));
            }
        }

        None
    }

    fn render_not_allowed<M: coap_message::MutableWritableMessage>(
        &self,
        message: &mut M,
    ) -> Result<(), ()> {
        use coap_message::Code;
        message.set_code(M::Code::new(coap_numbers::code::UNAUTHORIZED).map_err(|_| ())?);
        message
            .set_payload(self.request_creation_hints)
            .map_err(|_| ())?;
        Ok(())
    }
}

impl ConfigBuilder {
    /// Creates an empty server security configuration.
    ///
    /// Without any additional building steps, this is equivalent to [`DenyAll`].
    pub fn new() -> Self {
        Self {
            as_key_31: None,
            unauthenticated_scope: None,
            known_edhoc_clients: None,
            own_edhoc_credential: None,
            request_creation_hints: &[],
        }
    }

    /// Sets a single Authorization Server recognized by a shared `AES-16-128-256` (COSE algorithm
    /// 31) key.
    ///
    /// Scopes are accepted as given by the AS using the AIF REST model as understood by
    /// [`crate::scope::AifValue`].
    ///
    /// # Caveats and evolution
    ///
    /// Currently, this type just supports a single AS; it should therefore only be called once,
    /// and the latest value overwrites any earlier. Building these in type state (as `[(&as_key);
    /// { N+1 }]` (once that is possible) or `(&as_key1, (&as_key2, ()))` will make sense on the
    /// long run, but is not implemented yet.
    ///
    /// Depending on whether the keys are already referenced in a long-lived location, when
    /// implementing that, it can also make sense to allow using any `AsRef<[u8; 32]>` types at
    /// that point.
    ///
    /// Currently, keys are taken as byte sequence. With the expected flexibilization of crypto
    /// backends, this may later allow a more generic type that reflects secure element key slots.
    pub fn with_aif_symmetric_as_aesccm256(self, key: [u8; 32]) -> Self {
        Self {
            as_key_31: Some(key),
            ..self
        }
    }

    /// Allow use of the server within the limits of the given scope by EDHOC clients provided they
    /// present the given credential.
    ///
    /// # Caveats and evolution
    ///
    /// Currently, this type just supports a single credential; it should therefore only be called
    /// once, and the latest value overwrites any earlier. (See
    /// [`Self::with_aif_symmetric_as_aesccm256`] for plans).
    pub fn with_known_edhoc_credential(
        self,
        credential: lakers::Credential,
        scope: crate::scope::UnionScope,
    ) -> Self {
        Self {
            known_edhoc_clients: Some((credential, scope)),
            ..self
        }
    }

    /// Configures an EDHOC credential and private key to be presented by this server.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled, this panics if an own credential has already been
    /// configured.
    pub fn with_own_edhoc_credential(
        self,
        credential: lakers::Credential,
        key: lakers::BytesP256ElemLen,
    ) -> Self {
        debug_assert!(
            self.own_edhoc_credential.is_none(),
            "Overwriting previously configured own credential scope"
        );
        Self {
            own_edhoc_credential: Some((credential, key)),
            ..self
        }
    }

    /// Allow use of the server by unauthenticated clients using the given scope.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled, this panics if an unauthenticated scope has already been
    /// configured.
    pub fn allow_unauthenticated(self, scope: crate::scope::UnionScope) -> Self {
        debug_assert!(
            self.unauthenticated_scope.is_none(),
            "Overwriting previously configured unauthenticated scope"
        );
        Self {
            unauthenticated_scope: Some(scope),
            ..self
        }
    }

    /// Sets the payload of the "Unauthorized" response.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled, this panics if an unauthenticated scope has already been
    /// configured.
    pub fn with_request_creation_hints(self, request_creation_hints: &'static [u8]) -> Self {
        debug_assert!(
            self.request_creation_hints == [],
            "Overwriting previously configured unauthenticated scope"
        );
        Self {
            request_creation_hints,
            ..self
        }
    }
}
