#!/usr/bin/env -S cargo -Zscript
---cargo
[package]
edition = "2021"

[dependencies]
anyhow = "1.0.92"
argh = { version = "0.1.12" }
camino = { version = "1.1.9", features = ["serde"] }
cargo_toml = { version = "0.21.0", features = ["features"] }
glob = "0.3.1"
---

use anyhow::Result;
use camino::Utf8PathBuf;

#[derive(argh::FromArgs, Debug)]
/// Generate a list of Cargo.toml files belonging to a workspace
struct Args {
    #[argh(positional)]
    /// path of the input Cargo.toml
    manifest_path: Utf8PathBuf,
    #[argh(positional)]
    /// where to store the deps
    output_file: Utf8PathBuf,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let mut files = Vec::new();

    let manifest_path = args.manifest_path;

    let manifest = cargo_toml::Manifest::from_path(&manifest_path)?;

    if let Some(workspace) = manifest.workspace {
        for member in workspace.members {
            let glob = format!("{}/Cargo.toml", member);
            for entry in glob::glob(&glob)? {
                let entry = entry.unwrap();
                files.push(Utf8PathBuf::from_path_buf(entry).unwrap());
            }
        }
    }

    let mut depstr = String::new();

    depstr.push_str(&format!("{}:", args.output_file.canonicalize_utf8().unwrap()));
    for file in files {
        depstr.push(' ');
        depstr.push_str(file.canonicalize_utf8().unwrap().as_str());
    }

    std::fs::File::create(&args.output_file).unwrap();

    let depfile = args.output_file.with_extension("d");
    std::fs::write(depfile, &depstr).unwrap();
    let depfile = args.output_file.with_extension("dd");
    std::fs::write(depfile, &depstr).unwrap();

    Ok(())
}
