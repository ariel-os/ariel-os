#![expect(unsafe_code)]

use crate::stack::Stack;

#[esp_hal::main]
fn main() -> ! {
    crate::startup();
}

pub fn init() {}

/// Returns the current `SP` register value
pub(crate) fn sp() -> usize {
    let sp: usize;
    // Safety: reading SP is safe
    unsafe {
        core::arch::asm!(
            "mv {}, sp",
            out(reg) sp,
            options(nomem, nostack, preserves_flags)
        )
    };
    sp
}

/// Returns a `Stack` handle for the currently active thread.
pub(crate) fn stack() -> Stack {
    #[cfg(feature = "threading")]
    let (lowest, highest) = {
        let (lowest, highest) = crate::isr_stack::limits();
        let sp = sp();
        if !(lowest <= sp && highest >= sp) {
            ariel_os_threads::current_stack_limits().unwrap()
        } else {
            (lowest, highest)
        }
    };

    // When threading is disabled, the isr stack is used.
    #[cfg(not(feature = "threading"))]
    let (lowest, highest) = crate::isr_stack::limits();

    Stack::new(lowest, highest)
}
