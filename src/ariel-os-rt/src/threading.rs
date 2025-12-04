#![expect(unsafe_code)]

use ariel_os_threads::{THREAD_FNS, start_threading};

/// # Safety
///
/// The caller must ensure that this function is only called once.
pub unsafe fn start() -> ! {
    for thread_fn in THREAD_FNS {
        thread_fn();
    }

    // SAFETY: this function must only be called once, enforced by caller
    unsafe {
        start_threading();
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
