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

    if context("rp235xa") {
        copy("memory-rp235xa.x", out.join("memory-rp235xa.x")).unwrap();
    }

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=memory-rp235xa.x");
    println!("cargo:rerun-if-changed=build.rs");
}
