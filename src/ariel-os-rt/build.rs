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
    memoryx::write_memoryx(&out);

    println!("cargo:rustc-link-search={}", out.display());
}

#[cfg(feature = "memory-x")]
mod memoryx {
    use ariel_os_buildutils::{context, context_any, env_var_and_rerun_if_changed};
    use ld_memory::MemorySection;
    use memsolve::{Memory, section::Section};

    /// Writes `memory.x` based on `CHIP_[RAM|NVM]_` (or hardcoded) to `$OUTDIR`.
    ///
    /// # Panics
    /// Panics if called outside of a known laze context.
    pub fn write_memoryx(out_dir: &std::path::Path) {
        // Gather chip NVM & RAM config
        let nvm = Nvm::from_env();
        let ram = Ram::get();
        let chip = {
            memsolve::chip::Chip::new(nvm.page_size, nvm.start_address, nvm.total_size).unwrap()
        };

        println!(
            "NVM page_size={} start_address={} total_size={}",
            nvm.page_size, nvm.start_address, nvm.total_size
        );

        // define NVM layout
        let mut layout = memsolve::Memory::new(chip);

        #[cfg(not(any(feature = "embassy-boot-loader", feature = "embassy-boot-application")))]
        layout_default(&mut layout);

        #[cfg(any(feature = "embassy-boot-loader", feature = "embassy-boot-application"))]
        layout_embassy_boot(&mut layout);

        let mut memory = layout
            .resolve_layout()
            .expect("Unable to resolve nvm layout")
            .into_memory();

        // define RAM layout
        let ram_section = MemorySection::new("RAM", ram.start_address, ram.size)
            .attrs("rwx")
            .offset(u64_from_env_maybe("CHIP_RAM_RESERVE_BYTES").unwrap_or_default());

        memory = memory.add_section(ram_section);

        // get & add extra sections from environment variable
        memory = handle_extra_sections(memory);

        let section_range_constants = create_section_range_constants(&memory);
        std::fs::write(out_dir.join("linker_sections.rs"), &section_range_constants).unwrap();

        let mut memory_content = memory.to_ldmemory();

        // get & add filenames to include in memory.x
        handle_ld_includes(&mut memory_content);

        std::fs::write(out_dir.join("memory.x"), &memory_content).unwrap();
    }

    /// Builds a `String` with Rust constants for each section in `memory`.
    fn create_section_range_constants(memory: &ld_memory::Memory) -> String {
        // So previously we've renamed these sections to "FLASH" using the memsolve `linker_name`
        // mechanism (search `fix_linker_name()`.
        // Then we create ld_memory sections.
        // For exporting, having the previous names actually makes more sense, so we reverse the
        // renaming here before generating the range constants.
        fn fix_section_name(name: &str) -> &str {
            #[cfg(feature = "embassy-boot-loader")]
            if name == "FLASH" {
                return "BOOTLOADER";
            }
            #[cfg(feature = "embassy-boot-application")]
            if name == "FLASH" {
                return "ACTIVE";
            }
            name
        }

        use std::fmt::Write as _;
        let mut s = String::new();
        for section in memory.sections() {
            let (name, from, to) = (
                fix_section_name(section.get_name()),
                section.get_origin(),
                section.get_origin() + section.get_length(),
            );
            let _ = write!(
                s,
                "pub const {name}: core::ops::Range<u32> = {from}..{to};\n"
            );
        }
        s
    }

    /// Configures the default (no bootloader, all flash for application) layout.
    #[allow(clippy::missing_panics_doc, reason = "constant name is always correct")]
    #[allow(unused, reason = "use depending on features")]
    fn layout_default(layout: &mut Memory<()>) {
        println!("default memsolve layout");
        layout.add_section(
            Section::new("FLASH")
                .unwrap()
                .set_maximize(true)
                .set_boot(true),
        );
    }

    /// Configures the embassy-boot compatible layout.
    #[allow(
        clippy::missing_panics_doc,
        reason = "constant names are always correct"
    )]
    #[allow(unused, reason = "use depending on features")]
    fn layout_embassy_boot(layout: &mut Memory<()>) {
        fn fix_linker_name(section: Section<()>) -> Section<()> {
            #[cfg(feature = "embassy-boot-loader")]
            if section.name == "BOOTLOADER" {
                return section.set_linker_name("FLASH").unwrap();
            }
            #[cfg(feature = "embassy-boot-application")]
            if section.name == "ACTIVE" {
                return section.set_linker_name("FLASH").unwrap();
            }
            section
        }

        println!("embassy memsolve layout");

        layout.add_section(fix_linker_name(
            Section::new("BOOTLOADER")
                .unwrap()
                .set_pages(6)
                .set_boot(true),
        ));

        layout.add_section(Section::new("BOOTLOADER_STATE").unwrap().set_pages(2));
        layout.add_section(fix_linker_name(
            Section::new("ACTIVE").unwrap().set_maximize(true),
        ));
        layout.add_section(
            Section::new("DFU")
                .unwrap()
                .set_maximize(true)
                .set_relative_pages(1),
        );
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

    /// Parses `CHIP_LD_INCLUDES`.
    fn handle_ld_includes(memory_content: &mut String) {
        use std::fmt::Write as _;
        if let Ok(value) = &env_var_and_rerun_if_changed("CHIP_LD_INCLUDES") {
            let split = value.split(',');
            for entry in split {
                if entry.is_empty() {
                    continue;
                }
                write!(memory_content, "\nINCLUDE {entry}\n").unwrap();
            }
        }
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
}
