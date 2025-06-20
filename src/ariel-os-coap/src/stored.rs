//! Credential and key configuration backed by ariel-os storage

use ariel_os_debug::log::{Cbor, debug, info};
use cbor_macro::cbo;
use coapcore::seccfg::ServerSecurityConfig;

mod flash_peers {
    include!(concat!(env!("OUT_DIR"), "/peers.rs"));
}

pub async fn server_security_config() -> impl ServerSecurityConfig {
    StoredPolicy::load().await
}

// While this might be a ZST eventually, right now we're loading all this at startup because we
// don't have the async context to access any storage at CoAP time.
struct StoredPolicy {
    own_edhoc_credential: (lakers::Credential, lakers::BytesP256ElemLen),
}

impl ServerSecurityConfig for StoredPolicy {
    // FIXME: Decide based on peers.rs input (but so far tokens are not implemented)
    const PARSES_TOKENS: bool = false;
    const HAS_EDHOC: bool = true;
    type GeneralClaims = StoredClaims;

    fn own_edhoc_credential(&self) -> Option<(lakers::Credential, lakers::BytesP256ElemLen)> {
        Some(self.own_edhoc_credential)
    }

    fn expand_id_cred_x(
        &self,
        id_cred_x: lakers::IdCred,
    ) -> Option<(lakers::Credential, StoredClaims)> {
        debug!(
            "Peer presented ID_CRED_x {}",
            Cbor(id_cred_x.as_full_value())
        );

        for (credential, scope) in flash_peers::kccs() {
            if credential.by_kid().is_ok_and(|by_kid| by_kid == id_cred_x)
                || credential
                    .by_value()
                    .is_ok_and(|by_value| by_value == id_cred_x)
            {
                debug!("Credential recognized.");
                return Some((credential, StoredClaims { scope }));
            }
        }

        // FIXME: This should be a default behavior -- but should it be part of a utility function
        // for expand_id_cred_x, or should it be where that is called?
        if let Some(credential_by_value) = id_cred_x.get_ccs() {
            if let Some(unauthorized_claims) = self.nosec_authorization() {
                debug!("Credential by value accepted at nosec level.");
                #[expect(clippy::clone_on_copy, reason = "Lakers items are overly copy happy")]
                return Some((credential_by_value.clone(), unauthorized_claims));
            }
        }

        None
    }

    fn nosec_authorization(&self) -> Option<Self::GeneralClaims> {
        flash_peers::unauthenticated_scope().map(|scope| StoredClaims { scope })
    }
}

/// Generates a private key and some credential matching it.
///
/// The 60 byte is kind of arbitrary; it's long enough for this, but needs to also accommodate
/// anything that gets loaded. It currently contains an Key ID b"", which is convenient because it
/// enables sending the key by reference.
fn generate_credpair() -> (heapless::Vec<u8, 60>, lakers::BytesP256ElemLen) {
    use lakers::CryptoTrait;
    let mut crypto = lakers_crypto_rustcrypto::Crypto::new(ariel_os_random::crypto_rng());
    let (private, public) = crypto.p256_generate_key_pair();
    let mut credential = heapless::Vec::from_slice(&cbo!(
        r#"{
        /cnf/ 8: {/ COSE_Key / 1: {
            /kty/  1: /EC2/ 2,
            /kid/  2: '',
            /crv/ -1: /P-256/ 1,
            /x/   -2: h'0000000000000000000000000000000000000000000000000000000000000000'
        }}
    }"#
    ))
    .expect("Fits by construction");
    let public_start = credential.len() - 32;
    credential[public_start..].copy_from_slice(&public);
    debug!("Generated private/public key pair.");
    (credential, private)
}

impl StoredPolicy {
    async fn load() -> Self {
        // Storage format: ([u8], [u8; 32]), where the former is a CCS, and the latter the
        // corresponding key. We may need to extend the latter to be a COSE_Key when crypto agility
        // becomes a thing.
        const OWN_CREDENTIAL_KEY: &str = "ariel-os-coap.own-edhoc-credential";

        let (credential, key) = match ariel_os_storage::get(OWN_CREDENTIAL_KEY)
            .await
            .expect("flash error prevents startup")
        {
            Some(credpair) => credpair,
            None => {
                let credpair = generate_credpair();
                ariel_os_storage::insert(OWN_CREDENTIAL_KEY, credpair.clone())
                    .await
                    .expect("flash error prevents startup");
                credpair
            }
        };

        info!("CoAP server identity: {}", Cbor(&credential));

        let credential =
            lakers::Credential::parse_ccs(&credential).expect("Processable by construction");
        let own_edhoc_credential = (credential, key);

        Self {
            own_edhoc_credential,
        }
    }
}

#[derive(Debug)]
struct StoredClaims {
    scope: coapcore::scope::UnionScope,
}

impl coapcore::GeneralClaims for StoredClaims {
    type Scope = coapcore::scope::UnionScope;

    fn scope(&self) -> &Self::Scope {
        &self.scope
    }

    fn time_constraint(&self) -> coapcore::time::TimeConstraint {
        coapcore::time::TimeConstraint::unbounded()
    }

    fn is_important(&self) -> bool {
        false
    }
}
