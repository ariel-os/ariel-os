//! Descriptions of trust sources for the Resource Server (RS) which coapcore runs on.
//!
//! These descriptions, expressed in implementations of the [`ServerSecurityConfig`] trait,
//! typically encode known credentials (effectively raw public keys) or ACE Authorization Servers
//! (AS).

use defmt_or_log::{debug, error, trace};

use crate::ace::HeaderMap;
use crate::error::{CredentialError, CredentialErrorDetail};
use crate::generalclaims::{GeneralClaims, Unlimited};
use crate::log_helpers::Cbor;
use crate::time::TimeConstraint;

pub(crate) const MAX_AUD_SIZE: usize = 8;

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
pub trait ServerSecurityConfig {
    /// True if the type will at any time need to process tokens at `/authz-info`.
    ///
    /// This is used by the handler implementation to shortcut through some message processing
    /// paths.
    const PARSES_TOKENS: bool;

    /// True if the type will at any time need to process requests to `/.well-known/edhoc`.
    ///
    /// This is used by the handler implementation to shortcut through some message processing
    /// paths.
    const HAS_EDHOC: bool;

    /// The way scopes issued with this system as audience by this AS are expressed here.
    type GeneralClaims: GeneralClaims;

    /// Unprotects a symmetriclly encrypted token and processes the contained [CWT Claims
    /// Set][crate::ace::CwtClaimsSet] into a [`Self::GeneralClaims`] and returns the full claims.
    ///
    /// The steps are performed together rather than in separate functions because it is yet
    /// unclear how data would precisely be carried around. (Previous iterations of this API had a
    /// `ScopeGenerator` associated type that would carry such data, but that did not scale well to
    /// different kinds of tokens; for example, it would need a `TimeConstraintGenerator` in
    /// parallel because while the token may indicate some time, the issuer's trusted time might be
    /// more limited).
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
    ///
    /// # Errors
    ///
    /// This produces errors if the input (which is typically received from the network) is
    /// malformed or contains unsupported items.
    #[allow(
        unused_variables,
        reason = "Names are human visible part of API description"
    )]
    // The method is already sealed by the use of a HeaderMap and CwtClaimsSet, but that may become
    // more public over time, and that should not impct this method's publicness.
    fn decrypt_symmetric_token<'buf>(
        &self,
        headers: &HeaderMap,
        aad: &[u8],
        ciphertext_buffer: &'buf mut [u8],
    ) -> Result<(Self::GeneralClaims, crate::ace::CwtClaimsSet<'buf>), CredentialError> {
        Err(CredentialErrorDetail::KeyNotPresent.into())
    }

    /// Verify the signature on a symmetrically encrypted token
    ///
    /// `signed_payload` is the payload part of the signed CWT; while it is part of `signed_data` and
    /// can be recovered from it, `signed_data` currently typically resides in a copied buffer
    /// created for signature verification, and `signed_payload` is around inside the caller for
    /// longer. As common with signed data, it should only be parsed once the signature has been
    /// verified.
    ///
    /// Like [`Self::decrypt_symmetric_token()`], this conflates a few aspects, which is tolerated
    /// for the time being.
    ///
    /// # Errors
    ///
    /// This produces errors if the input (which is typically received from the network) is
    /// malformed or contains unsupported items.
    #[allow(
        unused_variables,
        reason = "Names are human visible part of API description"
    )]
    fn verify_asymmetric_token<'b>(
        &self,
        headers: &HeaderMap,
        signed_data: &[u8],
        signature: &[u8],
        signed_payload: &'b [u8],
    ) -> Result<(Self::GeneralClaims, crate::ace::CwtClaimsSet<'b>), CredentialError> {
        Err(CredentialErrorDetail::KeyNotPresent.into())
    }

    /// The credential (by value or by reference) and key that the server reports (and uses) in
    /// incoming EDHOC exchanges (i.e., in the role of the responder).
    fn own_edhoc_credential(&self) -> Option<(lakers::Credential, lakers::BytesP256ElemLen)> {
        None
    }

    /// Expands an EDHOC `ID_CRED_x` into a parsed `CRED_x` along with the associated
    /// authorizations.
    ///
    /// This is currently used for statically configured known static keys, might also be used in
    /// situations when a new EDHOC session is run with a credential previously stored, for example
    /// after an ACE token was submitted.
    #[allow(
        unused_variables,
        reason = "Names are human visible part of API description"
    )]
    fn expand_id_cred_x(
        &self,
        id_cred_x: lakers::IdCred,
    ) -> Option<(lakers::Credential, Self::GeneralClaims)> {
        None
    }

    /// Generates the scope representing unauthenticated access.
    ///
    /// Their time aspect is typically unbounded.
    fn nosec_authorization(&self) -> Option<Self::GeneralClaims> {
        None
    }

    /// Render the "not allowed" message in this scenario.
    ///
    /// The default (or any error) renderer produces a generic 4.01 Unauthorized in the handler;
    /// specifics can be useful in ACE scenarios to return a Request Creation Hint.
    ///
    /// # Errors
    ///
    /// The implementation may fail like [any CoAP response
    /// rendering][coap_handler::Handler::extract_request_data()]. No error details are conveyed;
    /// instead, the caller needs to produce a generic response.
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

impl ServerSecurityConfig for DenyAll {
    const PARSES_TOKENS: bool = false;
    const HAS_EDHOC: bool = false;

    type GeneralClaims = core::convert::Infallible;
}

/// A [`ServerSecurityConfig`] representing unconditionally allowed access without the option for
/// opportunistic EDHOC.
pub struct AllowAll;

impl ServerSecurityConfig for AllowAll {
    const PARSES_TOKENS: bool = false;
    const HAS_EDHOC: bool = false;

    type GeneralClaims = Unlimited<crate::scope::AllowAll>;

    fn nosec_authorization(&self) -> Option<Self::GeneralClaims> {
        Some(Unlimited(crate::scope::AllowAll))
    }
}

/// An implementation of [`ServerSecurityConfig`] that can be extended using builder methods.
///
/// This is very much in flux, and will need further exploration as to inhowmuch this can be
/// type-composed from components.
///
/// Lacking better sources of information, the scope's imporatance is chosen by source: Only
/// preconfigured EDHOC keys are regarded as important, and thus kept around even in the presence
/// of multiple competing token based contexts.
pub struct ConfigBuilder {
    /// Symmetric used when tokens are symmetrically encrypted with AES-CCM-16-128-256
    as_key_31: Option<[u8; 32]>,
    /// Asymmetric key used when tokens are signed with ES256
    ///
    /// Alogn with the key, this also holds the audience value of this RS (as signed tokens only
    /// make sense when the same signing key is used with multiple recipients).
    as_key_neg7: Option<([u8; 32], [u8; 32], heapless::String<MAX_AUD_SIZE>)>,
    unauthenticated_scope: Option<crate::scope::UnionScope>,
    own_edhoc_credential: Option<(lakers::Credential, lakers::BytesP256ElemLen)>,
    known_edhoc_clients: Option<(lakers::Credential, crate::scope::UnionScope)>,
    request_creation_hints: &'static [u8],
}

impl ServerSecurityConfig for ConfigBuilder {
    // We can't know at build time, assume yes
    const PARSES_TOKENS: bool = true;
    const HAS_EDHOC: bool = true;

    type GeneralClaims = ConfigBuilderClaims;

    fn decrypt_symmetric_token<'buf>(
        &self,
        headers: &HeaderMap,
        aad: &[u8],
        ciphertext_buffer: &'buf mut [u8],
    ) -> Result<(Self::GeneralClaims, crate::ace::CwtClaimsSet<'buf>), CredentialError> {
        use ccm::KeyInit;
        use ccm::aead::AeadInPlace;

        pub type Aes256Ccm = ccm::Ccm<aes::Aes256, ccm::consts::U16, ccm::consts::U13>;
        // FIXME: should be something Aes256Ccm::TagLength
        const TAG_SIZE: usize = 16;
        const NONCE_SIZE: usize = 13;

        let key = self.as_key_31.ok_or_else(|| {
            error!("Symmetrically encrypted token was sent, but no symmetric key is configured.");
            CredentialErrorDetail::KeyNotPresent
        })?;

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

        let claims: crate::ace::CwtClaimsSet = minicbor::decode(ciphertext)
            .map_err(|_| CredentialErrorDetail::UnsupportedExtension)?;

        // FIXME: Consider moving into general parser.
        let scope = crate::scope::AifValue::parse(claims.scope)
            .map_err(|_| CredentialErrorDetail::UnsupportedExtension)?
            .into();
        let time_constraint = crate::time::TimeConstraint::from_claims_set(&claims);

        Ok((
            ConfigBuilderClaims {
                scope,
                time_constraint,
                is_important: false,
            },
            claims,
        ))
    }

    fn verify_asymmetric_token<'b>(
        &self,
        headers: &HeaderMap,
        signed_data: &[u8],
        signature: &[u8],
        signed_payload: &'b [u8],
    ) -> Result<(Self::GeneralClaims, crate::ace::CwtClaimsSet<'b>), CredentialError> {
        use p256::ecdsa::{VerifyingKey, signature::Verifier};

        if headers.alg != Some(-7) {
            // ES256
            return Err(CredentialErrorDetail::UnsupportedAlgorithm.into());
        }

        let Some((x, y, rs_audience)) = self.as_key_neg7.as_ref() else {
            return Err(CredentialErrorDetail::KeyNotPresent.into());
        };

        let as_key = VerifyingKey::from_encoded_point(
            &p256::EncodedPoint::from_affine_coordinates(x.into(), y.into(), false),
        )
        .map_err(|_| CredentialErrorDetail::InconsistentDetails)?;
        let signature = p256::ecdsa::Signature::from_slice(signature)
            .map_err(|_| CredentialErrorDetail::InconsistentDetails)?;

        as_key
            .verify(signed_data, &signature)
            .map_err(|_| CredentialErrorDetail::VerifyFailed)?;

        let claims: crate::ace::CwtClaimsSet = minicbor::decode(signed_payload)
            .map_err(|_| CredentialErrorDetail::UnsupportedExtension)?;

        if claims.aud != Some(rs_audience) {
            // FIXME describe better? "Verified but we're not the audience?"
            return Err(CredentialErrorDetail::VerifyFailed.into());
        }

        // FIXME: Consider moving into general parser.
        let scope = crate::scope::AifValue::parse(claims.scope)
            .map_err(|_| CredentialErrorDetail::UnsupportedExtension)?
            .into();
        let time_constraint = crate::time::TimeConstraint::from_claims_set(&claims);

        Ok((
            ConfigBuilderClaims {
                scope,
                time_constraint,
                is_important: false,
            },
            claims,
        ))
    }

    fn nosec_authorization(&self) -> Option<Self::GeneralClaims> {
        self.unauthenticated_scope
            .clone()
            .map(|scope| ConfigBuilderClaims {
                scope,
                time_constraint: TimeConstraint::unbounded(),
                is_important: false,
            })
    }

    fn own_edhoc_credential(&self) -> Option<(lakers::Credential, lakers::BytesP256ElemLen)> {
        self.own_edhoc_credential
    }

    fn expand_id_cred_x(
        &self,
        id_cred_x: lakers::IdCred,
    ) -> Option<(lakers::Credential, Self::GeneralClaims)> {
        trace!(
            "Evaluating peer's credential {}",
            Cbor(id_cred_x.as_full_value())
        );

        #[expect(
            clippy::single_element_loop,
            reason = "Expected to be extended to actual loop soon"
        )]
        for (credential, scope) in &[self.known_edhoc_clients.as_ref()?] {
            trace!("Comparing to {}", Cbor(credential.bytes.as_slice()));
            if id_cred_x.reference_only() {
                // ad Ok: If our credential has no KID, it can't be recognized in this branch
                if credential.by_kid() == Ok(id_cred_x) {
                    debug!("Peer indicated use of the one preconfigured key by KID.");
                    #[expect(
                        clippy::clone_on_copy,
                        reason = "Lakers items are overly copy happy"
                    )]
                    return Some((
                        credential.clone(),
                        ConfigBuilderClaims {
                            scope: scope.clone(),
                            time_constraint: TimeConstraint::unbounded(),
                            is_important: true,
                        },
                    ));
                }
            } else {
                // ad Ok: This is always the case for CCSs, but inapplicable eg. for PSKs.
                if credential.by_value() == Ok(id_cred_x) {
                    debug!("Peer indicated use of the one preconfigured credential by value.");
                    #[expect(
                        clippy::clone_on_copy,
                        reason = "Lakers items are overly copy happy"
                    )]
                    return Some((
                        credential.clone(),
                        ConfigBuilderClaims {
                            scope: scope.clone(),
                            time_constraint: TimeConstraint::unbounded(),
                            is_important: true,
                        },
                    ));
                }
            }
        }

        if let Some(unauthorized_claims) = self.nosec_authorization() {
            trace!("Unauthenticated clients are generally accepted, evaluating credential.");
            if let Some(credential_by_value) = id_cred_x.get_ccs() {
                debug!("The unauthorized client provided a usable credential by value.");
                #[expect(clippy::clone_on_copy, reason = "Lakers items are overly copy happy")]
                return Some((credential_by_value.clone(), unauthorized_claims));
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            as_key_31: None,
            as_key_neg7: None,
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
    #[must_use]
    pub fn with_aif_symmetric_as_aesccm256(self, key: [u8; 32]) -> Self {
        Self {
            as_key_31: Some(key),
            ..self
        }
    }

    /// Sets a single Authorization Server recignized by its `ES256` (COSE algorithm -7) signing
    /// key.
    ///
    /// An audience identifier is taken along with the key; signed tokens are only accepted if they
    /// have that audience.
    ///
    /// Scopes are accepted as given by the AS using the AIF REST model as understood by
    /// [`crate::scope::AifValue`].
    ///
    /// # Caveats and evolution
    ///
    /// Same from [`Self::with_aif_symmetric_as_aesccm256`] apply, minus the considerations for
    /// secure key storage.
    #[must_use]
    pub fn with_aif_asymmetric_es256(
        self,
        x: [u8; 32],
        y: [u8; 32],
        audience: heapless::String<MAX_AUD_SIZE>,
    ) -> Self {
        Self {
            as_key_neg7: Some((x, y, audience)),
            ..self
        }
    }

    /// Allow use of the server within the limits of the given scope by EDHOC clients provided they
    /// present the given credential.
    ///
    /// Unlike many ACE tokens, this credential is accepted without any limitations on time.
    ///
    /// # Caveats and evolution
    ///
    /// Currently, this type just supports a single credential; it should therefore only be called
    /// once, and the latest value overwrites any earlier. (See
    /// [`Self::with_aif_symmetric_as_aesccm256`] for plans).
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

/// An implementation of [`GeneralClaims`] for [`ConfigBuilder`].
///
/// It stores a [`UnionScope`][crate::scope::UnionScope] (effectively a
/// [`AifValue`][crate::scope::AifValue]), a [`TimeConstraint`], and a flag for importance.
#[derive(Debug)]
pub struct ConfigBuilderClaims {
    /// The scope of the claims (providing [`GeneralClaims::scope()`]).
    pub scope: crate::scope::UnionScope,
    /// Time constraints on the claims (providing [`GeneralClaims::time_constraint()`]).
    pub time_constraint: crate::time::TimeConstraint,
    /// Importance of the security context (providing [`GeneralClaims::is_important()`], see there).
    pub is_important: bool,
}

impl GeneralClaims for ConfigBuilderClaims {
    type Scope = crate::scope::UnionScope;

    fn scope(&self) -> &Self::Scope {
        &self.scope
    }

    fn time_constraint(&self) -> crate::time::TimeConstraint {
        self.time_constraint
    }

    fn is_important(&self) -> bool {
        self.is_important
    }
}
