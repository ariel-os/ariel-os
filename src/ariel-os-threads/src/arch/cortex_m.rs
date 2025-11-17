#![expect(unsafe_code)]

use crate::{Arch, SCHEDULER, Thread, cleanup};
use core::{arch::global_asm, ptr::write_volatile};
use cortex_m::peripheral::{SCB, scb::SystemHandler};

#[cfg(not(any(armv6m, armv7m, armv8m)))]
compile_error!("no supported ARM variant selected");

// Default EXC_RETURN value used for newly created threads when returning to
// Thread mode. We know FPU hasn't been used because the thread hasn't run.
#[cfg(any(armv7m_eabihf, armv8m_eabihf))]
const EXC_RETURN_THREAD_NO_FPU: usize = 0xFFFFFFFD;

pub struct Cpu;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct ThreadData {
    sp: usize,
    high_regs: [usize; 8],
    #[cfg(any(armv7m_eabihf, armv8m_eabihf))]
    high_regs_float: [usize; 16],
    #[cfg(any(armv7m_eabihf, armv8m_eabihf))]
    exc_return: usize,
}

impl Arch for Cpu {
    /// Callee-save registers.
    type ThreadData = ThreadData;

    const DEFAULT_THREAD_DATA: Self::ThreadData = ThreadData {
        sp: 0,
        high_regs: [0; 8],
        #[cfg(any(armv7m_eabihf, armv8m_eabihf))]
        high_regs_float: [0; 16],
        #[cfg(any(armv7m_eabihf, armv8m_eabihf))]
        exc_return: EXC_RETURN_THREAD_NO_FPU,
    };

    /// The exact order in which Cortex-M pushes the registers to the stack when
    /// entering the ISR is:
    ///
    /// +---------+ <- sp
    /// |   r0    |
    /// |   r1    |
    /// |   r2    |
    /// |   r3    |
    /// |   r12   |
    /// |   LR    |
    /// |   PC    |
    /// |   PSR   |
    /// +---------+
    fn setup_stack(thread: &mut Thread, stack: &mut [u8], func: fn(), arg: Option<usize>) {
        let stack_start = stack.as_ptr() as usize;

        // 1. The stack starts at the highest address and grows downwards.
        // 2. Cortex-M expects the SP to be 8 byte aligned, so we chop the lowest
        //    3 bits by doing `& 0xFFFFFFF8`.
        let stack_highest = (stack_start + stack.len()) & 0xFFFFFFF8;

        // 3. Reserve 32 bytes on the stack to store the basic exception frame
        let stack_pos = (stack_highest - 32) as *mut usize;

        unsafe {
            write_volatile(stack_pos.offset(0), arg.unwrap_or_default()); // -> R0
            write_volatile(stack_pos.offset(1), 1); // -> R1
            write_volatile(stack_pos.offset(2), 2); // -> R2
            write_volatile(stack_pos.offset(3), 3); // -> R3
            write_volatile(stack_pos.offset(4), 12); // -> R12
            write_volatile(stack_pos.offset(5), cleanup as usize); // -> LR
            write_volatile(stack_pos.offset(6), func as usize); // -> PC
            write_volatile(stack_pos.offset(7), 0x01000000); // -> APSR
        }

        thread.data.sp = stack_pos as usize;
        thread.stack_lowest = stack_start;
        thread.stack_highest = stack_highest;

        // Safety: This is the place to initialize stack painting.
        unsafe { thread.stack_paint_init(stack_pos as usize) };
    }

    /// Triggers a PendSV exception.
    #[inline(always)]
    fn schedule() {
        SCB::set_pendsv();
        cortex_m::asm::isb();
    }

    #[inline(always)]
    fn start_threading() {
        unsafe {
            // Make sure PendSV has a low priority.
            let mut p = cortex_m::Peripherals::steal();
            p.SCB.set_priority(SystemHandler::PendSV, 0xFF);
        }
        Self::schedule();
    }

    fn wfi() {
        cortex_m::asm::wfi();

        // see https://cliffle.com/blog/stm32-wfi-bug/
        #[cfg(context = "stm32")]
        cortex_m::asm::isb();
    }
}

#[cfg(all(any(armv7m, armv8m), not(any(armv7m_eabihf, armv8m_eabihf))))]
macro_rules! define_pendsv_without_fpu {
    () => {
    global_asm!(
    "
    .thumb_func
    .global PendSV
    PendSV:
        bl {sched}

        // r0 == 0 means that
        // a) there was no previous thread, or
        // This is only the case if the scheduler was triggered for the first time,
        // which also means that next thread has no stored context yet.
        // b) the current thread didn't change.
        //
        // In both cases, storing and loading of r4-r11 can be skipped.
        cmp r0, #0

        /* label rules:
         * - number only
         * - no combination of *only* [01]
         * - add f or b for 'next matching forward/backward'
         */
        beq 99f

        stmia r0, {{r4-r11}}
        ldmia r1, {{r4-r11}}

        99:
        movw LR, #0xFFFd
        movt LR, #0xFFFF
        bx LR
    ",
    sched = sym sched,
    );
}
}

#[cfg(any(armv7m_eabihf, armv8m_eabihf))]
macro_rules! define_pendsv_with_fpu {
     ($fpu_directive:literal) => {
        global_asm!(
            concat!(
                $fpu_directive,
                "
                .thumb_func
                .global PendSV
                // lr is EXC_RETURN of current (outgoing) thread.
                // r0 points to current_td.high_regs
                // r1 points to next_td.high_regs
                PendSV:
                    // save EXC_RETURN value, also push r2
                    // to ensure 8 byte stack alignment
                    stmdb sp!, {{r2, lr}}

                    bl {sched}

                    ldmia sp!, {{r2, lr}}

                    cmp r0, #0
                    bne _PendSV_full_context_switch

                    cmp r1, #0
                    beq _PendSV_current_thread_continues

                    _PendSV_first_thread_start:
                    movw LR, #0xFFFD
                    movt LR, #0xFFFF
                    bx LR

                    _PendSV_current_thread_continues:
                    bx lr

                    _PendSV_full_context_switch:
                    // store current thread's context
                    str lr, [r0, #{exc_return_off}]
                    stmia r0!, {{r4-r11}}
                    // was FP extension active ? If yes, store fp registers
                    tst lr, #0x10
                    it eq
                    vstmiaeq r0!, {{s16-s31}}

                    // restore next thread's context
                    ldr lr, [r1, #{exc_return_off}]
                    ldmia r1!, {{r4-r11}}
                    // was FP extension active ? If yes, restore fp registers
                    tst lr, #0x10
                    it eq
                    vldmiaeq r1!, {{s16-s31}}

                    bx lr
            "),
            sched = sym sched,
            exc_return_off = const (
                core::mem::offset_of!(ThreadData, exc_return)
                - core::mem::offset_of!(ThreadData, high_regs)
            )
        );
    }
}

#[cfg(any(armv7m, armv8m))]
cfg_if::cfg_if! {
    if #[cfg(armv7m_eabihf)] {
        define_pendsv_with_fpu!(".fpu fpv4-sp-d16");
    }
    else if #[cfg(armv8m_eabihf)] {
        define_pendsv_with_fpu!(".fpu fpv5-sp-d16");
    }
    else {
        define_pendsv_without_fpu!();
    }
}

#[cfg(armv6m)]
global_asm!(
    "
    .thumb_func
    .global PendSV
    PendSV:
        bl {sched}

        // r0 == 0 means that
        // a) there was no previous thread, or
        // This is only the case if the scheduler was triggered for the first time,
        // which also means that next thread has no stored context yet.
        // b) the current thread didn't change.
        //
        // In both cases, storing and loading of r4-r11 can be skipped.
        cmp r0, #0

        //stmia r1!, {{r4-r7}}
        str r4, [r0, #16]
        str r5, [r0, #20]
        str r6, [r0, #24]
        str r7, [r0, #28]

        mov  r4, r8
        mov  r5, r9
        mov  r6, r10
        mov  r7, r11

        str r4, [r0, #0]
        str r5, [r0, #4]
        str r6, [r0, #8]
        str r7, [r0, #12]

        //
        ldmia r1!, {{r4-r7}}
        mov r11, r7
        mov r10, r6
        mov r9,  r5
        mov r8,  r4
        ldmia r1!, {{r4-r7}}

        99:
        ldr r0, 999f
        mov LR, r0
        bx lr

        .align 4
        999:
        .word 0xFFFFFFFD
    ",
    sched = sym sched,
);

/// Schedule the next thread.
///
/// It selects the next thread that should run from the runqueue.
/// This may be current thread, or a new one.
///
/// Returns:
///   - `r0`: pointer to [`Thread::high_regs`] from old thread (to store old register state)
///           or null pointer if there was no previously running thread, or the currently running
///           thread should not be changed.
///   - `r1`: pointer to [`Thread::high_regs`] from new thread (to load new register state)
///
/// This function is called in PendSV from assembly, so it must be `extern "C"`.
///
/// # Safety
///
/// - must not be called manually (only by PendSV)
unsafe extern "C" fn sched() -> u64 {
    let (current_high_regs, next_high_regs) = loop {
        if let Some(res) = critical_section::with(|cs| {
            let scheduler = unsafe { &mut *SCHEDULER.as_ptr(cs) };

            #[cfg(feature = "multi-core")]
            scheduler.add_current_thread_to_rq();

            let next_tid = match scheduler.get_next_tid() {
                Some(tid) => tid,
                None => {
                    #[cfg(feature = "multi-core")]
                    unreachable!("At least one idle thread is always present for each core.");

                    #[cfg(not(feature = "multi-core"))]
                    {
                        Cpu::wfi();
                        // this fence seems necessary, see #310.
                        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
                        return None;
                    }
                }
            };

            // `current_high_regs` will be null if there is no current thread.
            // This is only the case once, when the very first thread starts running.
            // The returned `r1` therefore will be null, and saving/ restoring
            // the context is skipped.
            let mut current_high_regs = core::ptr::null();
            if let Some(current_tid_ref) = scheduler.current_tid_mut() {
                if next_tid == *current_tid_ref {
                    return Some((0, 0));
                }
                let current_tid = *current_tid_ref;
                *current_tid_ref = next_tid;
                let current = scheduler.get_unchecked_mut(current_tid);
                current.data.sp = cortex_m::register::psp::read() as usize;
                current_high_regs = current.data.high_regs.as_ptr();
            } else {
                *scheduler.current_tid_mut() = Some(next_tid);
            }

            let next = scheduler.get_unchecked(next_tid);
            // SAFETY: changing the PSP as part of context switch
            unsafe { cortex_m::register::psp::write(next.data.sp as u32) };

            #[cfg(armv8m)]
            // SAFETY: changing the PSPLIM as part of context switch
            unsafe {
                cortex_m::register::psplim::write(next.stack_lowest as u32)
            };

            let next_high_regs = next.data.high_regs.as_ptr();

            Some((current_high_regs as u32, next_high_regs as u32))
        }) {
            break res;
        }
    };

    // The caller (`PendSV`) expects these two pointers in r0 and r1:
    // r0 = &current.data.high_regs (or 0)
    // r1 = &next.data.high_regs
    // The C ABI on ARM (AAPCS) defines u64 to be returned in r0 and r1, so we use that to fit our
    // values in there. `extern "C"` on this function ensures the Rust compiler adheres to those
    // rules.
    // See https://github.com/ARM-software/abi-aa/blob/a82eef0433556b30539c0d4463768d9feb8cfd0b/aapcs32/aapcs32.rst#6111handling-values-larger-than-32-bits
    (current_high_regs as u64) | (next_high_regs as u64) << 32
}
