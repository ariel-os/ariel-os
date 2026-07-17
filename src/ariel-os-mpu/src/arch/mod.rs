bitflags::bitflags! {
    pub struct MemoryAccess : u8 {
        const READABLE = 0b1 << 0; // Region is readable
        const WRITEABLE = 0b1 << 1; // Region is writeable
        const EXECUTABLE = 0b1 << 2; // Region is executable
    }
}

pub trait Mpu {
    const N_REGIONS: usize; // Maximum number of supported regions

    fn init();
    fn enable();
    fn disable();
    fn configure_region(
        range: core::ops::RangeInclusive<usize>,
        region_n: usize,
        access: MemoryAccess,
    );
}

cfg_if::cfg_if! {
    if #[cfg(all(any(armv8m)))] {
        mod cortex_m;
        pub use cortex_m::Cpu;
    }
    else
    {
        compile_error!("Unsupported mpu architecture");
    }
}
