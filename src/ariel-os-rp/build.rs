use std::env;
use std::fs::copy;
use std::path::PathBuf;

fn main() {
    if !is_in_current_contexts(&["ariel-os"]) {
        // Platform-independent tooling.
        return;
    }

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let memory_script = if is_in_current_contexts(&["rp2040"]) {
        "memory.x"
    } else if is_in_current_contexts(&["rp235xa"]) {
        "memory-rp235xa.x"
    } else {
        panic!("unsupported RP MCU");
    };

    copy(memory_script, out.join("memory.x")).unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}

/// Returns whether any of the current `cfg` contexts is one of the given contexts.
fn is_in_current_contexts(contexts: &[&str]) -> bool {
    let Ok(context_var) = std::env::var("CARGO_CFG_CONTEXT") else {
        return false;
    };

    // Contexts cannot include commas.
    context_var.split(',').any(|c| contexts.contains(&c))
}
