use std::env;
use std::path::PathBuf;

use ariel_os_build::{context, context_any};

fn main() {
    if !context("ariel-os") {
        // Platform-independent tooling.
        return;
    }

    // Put the linker scripts somewhere the linker can find them
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    if let Some(context) = context_any(&["esp32c3", "cortex-m", "riscv"]) {
        let insert_somewhere = match context {
            "esp32c3" => "INSERT AFTER .rwdata_dummy;",
            "cortex-m" => "INSERT BEFORE .data;",
            "riscv" => "INSERT BEFORE .trap;",
            _ => "",
        };

        let region = match context {
            "cortex-m" => "RAM",
            "riscv" | "esp32c3" => "RWDATA",
            _ => unreachable!(),
        };

        let mut isr_stack_template = std::fs::read_to_string("isr_stack.ld.in").unwrap();
        isr_stack_template = isr_stack_template.replace("${INSERT_SOMEWHERE}", insert_somewhere);
        isr_stack_template = isr_stack_template.replace("${STACK_REGION}", region);
        std::fs::write(out.join("isr_stack.x"), &isr_stack_template).unwrap();
        println!("cargo:rerun-if-changed=isr_stack.ld.in");
    }

    if context("riscv") {
        let region_alias = if context("esp32c3") {
            "REGION_ALIAS(FLASH, DROM)"
        } else if context("esp32c6") {
            "REGION_ALIAS(FLASH, ROM)"
        } else {
            panic!("unexpected riscv platform");
        };
        std::fs::write(out.join("linkme-region-alias.x"), region_alias).unwrap();
    }

    if context("xtensa") {
        let isr_stacksize =
            std::env::var("CONFIG_ISR_STACKSIZE").expect("CONFIG_ISR_STACKSIZE env var not set");
        let template = std::fs::read_to_string("isr_stack_xtensa.ld.in")
            .unwrap()
            .replace("${ISR_STACKSIZE}", &isr_stacksize);
        std::fs::write(out.join("isr_stack_xtensa.x"), &template).unwrap();
        println!("cargo:rerun-if-changed=isr_stack_xtensa.ld.in");
        println!("cargo:rerun-if-env-changed=CONFIG_ISR_STACKSIZE");
    }

    std::fs::copy("linkme.x", out.join("linkme.x")).unwrap();
    std::fs::copy("eheap.x", out.join("eheap.x")).unwrap();
    std::fs::copy("keep-stack-sizes.x", out.join("keep-stack-sizes.x")).unwrap();

    println!("cargo:rerun-if-changed=linkme.x");
    println!("cargo:rerun-if-changed=eheap.x");
    println!("cargo:rerun-if-changed=keep-stack-sizes.x");

    println!("cargo:rustc-link-search={}", out.display());
}
