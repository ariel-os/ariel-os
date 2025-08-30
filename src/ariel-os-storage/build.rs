use std::{env, path::PathBuf};

const KIBIBYTES: u32 = 1024;

fn main() {
    // NOTE(hal): values of `flash_page_size` from the datasheets, confirmed by HAL's constants.
    // Important: only homogeneous flash organizations are currently supported.
    // Trying to restrict the storage size to the subset of homogeneous flash would not work as it
    // could be pushed out of it by a large enough binary.
    let (storage_size_total, flash_page_size) =
        if is_in_current_contexts(&["stm32u083mc", "stm32l475vg", "nrf5340-net", "stm32wle5jc"]) {
            (4 * KIBIBYTES, 2 * KIBIBYTES)
        } else if is_in_current_contexts(&["nrf52", "nrf5340", "nrf91", "rp", "stm32wb55rg"]) {
            (8 * KIBIBYTES, 4 * KIBIBYTES)
        } else if is_in_current_contexts(&["stm32u585ai"]) {
            (16 * KIBIBYTES, 8 * KIBIBYTES)
        } else if is_in_current_contexts(&["stm32h755zi"]) {
            (256 * KIBIBYTES, 128 * KIBIBYTES)
        } else if !is_in_current_contexts(&["ariel-os"]) {
            // Dummy value for platform-independent tooling.
            (8 * KIBIBYTES, 4 * KIBIBYTES)
        } else {
            panic!("MCU not supported");
        };

    // `sequential-storage` needs at least two flash pages.
    assert!(storage_size_total / flash_page_size >= 2);

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut storage_template = std::fs::read_to_string("storage.ld.in").unwrap();
    storage_template = storage_template.replace("${ALIGNMENT}", &format!("{flash_page_size}"));
    storage_template = storage_template.replace("${SIZE}", &format!("{storage_size_total}"));

    std::fs::write(out.join("storage.x"), &storage_template).unwrap();

    println!("cargo:rerun-if-env-changed=CARGO_CFG_CONTEXT");
    println!("cargo:rerun-if-changed=storage.ld.in");
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
