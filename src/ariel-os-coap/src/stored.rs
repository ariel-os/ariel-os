//! Credential and key configuration backed by ariel-os storage

use ariel_os_debug::log::{debug, info};
use cbor_macro::cbo;
use coapcore::seccfg::ServerSecurityConfig;

mod flash_peers {
    include!(concat!(env!("OUT_DIR"), "/peers.rs"));
}

pub async fn server_security_config() -> impl ServerSecurityConfig {
    StoredPolicy::load().await
}

// While this might be a ZST eventually, right now we're loading a bunch of stuff at startup
// because we don't have the async context to access any storage at CoAP time.
struct StoredPolicy {
    own_edhoc_credential: (lakers::Credential, lakers::BytesP256ElemLen),
}

impl ServerSecurityConfig for StoredPolicy {
    // FIXME: Both could really be optional
    const PARSES_TOKENS: bool = true;
    const HAS_EDHOC: bool = true;
    type GeneralClaims = StoredClaims;

    fn own_edhoc_credential(&self) -> Option<(lakers::Credential, lakers::BytesP256ElemLen)> {
        Some(self.own_edhoc_credential)
    }

    fn expand_id_cred_x(
        &self,
        id_cred_x: lakers::IdCred,
    ) -> Option<(lakers::Credential, StoredClaims)> {
        for (credential, scope) in flash_peers::kccs() {
            if credential.by_kid().is_ok_and(|by_kid| by_kid == id_cred_x)
                || credential
                    .by_value()
                    .is_ok_and(|by_value| by_value == id_cred_x)
            {
                return Some((credential, StoredClaims { scope }));
            }
        }
        None
    }
}

impl StoredPolicy {
    async fn load() -> Self {
        // Storage format: ([u8], [u8; 32]), where the former is a CCS, and the latter the
        // corresponding key. We may need to extend the latter to be a COSE_Key when crypto agility
        // becomes a thing.
        const OWN_CREDENTIAL_KEY: &str = "coapcore.own-edhoc-credential";

        let mut storage = ariel_os_storage::lock().await;
        let (credential, key) = match storage
            .get(OWN_CREDENTIAL_KEY)
            .await
            .expect("flash error prevents startup")
        {
            Some(credpair) => credpair,
            None => {
                use lakers::CryptoTrait;
                let mut crypto =
                    lakers_crypto_rustcrypto::Crypto::new(ariel_os_random::crypto_rng());
                let (private, public) = crypto.p256_generate_key_pair();
                // 60 byte is kind of arbitrary; it's long enough for this, but needs to also
                // accommodate anything that gets loaded.
                let mut credential = heapless::Vec::<u8, 60>::from_slice(&cbo!(
                    r#"{
                    /cnf/ 8: {/ COSE_Key / 1: {
                        1: 2,
                        / empty key ID is handy because it enables sending by reference /
                        2: '',
                        -1: 1,
                        -2: h'0000000000000000000000000000000000000000000000000000000000000000'
                    }}
                }"#
                ))
                .expect("Fits by construction");
                let public_start = credential.len() - 32;
                credential[public_start..].copy_from_slice(&public);
                debug!("Generated private/public key pair.");
                let credpair = (credential, private);
                storage
                    .insert(OWN_CREDENTIAL_KEY, credpair.clone())
                    .await
                    .expect("flash error prevents startup");
                credpair
            }
        };

        info!("CoAP server identity: {=[u8]:02x}", credential); // :02x could be :cbor

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
