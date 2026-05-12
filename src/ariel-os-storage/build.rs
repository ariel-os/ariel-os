use std::{env, path::PathBuf};

const KIBIBYTES: u32 = 1024;

fn main() {
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
    } else if is_in_current_contexts(&["esp32", "esp32s2", "esp32s3", "esp32c3", "esp32c6"]) {
        // ESP32 uses 4KB sectors typically
        // Note: esp-storage handles flash directly, so storage.x is minimal
        (16 * KIBIBYTES, 4 * KIBIBYTES)
    } else if !is_in_current_contexts(&["ariel-os"]) {
        // Dummy value for platform-independent tooling.
        (8 * KIBIBYTES, 4 * KIBIBYTES)
    } else {
        panic!("MCU not supported");
    };

    // `sequential-storage` needs at least two flash pages.
    assert!(storage_size_total / flash_page_size >= 2);

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let is_esp = is_in_current_contexts(&["esp32", "esp32s2", "esp32s3", "esp32c3", "esp32c6"]);
    
    if is_esp {
        // ESP flash is 4MB (0x400000). Place storage at the last storage_size_total bytes.
        let storage_end: u32 = 0x400000;
        let storage_start: u32 = storage_end - storage_size_total;
        let esp_script = format!(
            "/* ESP32 storage region: last {}KB of 4MB flash */\n__storage_start = {:#x};\n__storage_end = {:#x};\n",
            storage_size_total / 1024, storage_start, storage_end
        );
        std::fs::write(out.join("storage.x"), esp_script).unwrap();
        println!("cargo:rerun-if-env-changed=CARGO_CFG_CONTEXT");
        println!("cargo:rerun-if-changed=storage-esp.ld.in");
        println!("cargo:rustc-link-search={}", out.display());
        return;
    }
    
    let (insert_where, region) = if is_in_current_contexts(&["cortex-m"]) {
        ("INSERT BEFORE .data;", "RAM")
    } else if is_in_current_contexts(&["riscv"]) {
        ("INSERT BEFORE .data;", "RWDATA")
    } else {
        ("INSERT BEFORE .data;", "RAM")
    };
    
    let template_name = "storage.ld.in";
    let mut storage_template = std::fs::read_to_string(template_name).unwrap();
    storage_template = storage_template.replace("${ALIGNMENT}", &format!("{flash_page_size}"));
    storage_template = storage_template.replace("${SIZE}", &format!("{storage_size_total}"));
    storage_template = storage_template.replace("${INSERT_WHERE}", insert_where);
    storage_template = storage_template.replace("${REGION}", region);

    std::fs::write(out.join("storage.x"), &storage_template).unwrap();

    println!("cargo:rerun-if-env-changed=CARGO_CFG_CONTEXT");
    println!("cargo:rerun-if-changed=storage.ld.in");
    if is_esp {
        println!("cargo:rerun-if-changed=storage-esp.ld.in");
    }
    println!("cargo:rustc-link-search={}", out.display());
}

/// Returns whether any of the current `cfg` contexts is one of the given contexts.
fn is_in_current_contexts(contexts: &[&str]) -> bool {
    let Ok(context_var) = std::env::var("CARGO_CFG_CONTEXT") else {
        return false;
    };

    context_var.split(',').any(|c| contexts.contains(&c))
}
