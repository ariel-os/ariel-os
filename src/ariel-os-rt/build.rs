use std::env;
use std::path::PathBuf;

use ariel_os_buildutils::{
    context, context_any, copy_and_rerun_if_changed, env_var_and_rerun_if_changed,
};

fn main() {
    if !context("ariel-os") {
        // Platform-independent tooling.
        return;
    }

    // Put the linker scripts somewhere the linker can find them
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    if let Some(context) = context_any(&["esp32c2", "esp32c3", "cortex-m", "riscv"]) {
        let insert_somewhere = match context {
            "esp32c2" | "esp32c3" => "INSERT AFTER .rwdata_dummy;",
            "cortex-m" => "INSERT BEFORE .data;",
            "riscv" => "INSERT BEFORE .trap;",
            _ => "",
        };

        let region = match context {
            "cortex-m" => "RAM",
            "riscv" | "esp32c2" | "esp32c3" => "RWDATA",
            _ => unreachable!(),
        };

        let mut isr_stack_template = std::fs::read_to_string("isr_stack.ld.in").unwrap();
        isr_stack_template = isr_stack_template.replace("${INSERT_SOMEWHERE}", insert_somewhere);
        isr_stack_template = isr_stack_template.replace("${STACK_REGION}", region);
        std::fs::write(out.join("isr_stack.x"), &isr_stack_template).unwrap();
        println!("cargo:rerun-if-changed=isr_stack.ld.in");
    }

    if context("riscv") {
        let region_alias = if context_any(&["esp32c2", "esp32c3"]).is_some() {
            "REGION_ALIAS(FLASH, DROM)"
        } else if context_any(&["esp32c6", "esp32h2"]).is_some() {
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
    memoryx::write_memoryx();

    println!("cargo:rustc-link-search={}", out.display());
}

#[cfg(feature = "memory-x")]
mod memoryx {
    use ariel_os_buildutils::{context, context_any, env_var_and_rerun_if_changed};
    use ld_memory::MemorySection;
    use memsolve::section::Section;

    /// Writes `memory.x` based on `CHIP_[RAM|NVM]_` (or hardcoded) to `$OUTDIR`.
    ///
    /// # Panics
    /// Panics if called outside of a known laze context.
    pub fn write_memoryx() {
        let nvm = Nvm::from_env();
        let ram = Ram::get();
        let chip = {
            memsolve::chip::Chip::new(nvm.page_size, nvm.start_address, nvm.total_size).unwrap()
        };

        let mut layout = memsolve::Memory::new(chip);
        layout.add_section(flash_section().set_boot(true));

        let mut memory = layout
            .resolve_layout()
            .expect("Unable to resolve nvm layout")
            .into_memory();

        let ram_section = MemorySection::new("RAM", ram.start_address, ram.size)
            .attrs("rwx")
            .offset(u64_from_env_maybe("CHIP_RAM_RESERVE_BYTES").unwrap_or_default());

        memory = memory.add_section(ram_section);

        memory = handle_extra_sections(memory);

        memory.to_cargo_outdir("memory.x").expect("wrote memory.x");
    }

    /// Parses `CHIP_EXTRA_SECTIONS`.
    /// # Panics
    /// Panics on invalid format (as defined by `ld_memory::parse::parse_section()`).
    fn handle_extra_sections(mut memory: ld_memory::Memory) -> ld_memory::Memory {
        if let Ok(value) = &env_var_and_rerun_if_changed("CHIP_EXTRA_SECTIONS") {
            let split = value.split(',');
            for entry in split {
                if entry.is_empty() {
                    continue;
                }
                let section = ld_memory::parse::parse_section(entry).expect("Parsing section");
                memory = memory.add_section(section);
            }
        }
        memory
    }

    /// Struct holding Non Volatile Memory info.
    struct Nvm {
        total_size: u64,
        start_address: u64,
        page_size: u64,
    }

    impl Nvm {
        /// Get NVM info from environment variables.
        /// # Panics
        /// Panics on invalid or missing `CHIP_NVM_*` values.
        pub fn from_env() -> Nvm {
            let nvm_start = u64_from_env("CHIP_NVM_START_ADDRESS");
            let nvm_page_count = u64_from_env("CHIP_NVM_PAGE_COUNT");
            let nvm_page_size = u64_from_env("CHIP_NVM_PAGE_SIZE_BYTES");
            Nvm {
                start_address: nvm_start,
                page_size: nvm_page_size,
                total_size: nvm_page_count * nvm_page_size,
            }
        }
    }

    /// Get RAM info.
    ///
    struct Ram {
        start_address: u64,
        size: u64,
    }

    impl Ram {
        pub fn get() -> Self {
            if context("nrf") {
                Ram::get_nrf()
            } else {
                Ram::from_env()
            }
        }

        /// Get RAM info from environment variables.
        /// # Panics
        /// Panics on invalid or missing `CHIP_RAM_*` values.
        pub fn from_env() -> Ram {
            let start_address = u64_from_env("CHIP_RAM_START_ADDRESS");
            let size = u64_from_env("CHIP_RAM_SIZE_BYTES");
            Ram {
                start_address,
                size,
            }
        }

        /// Get nrf RAM info.
        /// # Panics
        /// Panics on unhandled nrf context.
        fn get_nrf() -> Self {
            let size_kb = if context("nrf51822-xxaa") {
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
                256
            } else {
                panic!("please set the MCU laze context");
            };

            let ram_base = if context("nrf5340-net") {
                0x2100_0000
            } else {
                0x2000_0000
            };

            Self {
                start_address: ram_base,
                size: size_kb * 1024,
            }
        }
    }

    /// Parses a number, supporting hexadecimal and decimal format.
    ///
    /// # Errors
    ///
    /// Returns ``std::num::ParseIntError`` when the number is neither decimal, nor hexadecimal.
    fn parse_dec_or_hex(input: &str) -> Result<u64, std::num::ParseIntError> {
        if let Some(hex) = input.strip_prefix("0x") {
            u64::from_str_radix(hex, 16)
        } else {
            input.parse::<u64>()
        }
    }

    /// Get an u64 value (hex or dec) from env or panic.
    /// # Panics
    /// Panics if the env var is not set or does not parse as `u64` in hex or decimal.
    fn u64_from_env(key: &'static str) -> u64 {
        parse_dec_or_hex(
            &env_var_and_rerun_if_changed(key).unwrap_or_else(|_| panic!("{key} env var not set")),
        )
        .unwrap_or_else(|_| panic!("{key} is not a decimal or hex value"))
    }

    /// Get an u64 value (hex or dec) from env, if set and it parses correctly.
    /// # Panics
    /// Panics then `key` is in env but does not parse as `u64` in hex or decimal.
    fn u64_from_env_maybe(key: &'static str) -> Option<u64> {
        if let Ok(value) = &env_var_and_rerun_if_changed(key) {
            Some(
                parse_dec_or_hex(value)
                    .unwrap_or_else(|_| panic!("{key} is not a decimal or hex value")),
            )
        } else {
            None
        }
    }

    /// Creates the flash section for memsolve.
    #[allow(
        clippy::missing_panics_doc,
        reason = "Panic only happens with incorrect section names"
    )]
    fn flash_section() -> Section<()> {
        Section::new("FLASH").unwrap().set_maximize(true)
    }
}
