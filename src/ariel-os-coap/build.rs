use serde::Deserialize;
use std::fmt::Write as _;

/// Second-level item for deserializing a `peers.yml`
///
/// (The top level is a list thereof).
#[derive(Deserialize)]
struct Peer {
    kccs: Option<String>,
    from: Option<KnownSource>,
    scope: Scope,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Scope {
    KnownScope(KnownScope),
    Aif(std::collections::HashMap<String, Permission>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Permission {
    Set(Vec<SinglePermission>),
    Single(SinglePermission),
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[allow(clippy::upper_case_acronyms, reason = "used to guide serde values")]
#[repr(u8)]
enum SinglePermission {
    GET = coap_numbers::code::GET,
    POST = coap_numbers::code::POST,
    PUT = coap_numbers::code::PUT,
    DELETE = coap_numbers::code::DELETE,
    FETCH = coap_numbers::code::FETCH,
    PATCH = coap_numbers::code::PATCH,
    #[allow(non_camel_case_types, reason = "that's how that code is named")]
    iPATCH = coap_numbers::code::IPATCH,
}

impl Permission {
    fn mask(&self) -> u32 {
        match self {
            Permission::Set(p) => p.iter().fold(0, |old, value| old | value.mask()),
            Permission::Single(p) => p.mask(),
        }
    }
}

impl SinglePermission {
    /// The `Tperm` unsigned integer representation of the REST-specific AIF model described in
    /// RFC9237.
    fn mask(self) -> u32 {
        1 << (self as u8 - 1)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum KnownScope {
    AllowAll,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum KnownSource {
    Unauthenticated,
}

fn main() {
    if !build_rs::input::cargo_feature("coap-server-config-storage") {
        return;
    }

    build_rs::output::rerun_if_env_changed("PEERS_YML");
    let peers_yml = std::path::PathBuf::from(std::env::var("PEERS_YML").unwrap());

    build_rs::output::rerun_if_changed(&peers_yml);
    let peers_file = std::fs::File::open(&peers_yml)
        .map_err(|e| {
            format!(
                "{} while opening {} inside {}",
                e,
                peers_yml.display(),
                std::env::current_dir().unwrap().display()
            )
        })
        .expect("no peers.yml usable in specified location");

    let peers: Vec<Peer> = serde_yaml::from_reader(peers_file).expect("failed to parse peers.yml");

    let mut unauthenticated_scope = None;
    let mut chain_once_per_kccs = String::new();
    for peer in peers {
        // FIXME: Should we pre-parse the KCCS and have the parsed credentials as const in flash? Or
        // just parsed enough that there is no CBOR parsing but credential and material point to
        // overlapping slices?
        let scope = match peer.scope {
            Scope::KnownScope(KnownScope::AllowAll) => {
                "coapcore::scope::UnionScope::AllowAll".to_string()
            }
            Scope::Aif(aif) => {
                let data: Vec<_> = aif
                    .into_iter()
                    .map(|(toid, tperm)| (toid, tperm.mask()))
                    .collect();
                let mut bytes = vec![];
                minicbor::encode(data, &mut bytes).unwrap();
                format!(
                    "coapcore::scope::UnionScope::AifValue(coapcore::scope::AifValue::parse(&{bytes:?}).unwrap())"
                )
            }
        };

        match (peer.kccs, peer.from) {
            (Some(kccs), None) => {
                let kccs = cbor_edn::StandaloneItem::parse(&kccs)
                    .expect("data in kccs is not valid CBOR Diagnostic Notation (EDN)")
                    .to_cbor()
                    .expect("CBOR Diagnostic Notation (EDN) is not expressible in CBOR");

                write!(
                    chain_once_per_kccs,
                    ".chain(core::iter::once((lakers::Credential::parse_ccs(
                            &{kccs:?}).unwrap(),
                            {scope},
                            )))"
                )
                .expect("writing to String is infallible");
            }
            (None, Some(KnownSource::Unauthenticated)) => {
                assert!(
                    unauthenticated_scope.is_none(),
                    "Only a single `from: unauthenticated` record is usable.",
                );

                unauthenticated_scope = Some(format!("Some({scope})"));
            }
            _ => {
                panic!(
                    "Every peer record needs to have either a `kccs: ...` or a `from: unauthenticated` key."
                )
            }
        }
    }

    let unauthenticated_scope = unauthenticated_scope.unwrap_or("None".to_string());

    let peers_data = format!(
        "
        pub(super) fn kccs() -> impl Iterator<Item=(lakers::Credential, coapcore::scope::UnionScope)> {{
            core::iter::empty()
                {chain_once_per_kccs}
        }}

        pub(super) fn unauthenticated_scope() -> Option<coapcore::scope::UnionScope> {{
            {unauthenticated_scope}
        }}
    ");

    let peers_file = build_rs::input::out_dir().join("peers.rs");
    std::fs::write(peers_file, peers_data).unwrap();
}
