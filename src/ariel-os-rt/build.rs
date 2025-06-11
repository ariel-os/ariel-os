use std::env;
use std::path::PathBuf;

fn main() {
    if !context("ariel-os") {
        // Platform-independent tooling.
        return;
    }

    // Put the linker scripts somewhere the linker can find them
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    if let Some(context) = context_any(&["cortex-m", "riscv", "xtensa"]) {
        let insert_somewhere = match context {
            "cortex-m" => "INSERT BEFORE .data;",
            "riscv" => "INSERT BEFORE .trap;",
            _ => "",
        };

        let region = match context {
            "cortex-m" => "RAM",
            "riscv" | "xtensa" => "RWDATA",
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

    std::fs::copy("linkme.x", out.join("linkme.x")).unwrap();
    std::fs::copy("eheap.x", out.join("eheap.x")).unwrap();
    std::fs::copy("keep-stack-sizes.x", out.join("keep-stack-sizes.x")).unwrap();

    #[cfg(feature = "memory-x")]
    write_memoryx();

    println!("cargo:rerun-if-changed=linkme.x");
    println!("cargo:rerun-if-changed=eheap.x");
    println!("cargo:rerun-if-changed=keep-stack-sizes.x");

    println!("cargo:rustc-link-search={}", out.display());
}

/// Writes `memory.x` based on `ld-memory` settings to `$OUTDIR`.
///
/// # Panics
/// Panics if called outside of a known laze context.
#[cfg(feature = "memory-x")]
fn write_memoryx() {
    use ld_memory::{Memory, MemorySection};
    let (ram, flash) = if context("nrf51822-xxaa") {
        (16, 256)
    } else if context("nrf52832") {
        (64, 256)
    } else if context("nrf52833") {
        (128, 512)
    } else if context("nrf52840") {
        (256, 1024)
    } else if context("nrf5340") {
        (512, 1024)
    } else if context("nrf5340-net") {
        (64, 256)
    } else if context_any(&["nrf9151", "nrf9160"]).is_some() {
        (256, 1024)
    } else {
        panic!("please set the MCU laze context");
    };

    let (pagesize, ram_base, flash_base) = if context("nrf5340-net") {
        (2048, 0x2100_0000, 0x0100_0000)
    } else {
        (4096, 0x2000_0000, 0)
    };

    // generate linker script
    let memory = Memory::new()
        .add_section(MemorySection::new("RAM", ram_base, ram * 1024))
        .add_section(
            MemorySection::new("FLASH", flash_base, flash * 1024)
                .pagesize(pagesize)
                .from_env(),
        );

    memory.to_cargo_outdir("memory.x").expect("wrote memory.x");
}

/// Returns the first of the given contexts that is in the current `cfg` contexts
fn context_any(contexts: &[&'static str]) -> Option<&'static str> {
    // Contexts cannot include commas.
    contexts.iter().find(|c| context(c)).copied()
}

/// Returns whether the given context is in the current 'cfg' contexts
fn context(context: &'static str) -> bool {
    let Ok(context_var) = std::env::var("CARGO_CFG_CONTEXT") else {
        return false;
    };

    // Contexts cannot include commas.
    context_var.split(',').any(|c| c == context)
}
