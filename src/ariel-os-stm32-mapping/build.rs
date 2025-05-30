//! This build script populates the Cargo manifest of this crate.
//!
//! Because Cargo requires all the dependencies to be known to it when starting, changes to this
//! build script or the data require compiling twice, so as to update the Cargo manifest and so the
//! changes get picked up by Cargo on the second run.

// This CSV file is generated from the JSON files from
// https://github.com/embassy-rs/stm32-data-generated/tree/main/data/chips
// using the following command in that directory:
// jq --raw-output --slurp 'map([.name | ascii_downcase]) | flatten(0)[] | @csv' *.json
const MAPPING_FILE_NAME: &str = "stm32_mapping.csv";

const START_PATTERN: &str = "BEGIN AUTO-GENERATED STM32 MAPPING";
const END_PATTERN: &str = "END AUTO-GENERATED STM32 MAPPING";

// These MCUs have multiple cores.
const CM4_CM7_MCU_PREFIXES: &[&str] = &["stm32h745", "stm32h747", "stm32h755", "stm32h757"];
const CM0P_CM4_MCU_PREFIXES: &[&str] = &["stm32wl54", "stm32wl55"];

fn main() {
    let cargo_manifest_path = std::env::var("CARGO_MANIFEST_PATH").unwrap();
    let mut cargo_manifest = std::fs::read_to_string(&cargo_manifest_path).unwrap();

    let crate_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let csv_mapping = std::fs::read_to_string(crate_dir.join(MAPPING_FILE_NAME)).unwrap();
    let dependencies: String = csv_mapping.lines().map(generate_target_table).collect();

    // Replace the delimited section in the Cargo manifest.
    let start_bytes = cargo_manifest.find(START_PATTERN).unwrap();
    let end_bytes = cargo_manifest.find(END_PATTERN).unwrap();
    cargo_manifest.replace_range(
        start_bytes..end_bytes,
        &format!("{START_PATTERN}{dependencies}# "),
    );
    std::fs::write(&cargo_manifest_path, cargo_manifest).unwrap();

    println!("cargo::rerun-if-changed={MAPPING_FILE_NAME}");
}

fn generate_target_table(row: &str) -> String {
    let mut cols = row.split(',');

    let mcu = cols.next().unwrap();
    let cargo_feature = {
        let mut mcu_base = mcu.to_owned();
        let skip_leading_quotation_mark = &mcu_base[1..];
        if CM4_CM7_MCU_PREFIXES
            .iter()
            .any(|prefix| skip_leading_quotation_mark.starts_with(prefix))
        {
            // Remove the trailing quotation mark.
            mcu_base.pop();
            format!("{mcu_base}-cm7\"")
        } else if CM0P_CM4_MCU_PREFIXES
            .iter()
            .any(|prefix| skip_leading_quotation_mark.starts_with(prefix))
        {
            // Remove the trailing quotation mark.
            mcu_base.pop();
            format!("{mcu_base}-cm4\"")
        } else {
            mcu_base
        }
    };

    // The TOML quotation marks come from the already-quoted CSV strings.
    format!(
        r#"
[target.'cfg(context = {mcu})'.dependencies]
embassy-stm32 = {{ workspace = true, features = [{cargo_feature}] }}
"#,
    )
}
