use std::{
    env,
    path::{Path, PathBuf},
};

const KIBIBYTES: u32 = 1024;

fn main() {
    // NOTE(hal): values of `flash_page_size` from the datasheets, confirmed by HAL's constants.
    // Important: only homogeneous flash organizations are currently supported.
    // Trying to restrict the storage size to the subset of homogeneous flash would not work as it
    // could be pushed out of it by a large enough binary.
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    if is_in_current_contexts(&["esp"]) {
        emit_esp_storage_symbols(&out);
        return;
    }

    let (storage_size_total, flash_page_size) = if is_in_current_contexts(&[
        "stm32f303cb",
        "stm32f303re",
        "stm32u073kc",
        "stm32u083mc",
        "stm32l475vg",
        "nrf5340-net",
        "stm32wle5jc",
    ]) {
        (4 * KIBIBYTES, 2 * KIBIBYTES)
    } else if is_in_current_contexts(&["nrf52", "nrf5340-app", "nrf91", "rp", "stm32wb55rg"]) {
        (8 * KIBIBYTES, 4 * KIBIBYTES)
    } else if is_in_current_contexts(&["stm32u585ai", "stm32wba65ri"]) {
        (16 * KIBIBYTES, 8 * KIBIBYTES)
    } else if is_in_current_contexts(&["stm32h755zi", "stm32h753zi"]) {
        (256 * KIBIBYTES, 128 * KIBIBYTES)
    } else if !is_in_current_contexts(&["ariel-os"]) {
        // Dummy value for platform-independent tooling.
        (8 * KIBIBYTES, 4 * KIBIBYTES)
    } else {
        panic!("MCU not supported");
    };

    // `sequential-storage` needs at least two flash pages.
    assert!(storage_size_total / flash_page_size >= 2);

    let mut storage_template = std::fs::read_to_string("storage.ld.in").unwrap();
    storage_template = storage_template.replace("${ALIGNMENT}", &format!("{flash_page_size}"));
    storage_template = storage_template.replace("${SIZE}", &format!("{storage_size_total}"));

    std::fs::write(out.join("storage.x"), &storage_template).unwrap();

    println!("cargo:rerun-if-env-changed=CARGO_CFG_CONTEXT");
    println!("cargo:rerun-if-changed=storage.ld.in");
    println!("cargo:rustc-link-search={}", out.display());
}

fn emit_esp_storage_symbols(out: &Path) {
    const MISSING_ESP_PARTITION_TABLE_HINT: &str = concat!(
        "ESP storage is enabled, but no ESP partition table was configured.\n",
        "\n",
        "Add a partition table CSV to your application and point ESP_PARTITION_TABLE ",
        "at it from your application's laze-project.yml:\n",
        "\n",
        "env:\n",
        "  global:\n",
        "    ESP_PARTITION_TABLE: \"./esp32.csv\"\n",
        "\n",
        "The build system resolves this path and passes it to Cargo as ",
        "ARIEL_ESP_PARTITION_TABLE. The table must contain the storage partition ",
        "configured by ARIEL_ESP_STORAGE_PARTITION; the default name is `ariel_store`."
    );
    const ESP_FLASH_SECTOR_SIZE: u32 = 4096;

    let partition_table = std::env::var("ARIEL_ESP_PARTITION_TABLE")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| panic!("{MISSING_ESP_PARTITION_TABLE_HINT}"));

    let partition_name = std::env::var("ARIEL_ESP_STORAGE_PARTITION")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "ariel_store".to_owned());

    let csv = std::fs::read_to_string(&partition_table).unwrap_or_else(|err| {
        panic!("failed to read ESP partition table `{partition_table}`: {err}")
    });

    let table = esp_idf_part::PartitionTable::try_from_str(&csv).unwrap_or_else(|err| {
        panic!("failed to read ESP partition table `{partition_table}`: {err}")
    });

    let part = table.find(&partition_name).unwrap_or_else(|| {
        panic!("failed to find ESP storage partition `{partition_name}` in `{partition_table}`")
    });

    let storage_start = part.offset();
    let storage_size = part.size();
    let storage_end = storage_start
        .checked_add(storage_size)
        .expect("ESP storage partition range overflows u32");

    assert!(
        storage_start.is_multiple_of(ESP_FLASH_SECTOR_SIZE),
        "ESP storage partition `{partition_name}` offset 0x{storage_start:08x} is not sector-aligned"
    );
    assert!(
        storage_size.is_multiple_of(ESP_FLASH_SECTOR_SIZE),
        "ESP storage partition `{partition_name}` size 0x{storage_size:08x} is not sector-aligned"
    );
    assert!(
        storage_size >= 2 * ESP_FLASH_SECTOR_SIZE,
        "ESP storage partition `{partition_name}` must be at least two flash sectors"
    );

    let storage_x = format!(
        "\
        PROVIDE(__storage_start = 0x{storage_start:08x});
        PROVIDE(__storage_end   = 0x{storage_end:08x});
        "
    );

    std::fs::write(out.join("storage.x"), storage_x).unwrap();

    println!("cargo:rerun-if-env-changed=ARIEL_ESP_PARTITION_TABLE");
    println!("cargo:rerun-if-env-changed=ARIEL_ESP_STORAGE_PARTITION");
    println!("cargo:rerun-if-changed={partition_table}");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_CONTEXT");
    println!("cargo:rustc-link-search={}", out.display());
}

/// Returns whether any of the current `cfg` contexts is one of the given contexts.
fn is_in_current_contexts(contexts: &[&str]) -> bool {
    let Ok(context_var) = std::env::var("CARGO_CFG_CONTEXT") else {
        return false;
    };

    // Contexts cannot include commas.
    context_var.split(',').any(|c| contexts.contains(&c))
}
