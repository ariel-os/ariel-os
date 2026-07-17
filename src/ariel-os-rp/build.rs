use std::env;
use std::fs::copy;
use std::path::PathBuf;

use ariel_os_buildutils::context;

fn main() {
    if !context("ariel-os") {
        // Platform-independent tooling.
        return;
    }

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let memory_script = if context("rp2040") {
        "memory.x"
    } else if context("rp235xa") {
        "memory-rp235xa.x"
    } else {
        panic!("unsupported RP MCU");
    };

    copy(memory_script, out.join("memory.x")).unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
