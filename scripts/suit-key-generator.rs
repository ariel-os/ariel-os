#!/usr/bin/env -S cargo +nightly -Zscript

---cargo
[package]
name = "keygen"
version = "0.1.0"
edition = "2024"

[dependencies]
cose_minicbor = "0.1.1"
minicbor = { version = "2.3.0", features = ["std"] }
p256 = { version = "0.14.0", features = ["serde", "ecdsa-core"]}
rand = "0.10.2"
---

use std::{fs, io::Write, path::Path};

use cose_minicbor::cose_keys::{CoseAlg, CoseKey, Curve, KeyType};
use p256::{
    ecdsa::SigningKey,
    elliptic_curve::{Generate, point::AffineCoordinates},
    pkcs8::EncodePrivateKey,
};
use rand::rngs::StdRng;

const DEFAULT_PRIVATE_KEY_PATH: &str = "suit_private_key.pem";
const DEFAULT_PUBLIC_KEY_PATH: &str = "key_cose_minicbor.cbor";

fn main() {
    // TODO: Use cli args.
    let private_key_path = Path::new(DEFAULT_PRIVATE_KEY_PATH);
    let public_key_path = Path::new(DEFAULT_PUBLIC_KEY_PATH);

    let mut rng: StdRng = rand::make_rng();

    let private_key = SigningKey::generate_from_rng(&mut rng);

    let private_key_pem = private_key
        .to_pkcs8_pem(p256::pkcs8::LineEnding::LF)
        .unwrap();

    let public_key_point = private_key.verifying_key().as_affine();

    let mut private_key_file = fs::File::create(private_key_path).unwrap();
    private_key_file
        .write_all(&private_key_pem.as_bytes())
        .unwrap();

    let mut cose_key = CoseKey::new(KeyType::Ec2);
    cose_key.alg(CoseAlg::ES256P256);
    cose_key.crv(Curve::P256).unwrap();
    let x_array = public_key_point.x();
    let x = x_array.as_slice();
    cose_key.x(x).unwrap();
    let y_array = public_key_point.y();
    let y = y_array.as_slice();
    cose_key.y(y).unwrap();

    let public_key_file = fs::File::create(public_key_path).unwrap();

    let public_key_writer = minicbor::encode::write::Writer::new(public_key_file);

    minicbor::encode([cose_key], public_key_writer).unwrap();
}
