use core::arch::asm;
use core::ptr::write_volatile;
use cortex_m::peripheral::SCB;
use critical_section::CriticalSection;

use crate::{cleanup, THREADS};

/// Sets up the stack for newly created threads and returns the sp.
///
/// After running this, the stack should look as if the thread was
/// interrupted by an ISR.
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
///
/// This function sets up the stack so when the context is switched to this thread,
/// it starts executing `func` with argument `arg`.
/// Furthermore, it sets up the link-register with the [`crate::cleanup`] function that
/// will be executed after the thread function returned.
pub(crate) fn setup_stack(stack: &mut [u8], func: usize, arg: usize) -> usize {
    let stack_start = stack.as_ptr() as usize;

    // 1. The stack starts at the highest address and grows downwards.
    // 2. A full stored context also contains R4-R11 and the stack pointer,
    //    thus an additional 36 bytes need to be reserved.
    // 3. Cortex-M expects the SP to be 8 byte aligned, so we chop the lowest
    //    7 bits by doing `& 0xFFFFFFF8`.
    let stack_pos = ((stack_start + stack.len() - 36) & 0xFFFFFFF8) as *mut usize;

    unsafe {
        write_volatile(stack_pos.offset(0), arg); // -> R0
        write_volatile(stack_pos.offset(1), 1); // -> R1
        write_volatile(stack_pos.offset(2), 2); // -> R2
        write_volatile(stack_pos.offset(3), 3); // -> R3
        write_volatile(stack_pos.offset(4), 12); // -> R12
        write_volatile(stack_pos.offset(5), cleanup as usize); // -> LR
        write_volatile(stack_pos.offset(6), func); // -> PC
        write_volatile(stack_pos.offset(7), 0x01000000); // -> APSR
    }

    stack_pos as usize
}

/// Triggers a PendSV exception to initiate a context switch.
///
/// If this is called from within a critical section the exception
/// happens after the critical section was left.
#[inline(always)]
pub fn schedule() {
    SCB::set_pendsv();
    cortex_m::asm::isb();
}

#[inline(always)]
pub(crate) fn start_threading(next_sp: usize) {
    cortex_m::interrupt::disable();
    schedule();
    unsafe {
        asm!(
            "
            msr psp, r1 // set new thread's SP to PSP
            cpsie i     // enable interrupts, otherwise svc hard faults
            svc 0       // SVC 0 handles switching
            ",
        in("r1")next_sp);
    }
}

#[cfg(armv7m)]
#[naked]
#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "C" fn SVCall() {
    asm!(
        "
            movw LR, #0xFFFd
            movt LR, #0xFFFF
            bx lr
            ",
        options(noreturn)
    );
}

#[cfg(armv6m)]
#[naked]
#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "C" fn SVCall() {
    asm!(
        "
            /* label rules:
             * - number only
             * - no combination of *only* [01]
             * - add f or b for 'next matching forward/backward'
             * so let's use '99' forward ('99f')
             */
            ldr r0, 99f
            mov LR, r0
            bx lr

            .align 4
            99:
            .word 0xFFFFFFFD
            ",
        options(noreturn)
    );
}

#[cfg(armv7m)]
#[naked]
#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "C" fn PendSV() {
    asm!(
        "
            mrs r0, psp
            cpsid i
            bl {sched}
            cpsie i
            cmp r0, #0
            /* label rules:
             * - number only
             * - no combination of *only* [01]
             * - add f or b for 'next matching forward/backward'
             * so let's use '99' forward ('99f')
             */
            beq 99f
            stmia r0, {{r4-r11}}
            ldmia r1, {{r4-r11}}
            msr.n psp, r2
            99:
            movw LR, #0xFFFd
            movt LR, #0xFFFF
            bx LR
            ",
        sched = sym sched,
        options(noreturn)
    );
}

#[cfg(any(armv6m))]
#[naked]
#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "C" fn PendSV() {
    asm!(
        "
            mrs r0, psp
            cpsid i
            bl sched
            cpsie i
            cmp r0, #0
            beq 99f

            //stmia r0!, {{r4-r7}}
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

            msr.n psp, r2
            99:
            ldr r0, 999f
            mov LR, r0
            bx lr

            .align 4
            999:
            .word 0xFFFFFFFD
            ",
        options(noreturn)
    );
}

/// Schedule the next thread.
///
/// It selects the next thread that should run from the runqueue.
/// This may be current thread, or a new one.
///
/// Input:
/// - old_sp (`r0``): the stack pointer of the currently running thread.
///
/// Returns:
/// - `0` in `r0` if the next thread in the runqueue is the currently running thread
/// - Else it writes into the following registers:
///   - `r0`: pointer to [`Thread::high_regs`] from old thread (to store old register state)
///   - `r1`: pointer to [`Thread::high_regs`] from new thread (to load new register state)
///   - `r2`: stack-pointer for new thread
///
/// On Cortex-M, this is called in PendSV.
// TODO: make arch independent, or move to arch
#[no_mangle]
unsafe fn sched(old_sp: usize) {
    let cs = CriticalSection::new();
    let next_pid;

    loop {
        {
            if let Some(pid) = (&*THREADS.as_ptr(cs)).runqueue.get_next() {
                next_pid = pid;
                break;
            }
        }
        //pm_set_lowest();
        cortex_m::asm::wfi();
        cortex_m::interrupt::enable();
        cortex_m::asm::isb();
        // pending interrupts would now get to run their ISRs
        cortex_m::interrupt::disable();
    }

    let threads = &mut *THREADS.as_ptr(cs);
    let current_high_regs;

    if let Some(current_pid) = threads.current_pid() {
        if next_pid == current_pid {
            asm!("", in("r0") 0);
            return;
        }
        //println!("current: {} next: {}", current_pid, next_pid);
        threads.threads[current_pid as usize].sp = old_sp;
        threads.current_thread = Some(next_pid);
        current_high_regs = threads.threads[current_pid as usize].high_regs.as_ptr();
    } else {
        current_high_regs = core::ptr::null();
    }

    let next = &threads.threads[next_pid as usize];
    let next_sp = next.sp;
    let next_high_regs = next.high_regs.as_ptr();

    //println!("old_sp: {:x} next.sp: {:x}", old_sp, next_sp);

    // PendSV expects these three pointers in r0, r1 and r2:
    // r0= &current.high_regs
    // r1= &next.high_regs
    // r2= &next.sp
    //
    // write to registers manually, as ABI would return the values via stack
    asm!("", in("r0") current_high_regs, in("r1") next_high_regs, in("r2")next_sp);
}
