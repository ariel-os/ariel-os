use std::env;
use std::path::PathBuf;

use ariel_os_buildutils::{context, copy_and_rerun_if_changed};

fn main() {
    if !context("ariel-os") {
        // Platform-independent tooling.
        return;
    }

    if context("rp235xa") {
        // Put the extra linker script somewhere the linker can find it
        copy_and_rerun_if_changed("memory-rp235xa.x");

        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
    }
}
