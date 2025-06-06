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
shlex = { version = "1.3.0" }

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
    #[error("Environment variable CARGO_ENV not set")]
    CargoEnvNotSet,
    #[error("Failed to parse CARGO_ENV")]
    CargoEnvParse,
}

use std::{fs, io, path::PathBuf};

use miette::Diagnostic;
use serde_json::{Map, Value};

#[derive(Debug, thiserror::Error, Diagnostic)]
enum Error {
    #[error("I/O error : {0}")]
    Io(#[from] io::Error),
    #[error("JSON error : {0}")]
    Json(#[from] serde_json::Error),
    #[error("Environment variable CARGO_ENV not set")]
    CargoEnvNotSet,
    #[error("Failed to parse CARGO_ENV")]
    CargoEnvParse,
}

fn main() -> miette::Result<()> {
    // create directory if it doesn't exist
    let directory_path = PathBuf::from(".vscode");
    if !directory_path.exists() {
        fs::create_dir_all(&directory_path).map_err(Error::from)?;
    }

    // create .vscode/settings.json file if it doesn't exist
    let settings_path = directory_path.join("settings.json");
    if !settings_path.exists() {
        fs::write(&settings_path, "{}").map_err(Error::from)?;
    }

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

    let features_str = features_args.replace("--features=", "");
    let features_list = features_str
        .split(",")
        .map(|s| Value::String(s.trim().to_string()))
        .collect::<Vec<Value>>();
    if !features_list.is_empty() {
        settings_json.insert(
            "rust-analyzer.cargo.features".to_string(),
            Value::Array(features_list),
        );
    }

    let mut extra_env = Map::new();

    // Parse CARGO_ENV into a map of key-value pairs to be used as extra environment variables
    let cargo_env = std::env::var("CARGO_ENV").map_err(|_| Error::CargoEnvNotSet)?;
    let tokens = shlex::split(&cargo_env).ok_or(Error::CargoEnvParse)?;
    for token in tokens {
        if let Some((key, value)) = token.split_once('=') {
            extra_env.insert(key.to_string(), Value::String(value.to_string()));
        }
    }

    // Set some default environment variables for wifi so clippy doesn't block on it when using the feature
    extra_env.insert(
        "CONFIG_WIFI_NETWORK".to_string(),
        Value::String("test-wifi".to_string()),
    );
    extra_env.insert(
        "CONFIG_WIFI_PASSWORD".to_string(),
        Value::String("test-password".to_string()),
    );

    // Default to nightly so cargo-metadata can correctly read the config
    let mut toolchain = "+nightly".to_string();
    if let Ok(toolchain_env) = std::env::var("_CARGO_TOOLCHAIN") {
        if !toolchain_env.is_empty() {
            toolchain = toolchain_env
        }
    }
    let toolchain = toolchain[1..].to_string(); // Remove the leading '+' character
    extra_env.insert("RUSTUP_TOOLCHAIN".to_string(), Value::String(toolchain));

    if !extra_env.is_empty() {
        settings_json.insert(
            "rust-analyzer.server.extraEnv".to_string(),
            Value::Object(extra_env),
        );
    }

    let settings_json_string = serde_json::to_string_pretty(&settings_json).map_err(Error::from)?;
    fs::write(&settings_path, settings_json_string).map_err(Error::from)?;
    println!(
        "Updated settings in {}",
        std::path::absolute(settings_path)
            .map_err(Error::from)?
            .to_string_lossy()
    );

    Ok(())
}
