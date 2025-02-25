use serde::Deserialize;

#[derive(Deserialize)]
struct Peer {
    kccs: String,
    scope: Scope,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Scope {
    String(String),
    Aif(std::collections::HashMap<String, Permission>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Permission {
    Set(Vec<SinglePermission>),
    Single(SinglePermission),
}

impl Permission {
    fn mask(&self) -> u32 {
        match self {
            Permission::Set(p) => p.iter().fold(0, |old, value| old | value.mask()),
            Permission::Single(p) => p.mask(),
        }
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[repr(u8)]
enum SinglePermission {
    GET = coap_numbers::code::GET,
    POST = coap_numbers::code::POST,
    PUT = coap_numbers::code::PUT,
    DELETE = coap_numbers::code::DELETE,
    FETCH = coap_numbers::code::FETCH,
    PATCH = coap_numbers::code::PATCH,
    #[allow(non_camel_case_types)]
    iPATCH = coap_numbers::code::IPATCH,
}

impl SinglePermission {
    fn mask(&self) -> u32 {
        1 << (*self as u8 - 1)
    }
}

fn main() {
    let is_storage = build::cargo_feature("coap-server-config-storage");
    if !is_storage {
        return;
    }

    build::rerun_if_env_changed("PEERS_YML");
    let peers_yml = std::path::PathBuf::from(std::env::var("PEERS_YML").unwrap());

    build::rerun_if_changed(&peers_yml);
    build::warning(&format!("{:?}", &peers_yml));
    let peers_file = std::fs::File::open(peers_yml).expect("");

    let peers: Vec<Peer> = serde_yml::from_reader(peers_file).unwrap();

    let mut chain_once_per_kccs = vec![];
    for peer in peers {
        let kccs = cbor_edn::StandaloneItem::parse(&peer.kccs)
            .unwrap()
            .to_cbor()
            .unwrap();
        // FIXME: Should we pre-parse the KCCS and have the parsed credentials as const in flash? Or
        // just parsed enough that there is no CBOR parsing but credential and material point to
        // overlapping slices?
        let scope = match peer.scope {
            Scope::String(s) if s == "allow-all" => {
                "coapcore::scope::UnionScope::AllowAll".to_string()
            }
            Scope::Aif(aif) => {
                let data: Vec<_> = aif
                    .into_iter()
                    .map(|(toid, tperm)| (toid, tperm.mask()))
                    .collect();
                let mut bytes = vec![];
                minicbor::encode(data, &mut bytes).unwrap();
                format!("coapcore::scope::UnionScope::AifValue(coapcore::scope::AifValue::parse(&{bytes:?}).unwrap())")
            }
            e => panic!("Scope configuration {e:?} is not recognized"),
        };
        chain_once_per_kccs.push(format!(
            ".chain(core::iter::once((lakers::Credential::parse_ccs(
                            &{kccs:?}).unwrap(),
                            {scope},
                            )))"
        ));
    }

    let peers_data = format!(
        "
        pub(super) fn kccs() -> impl Iterator<Item=(lakers::Credential, coapcore::scope::UnionScope)> {{
            core::iter::empty()
                {}
        }}
    ",
        chain_once_per_kccs.join("")
    );

    let peers_file = build::out_dir().join("peers.rs");
    std::fs::write(peers_file, peers_data).unwrap();
}
