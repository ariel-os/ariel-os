use std::{env, path::PathBuf};

const KIBIBYTES: u32 = 1024;

fn main() {
    // NOTE(hal): values of `flash_page_size` from the datasheets, confirmed by HAL's constants.
    // Important: only homogeneous flash organizations are currently supported.
    // Trying to restrict the storage size to the subset of homogeneous flash would not work as it
    // could be pushed out of it by a large enough binary.

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    if is_in_current_contexts(&["esp"]) {
        let offset = env::var("ESP_STORAGE_OFFSET");
        let storage_size = env::var("ESP_STORAGE_SIZE");

        let (offset, size) = match (offset, storage_size) {
            (Ok(offset), Ok(storage_size)) => (offset, storage_size),
            (Err(_), Err(_)) => panic!(
                "ESP storage support was selected, but ESP_STORAGE_OFFSET and\
                ESP_STORAGE_SIZE are not set. Is the ESP storage partition\
                module missing from laze?"
            ),
            _ => panic!(
                "ESP storage support was selected, but one of ESP_STORAGE_OFFSET or ESP_STORAGE_SIZE is not set."
            ),
        };

        let storage_offset =
            u32::from_str_radix(offset.trim_start_matches("0x").trim_start_matches("0X"), 16)
                .expect("Invalid ESP_STORAGE_OFFSET env var");

        let storage_size =
            u32::from_str_radix(size.trim_start_matches("0x").trim_start_matches("0X"), 16)
                .expect("Invalid ESP_STORAGE_SIZE env var");

        let storage_end = storage_offset
            .checked_add(storage_size)
            .expect("ESP storage range overflows u32");

        let storage_x = format!(
            "\
            PROVIDE(__storage_start = 0x{storage_offset:08x});
            PROVIDE(__storage_end   = 0x{storage_end:08x});
            "
        );

        std::fs::write(out.join("storage.x"), storage_x).unwrap();

        println!("cargo:rerun-if-env-changed=CARGO_CFG_CONTEXT");
        println!("cargo:rerun-if-env-changed=ESP_STORAGE_OFFSET");
        println!("cargo:rerun-if-env-changed=ESP_STORAGE_SIZE");
        println!("cargo:rustc-link-search={}", out.display());
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

/// Returns whether any of the current `cfg` contexts is one of the given contexts.
fn is_in_current_contexts(contexts: &[&str]) -> bool {
    let Ok(context_var) = std::env::var("CARGO_CFG_CONTEXT") else {
        return false;
    };

    // Contexts cannot include commas.
    context_var.split(',').any(|c| contexts.contains(&c))
}
