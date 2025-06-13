//! The main workhorse module of this crate.
#![expect(
    clippy::redundant_closure_for_method_calls,
    reason = "all occurrences of this make the code strictly less obvious to understand"
)]

use core::marker::PhantomData;

use coap_message::{
    Code, MessageOption, MinimalWritableMessage, MutableWritableMessage, ReadableMessage,
    error::RenderableOnMinimal,
};
use coap_message_utils::{Error as CoAPError, OptionsExt as _};
use defmt_or_log::{Debug2Format, debug, error, trace};

use crate::generalclaims::{self, GeneralClaims as _};
use crate::helpers::COwn;
use crate::scope::Scope;
use crate::seccfg::ServerSecurityConfig;

use crate::time::TimeProvider;

const MAX_CONTEXTS: usize = 4;
const _MAX_CONTEXTS_CHECK: () = assert!(MAX_CONTEXTS <= COwn::GENERATABLE_VALUES);

/// Helper for cutting branches that can not be reached; could be a provided function of the
/// [`ServerSecurityConfig`], but we need it const.
const fn has_oscore<SSC: ServerSecurityConfig>() -> bool {
    SSC::HAS_EDHOC || SSC::PARSES_TOKENS
}

/// Space allocated for the message into which an EDHOC request is copied to remove EDHOC option
/// and payload.
///
/// embedded-nal-coap uses this max size, and our messages are same size or smaller,
/// so it's a guaranteed fit.
///
/// # FIXME: Having a buffer here should just go away
///
/// Until liboscore can work on an arbitrary message, in particular a
/// `StrippingTheEdhocOptionAndPayloadPart<M>`, we have to create a copy to remove the EDHOC option
/// and payload. (Conveniently, that also sidesteps the need to `downcast_from` to a type libOSCORE
/// knows, but that's not why we do it, that's what downcasting would be for.)
///
/// Furthermore, we need mutable access (something we can't easily gain by just downcasting).
const EDHOC_COPY_BUFFER_SIZE: usize = 1152;

/// A pool of security contexts shareable by several users inside a thread.
type SecContextPool<Crypto, Claims> =
    crate::oluru::OrderedPool<SecContextState<Crypto, Claims>, MAX_CONTEXTS, LEVEL_COUNT>;

/// Copy of the OSCORE option
type OscoreOption = heapless::Vec<u8, 16>;

struct SecContextState<Crypto: lakers::Crypto, GeneralClaims: generalclaims::GeneralClaims> {
    // FIXME: Updating this should also check the timeout.

    // This is Some(...) unless the stage is unusable.
    authorization: Option<GeneralClaims>,
    protocol_stage: SecContextStage<Crypto>,
}

impl<Crypto: lakers::Crypto, GeneralClaims: generalclaims::GeneralClaims> Default
    for SecContextState<Crypto, GeneralClaims>
{
    fn default() -> Self {
        Self {
            authorization: None,
            protocol_stage: SecContextStage::Empty,
        }
    }
}

#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "requiring more memory during connection setup is expected, but the complexity of an inhmogenous pool is currently impractical"
)]
enum SecContextStage<Crypto: lakers::Crypto> {
    Empty,

    // if we have time to spare, we can have empty-but-prepared-with-single-use-random-key entries
    // :-)

    // actionable in response building
    EdhocResponderProcessedM1 {
        responder: lakers::EdhocResponderProcessedM1<Crypto>,
        // May be removed if lakers keeps access to those around if they are set at this point at
        // all
        c_r: COwn,
        c_i: lakers::ConnId,
    },
    //
    EdhocResponderSentM2 {
        responder: lakers::EdhocResponderWaitM3<Crypto>,
        c_r: COwn,
        c_i: lakers::ConnId,
    },

    // FIXME: Also needs a flag for whether M4 was received; if not, it's GC'able
    Oscore(liboscore::PrimitiveContext),
}

const LEVEL_ADMIN: usize = 0;
const LEVEL_AUTHENTICATED: usize = 1;
const LEVEL_ONGOING: usize = 2;
const LEVEL_EMPTY: usize = 3;
// FIXME introduce a level for expired states; they're probably the least priority.
const LEVEL_COUNT: usize = 4;

impl<Crypto: lakers::Crypto, GeneralClaims: generalclaims::GeneralClaims>
    crate::oluru::PriorityLevel for SecContextState<Crypto, GeneralClaims>
{
    fn level(&self) -> usize {
        match &self.protocol_stage {
            SecContextStage::Empty => LEVEL_EMPTY,
            SecContextStage::EdhocResponderProcessedM1 { .. } => {
                // If this is ever tested, means we're outbound message limited, so let's try to
                // get one through rather than pointlessly sending errors
                LEVEL_ONGOING
            }
            SecContextStage::EdhocResponderSentM2 { .. } => {
                // So far, the peer didn't prove they have anything other than entropy (maybe not
                // even that)
                LEVEL_ONGOING
            }
            SecContextStage::Oscore(_) => {
                if self
                    .authorization
                    .as_ref()
                    .is_some_and(|a| a.is_important())
                {
                    LEVEL_ADMIN
                } else {
                    LEVEL_AUTHENTICATED
                }
            }
        }
    }
}

impl<Crypto: lakers::Crypto, GeneralClaims: generalclaims::GeneralClaims>
    SecContextState<Crypto, GeneralClaims>
{
    fn corresponding_cown(&self) -> Option<COwn> {
        match &self.protocol_stage {
            SecContextStage::Empty => None,
            // We're keeping a c_r in there assigned early so that we can find the context when
            // building the response; nothing in the responder is tied to c_r yet.
            SecContextStage::EdhocResponderProcessedM1 { c_r, .. }
            | SecContextStage::EdhocResponderSentM2 { c_r, .. } => Some(*c_r),
            SecContextStage::Oscore(ctx) => COwn::from_kid(ctx.recipient_id()),
        }
    }
}

/// A CoAP handler wrapping inner resources, and adding EDHOC, OSCORE and ACE support.
///
/// While the ACE (authz-info) and EDHOC parts could be implemented as a handler that is to be
/// added into the tree, the OSCORE part needs to wrap the inner handler anyway, and EDHOC and
/// OSCORE are intertwined rather strongly in processing the EDHOC option.
pub struct OscoreEdhocHandler<
    H: coap_handler::Handler,
    Crypto: lakers::Crypto,
    CryptoFactory: Fn() -> Crypto,
    SSC: ServerSecurityConfig,
    RNG: rand_core::RngCore + rand_core::CryptoRng,
    TP: TimeProvider,
> {
    // It'd be tempted to have sharing among multiple handlers for multiple CoAP stacks, but
    // locks for such sharing could still be acquired in a factory (at which point it may make
    // sense to make this a &mut).
    pool: SecContextPool<Crypto, SSC::GeneralClaims>,

    authorities: SSC,

    // FIXME: This currently bakes in the assumption that there is a single tree both for
    // unencrypted and encrypted resources. We may later generalize this by making this a factory,
    // or a single item that has two AsMut<impl Handler> accessors for separate encrypted and
    // unencrypted tree.

    // FIXME That assumption could be easily violated by code changes that don't take the big
    // picture into account. It might make sense to wrap the inner into some
    // zero-cost/build-time-only wrapper that verifies that either request_is_allowed() has been
    // called, or an AuthorizationChecked::Allowed is around.
    inner: H,

    time: TP,

    crypto_factory: CryptoFactory,
    rng: RNG,
}

impl<
    H: coap_handler::Handler,
    Crypto: lakers::Crypto,
    CryptoFactory: Fn() -> Crypto,
    SSC: ServerSecurityConfig,
    RNG: rand_core::RngCore + rand_core::CryptoRng,
    TP: TimeProvider,
> OscoreEdhocHandler<H, Crypto, CryptoFactory, SSC, RNG, TP>
{
    /// Creates a new CoAP server implementation (a [Handler][coap_handler::Handler]), wrapping an
    /// inner (application) handler.
    ///
    /// The main configuration is passed in as `authorities`; the [`seccfg`][crate::seccfg] module
    /// has suitable implementations.
    ///
    /// The time provider is used to evaluate any time limited tokens leniently; choosing a "bad"
    /// time source here (in particular [`crate::time::TimeUnknown`]) leads to acceptance of expired
    /// tokens.
    ///
    /// `rng` and `crypto_factory` are used to pass in platform specific implementations of what
    /// may be accelerated by hardware or reuse operating system infrastructure. Any CSPRNG is
    /// suitable for `rng` (Ariel OS picks `rand_chacha::ChaCha20Rng` at the time of writing); the
    /// crypto factory can come from the `lakers_crypto_rustcrypto::Crypto` or any more specialized
    /// hardware based implementation.
    pub fn new(
        inner: H,
        authorities: SSC,
        crypto_factory: CryptoFactory,
        rng: RNG,
        time: TP,
    ) -> Self {
        Self {
            pool: crate::oluru::OrderedPool::new(),
            inner,
            crypto_factory,
            authorities,
            rng,
            time,
        }
    }

    /// Produces a [`COwn`] (as a recipient identifier) that is both available and not equal to the
    /// peer's recipient identifier.
    fn cown_but_not(&self, c_peer: &[u8]) -> COwn {
        // Let's pick one now already: this allows us to use the identifier in our
        // request data.
        COwn::not_in_iter(
            self.pool
                .iter()
                .filter_map(|entry| entry.corresponding_cown())
                // C_R does not only need to be unique, it also must not be identical
                // to C_I. If it is not expressible as a COwn (as_slice gives []),
                // that's fine and we don't have to consider it.
                .chain(COwn::from_kid(c_peer).as_slice().iter().copied()),
        )
    }

    /// Processes a CoAP request containing a message sent to /.well-known/edhoc.
    ///
    /// The caller has already checked Uri-Path and all other critical options, and that the
    /// request was a POST.
    ///
    /// # Errors
    ///
    /// This produces errors if the input (which is typically received from the network) is
    /// malformed or contains unsupported items.
    #[allow(
        clippy::type_complexity,
        reason = "Type is subset of RequestData that has no alias in the type"
    )]
    fn extract_edhoc<M: ReadableMessage>(
        &mut self,
        request: &M,
    ) -> Result<OwnRequestData<Result<H::RequestData, H::ExtractRequestError>>, CoAPError> {
        let own_identity = self
            .authorities
            .own_edhoc_credential()
            // 4.04 Not Found does not precisely capture it when we later support reverse flow, but
            // until then, "there is no EDHOC" is a good rendition of lack of own key.
            .ok_or_else(CoAPError::not_found)?;

        let (first_byte, edhoc_m1) = request.payload().split_first().ok_or_else(|| {
            error!("Empty EDHOC requests (reverse flow) not supported yet.");
            CoAPError::bad_request()
        })?;
        let starts_with_true = first_byte == &0xf5;

        if starts_with_true {
            trace!("Processing incoming EDHOC message 1");
            let message_1 =
                &lakers::EdhocMessageBuffer::new_from_slice(edhoc_m1).map_err(too_small)?;

            let (responder, c_i, ead_1) = lakers::EdhocResponder::new(
                (self.crypto_factory)(),
                lakers::EDHOCMethod::StatStat,
                own_identity.1,
                own_identity.0,
            )
            .process_message_1(message_1)
            .map_err(render_error)?;

            if ead_1.is_some_and(|e| e.is_critical) {
                error!("Critical EAD1 item received, aborting");
                // FIXME: send error message
                return Err(CoAPError::bad_request());
            }

            let c_r = self.cown_but_not(c_i.as_slice());

            let _evicted = self.pool.force_insert(SecContextState {
                protocol_stage: SecContextStage::EdhocResponderProcessedM1 {
                    c_r,
                    c_i,
                    responder,
                },
                authorization: self.authorities.nosec_authorization(),
            });

            Ok(OwnRequestData::EdhocOkSend2(c_r))
        } else {
            // for the time being we'll only take the EDHOC option
            error!(
                "Sending EDHOC message 3 to the /.well-known/edhoc resource is not supported yet"
            );
            Err(CoAPError::bad_request())
        }
    }

    /// Builds an EDHOC response message 2 after successful processing of a request in
    /// [`Self::extract_edhoc()`]
    ///
    /// # Errors
    ///
    /// This produces errors if the input (which is typically received from the network) is
    /// malformed or contains unsupported items.
    fn build_edhoc_message_2<M: MutableWritableMessage>(
        &mut self,
        response: &mut M,
        c_r: COwn,
    ) -> Result<(), Result<CoAPError, M::UnionError>> {
        let message_2 = self.pool.lookup(
            |c| c.corresponding_cown() == Some(c_r),
            |matched| -> Result<_, lakers::EDHOCError> {
                // temporary default will not live long (and may be only constructed if
                // prepare_message_2 fails)
                let taken = core::mem::take(matched);
                let SecContextState {
                    protocol_stage:
                        SecContextStage::EdhocResponderProcessedM1 {
                            c_r: matched_c_r,
                            c_i,
                            responder: taken,
                        },
                    authorization,
                } = taken
                else {
                    todo!();
                };
                debug_assert_eq!(
                    matched_c_r, c_r,
                    "The first lookup function ensured this property"
                );
                let (responder, message_2) = taken
                    // We're sending our ID by reference: we have a CCS and don't expect anyone to
                    // run EDHOC with us who can not verify who we are (and from the CCS there is
                    // no better way). Also, conveniently, this covers our privacy well.
                    // (Sending ByValue would still work)
                    .prepare_message_2(
                        lakers::CredentialTransfer::ByReference,
                        Some(c_r.into()),
                        &None,
                    )?;
                *matched = SecContextState {
                    protocol_stage: SecContextStage::EdhocResponderSentM2 {
                        responder,
                        c_i,
                        c_r,
                    },
                    authorization,
                };
                Ok(message_2)
            },
        );

        let message_2 = match message_2 {
            Some(Ok(m)) => m,
            Some(Err(e)) => {
                render_error(e).render(response).map_err(Err)?;
                return Ok(());
            }
            // Can't happen with the current CoAP stack, but might happen when there is some
            // possibly possible concurrency.
            None => {
                response.set_code(
                    M::Code::new(coap_numbers::code::INTERNAL_SERVER_ERROR)
                        .map_err(|x| Err(x.into()))?,
                );
                return Ok(());
            }
        };

        // FIXME: Why does the From<O> not do the map_err?
        response.set_code(M::Code::new(coap_numbers::code::CHANGED).map_err(|x| Err(x.into()))?);

        response
            .set_payload(message_2.as_slice())
            .map_err(|x| Err(x.into()))?;

        Ok(())
    }

    /// Processes a CoAP request containing an OSCORE option and possibly an EDHOC option.
    ///
    /// # Errors
    ///
    /// This produces errors if the input (which is typically received from the network) is
    /// malformed, contains unsupported items, or is too large for the allocated buffers.
    #[allow(
        clippy::type_complexity,
        reason = "type is subset of RequestData that has no alias in the type"
    )]
    fn extract_oscore_edhoc<M: ReadableMessage>(
        &mut self,
        request: &M,
        oscore_option: &OscoreOption,
        with_edhoc: bool,
    ) -> Result<OwnRequestData<Result<H::RequestData, H::ExtractRequestError>>, CoAPError> {
        let payload = request.payload();

        // We know this to not fail b/c we only got here due to its presence
        let oscore_option = liboscore::OscoreOption::parse(oscore_option).map_err(|_| {
            error!("OSCORE option could not be parsed");
            CoAPError::bad_option(coap_numbers::option::OSCORE)
        })?;

        let kid = COwn::from_kid(oscore_option.kid().ok_or_else(|| {
            error!("OSCORE KID is not in our value space");
            CoAPError::bad_option(coap_numbers::option::OSCORE)
        })?)
        // same as if it's not found in the pool
        .ok_or_else(CoAPError::bad_request)?;
        // If we don't make progress, we're dropping it altogether. Unless we use the
        // responder we might legally continue (because we didn't send data to EDHOC), but
        // once we've received something that (as we now know) looks like a message 3 and
        // isn't processable, it's unlikely that another one would come up and be.
        let taken = self
            .pool
            .lookup(|c| c.corresponding_cown() == Some(kid), core::mem::take)
            // following RFC8613 Section 8.2 item 2.2
            .ok_or_else(|| {
                error!("No security context with this KID.");
                // FIXME unauthorized (unreleased in coap-message-utils)
                CoAPError::bad_request()
            })?;

        let (taken, front_trim_payload) = if with_edhoc {
            if !SSC::HAS_EDHOC {
                unreachable!(
                    "In this variant, that option is not consumed so the argument is always false"
                );
            }
            self.process_edhoc_in_payload(payload, taken)?
        } else {
            (taken, 0)
        };

        let SecContextState {
            protocol_stage: SecContextStage::Oscore(mut oscore_context),
            authorization: Some(authorization),
        } = taken
        else {
            // FIXME: How'd we even get there? Should this be unreachable?
            error!("Found empty security context.");
            return Err(CoAPError::bad_request());
        };

        if !authorization
            .time_constraint()
            .is_valid_with(&mut self.time)
        {
            // Token expired.
            //
            // By returning early after having taken the context, we discard it completely.
            //
            // FIXME: Find out whether there is any merit in retaining the security context without
            // authorization at all -- it may be that for the purpose of time series it is useful
            // to retain the authorization (if there is some kind of renewal tokens / token
            // series).
            debug!("Discarding expired context");
            return Err(CoAPError::bad_request());
        }

        // See comment on EDHOC_COPY_BUFFER_SIZE
        let mut read_copy = [0u8; EDHOC_COPY_BUFFER_SIZE];
        let mut code_copy = 0;
        let mut copied_message = coap_message_implementations::inmemory_write::Message::new(
            &mut code_copy,
            &mut read_copy[..],
        );
        // We could also do
        //     copied_message.set_from_message(request);
        // if we specified a "hiding EDHOC" message view.
        copied_message.set_code(request.code().into());
        // This may panic in theory on options being added in the wrong sequence; as we
        // don't downcast, we don't get the information on whether the underlying
        // implementation produces the options in the right sequence. Practically
        // (typically, and concretely in Ariel OS), it is given. (And it's not like we have
        // a fallback: inmemory_write has no more expensive option for reshuffling).
        for opt in request.options() {
            if opt.number() == coap_numbers::option::EDHOC {
                continue;
            }
            copied_message
                .add_option(opt.number(), opt.value())
                .map_err(|_| {
                    error!("Options produced in unexpected sequence.");
                    CoAPError::internal_server_error()
                })?;
        }
        #[allow(clippy::indexing_slicing, reason = "slice fits by construction")]
        copied_message
            .set_payload(&payload[front_trim_payload..])
            .map_err(|_| {
                error!("Unexpectedly large EDHOC-less message");
                CoAPError::internal_server_error()
            })?;

        let decrypted = liboscore::unprotect_request(
            &mut copied_message,
            oscore_option,
            &mut oscore_context,
            |request| {
                if authorization.scope().request_is_allowed(request) {
                    AuthorizationChecked::Allowed(self.inner.extract_request_data(request))
                } else {
                    AuthorizationChecked::NotAllowed
                }
            },
        );

        // With any luck, this never moves out.
        //
        // Storing it even on decryption failure to avoid DoS from the first message (but
        // FIXME, should we increment an error count and lower priority?)
        #[allow(clippy::used_underscore_binding, reason = "used only in debug asserts")]
        let _evicted = self.pool.force_insert(SecContextState {
            protocol_stage: SecContextStage::Oscore(oscore_context),
            authorization: Some(authorization),
        });
        debug_assert!(
            matches!(
                _evicted,
                Some(SecContextState {
                    protocol_stage: SecContextStage::Empty,
                    ..
                }) | None
            ),
            "A Default (Empty) was placed when an item was taken, which should have the lowest priority"
        );

        let Ok((correlation, extracted)) = decrypted else {
            // FIXME is that the right code?
            error!("Decryption failure");
            return Err(CoAPError::unauthorized());
        };

        Ok(OwnRequestData::EdhocOscoreRequest {
            kid,
            correlation,
            extracted,
        })
    }

    /// Processes an EDHOC message 3 at the beginning of a payload, and returns the number of bytes
    /// that were in the message.
    ///
    /// # Errors
    ///
    /// This produces errors if the input (which is typically received from the network) is
    /// malformed or contains unsupported items.
    ///
    /// # Panics
    ///
    /// This panics if cipher suite negotiation passed for a suite whose algorithms are unsupported
    /// in libOSCORE.
    fn process_edhoc_in_payload(
        &self,
        payload: &[u8],
        sec_context_state: SecContextState<Crypto, SSC::GeneralClaims>,
    ) -> Result<(SecContextState<Crypto, SSC::GeneralClaims>, usize), CoAPError> {
        // We're not supporting block-wise here -- but could later, to the extent we support
        // outer block-wise.

        // Workaround for https://github.com/openwsn-berkeley/lakers/issues/255
        let mut decoder = minicbor::decode::Decoder::new(payload);
        let _ = decoder
            .decode::<&minicbor::bytes::ByteSlice>()
            .map_err(|_| {
                error!("EDHOC request is not prefixed with valid CBOR.");
                CoAPError::bad_request()
            })?;
        let cutoff = decoder.position();

        let sec_context_state = if let SecContextState {
            protocol_stage:
                SecContextStage::EdhocResponderSentM2 {
                    responder,
                    c_r,
                    c_i,
                },
            .. // Discarding original authorization
        } = sec_context_state
        {
            #[allow(clippy::indexing_slicing, reason = "slice fits by construction")]
            let msg_3 = lakers::EdhocMessageBuffer::new_from_slice(&payload[..cutoff])
                .map_err(too_small)?;

            let (responder, id_cred_i, mut ead_3) =
                responder.parse_message_3(&msg_3).map_err(render_error)?;

            let mut cred_i_and_authorization = None;

            if let Some(lakers::EADItem { label: crate::iana::edhoc_ead::ACETOKEN, value: Some(value), .. }) = ead_3.take() {
                match crate::ace::process_edhoc_token(value.as_slice(), &self.authorities) {
                    Ok(ci_and_a) => cred_i_and_authorization = Some(ci_and_a),
                    Err(e) => {
                        error!("Received unprocessable token {=[u8]:02x}, error: {}", value.as_slice(), Debug2Format(&e)); // :02x could be :cbor
                    }
                }
            }

            if cred_i_and_authorization.is_none() {
                cred_i_and_authorization = self
                    .authorities
                    .expand_id_cred_x(id_cred_i);
            }

            let Some((cred_i, authorization)) = cred_i_and_authorization else {
                // FIXME: send better message; how much variability should we allow?
                error!("Peer's ID_CRED_I could not be resolved into CRED_I.");
                return Err(CoAPError::bad_request());
            };

            if let Some(ead_3) = ead_3 {
                if ead_3.is_critical {
                    error!("Critical EAD3 item received, aborting");
                    // FIXME: send error message
                    return Err(CoAPError::bad_request());
                }
            }

            let (responder, _prk_out) =
                responder.verify_message_3(cred_i).map_err(render_error)?;

            let mut responder = responder.completed_without_message_4().map_err(render_error)?;

            // Once this gets updated beyond Lakers 0.7.2 (likely to 0.8), this will be needed:
            // let mut responder = responder.completed_without_message_4()
            //     .map_err(render_error)?;

            let oscore_secret = responder.edhoc_exporter(0u8, &[], 16); // label is 0
            let oscore_salt = responder.edhoc_exporter(1u8, &[], 8); // label is 1
            let oscore_secret = &oscore_secret[..16];
            let oscore_salt = &oscore_salt[..8];

            let sender_id = c_i.as_slice();
            let recipient_id = c_r.as_slice();

            // FIXME probe cipher suite
            let hkdf = liboscore::HkdfAlg::from_number(crate::iana::cose_alg::HKDF_HMAC256256).unwrap();
            let aead = liboscore::AeadAlg::from_number(crate::iana::cose_alg::AES_CCM_16_64_128).unwrap();

            let immutables = liboscore::PrimitiveImmutables::derive(
                hkdf,
                oscore_secret,
                oscore_salt,
                None,
                aead,
                sender_id,
                recipient_id,
            )
            // FIXME convert error
            .unwrap();

            let context = liboscore::PrimitiveContext::new_from_fresh_material(immutables);

            SecContextState {
                protocol_stage: SecContextStage::Oscore(context),
                authorization: Some(authorization),
            }
        } else {
            // Return the state. Best bet is that it was already advanced to an OSCORE
            // state, and the peer sent message 3 with multiple concurrent in-flight
            // messages. We're ignoring the EDHOC value and continue with OSCORE
            // processing.
            sec_context_state
        };

        debug!(
            "Processing {} bytes at start of message into new EDHOC Message 3.",
            cutoff
        );

        Ok((sec_context_state, cutoff))
    }

    /// Builds an OSCORE response message after successful processing of a request in
    /// [`Self::extract_oscore_edhoc()`].
    ///
    /// # Errors
    ///
    /// This produces errors if requests are processed in unexpected out-of-order ways.
    ///
    /// # Panics
    ///
    /// Panics if the writable message is not a
    /// [`coap_message_implementations::inmemory_write::Message`]. See module level documentation
    /// for details.
    fn build_oscore_response<M: MutableWritableMessage>(
        &mut self,
        response: &mut M,
        kid: COwn,
        mut correlation: liboscore::raw::oscore_requestid_t,
        extracted: AuthorizationChecked<Result<H::RequestData, H::ExtractRequestError>>,
    ) -> Result<(), Result<CoAPError, M::UnionError>> {
        response.set_code(M::Code::new(coap_numbers::code::CHANGED).map_err(|x| Err(x.into()))?);

        // BIG FIXME: We have currently no way to rewind through a message once we've started
        // building it.
        //
        // We *could* to some extent rewind if we sent things out in an error, but that error would
        // need to have a clone of the correlation data, and that means that all our errors would
        // become much larger than needed, because they all consume own sequence numbers.
        //
        // Putting this aside for the moment and accepting that in some few cases there will be
        // unexpected options from the first attempt to render in the eventual message (in theory
        // even panics when a payload is already set and then the error adds options), but the
        // easiest path there is to wait for the next iteration of handler where everything is
        // async and the handler has a method to start writing to the message (which kind'a
        // implies rewinding)

        self.pool
                    .lookup(|c| c.corresponding_cown() == Some(kid), |matched| {
                        // Not checking authorization any more: we don't even have access to the
                        // request any more, that check was done.
                        let SecContextState { protocol_stage: SecContextStage::Oscore(oscore_context), .. } = matched else {
                            // State vanished before response was built.
                            //
                            // As it is, depending on the CoAP stack, there may be DoS if a peer
                            // can send many requests before the server starts rendering responses.
                            error!("State vanished before response was built.");
                            return Err(CoAPError::internal_server_error());
                        };

                        let response = coap_message_implementations::inmemory_write::Message::downcast_from(response)
                            .expect("OSCORE handler currently requires a response message implementation that is of fixed type");

                        response.set_code(coap_numbers::code::CHANGED);

                        if liboscore::protect_response(
                            response,
                            // SECURITY BIG FIXME: How do we make sure that our correlation is really for
                            // what we find in the pool and not for what wound up there by the time we send
                            // the response? (Can't happen with the current stack, but conceptually there
                            // should be a tie; carry the OSCORE context in an owned way?).
                            oscore_context,
                            &mut correlation,
                            |response| match extracted {
                                AuthorizationChecked::Allowed(Ok(extracted)) => match self.inner.build_response(response, extracted) {
                                    Ok(()) => {
                                        // All fine, response was built
                                    },
                                    // One attempt to render rendering errors
                                    // FIXME rewind message
                                    Err(e) => {
                                        error!("Rendering successful extraction failed with {:?}", Debug2Format(&e));
                                        match e.render(response) {
                                            Ok(()) => {
                                                error!("Error rendered.");
                                            },
                                            Err(e2) => {
                                                error!("Error could not be rendered: {:?}.", Debug2Format(&e2));
                                                // FIXME rewind message
                                                response.set_code(coap_numbers::code::INTERNAL_SERVER_ERROR);
                                            }
                                        }
                                    },
                                },
                                AuthorizationChecked::Allowed(Err(inner_request_error)) => {
                                    error!("Extraction failed with {:?}.", Debug2Format(&inner_request_error));
                                    match inner_request_error.render(response) {
                                        Ok(()) => {
                                            error!("Original error rendered successfully.");
                                        },
                                        Err(e) => {
                                            error!("Original error could not be rendered due to {:?}:", Debug2Format(&e));
                                            // Two attempts to render extraction errors
                                            // FIXME rewind message
                                            match e.render(response) {
                                                Ok(()) => {
                                                    error!("Error was rendered fine.");
                                                },
                                                Err(e2) => {
                                                    error!("Rendering error caused {:?}.", Debug2Format(&e2));
                                                    // FIXME rewind message
                                                    response.set_code(
                                                        coap_numbers::code::INTERNAL_SERVER_ERROR,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                                AuthorizationChecked::NotAllowed => {
                                    if self.authorities.render_not_allowed(response).is_err() {
                                        // FIXME rewind message
                                        response.set_code(coap_numbers::code::UNAUTHORIZED);
                                    }
                                }
                            },
                        )
                        .is_err()
                        {
                            error!("Oups, responding with weird state");
                            // todo!("Thanks to the protect API we've lost access to our response");
                        }
                        Ok(())
                    })
                .transpose().map_err(Ok)?;
        Ok(())
    }

    /// Processes a CoAP request containing an ACE token for /authz-info.
    ///
    /// This assumes that the content format was pre-checked to be application/ace+cbor, both in
    /// Content-Format and Accept (absence is fine too), no other critical options are present,
    /// and the code was POST.
    ///
    /// # Errors
    ///
    /// This produces errors if the input (which is typically received from the network) is
    /// malformed or contains unsupported items.
    fn extract_token(
        &mut self,
        payload: &[u8],
    ) -> Result<crate::ace::AceCborAuthzInfoResponse, CoAPError> {
        let mut nonce2 = [0; crate::ace::OWN_NONCE_LEN];
        self.rng.fill_bytes(&mut nonce2);

        let (response, oscore, generalclaims) =
            crate::ace::process_acecbor_authz_info(payload, &self.authorities, nonce2, |nonce1| {
                // This preferably (even exclusively) produces EDHOC-ideal recipient IDs, but as long
                // as we're having more of those than slots, no point in not reusing the code.
                self.cown_but_not(nonce1)
            })
            .map_err(|e| {
                error!("Sending out error:");
                error!("{}", Debug2Format(&e));
                e.position
                    // FIXME: Could also come from processing inner
                    .map_or(CoAPError::bad_request(), CoAPError::bad_request_with_rbep)
            })?;

        debug!(
            "Established OSCORE context with recipient ID {:?} and authorization {:?} through ACE-OSCORE",
            oscore.recipient_id(),
            Debug2Format(&generalclaims)
        );
        // FIXME: This should be flagged as "unconfirmed" for rapid eviction, as it could be part
        // of a replay.
        let _evicted = self.pool.force_insert(SecContextState {
            protocol_stage: SecContextStage::Oscore(oscore),
            authorization: Some(generalclaims),
        });

        Ok(response)
    }
}

/// A wrapper around for a handler's inner RequestData used by [`OscoreEdhocHandler`] both for
/// OSCORE and plain text requests.
///
/// Other crates should not rely on this (but making it an enum wrapped in a struct for privacy is
/// considered excessive at this point).
#[doc(hidden)]
pub enum AuthorizationChecked<I> {
    /// Middleware checks were successful, data was extracted
    Allowed(I),
    /// Middleware checks failed, return a 4.01 Unauthorized
    NotAllowed,
}

/// Request state created by an [`OscoreEdhocHandler`] for successful non-plaintext cases.
///
/// Other crates should not rely on this (but making it an enum wrapped in a struct for privacy is
/// considered excessive at this point).
#[doc(hidden)]
pub enum OwnRequestData<I> {
    // Taking a small state here: We already have a slot in the pool, storing the big data there
    #[expect(private_interfaces, reason = "should be addressed eventually")]
    EdhocOkSend2(COwn),
    // Could have a state Message3Processed -- but do we really want to implement that? (like, just
    // use the EDHOC option)
    EdhocOscoreRequest {
        #[expect(private_interfaces, reason = "should be addressed eventually")]
        kid: COwn,
        correlation: liboscore::raw::oscore_requestid_t,
        extracted: AuthorizationChecked<I>,
    },
    ProcessedToken(crate::ace::AceCborAuthzInfoResponse),
}

// FIXME: It'd be tempting to implement Drop for Response to set the slot back to Empty -- but
// that'd be easier if we could avoid the Drop during enum destructuring, which AIU is currently
// not supported in match or let destructuring. (But our is_gc_eligible should be good enough
// anyway).

/// Renders a [`lakers::EdhocBufferError`] into the common Error type.
///
/// It is yet to be determined whether anything more informative should be returned (likely it
/// should; maybe Request Entity Too Large or some error code about unusable credential.
///
/// Places using this function may be simplified if From/Into is specified (possibly after
/// enlarging the Error type)
#[track_caller]
#[expect(
    clippy::needless_pass_by_value,
    reason = "ergonomics at the call sites need this"
)]
fn too_small(e: lakers::EdhocBufferError) -> CoAPError {
    #[allow(
        clippy::match_same_arms,
        reason = "https://github.com/rust-lang/rust-clippy/issues/13522"
    )]
    match e {
        lakers::EdhocBufferError::BufferAlreadyFull => {
            error!("Lakers buffer size exceeded: Buffer full.");
        }
        lakers::EdhocBufferError::SliceTooLong => {
            error!("Lakers buffer size exceeded: Slice too long.");
        }
    }
    CoAPError::bad_request()
}

/// Renders a [`lakers::EDHOCError`] into the common Error type.
///
/// It is yet to be decided based on the EDHOC specification which
/// [`EDHOCError`][lakers::EDHOCError] values would be reported with precise data, and which should
/// rather produce a generic response.
///
/// Places using this function may be simplified if From/Into is specified (possibly after
/// enlarging the Error type)
#[track_caller]
#[expect(
    clippy::needless_pass_by_value,
    reason = "ergonomics at the call sites need this"
)]
fn render_error(e: lakers::EDHOCError) -> CoAPError {
    #[allow(
        clippy::match_same_arms,
        reason = "https://github.com/rust-lang/rust-clippy/issues/13522"
    )]
    match e {
        lakers::EDHOCError::UnexpectedCredential => error!("Lakers error: UnexpectedCredential"),
        lakers::EDHOCError::MissingIdentity => error!("Lakers error: MissingIdentity"),
        lakers::EDHOCError::IdentityAlreadySet => error!("Lakers error: IdentityAlreadySet"),
        lakers::EDHOCError::MacVerificationFailed => error!("Lakers error: MacVerificationFailed"),
        lakers::EDHOCError::UnsupportedMethod => error!("Lakers error: UnsupportedMethod"),
        lakers::EDHOCError::UnsupportedCipherSuite => {
            error!("Lakers error: UnsupportedCipherSuite");
        }
        lakers::EDHOCError::ParsingError => error!("Lakers error: ParsingError"),
        lakers::EDHOCError::EncodingError => error!("Lakers error: EncodingError"),
        lakers::EDHOCError::CredentialTooLongError => {
            error!("Lakers error: CredentialTooLongError");
        }
        lakers::EDHOCError::EadLabelTooLongError => error!("Lakers error: EadLabelTooLongError"),
        lakers::EDHOCError::EadTooLongError => error!("Lakers error: EadTooLongError"),
        lakers::EDHOCError::EADUnprocessable => error!("Lakers error: EADUnprocessable"),
        lakers::EDHOCError::AccessDenied => error!("Lakers error: AccessDenied"),
        _ => error!("Lakers error (unknown)"),
    }
    CoAPError::bad_request()
}

/// An Either-style type used internally by [`OscoreEdhocHandler`].
///
/// Other crates should not rely on this (but making it an enum wrapped in a struct for privacy is
/// considered excessive at this point).
#[doc(hidden)]
#[derive(Debug)]
pub enum OrInner<O, I> {
    Own(O),
    Inner(I),
}

impl<O, I> From<O> for OrInner<O, I> {
    fn from(own: O) -> Self {
        OrInner::Own(own)
    }
}

impl<O: RenderableOnMinimal, I: RenderableOnMinimal> RenderableOnMinimal for OrInner<O, I> {
    type Error<IE>
        = OrInner<O::Error<IE>, I::Error<IE>>
    where
        IE: RenderableOnMinimal,
        IE: core::fmt::Debug;
    fn render<M: MinimalWritableMessage>(
        self,
        msg: &mut M,
    ) -> Result<(), Self::Error<M::UnionError>> {
        match self {
            OrInner::Own(own) => own.render(msg).map_err(OrInner::Own),
            OrInner::Inner(inner) => inner.render(msg).map_err(OrInner::Inner),
        }
    }
}

impl<
    H: coap_handler::Handler,
    Crypto: lakers::Crypto,
    CryptoFactory: Fn() -> Crypto,
    SSC: ServerSecurityConfig,
    RNG: rand_core::RngCore + rand_core::CryptoRng,
    TP: TimeProvider,
> coap_handler::Handler for OscoreEdhocHandler<H, Crypto, CryptoFactory, SSC, RNG, TP>
{
    type RequestData = OrInner<
        OwnRequestData<Result<H::RequestData, H::ExtractRequestError>>,
        AuthorizationChecked<H::RequestData>,
    >;

    type ExtractRequestError = OrInner<CoAPError, H::ExtractRequestError>;
    type BuildResponseError<M: MinimalWritableMessage> =
        OrInner<Result<CoAPError, M::UnionError>, H::BuildResponseError<M>>;

    #[expect(clippy::too_many_lines, reason = "no good refactoring point known")]
    fn extract_request_data<M: ReadableMessage>(
        &mut self,
        request: &M,
    ) -> Result<Self::RequestData, Self::ExtractRequestError> {
        use OrInner::{Inner, Own};

        #[derive(Default, Debug)]
        // SSC could be boolean AS_PARSES_TOKENS but not until feature(generic_const_exprs)
        enum Recognition<SSC: ServerSecurityConfig> {
            #[default]
            Start,
            /// Seen an OSCORE option
            Oscore { oscore: OscoreOption },
            /// Seen an OSCORE option and an EDHOC option
            Edhoc { oscore: OscoreOption },
            /// Seen path ".well-known" (after not having seen an OSCORE option)
            WellKnown,
            /// Seen path ".well-known" and "edhoc"
            WellKnownEdhoc,
            /// Seen path "authz-info"
            // FIXME: Should we allow arbitrary paths here?
            //
            // Also, in the !PARSES_TOKENS case, this would ideally be marked uninhabitable, but that's
            // hard to express in associated types and functions.
            //
            // Also, the PhantomData doesn't actually need to be precisely in here, but it needs to
            // be somewhere.
            AuthzInfo(PhantomData<SSC>),
            /// Seen anything else (where the request handler, or more likely the ACL filter, will
            /// trip over the critical options)
            Unencrypted,
        }
        #[allow(clippy::enum_glob_use, reason = "local use")]
        use Recognition::*;

        impl<SSC: ServerSecurityConfig> Recognition<SSC> {
            /// Given a state and an option, produce the next state and whether the option should
            /// be counted as consumed for the purpose of assessing .well-known/edchoc's
            /// [`ignore_elective_others()`][coap_message_utils::option_processing::OptionsExt::ignore_elective_others].
            fn update(self, o: &impl MessageOption) -> (Self, bool) {
                use coap_numbers::option;

                match (self, o.number(), o.value()) {
                    // FIXME: Store full value (but a single one is sufficient while we do EDHOC
                    // extraction)
                    (Start, option::OSCORE, optval) if has_oscore::<SSC>() => match optval.try_into() {
                        Ok(oscore) => (Oscore { oscore }, false),
                        _ => (Start, true),
                    },
                    (Start, option::URI_PATH, b".well-known") if SSC::HAS_EDHOC /* or anything else that lives in here */ => (WellKnown, false),
                    (Start, option::URI_PATH, b"authz-info") if SSC::PARSES_TOKENS => {
                        (AuthzInfo(PhantomData), false)
                    }
                    (Start, option::URI_PATH, _) => (Unencrypted, true /* doesn't matter */),
                    (Oscore { oscore }, option::EDHOC, b"") if SSC::HAS_EDHOC => {
                        (Edhoc { oscore }, true /* doesn't matter */)
                    }
                    (WellKnown, option::URI_PATH, b"edhoc") if SSC::HAS_EDHOC => (WellKnownEdhoc, false),
                    (AuthzInfo(ai), option::CONTENT_FORMAT, &[19]) if SSC::PARSES_TOKENS => {
                        (AuthzInfo(ai), false)
                    }
                    (AuthzInfo(ai), option::ACCEPT, &[19]) if SSC::PARSES_TOKENS => {
                        (AuthzInfo(ai), false)
                    }
                    (any, _, _) => (any, true),
                }
            }

            /// Return true if the options in a request are only handled by this handler
            ///
            /// In all other cases, critical options are allowed to be passed on; the next-stage
            /// processor check on its own.
            fn errors_handled_here(&self) -> bool {
                match self {
                    WellKnownEdhoc | AuthzInfo(_) => true,
                    Start | Oscore { .. } | Edhoc { .. } | WellKnown | Unencrypted => false,
                }
            }
        }

        // This will always be Some in practice, just taken while it is being updated.
        let mut state = Some(Recognition::<SSC>::Start);

        // Some small potential for optimization by cutting iteration short on Edhoc, but probably
        // not worth it.
        let extra_options = request
            .options()
            .filter(|o| {
                let (new_state, filter) = state.take().unwrap().update(o);
                state = Some(new_state);
                filter
            })
            // FIXME: This aborts early on critical options, even when the result is later ignored
            .ignore_elective_others();
        let state = state.unwrap();

        if state.errors_handled_here() {
            if let Err(error) = extra_options {
                // Critical options in all other cases are handled by the Unencrypted or Oscore
                // handlers
                return Err(Own(error));
            }
        }

        let require_post = || {
            if coap_numbers::code::POST == request.code().into() {
                Ok(())
            } else {
                Err(CoAPError::method_not_allowed())
            }
        };

        match state {
            Start | WellKnown | Unencrypted => {
                if self.authorities.nosec_authorization().is_some_and(|s| {
                    s.scope().request_is_allowed(request)
                        && s.time_constraint().is_valid_with(&mut self.time)
                }) {
                    self.inner
                        .extract_request_data(request)
                        .map(|extracted| Inner(AuthorizationChecked::Allowed(extracted)))
                        .map_err(Inner)
                } else {
                    Ok(Inner(AuthorizationChecked::NotAllowed))
                }
            }
            WellKnownEdhoc => {
                if !SSC::HAS_EDHOC {
                    unreachable!("State is not constructed");
                }
                require_post()?;
                self.extract_edhoc(&request).map(Own).map_err(Own)
            }
            AuthzInfo(_) => {
                if !SSC::PARSES_TOKENS {
                    // This makes extract_token and everything down the line effectively dead code on
                    // setups with empty SSC, without triggering clippy's nervous dead code warnings.
                    //
                    // The compiler should be able to eliminiate even this one statement based on
                    // this variant not being constructed under the same condition, but that
                    // property is not being tested.
                    unreachable!("State is not constructed");
                }
                require_post()?;
                self.extract_token(request.payload())
                    .map(|r| Own(OwnRequestData::ProcessedToken(r)))
                    .map_err(Own)
            }
            Edhoc { oscore } => {
                if !SSC::HAS_EDHOC {
                    // We wouldn't get that far in a non-EDHOC situation because the option is not processed,
                    // but the optimizer may not see that, and this is the place where a reviewer of
                    // extract_oscore_edhoc can convince themself that indeed the with_edhoc=true case is
                    // unreachable when HAS_EDHOC is not set.
                    unreachable!("State is not constructed");
                }
                self.extract_oscore_edhoc(&request, &oscore, true)
                    .map(Own)
                    .map_err(Own)
            }
            Oscore { oscore } => {
                if !has_oscore::<SSC>() {
                    unreachable!("State is not constructed");
                }
                self.extract_oscore_edhoc(&request, &oscore, false)
                    .map(Own)
                    .map_err(Own)
            }
        }
    }
    fn estimate_length(&mut self, req: &Self::RequestData) -> usize {
        match req {
            OrInner::Own(_) => 2 + lakers::MAX_BUFFER_LEN,
            OrInner::Inner(AuthorizationChecked::Allowed(i)) => self.inner.estimate_length(i),
            OrInner::Inner(AuthorizationChecked::NotAllowed) => 1,
        }
    }
    fn build_response<M: MutableWritableMessage>(
        &mut self,
        response: &mut M,
        req: Self::RequestData,
    ) -> Result<(), Self::BuildResponseError<M>> {
        use OrInner::{Inner, Own};

        match req {
            Own(OwnRequestData::EdhocOkSend2(c_r)) => {
                if !SSC::HAS_EDHOC {
                    unreachable!("State is not constructed");
                }
                self.build_edhoc_message_2(response, c_r).map_err(Own)?;
            }
            Own(OwnRequestData::ProcessedToken(r)) => {
                if !SSC::PARSES_TOKENS {
                    unreachable!("State is not constructed");
                }
                r.render(response).map_err(|e| Own(Err(e)))?;
            }
            Own(OwnRequestData::EdhocOscoreRequest {
                kid,
                correlation,
                extracted,
            }) => {
                if !has_oscore::<SSC>() {
                    unreachable!("State is not constructed");
                }
                self.build_oscore_response(response, kid, correlation, extracted)
                    .map_err(Own)?;
            }
            Inner(AuthorizationChecked::Allowed(i)) => {
                self.inner.build_response(response, i).map_err(Inner)?;
            }
            Inner(AuthorizationChecked::NotAllowed) => {
                self.authorities
                    .render_not_allowed(response)
                    .map_err(|_| Own(Ok(CoAPError::unauthorized())))?;
            }
        }
        Ok(())
    }
}
