#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, no_main)]
//
#![allow(incomplete_features)]
#![cfg_attr(context = "xtensa", feature(asm_experimental_arch))]

pub mod stack;

#[cfg(feature = "threading")]
mod threading;

#[cfg(all(feature = "single-core", feature = "multi-core"))]
compile_error!(
    "feature \"single-core\" and feature \"multi-core\" cannot be enabled at the same time"
);

use ariel_os_debug::log::debug;

cfg_if::cfg_if! {
    if #[cfg(context = "cortex-m")] {
        mod cortexm;
        use cortexm as arch;
    }
    else if #[cfg(context = "xtensa")] {
        mod xtensa;
        use xtensa as arch;
    }
    else if #[cfg(context = "riscv")] {
        mod riscv;
        use riscv as arch;
    }
    else if #[cfg(context = "ariel-os")] {
        // When run with laze but the MCU family is not supported
        compile_error!("no runtime is defined for this MCU family");
    } else {
        // Provide a default implementation, for arch-independent tooling
        #[cfg_attr(not(context = "ariel-os"), allow(dead_code))]
        mod arch {
            use crate::stack::Stack;

            pub fn init() {}
            pub fn sp() -> usize { 0 }
            pub fn stack() -> Stack { Stack::default() }
        }
    }
}

#[cfg(any(context = "cortex-m", context = "riscv", context = "xtensa"))]
mod isr_stack {
    pub(crate) const ISR_STACKSIZE: usize = {
        const CONFIG_ISR_STACKSIZE: usize = ariel_os_utils::usize_from_env_or!(
            "CONFIG_ISR_STACKSIZE",
            2048,
            "ISR stack size (in bytes)"
        );

        #[cfg(feature = "executor-interrupt")]
        {
            const CONFIG_EXECUTOR_STACKSIZE: usize = ariel_os_utils::usize_from_env_or!(
                "CONFIG_EXECUTOR_STACKSIZE",
                8192,
                "System executor stack size (in bytes)"
            );

            CONFIG_ISR_STACKSIZE + CONFIG_EXECUTOR_STACKSIZE
        }

        #[cfg(not(feature = "executor-interrupt"))]
        CONFIG_ISR_STACKSIZE
    };

    core::arch::global_asm!(
        r#"
        .section .isr_stack, "wa"
        .skip {size}
        "#,
        size = const ISR_STACKSIZE
    );

    pub fn limits() -> (usize, usize) {
        #[cfg(not(feature = "multi-core"))]
        {
            crate::isr_stack::limits_core0()
        }

        #[cfg(feature = "multi-core")]
        {
            use ariel_os_threads::{CoreId, core_id};
            if core_id() == CoreId::new(0) {
                crate::isr_stack::limits_core0()
            } else {
                crate::isr_stack::limits_core1()
            }
        }
    }

    pub fn limits_core0() -> (usize, usize) {
        // ISR stack for core0 is defined via linker script.
        unsafe extern "C" {
            static _stack_lowest: u32;
            static _stack_highest: u32;
        }

        let lowest = &raw const _stack_lowest as usize;
        let highest = &raw const _stack_highest as usize;
        (lowest, highest)
    }

    #[cfg(feature = "multi-core")]
    pub fn limits_core1() -> (usize, usize) {
        // ISR stack for core1 is exported from the threading module.
        ariel_os_threads::isr_stack_core1_get_limits()
    }

    pub fn init() {
        let stack = crate::stack::Stack::get();
        crate::debug!("ariel-os-rt: ISR stacksize: {}", stack.size());

        // initial stack paint
        stack.repaint();
    }
}

#[cfg(all(feature = "_panic-handler", not(feature = "_test")))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "panic-printing")]
    ariel_os_debug::print_panic(_info);

    ariel_os_debug::exit(ariel_os_debug::ExitCode::FAILURE);

    #[allow(clippy::empty_loop)]
    loop {}
}

use linkme::distributed_slice;

#[doc(hidden)]
#[distributed_slice]
pub static INIT_FUNCS: [fn()] = [..];

#[inline]
#[cfg_attr(not(context = "ariel-os"), allow(dead_code))]
fn startup() -> ! {
    arch::init();

    #[cfg(feature = "debug-console")]
    ariel_os_debug::init();

    debug!("ariel_os_rt::startup()");

    #[cfg(any(context = "cortex-m", context = "riscv", context = "xtensa"))]
    crate::isr_stack::init();

    #[cfg(feature = "alloc")]
    // SAFETY: *this* is the only place alloc should be initialized.
    unsafe {
        ariel_os_alloc::init();
    }

    #[cfg(test)]
    debug!("ariel_os_rt::startup() cfg(test)");

    for f in INIT_FUNCS {
        f();
    }

    #[cfg(feature = "threading")]
    {
        // SAFETY: this function must not be called more than once
        unsafe {
            threading::start();
        }
    }

    #[cfg(feature = "executor-single-thread")]
    {
        unsafe extern "Rust" {
            fn __ariel_os_embassy_init() -> !;
        }
        debug!("ariel_os_rt::startup() launching single thread executor");
        unsafe { __ariel_os_embassy_init() };
    }

    #[cfg(not(any(feature = "threading", feature = "executor-single-thread")))]
    {
        #[cfg(test)]
        test_main();
        #[allow(clippy::empty_loop)]
        loop {}
    }
}
