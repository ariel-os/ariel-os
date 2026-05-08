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
toml = { version = "1" }

---
use std::{collections::HashMap, fs, io, path::PathBuf};

use miette::Diagnostic;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Diagnostic)]
enum Error {
    #[error("I/O error : {0}")]
    Io(#[from] io::Error),
    #[error("JSON error : {0}")]
    Json(#[from] serde_json::Error),
    #[error("TOML deserialize error : {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("TOML serialize error : {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("Environment variable CARGO_ENV not set")]
    CargoEnvNotSet,
    #[error("Failed to parse CARGO_ENV")]
    CargoEnvParse,
}
#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    VsCode(VsCode),
    RustAnalyzer(RustAnalyzer),
    Helix(Helix),
    Zed(Zed),
    Gram(Gram),
}

impl SubCommand {
    fn run(self) -> miette::Result<()> {
        match self {
            Self::RustAnalyzer(command) => command.run(),
            Self::VsCode(command) => command.run(),
            Self::Helix(command) => command.run(),
            Self::Zed(command) => command.run(),
            Self::Gram(command) => command.run(),
        }
    }
}

#[derive(argh::FromArgs)]
/// Tool to generate rust-analyzer configuration to work on Ariel OS in different editors
struct Args {
    #[argh(subcommand)]
    command: SubCommand,
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "vscode")]
/// Generate configuration for VSCode.
struct VsCode {}

impl VsCode {
    fn run(&self) -> miette::Result<()> {
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

        let settings_file = fs::File::open(&settings_path).map_err(Error::from)?;

        let mut settings_json: serde_json::Map<String, serde_json::Value> =
            serde_json::from_reader(settings_file).map_err(Error::from)?;

        let settings = settings_from_env()?;

        for (key, value) in Self::parse_recursive(settings, "rust-analyzer".to_string()) {
            settings_json.insert(key, value);
        }

        let settings_json_string =
            serde_json::to_string_pretty(&settings_json).map_err(Error::from)?;
        fs::write(&settings_path, settings_json_string).map_err(Error::from)?;
        println!(
            "Updated settings in {}",
            std::path::absolute(settings_path)
                .map_err(Error::from)?
                .to_string_lossy()
        );
        Ok(())
    }

    // Parses the output of `settings_from_env()` to make it match the structure expected by VSCode's rust-analyzer extension.
    fn parse_recursive(
        values: HashMap<String, Value>,
        path: String,
    ) -> serde_json::map::Map<String, serde_json::Value> {
        let mut out = serde_json::map::Map::new();
        for (key, value) in values {
            let new_path = format!("{}.{}", path, key);
            match value {
                Value::Map(map) => {
                    let mut res = Self::parse_recursive(map, new_path);
                    out.append(&mut res);
                }
                other => {
                    out.insert(new_path.clone(), other.into());
                }
            }
        }
        out
    }
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "zed")]
/// Generate configuration for Zed.
struct Zed {}

impl Zed {
    fn run(&self) -> miette::Result<()> {
        zed_and_forks(ZedFlavor::Zed)
    }
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "gram")]
/// Generate configuration for Gram.
struct Gram {}

impl Gram {
    fn run(&self) -> miette::Result<()> {
        zed_and_forks(ZedFlavor::Gram)
    }
}

enum ZedFlavor {
    Zed,
    Gram,
}

fn zed_and_forks(flavor: ZedFlavor) -> miette::Result<()> {
    let directory_path = match flavor {
        ZedFlavor::Zed => PathBuf::from(".zed"),
        ZedFlavor::Gram => PathBuf::from(".gram"),
    };

    // create directory if it doesn't exist
    if !directory_path.exists() {
        fs::create_dir_all(&directory_path).map_err(Error::from)?;
    }

    let settings_path = match flavor {
        ZedFlavor::Zed => directory_path.join("settings.json"),
        ZedFlavor::Gram => directory_path.join("settings.jsonc"),
    };
    // create settings.json file if it doesn't exist
    if !settings_path.exists() {
        fs::write(&settings_path, "{}").map_err(Error::from)?;
    }

    let settings_file = fs::File::open(&settings_path).map_err(Error::from)?;

    // FIXME: properly parse jsonc files, this will throw an error if there's a comment.
    let mut settings_json: serde_json::Map<String, serde_json::Value> =
        serde_json::from_reader(settings_file).map_err(Error::from)?;

    let settings = settings_from_env()?;

    // rust-analyzer config in Zed is in object `lsp.rust-analyzer.initialization_options`

    let mut wrapped_settings_rust_analyzer = HashMap::new();
    wrapped_settings_rust_analyzer
        .insert("initialization_options".to_string(), Value::Map(settings));

    let mut wrapped_settings_lsp = HashMap::new();
    wrapped_settings_lsp.insert(
        "rust-analyzer".to_string(),
        Value::Map(wrapped_settings_rust_analyzer),
    );

    let mut wrapped_settings = HashMap::new();
    wrapped_settings.insert("lsp".to_string(), Value::Map(wrapped_settings_lsp));

    for (key, value) in wrapped_settings {
        settings_json.insert(key, value.into());
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

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "rust-analyzer")]
/// Generate configuration for generi rust-analyzer (rust-analyzer.toml).
struct RustAnalyzer {}

impl RustAnalyzer {
    fn run(&self) -> miette::Result<()> {
        let settings_path = PathBuf::from("rust-analyzer.toml");
        if !settings_path.exists() {
            fs::write(&settings_path, "").map_err(Error::from)?;
        }

        let settings_file = fs::read_to_string(&settings_path).map_err(Error::from)?;

        let mut settings_toml: toml::map::Map<String, toml::Value> =
            toml::from_str(&settings_file).map_err(Error::from)?;

        let settings = settings_from_env()?;

        for (key, value) in settings {
            settings_toml.insert(key, value.into());
        }

        let settings_json_string = toml::to_string_pretty(&settings_toml).map_err(Error::from)?;
        fs::write(&settings_path, settings_json_string).map_err(Error::from)?;
        println!(
            "Updated settings in {}",
            std::path::absolute(settings_path)
                .map_err(Error::from)?
                .to_string_lossy()
        );
        Ok(())
    }
}

#[derive(argh::FromArgs)]
#[argh(subcommand, name = "helix")]
/// Generate configuration for Helix (.helix/languages.toml).
struct Helix {}

impl Helix {
    fn run(&self) -> miette::Result<()> {
        // create directory if it doesn't exist
        let directory_path = PathBuf::from(".helix");
        if !directory_path.exists() {
            fs::create_dir_all(&directory_path).map_err(Error::from)?;
        }

        // create .helix/languages.toml file if it doesn't exist
        let settings_path = directory_path.join("languages.toml");
        if !settings_path.exists() {
            fs::write(&settings_path, "").map_err(Error::from)?;
        }

        let settings_file = fs::read_to_string(&settings_path).map_err(Error::from)?;

        let mut settings_toml: toml::map::Map<String, toml::Value> =
            toml::from_str(&settings_file).map_err(Error::from)?;

        let settings = settings_from_env()?;

        // rust-analyzer config in Helix is in object `language-server.rust-analyzer.config`

        let mut wrapped_settings_rust_analyzer = HashMap::new();
        wrapped_settings_rust_analyzer.insert("config".to_string(), settings);

        let mut wrapped_settings_language_server = HashMap::new();
        wrapped_settings_language_server
            .insert("rust-analyzer".to_string(), wrapped_settings_rust_analyzer);

        let mut wrapped_settings = HashMap::new();
        wrapped_settings.insert(
            "language-server".to_string(),
            wrapped_settings_language_server,
        );

        for (key, value) in wrapped_settings {
            settings_toml.insert(key, value.into());
        }

        let settings_json_string = toml::to_string_pretty(&settings_toml).map_err(Error::from)?;
        fs::write(&settings_path, settings_json_string).map_err(Error::from)?;
        println!(
            "Updated settings in {}",
            std::path::absolute(settings_path)
                .map_err(Error::from)?
                .to_string_lossy()
        );
        Ok(())
    }
}

fn main() -> miette::Result<()> {
    let args: Args = argh::from_env();
    args.command.run()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Value {
    String(String),
    Map(HashMap<String, Value>),
    // VSCode configuration puts the hierarchy in the key, see `VSCode::parse_recursive`,
    // but this kind of map shouldn't be "unwrapped" into the key.
    FinalMap(HashMap<String, Value>),
    Bool(bool),
    Array(Vec<Value>),
}

impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => serde_json::Value::String(s),
            Value::Map(hash_map) | Value::FinalMap(hash_map) => {
                serde_json::Value::Object(serde_json::Map::from_iter(
                    hash_map
                        .iter()
                        .map(|(k, v)| (k.clone(), serde_json::Value::from(v.clone()))),
                ))
            }
            Value::Bool(b) => serde_json::Value::Bool(b),
            Value::Array(values) => serde_json::Value::Array(
                values
                    .iter()
                    .map(|v| serde_json::Value::from(v.clone()))
                    .collect(),
            ),
        }
    }
}

impl From<Value> for toml::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => toml::Value::String(s),
            Value::Map(hash_map) | Value::FinalMap(hash_map) => {
                toml::Value::Table(toml::Table::from_iter(
                    hash_map
                        .iter()
                        .map(|(k, v)| (k.clone(), toml::Value::from(v.clone()))),
                ))
            }
            Value::Bool(b) => toml::Value::Boolean(b),
            Value::Array(values) => toml::Value::Array(
                values
                    .iter()
                    .map(|v| toml::Value::from(v.clone()))
                    .collect(),
            ),
        }
    }
}

fn settings_from_env() -> miette::Result<HashMap<String, Value>> {
    let mut settings = HashMap::new();

    // Having the client watch for file changes can lead to infinite loops in rust-analyzer
    let mut files = HashMap::new();
    files.insert("watcher".to_string(), Value::String("server".to_string()));
    settings.insert("files".to_string(), Value::Map(files));

    let cargo_args = std::env::var("CARGO_ARGS").unwrap_or("".to_string());
    let features_args = std::env::var("FEATURES").unwrap_or("".to_string());
    let extra_args = format!("{} {}", cargo_args, features_args)
        .trim()
        .split(" ")
        .map(|s| Value::String(s.to_string()))
        .collect::<Vec<Value>>();

    let mut check = HashMap::new();
    let mut cargo = HashMap::new();

    check.insert("command".to_string(), Value::String("clippy".to_string()));
    check.insert("allTargets".to_string(), Value::Bool(false));

    if !extra_args.is_empty() {
        check.insert("extraArgs".to_string(), Value::Array(extra_args.clone()));

        // Need to override the default check command to have the proc-macro build correctly

        let mut override_command = ["cargo", "check"]
            .map(|s| Value::String(s.to_string()))
            .to_vec();
        override_command.extend(extra_args);
        override_command.push(Value::String("--message-format=json".to_string()));

        let mut build_script = HashMap::new();

        build_script.insert(
            "overrideCommand".to_string(),
            Value::Array(override_command),
        );

        cargo.insert("buildScripts".to_string(), Value::Map(build_script));
    }

    settings.insert("check".to_string(), Value::Map(check));

    let features_str = features_args.replace("--features=", "");
    let features_list = features_str
        .split(",")
        .map(|s| Value::String(s.trim().to_string()))
        .collect::<Vec<Value>>();
    if !features_list.is_empty() {
        cargo.insert("features".to_string(), Value::Array(features_list));
    }

    let mut extra_env = HashMap::new();

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

    if let Ok(toolchain_env) = std::env::var("_CARGO_TOOLCHAIN")
        && toolchain_env.len() > 1
    {
        let toolchain = toolchain_env[1..].to_string(); // Remove the leading '+' character
        extra_env.insert("RUSTUP_TOOLCHAIN".to_string(), Value::String(toolchain));
    }

    // Copy the RUSTFLAGS environment variable, without the target prefix
    if let Ok(rustflags_env) = std::env::var("_RUSTFLAGS") {
        extra_env.insert("RUSTFLAGS".to_string(), Value::String(rustflags_env));
    }

    if !extra_env.is_empty() {
        cargo.insert("extraEnv".to_string(), Value::FinalMap(extra_env));
    }

    settings.insert("cargo".to_string(), Value::Map(cargo));

    Ok(settings)
}
