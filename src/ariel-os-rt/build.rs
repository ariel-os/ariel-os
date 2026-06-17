use std::env;
use std::path::PathBuf;

use ariel_os_buildutils::{
    context, context_any, copy_and_rerun_if_changed, env_var_and_rerun_if_changed,
};

#[cfg(feature = "memory-x")]
use ld_memory::MemorySection;

// 32 KiB recommended by [nrf-modem](https://github.com/diondokter/nrf-modem?tab=readme-ov-file#memory)
#[allow(dead_code, reason = "only used when the feature is enabled")]
const NRF91_MODEM_IPC_KB: u64 = 32;

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
        let isr_stacksize = env_var_and_rerun_if_changed("CONFIG_ISR_STACKSIZE")
            .expect("CONFIG_ISR_STACKSIZE env var not set");
        let template = std::fs::read_to_string("isr_stack_xtensa.ld.in")
            .unwrap()
            .replace("${ISR_STACKSIZE}", &isr_stacksize);
        std::fs::write(out.join("isr_stack_xtensa.x"), &template).unwrap();
        println!("cargo:rerun-if-changed=isr_stack_xtensa.ld.in");
    }

    copy_and_rerun_if_changed("linkme.x");
    copy_and_rerun_if_changed("eheap.x");
    copy_and_rerun_if_changed("keep-stack-sizes.x");

    #[cfg(feature = "memory-x")]
    write_memoryx();

    println!("cargo:rustc-link-search={}", out.display());
}

/// Writes `memory.x` based on `ld-memory` settings to `$OUTDIR`.
///
/// # Panics
/// Panics if called outside of a known laze context.
#[cfg(feature = "memory-x")]
fn write_memoryx() {
    let rom_start = parse_dec_or_hex(
        &env_var_and_rerun_if_changed("CHIP_ROM_START_ADDRESS")
            .expect("CHIP_ROM_START_ADDRESS env var not set"),
    )
    .expect("CHIP_ROM_START_ADDRESS is not a decimal or hex value");
    let rom_page_size = parse_dec_or_hex(
        &env_var_and_rerun_if_changed("CHIP_ROM_PAGE_SIZE_BYTES")
            .expect("CHIP_ROM_PAGE_SIZE_BYTES env var not set"),
    )
    .expect("CHIP_ROM_PAGE_SIZE_BYTES is not a decimal or hex value");
    let rom_page_count = env_var_and_rerun_if_changed("CHIP_ROM_PAGE_COUNT")
        .expect("CHIP_ROM_PAGE_COUNT env var not set")
        .parse::<u64>()
        .expect("CHIP_ROM_PAGE_COUNT is not a decimal number");

    let chip = memsolve::chip::Chip::new(rom_page_size, rom_start, rom_page_size * rom_page_count)
        .unwrap();
    let layout = memsolve::Memory::new(chip);
    let layout = if context("nrf") {
        layout_nrf(layout)
    } else {
        panic!("unknown MCU laze context");
    };

    let memory = layout
        .resolve_layout()
        .expect("Unable to resolve flash layout")
        .into_memory();
    let memory = if context("nrf") {
        memory_nrf(memory)
    } else {
        panic!("unknown MCU laze context");
    };
    memory.to_cargo_outdir("memory.x").expect("wrote memory.x");
}

/// Generates the nrf flash layout.
///
/// # Panics
/// Panics if called outside of a known laze context.
#[cfg(feature = "memory-x")]
fn layout_nrf(mut layout: memsolve::Memory) -> memsolve::Memory {
    layout.add_section(flash_section().set_boot(true));
    layout
}

/// Adds the nrf memory sections to the generated layout.
///
/// # Panics
/// Panics if called outside of a known laze context.
#[cfg(feature = "memory-x")]
fn memory_nrf(memory: ld_memory::Memory) -> ld_memory::Memory {
    let ram = if context("nrf51822-xxaa") {
        16
    } else if context("nrf52832") {
        64
    } else if context("nrf52833") {
        128
    } else if context("nrf52840") {
        256
    } else if context("nrf5340-app") {
        512
    } else if context("nrf5340-net") {
        64
    } else if context_any(&["nrf9151", "nrf9160"]).is_some() {
        let ram = 256;
        if cfg!(feature = "nrf91-modem") {
            ram - NRF91_MODEM_IPC_KB
        } else {
            ram
        }
    } else {
        panic!("please set the MCU laze context");
    };

    let ram_base = if context("nrf5340-net") {
        0x2100_0000
    } else if cfg!(feature = "nrf91-modem") {
        0x2000_0000 + NRF91_MODEM_IPC_KB * 1024
    } else {
        0x2000_0000
    };

    #[cfg(feature = "nrf91-modem")]
    let memory = memory.add_section(MemorySection::new(
        "MODEM",
        0x2000_0000,
        NRF91_MODEM_IPC_KB * 1024,
    ));

    memory.add_section(MemorySection::new("RAM", ram_base, ram * 1024))
}

/// Parses a number, supporting hexadecimal and decimal format.
///
/// # Errors
///
/// Returns ``std::num::ParseIntError`` when the number is neither decimal, nor hexadecimal.
#[cfg(feature = "memory-x")]
fn parse_dec_or_hex(input: &str) -> Result<u64, std::num::ParseIntError> {
    if let Some(hex) = input.strip_prefix("0x") {
        u64::from_str_radix(hex, 16)
    } else {
        input.parse::<u64>()
    }
}

/// Creates the flash section for memsolve.
#[cfg(feature = "memory-x")]
#[allow(
    clippy::missing_panics_doc,
    reason = "Panic only happens with incorrect section names"
)]
fn flash_section() -> memsolve::section::Section {
    memsolve::section::Section::new("FLASH")
        .unwrap()
        .set_maximize(true)
}
