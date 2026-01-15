#![expect(unsafe_code)]

use crate::stack::Stack;

#[esp_hal::main]
fn main() -> ! {
    crate::startup();
}

pub fn init() {
    disable_watchdog();
}

// Disables the esp watchdogs.
//
// `esp-hal::init()` actually does this already, but our initial stack painting needs to run before
// that, and with some log messages before that, it takes enough time to trigger the watchdog and
// reboot.
fn disable_watchdog() {
    ariel_os_debug::log::debug!("ariel-os-rt: disabling watchdog timers");
    // RTC domain must be enabled before we try to disable
    // SAFETY: creating an LPWR instance out of thin air here, dropping it later.
    let mut rtc = unsafe { esp_hal::rtc_cntl::Rtc::new(esp_hal::peripherals::LPWR::steal()) };

    // Disable watchdog timers
    #[cfg(not(any(context = "esp32", context = "esp32s2")))]
    rtc.swd.disable();

    rtc.rwdt.disable();

    esp_hal::timer::timg::Wdt::<esp_hal::peripherals::TIMG0<'static>>::new().disable();
    esp_hal::timer::timg::Wdt::<esp_hal::peripherals::TIMG1<'static>>::new().disable();
}

/// Returns the current stack pointer register value
pub(crate) fn sp() -> usize {
    let sp: usize;
    // Safety: reading SP is safe
    unsafe {
        core::arch::asm!(
            "mov {}, sp",
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
