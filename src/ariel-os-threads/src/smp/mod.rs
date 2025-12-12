use crate::CoreId;
use ariel_os_utils::usize_from_env_or;
use portable_atomic::{AtomicUsize, Ordering};

pub(crate) static STACK_BOTTOM_CORE1: AtomicUsize = AtomicUsize::new(0);
pub(crate) static STACK_TOP_CORE1: AtomicUsize = AtomicUsize::new(0);

// (just an alias to save typing)
pub(crate) type StackType = <Chip as Multicore>::Stack;

pub(crate) fn isr_stack_core1_set_limits(stack: &StackType) {
    let (lowest, highest) = stack.limits();

    // set lowest first so just in case both are read in between,
    // the `lowest <= highest` is invalid during that time, which is checked
    // where needed.
    STACK_BOTTOM_CORE1.store(lowest, Ordering::Release);
    STACK_TOP_CORE1.store(highest, Ordering::Release);
}

/// Returns the isr stack limits for the second core as `(lowest, highest)`.
pub fn isr_stack_core1_get_limits() -> (usize, usize) {
    // read `highest` first so that when this is called before `isr_stack_core1_set_limits()`,
    // `lowest > highest` -> invalid, which is checked elsewhere.
    let highest = STACK_TOP_CORE1.load(Ordering::Acquire);
    let lowest = STACK_BOTTOM_CORE1.load(Ordering::Acquire);
    (lowest, highest)
}

pub trait Multicore {
    /// Number of available core.
    const CORES: u32;
    /// Stack size for the idle threads.
    const IDLE_THREAD_STACK_SIZE: usize = 256;
    type Stack;

    /// Returns the ID of the current core.
    fn core_id() -> CoreId;

    /// Starts other available cores.
    ///
    /// This is called at boot time by the first core.
    ///
    /// TODO: This passes *one* stack as argument, assuming the first core
    /// already has a stack and there is only one more core to initialize.
    fn startup_other_cores(stack: &'static mut Self::Stack);

    /// Triggers the scheduler on core `id`.
    fn schedule_on_core(id: CoreId);
}

pub trait StackLimits {
    /// Returns (lowest, highest) of this stack.
    fn limits(&self) -> (usize, usize) {
        (0, 0)
    }
}

cfg_if::cfg_if! {
    if #[cfg(context = "rp")] {
        mod rp;
        pub use rp::Chip;
    } else if #[cfg(context = "esp32s3")] {
        mod esp32s3;
        pub use esp32s3::Chip;
    }
    else {
        use crate::{Arch as _, Cpu};

        pub struct Chip;

        // need something that has a `new()`
        pub struct Dummy;
        impl Dummy {
            pub const fn new() -> Self { Self {}}
        }
        impl StackLimits for Dummy {}

        impl Multicore for Chip {
            const CORES: u32 = 1;
            type Stack = Dummy;

            fn core_id() -> CoreId {
                CoreId(0)
            }

            fn startup_other_cores(_stack: &'static mut Self::Stack) {}

            fn schedule_on_core(_id: CoreId) {
                Cpu::schedule();
            }
        }
    }
}

/// Triggers the scheduler on core `id`.
pub fn schedule_on_core(id: CoreId) {
    Chip::schedule_on_core(id)
}

/// Main stack size for the second core, that is also used by the ISR.
///
/// Uses default from `ariel-os-rt` if not specified.
/// The `CONFIG_ISR_STACKSIZE` env name and default is copied from
/// `ariel-os-rt`.
#[allow(dead_code, reason = "used in chip submodules")]
const ISR_STACKSIZE_CORE1: usize = usize_from_env_or!(
    "CONFIG_ISR_STACKSIZE_CORE1",
    usize_from_env_or!("CONFIG_ISR_STACKSIZE", 8192, "ISR stack size (in bytes)"),
    "Core 1 ISR stack size (in bytes)"
);
