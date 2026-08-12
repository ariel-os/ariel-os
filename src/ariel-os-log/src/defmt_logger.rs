#![expect(unsafe_code)]

/// Implementation of a locking [`defmt::Logger`].
// Most of the code is taken from [defmt-itm](https://github.com/knurling-rs/defmt/tree/main/firmware/defmt-itm) under Apache 2.0 license / MIT license.
use core::sync::atomic::{AtomicBool, Ordering};

#[defmt::global_logger]
struct Logger;

static TAKEN: AtomicBool = AtomicBool::new(false);
static mut CS_RESTORE: critical_section::RestoreState = critical_section::RestoreState::invalid();
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();

// safety: implementing an unsafe trait and following it's safety rules.
unsafe impl defmt::Logger for Logger {
    fn acquire() {
        // safety: Must be paired with corresponding call to release(), see below
        let restore = unsafe { critical_section::acquire() };

        // safety: accessing the atomic without CAS is OK because we have acquired a critical section.
        assert!(
            !TAKEN.load(Ordering::Relaxed),
            "defmt logger taken reentrantly"
        );

        // safety: accessing the atomic without CAS is OK because we have acquired a critical section.
        TAKEN.store(true, Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        unsafe { CS_RESTORE = restore };

        // Espflash needs this to indicate the start of a defmt frame. This doesn't seem to cause
        // issues with defmt-print.
        #[cfg(context = "esp")]
        do_write(&[0xFF, 0x00]);
        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        unsafe {
            let encoder: &mut defmt::Encoder = &mut *core::ptr::addr_of_mut!(ENCODER);
            encoder.start_frame(do_write);
        }
    }

    unsafe fn flush() {
        crate::custom_transport::flush();
    }

    unsafe fn release() {
        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        unsafe {
            let encoder: &mut defmt::Encoder = &mut *core::ptr::addr_of_mut!(ENCODER);
            encoder.end_frame(do_write);
        }

        // safety: accessing the atomic without CAS is OK because we have acquired a critical section.
        TAKEN.store(false, Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        let restore = unsafe { CS_RESTORE };

        // safety: Must be paired with corresponding call to acquire(), see above
        unsafe {
            critical_section::release(restore);
        }
    }

    unsafe fn write(bytes: &[u8]) {
        // safety: accessing the `static mut` is OK because we have acquired a critical section.
        unsafe {
            let encoder: &mut defmt::Encoder = &mut *core::ptr::addr_of_mut!(ENCODER);
            encoder.write(bytes, do_write);
        }
    }
}

fn do_write(bytes: &[u8]) {
    crate::custom_transport::write_bytes(bytes);
}
