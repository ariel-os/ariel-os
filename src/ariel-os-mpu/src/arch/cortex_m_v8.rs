#![expect(unsafe_code)]

use crate::{Mpu, MpuRegionUsage};
use cortex_m::{self as _, Peripherals};

use crate::arch::MemoryAccess;

#[cfg(not(any(armv8m)))]
compile_error!("no supported ARM variant selected");

pub struct Cpu;

impl Mpu for Cpu {
    const N_REGIONS: usize = 8; // Armv8-M supports 8 regions

    fn init() {
        unsafe {
            const MEMFAULTENA: u32 = 0b1 << 16;
            let mut peripherals = Peripherals::steal();

            // The MemoryManagement handler gets a higher priority than the PendSV handler
            // PendSV needs to have the lowest priority in the system.
            peripherals.SCB.set_priority(
                cortex_m::peripheral::scb::SystemHandler::MemoryManagement,
                0xFE,
            );

            // Configure the CPU to trigger a MemoryManagement fault rather then a hardfault when an access violation occurs.
            peripherals.SCB.shcsr.modify(|reg| reg | MEMFAULTENA);
        }

        Self::enable();
    }

    fn enable() {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };
            // Enable the MPU with the default memory map as a background region for privileged access. This allows all regions in the memory map to be accessed by privileged code.
            const ENABLE: u32 = 0b1;
            const PRIVDEFENA: u32 = 0b1 << 2;
            mpu.ctrl.write(ENABLE | PRIVDEFENA);
            // ARM recommends a data and instruction barrier after enabling the MPU to ensure that all subsequent instructions are fetched with the new MPU settings.
            cortex_m::asm::dsb();
            cortex_m::asm::isb();
        }
    }
    fn disable() {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };
            mpu.ctrl.write(0x00);
        }
    }

    fn configure_region(
        range: core::ops::RangeInclusive<usize>,
        region_n: usize,
        access: MemoryAccess,
    ) {
        unsafe {
            let mpu = { &*cortex_m::peripheral::MPU::PTR };

            const OUTER_NON_CACHEABLE: u32 = 0b0100 << 4;
            const INNER_NON_CACHEABLE: u32 = 0b0100;

            // Caching is disabled at the moment
            mpu.mair[0].write(INNER_NON_CACHEABLE | OUTER_NON_CACHEABLE);

            // Select region number that should be configured.
            mpu.rnr.write(region_n as u32);

            // Only the upper 27 bits are used for addressing.
            // The lower 5 bits are automatically set to zero.
            //[BASE=31:5|4:3=SH|AP=2:1|XN=0]
            let start_address_truncated = (*range.start() as u32) & !0b1_1111;
            // Memory is not shared at the moment
            let shareability = 0b00u32 << 2;
            // Armv8-M does not support non reading permissions, so only write can be enabled or disabled.
            let access_permission = if access.contains(MemoryAccess::WRITEABLE) {
                const READ_WRITE_PRIVILEGED: u32 = 0;
                READ_WRITE_PRIVILEGED
            } else {
                const READ_ONLY_PRIVILEGED: u32 = 0b10 << 1;
                READ_ONLY_PRIVILEGED
            };

            // Execute permission
            let execute_never: u32 = !access.contains(MemoryAccess::EXECUTABLE) as u32;

            mpu.rbar
                .write(start_address_truncated | shareability | access_permission | execute_never);

            // Only the upper 27 bits are used for addressing.
            // The lower 5 bits are automatically set to zero.
            // [LIMIT=31:5|4=PXN|ATTRIndx=3:1|EN=0]
            let end_address_truncated = (*range.end() as u32) & !0b1_1111;

            // Ariel-OS does not support user mode, so all regions are privileged. We still don't allow execution a privileged region from non privileged code as a good safety measure for further development
            const PRIVILEGED_EXECUTE_NEVER: u32 = 0b0 << 4;
            // We do not use this at the moment
            let attribute_idx = 0b0u32 << 1;
            // Enable this region
            let enable = 0b1u32;

            mpu.rlar
                .write(end_address_truncated | PRIVILEGED_EXECUTE_NEVER | attribute_idx | enable);
        };
    }
}
