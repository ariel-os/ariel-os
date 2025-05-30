#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, no_main)]
//
#![allow(incomplete_features)]
#![cfg_attr(context = "xtensa", feature(asm_experimental_arch))]

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
    else if #[cfg(context = "esp")] {
        mod esp;
        use esp as arch;
    }
    else if #[cfg(context = "ariel-os")] {
        // When run with laze but the MCU family is not supported
        compile_error!("no runtime is defined for this MCU family");
    } else {
        // Provide a default implementation, for arch-independent tooling
        mod arch {
            #[cfg_attr(not(context = "ariel-os"), allow(dead_code))]
            pub fn init() {}
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
}

#[cfg(feature = "_panic-handler")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "panic-printing")]
    ariel_os_debug::println!("panic: {}\n", _info);

    ariel_os_debug::exit(ariel_os_debug::ExitCode::FAILURE);

    #[allow(clippy::empty_loop)]
    loop {}
}

use linkme::distributed_slice;

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
    debug!("ariel_os_rt: ISR_STACKSIZE={}", isr_stack::ISR_STACKSIZE);

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
