//! Descriptions of ACE Authorization Servers (AS) and other trust anchors, as viewed from the
//! Resource Server (RS) which coapcore runs on.

use defmt_or_log::{debug, error, trace};

use crate::ace::HeaderMap;
use crate::error::{CredentialError, CredentialErrorDetail};

/// Error type of [`ServerSecurityConfig::render_not_allowed`].
///
/// This represents a failure to express the Request Creation Hints of ACE in a message. Unlike
/// most CoAP rendering errors, this can not just fall back to rendering that produces an Internal
/// Server Error, as that would be misunderstood by the client to mean that the requested operation
/// was being performed and failed at runtime (whereas with this error, the requested operation was
/// not performed). Therefore, no error details can be communicated to the client reliably.
///
/// Implementers are encouraged to log an error when returning this.
pub struct NotAllowedRenderingFailed;

/// A single or collection of authorization servers that a handler trusts to create ACE tokens.
pub trait ServerSecurityConfig: crate::Sealed {
    /// True if the type will at any time need to process tokens at /authz-info
    ///
    /// This is used by the handler implementation to shortcut through some message processing
    /// paths.
    const PARSES_TOKENS: bool;

    /// The way scopes issued with this system as audience by this AS are expressed here.
    type Scope: crate::scope::Scope;

    /// Unprotects a symmetriclly encrypted token and processes the contained [CWT Claims
    /// Set][crate::ace::CwtClaimsSet] into a [`Self::Scope`] and returns the claims.
    ///
    /// The steps are performed together rather than in separate functions because it is yet
    /// unclear how data would precisely be carried around. (Previous iterations of this API had a
    /// `ScopeGenerator` associated type that would carry such data, but that did not scale well to
    /// different kinds of tokens).
    ///
    /// As part of such a dissection it would be preferable to return a decryption key and let the
    /// `ace` module do the decryption, but the key is not dyn safe, and
    /// [`aead::AeadInPlace`](https://docs.rs/aead/latest/aead/trait.AeadInPlace.html) can not be
    /// enum'd around different potential key types because the associated types are fixed length.
    /// (Returning a key in some COSE crypto abstraction may work better).
    ///
    /// Note that the full AAD (COSE's AAD including the external AAD) is built by the caller; the
    /// headers are only passed in to enable the AS to select the right key.
    ///
    /// The buffer is given as heapless buffer rather than an an
    /// [`aead::Buffer`](https://docs.rs/aead/latest/aead/trait.Buffer.html) because the latter is
    /// not on the latest heaples version in its released version.
    #[allow(
        unused_variables,
        reason = "Names are human visible part of API description"
    )]
    #[expect(
        rustdoc::private_intra_doc_links,
        reason = "Method is sealed by private types"
    )]
    // The method is already sealed by the use of a HeaderMap and CwtClaimsSet, but that may become
    // more public over time, and that should not impct this method's publicness.
    fn decrypt_symmetric_token<'buf, const N: usize>(
        &self,
        headers: &HeaderMap,
        aad: &[u8],
        ciphertext_buffer: &'buf mut heapless::Vec<u8, N>,
        _: crate::PrivateMethod,
    ) -> Result<(Self::Scope, crate::ace::CwtClaimsSet<'buf>), CredentialError> {
        Err(CredentialErrorDetail::KeyNotPresent.into())
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
    ) -> Result<(), NotAllowedRenderingFailed> {
        Err(NotAllowedRenderingFailed)
    }
}

/// The default empty configuration that denies all access.
pub struct DenyAll;

impl crate::Sealed for DenyAll {}

impl ServerSecurityConfig for DenyAll {
    const PARSES_TOKENS: bool = false;

    type Scope = core::convert::Infallible;
}

/// An SSC representing unconditionally allowed access, including unencrypted.
pub struct AllowAll;

impl crate::Sealed for AllowAll {}

impl ServerSecurityConfig for AllowAll {
    const PARSES_TOKENS: bool = false;

    type Scope = crate::scope::AllowAll;

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

    fn decrypt_symmetric_token<'buf, const N: usize>(
        &self,
        headers: &HeaderMap,
        aad: &[u8],
        ciphertext_buffer: &'buf mut heapless::Vec<u8, N>,
        _: crate::PrivateMethod,
    ) -> Result<(Self::Scope, crate::ace::CwtClaimsSet<'buf>), CredentialError> {
        use ccm::aead::AeadInPlace;
        use ccm::KeyInit;

        let key = self.as_key_31.ok_or_else(|| {
            error!("Symmetrically encrypted token was sent, but no symmetric key is configured.");
            CredentialErrorDetail::KeyNotPresent
        })?;

        // FIXME: should be something Aes256Ccm::TagLength
        const TAG_SIZE: usize = 16;
        const NONCE_SIZE: usize = 13;

        pub type Aes256Ccm = ccm::Ccm<aes::Aes256, ccm::consts::U16, ccm::consts::U13>;
        let cipher = Aes256Ccm::new((&key).into());

        let nonce: &[u8; NONCE_SIZE] = headers
            .iv
            .ok_or_else(|| {
                error!("IV missing from token.");
                CredentialErrorDetail::InconsistentDetails
            })?
            .try_into()
            .map_err(|_| {
                error!("Token's IV length mismatches algorithm.");
                CredentialErrorDetail::InconsistentDetails
            })?;

        let ciphertext_len = ciphertext_buffer
            .len()
            .checked_sub(TAG_SIZE)
            .ok_or_else(|| {
                error!("Token's ciphertext too short for the algorithm's tag.");
                CredentialErrorDetail::InconsistentDetails
            })?;

        let (ciphertext, tag) = ciphertext_buffer.split_at_mut(ciphertext_len);

        cipher
            .decrypt_in_place_detached(nonce.into(), aad, ciphertext, ccm::Tag::from_slice(tag))
            .map_err(|_| {
                error!("Token decryption failed.");
                CredentialErrorDetail::VerifyFailed
            })?;

        ciphertext_buffer.truncate(ciphertext_len);

        let claims: crate::ace::CwtClaimsSet = minicbor::decode(ciphertext_buffer.as_slice())
            .map_err(|_| CredentialErrorDetail::UnsupportedExtension)?;

        let scope = crate::scope::AifValue::parse(claims.scope)
            .map_err(|_| CredentialErrorDetail::UnsupportedExtension)?;

        Ok((scope.into(), claims))
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
        trace!(
            "Evaluating peer's credential {=[u8]:02x}", // :02x could be :cbor
            id_cred_x.as_full_value()
        );

        #[expect(
            clippy::single_element_loop,
            reason = "Expected to be extended to actual loop soon"
        )]
        for (credential, scope) in &[self.known_edhoc_clients.as_ref()?] {
            trace!("Comparing to {=[u8]:02x}", credential.bytes.as_slice()); // :02x could be :cbor
            if id_cred_x.reference_only() {
                // ad Ok: If our credential has no KID, it can't be recognized in this branch
                if credential.by_kid() == Ok(id_cred_x) {
                    debug!("Peer indicated use of the one preconfigured key by KID.");
                    #[expect(
                        clippy::clone_on_copy,
                        reason = "Lakers items are overly copy happy"
                    )]
                    return Some((credential.clone(), scope.clone()));
                }
            } else {
                // ad Ok: This is always the case for CCSs, but inapplicable eg. for PSKs.
                if credential.by_value() == Ok(id_cred_x) {
                    debug!("Peer indicated use of the one preconfigured credential by value.");
                    #[expect(
                        clippy::clone_on_copy,
                        reason = "Lakers items are overly copy happy"
                    )]
                    return Some((credential.clone(), scope.clone()));
                }
            }
        }

        if let Some(small_scope) = self.nosec_authorization() {
            trace!("Unauthenticated clients are generally accepted, evaluating credential.");
            if let Some(credential_by_value) = id_cred_x.get_ccs() {
                debug!("The unauthorized client provided a usable credential by value.");
                #[expect(clippy::clone_on_copy, reason = "Lakers items are overly copy happy")]
                return Some((credential_by_value.clone(), small_scope.clone()));
            }
        }

        None
    }

    fn render_not_allowed<M: coap_message::MutableWritableMessage>(
        &self,
        message: &mut M,
    ) -> Result<(), NotAllowedRenderingFailed> {
        use coap_message::Code;
        message.set_code(M::Code::new(coap_numbers::code::UNAUTHORIZED).map_err(|_| {
            error!("CoAP stack can not represent Unauthorized responses.");
            NotAllowedRenderingFailed
        })?);
        message
            .set_payload(self.request_creation_hints)
            .map_err(|_| {
                error!("Request creation hints do not fit in error message.");
                NotAllowedRenderingFailed
            })?;
        Ok(())
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
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
            self.request_creation_hints.is_empty(),
            "Overwriting previously configured unauthenticated scope"
        );
        Self {
            request_creation_hints,
            ..self
        }
    }
}
