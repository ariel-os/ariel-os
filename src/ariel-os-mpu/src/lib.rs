#![no_std]
#![expect(unsafe_code)]

mod arch;

use arch::{Cpu, Mpu};

use crate::arch::MemoryAccess;

// Other regions can be added in the future here
pub enum MpuRegionUsage {
   StackRedzone = 0
}

pub unsafe fn init_mpu() {
    <Cpu as Mpu>::init();
}

pub fn context_switch(stack_begin: usize) {
    let truncated_start = stack_begin & !0b1_1111;

    const PAGESIZE: usize = 32;

    // Make sure we do not run into an underflow that will interfere with a stack of another thread
    let redzone_range = if truncated_start == stack_begin {
        stack_begin..=stack_begin
    } else {
        stack_begin.saturating_add(PAGESIZE)..=stack_begin.saturating_add(PAGESIZE)
    };

    // Disallow access, so that we detect a stack overflow with redzone
    <Cpu as Mpu>::configure_region(
        redzone_range,
        MpuRegionUsage::StackRedzone as usize,
        MemoryAccess::READABLE,
    );
}
