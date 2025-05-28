#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
edition = "2024"

[dependencies]
argh = { version = "0.1.13" }
miette = { version = "7.2.0", features = ["fancy"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0" }
thiserror = { version = "2.0.12" }

---
use std::{fs, io, path::PathBuf};

use miette::Diagnostic;
use serde_json::{Map, Value};

#[derive(Debug, thiserror::Error, Diagnostic)]
enum Error {
    #[error("I/O error : {0}")]
    Io(#[from] io::Error),
    #[error("JSON error : {0}")]
    Json(#[from] serde_json::Error),
}

fn main() -> miette::Result<()> {
    println!("folder");
    // create folder if it doesn't exist
    let folder_path = PathBuf::from(".vscode");
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).map_err(Error::from)?;
    }

    println!("settings.json");

    // create .vscode/settings.json file if it doesn't exist
    let settings_path = folder_path.join("settings.json");
    if !settings_path.exists() {
        fs::write(&settings_path, "{}").map_err(Error::from)?;
    }

    println!("read settings.json");
    let settings = fs::File::open(&settings_path).map_err(Error::from)?;

    let mut settings_json: Map<String, Value> =
        serde_json::from_reader(settings).map_err(Error::from)?;

    settings_json.insert(
        "rust-analyzer.check.command".to_string(),
        Value::String("clippy".to_string()),
    );
    settings_json.insert(
        "rust-analyzer.check.allTargets".to_string(),
        Value::Bool(false),
    );

    let cargo_args = std::env::var("CARGO_ARGS").unwrap_or("".to_string());
    let features_args = std::env::var("FEATURES").unwrap_or("".to_string());
    let extra_args = format!("{} {}", cargo_args, features_args)
        .trim()
        .split(" ")
        .map(|s| Value::String(s.to_string()))
        .collect::<Vec<Value>>();

    if !extra_args.is_empty() {
        settings_json.insert(
            "rust-analyzer.check.extraArgs".to_string(),
            Value::Array(extra_args),
        );
    }

    let mut extra_env = Map::new();

    let toolchain = std::env::var("_CARGO_TOOLCHAIN").map(Some).unwrap_or(None);
    if let Some(toolchain) = toolchain {
        if !toolchain.is_empty() {
            let toolchain = toolchain[1..].to_string(); // Remove the leading '+' character
            extra_env.insert("RUSTUP_TOOLCHAIN".to_string(), Value::String(toolchain));
        }
    }

    if let Ok(config_board) = std::env::var("CONFIG_BOARD") {
        extra_env.insert("CONFIG_BOARD".to_string(), Value::String(config_board));
    }
    if let Ok(cargo_build_target) = std::env::var("_RUSTC_TARGET") {
        extra_env.insert(
            "CARGO_BUILD_TARGET".to_string(),
            Value::String(cargo_build_target),
        );
    }

    if let Ok(cargo_target_prefix) = std::env::var("CARGO_TARGET_PREFIX") {
        let rustflags = std::env::var("_RUSTFLAGS").unwrap_or("".to_string());
        extra_env.insert(
            format!("{}_RUSTFLAGS", cargo_target_prefix),
            Value::String(rustflags),
        );
    }

    if !extra_env.is_empty() {
        settings_json.insert(
            "rust-analyzer.server.extraEnv".to_string(),
            Value::Object(extra_env),
        );
    }

    let settings_json_string = serde_json::to_string_pretty(&settings_json).map_err(Error::from)?;
    fs::write(&settings_path, settings_json_string).map_err(Error::from)?;
    println!("Updated settings in {}", settings_path.to_string_lossy());

    Ok(())
}
