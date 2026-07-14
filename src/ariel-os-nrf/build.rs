use ariel_os_build::{context, context_any};
use ld_memory::{Memory, MemorySection};

// 32 KiB recommended by [nrf-modem](https://github.com/diondokter/nrf-modem?tab=readme-ov-file#memory)
#[allow(dead_code, reason = "only used when the feature is enabled")]
const NRF91_MODEM_IPC_KB: u64 = 32;

fn main() {
    if !context("ariel-os") {
        // Platform-independent tooling.
        return;
    }

    let (ram, flash) = if context("nrf51822-xxaa") {
        (16, 256)
    } else if context("nrf52832") {
        (64, 256)
    } else if context("nrf52833") {
        (128, 512)
    } else if context("nrf52840") {
        (256, 1024)
    } else if context("nrf5340-app") {
        (512, 1024)
    } else if context("nrf5340-net") {
        (64, 256)
    } else if context_any(&["nrf9151", "nrf9160"]).is_some() {
        let ram = 256;
        let flash = 1024;
        if cfg!(feature = "nrf91-modem") {
            (ram - NRF91_MODEM_IPC_KB, flash)
        } else {
            (ram, flash)
        }
    } else {
        panic!("please set the MCU laze context");
    };

    let (pagesize, ram_base, flash_base) = if context("nrf5340-net") {
        (2048, 0x2100_0000, 0x0100_0000)
    } else if cfg!(feature = "nrf91-modem") {
        (4096, 0x2000_0000 + NRF91_MODEM_IPC_KB * 1024, 0)
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

    #[cfg(feature = "nrf91-modem")]
    let memory = memory.add_section(MemorySection::new(
        "MODEM",
        0x2000_0000,
        NRF91_MODEM_IPC_KB * 1024,
    ));

    memory.to_cargo_outdir("memory.x").expect("wrote memory.x");
}
