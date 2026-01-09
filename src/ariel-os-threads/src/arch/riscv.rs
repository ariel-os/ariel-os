#![expect(unsafe_code)]

use ariel_os_debug::log::{debug, trace};
use core::arch::global_asm;
use esp_hal::{
    interrupt::{self, Priority, software::SoftwareInterrupt},
    peripherals::Interrupt,
    riscv,
    system::Cpu as EspHalCpu,
};

use crate::{Arch, SCHEDULER, Thread, cleanup};

pub struct Cpu;

#[derive(Debug, Default)]
#[repr(C)]
pub struct ThreadData {
    ra: usize,
    sp: usize,
    gp: usize,
    tp: usize,
    t0: usize,
    t1: usize,
    t2: usize,
    s0: usize,
    s1: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,
    mstatus: usize,
    mepc: usize,
}

impl Arch for Cpu {
    type ThreadData = ThreadData;
    const DEFAULT_THREAD_DATA: Self::ThreadData = default_trap_frame();

    /// Triggers software interrupt for the context switch.
    fn schedule() {
        // SAFETY: `steal().raise()` is safe on an initialized software interrupt
        unsafe { SoftwareInterrupt::<0>::steal().raise() }
    }

    fn setup_stack(thread: &mut Thread, stack: &mut [u8], func: fn(), arg: Option<usize>) {
        let stack_start = stack.as_ptr() as usize;
        // 16 byte alignment.
        let stack_pos = (stack_start + stack.len()) & 0xFFFFFFE0;
        // Set up PC, SP, RA and first argument for function.
        // sp
        thread.data.sp = stack_pos;
        // a0
        thread.data.a0 = arg.unwrap_or_default();

        // ra
        thread.data.ra = cleanup as *const () as usize;
        // pc
        thread.data.mepc = func as usize;

        thread.stack_lowest = stack_start;
        thread.stack_highest = stack_pos;

        // Safety: This is the place to initialize stack painting.
        unsafe { thread.stack_paint_init(stack_pos) };
    }

    /// Enable and trigger the appropriate software interrupt.
    fn start_threading() {
        interrupt::disable(EspHalCpu::ProCpu, Interrupt::FROM_CPU_INTR0);

        Self::schedule();

        interrupt::enable_direct(
            Interrupt::FROM_CPU_INTR0,
            Priority::Priority1,
            interrupt::CpuInterrupt::Interrupt20,
            FROM_CPU_INTR0,
        )
        .unwrap();

        // Panics if `FROM_CPU_INTR0` is among `esp_hal::interrupt::RESERVED_INTERRUPTS`,
        // which isn't the case.
        let e = interrupt::enable(Interrupt::FROM_CPU_INTR0, interrupt::Priority::Priority1);
        debug!("e : {:?}", e);
        debug!("interrupt enabled");
    }

    fn wfi() {
        riscv::asm::wfi();
    }
}

const fn default_trap_frame() -> ThreadData {
    ThreadData {
        ra: 0,
        sp: 0,
        gp: 0,
        tp: 0,
        t0: 0,
        t1: 0,
        t2: 0,
        s0: 0,
        s1: 0,
        a0: 0,
        a1: 0,
        a2: 0,
        a3: 0,
        a4: 0,
        a5: 0,
        a6: 0,
        a7: 0,
        s2: 0,
        s3: 0,
        s4: 0,
        s5: 0,
        s6: 0,
        s7: 0,
        s8: 0,
        s9: 0,
        s10: 0,
        s11: 0,
        t3: 0,
        t4: 0,
        t5: 0,
        t6: 0,
        mstatus: 0x80, // MPIE set
        mepc: 0,
    }
}

unsafe extern "C" {
    fn FROM_CPU_INTR0();
}

global_asm!(
    r#"

    .section .trap.rust, "ax"          // FIXME: is this right ?
    .globl FROM_CPU_INTR0
    .align 4
    FROM_CPU_INTR0:
        // save non callee-saved registers
        addi sp, sp, -80
        sw ra, 76(sp)
        sw gp, 72(sp)
        sw tp, 68(sp)
        sw t0, 64(sp)
        sw t1, 60(sp)
        sw t2, 56(sp)
        sw a0, 52(sp)
        sw a1, 48(sp)
        sw a2, 44(sp)
        sw a3, 40(sp)
        sw a4, 36(sp)
        sw a5, 32(sp)
        sw a6, 28(sp)
        sw a7, 24(sp)
        sw t3, 20(sp)
        sw t4, 16(sp)
        sw t5, 12(sp)
        sw t6, 8(sp)


        csrr t0, mepc
        sw t0, 4(sp)

        csrr t0, mstatus
        sw t0, 0(sp)


        call {sched}

        // if a1 is null, we need to return to the previous task
        beqz    a1, restore_stack
        // if a0 is null, no need to save
        beqz    a0, restore


        // save registers

        // mepc
        lw t0, 4(sp)
        sw t0, 32*4(a0)

        //ra
        lw t0, 76(sp)
        sw t0, 0*4(a0)

        // gp
        lw t0, 72(sp)
        sw t0, 2*4(a0)

        // tp
        lw t0, 68(sp)
        sw t0, 3*4(a0)

        // t0
        lw t0, 64(sp)
        sw t0, 4*4(a0)

        // t1
        lw t0, 60(sp)
        sw t0, 5*4(a0)

        // t2
        lw t0, 56(sp)
        sw t0, 6*4(a0)

        sw s0, 7*4(a0)
        sw s1, 8*4(a0)

        // a0
        lw t0, 52(sp)
        sw t0, 9*4(a0)

        // a1
        lw t0, 48(sp)
        sw t0, 10*4(a0)

        // a2
        lw t0, 44(sp)
        sw t0, 11*4(a0)

        // a3
        lw t0, 40(sp)
        sw t0, 12*4(a0)

        // a4
        lw t0, 36(sp)
        sw t0, 13*4(a0)

        // a5
        lw t0, 32(sp)
        sw t0, 14*4(a0)

        // a6
        lw t0, 28(sp)
        sw t0, 15*4(a0)

        // a7
        lw t0, 24(sp)
        sw t0, 16*4(a0)

        sw s2, 17*4(a0)
        sw s3, 18*4(a0)
        sw s4, 19*4(a0)
        sw s5, 20*4(a0)
        sw s6, 21*4(a0)
        sw s7, 22*4(a0)
        sw s8, 23*4(a0)
        sw s9, 24*4(a0)
        sw s10, 25*4(a0)
        sw s11, 26*4(a0)

        // t3
        lw t3, 20(sp)
        sw t3, 27*4(a0)

        // t4
        lw t4, 16(sp)
        sw t4, 28*4(a0)

        // t5
        lw t5, 12(sp)
        sw t5, 29*4(a0)

        // t6
        lw t6, 8(sp)
        sw t6, 30*4(a0)

        addi sp, sp, 80
        sw sp, 1*4(a0)

    restore:

        beqz    a1, restore_stack

        // restore mepc and mstatus
        lw t0, 31*4(a1)
        csrw mstatus, t0
        lw t1, 32*4(a1)
        csrw mepc,t1

        // load registers
        lw ra, 0*4(a1)
        lw sp, 1*4(a1)
        lw gp, 2*4(a1)
        lw tp, 3*4(a1)
        lw t0, 4*4(a1)
        lw t1, 5*4(a1)
        lw t2, 6*4(a1)
        lw s0, 7*4(a1)
        lw s1, 8*4(a1)
        lw a0, 9*4(a1)
        lw a2, 11*4(a1)
        lw a3, 12*4(a1)
        lw a4, 13*4(a1)
        lw a5, 14*4(a1)
        lw a6, 15*4(a1)
        lw a7, 16*4(a1)
        lw s2, 17*4(a1)
        lw s3, 18*4(a1)
        lw s4, 19*4(a1)
        lw s5, 20*4(a1)
        lw s6, 21*4(a1)
        lw s7, 22*4(a1)
        lw s8, 23*4(a1)
        lw s9, 24*4(a1)
        lw s10, 25*4(a1)
        lw s11, 26*4(a1)
        lw t3, 27*4(a1)
        lw t4, 28*4(a1)
        lw t5, 29*4(a1)
        lw t6, 30*4(a1)


        lw a1, 10*4(a1)
        csrs mstatus, 0x8

        mret

    restore_stack:

        // restore mepc
        lw t0, 4(sp)
        csrw mepc,t0

        // restore mstatus
        lw t0, 0(sp)
        csrw mstatus,t0

        lw ra, 76(sp)
        lw gp, 72(sp)
        lw tp, 68(sp)
        lw t0, 64(sp)
        lw t1, 60(sp)
        lw t2, 56(sp)
        lw a0, 52(sp)
        lw a1, 48(sp)
        lw a2, 44(sp)
        lw a3, 40(sp)
        lw a4, 36(sp)
        lw a5, 32(sp)
        lw a6, 28(sp)
        lw a7, 24(sp)
        lw t3, 20(sp)
        lw t4, 16(sp)
        lw t5, 12(sp)
        lw t6, 8(sp)
        addi sp, sp, 80

        mret
    "#,
    sched = sym sched
);

/// Probes the runqueue for the next thread and switches context if needed.
unsafe extern "C" fn sched() -> u64 {
    let mepc = esp_hal::riscv::register::mepc::read();
    trace!("sched start mepc: {}", mepc);

    let mstatus_st = esp_hal::riscv::register::mstatus::read();
    let mstatus = mstatus_st.bits();

    // clear FROM_CPU_INTR0
    // SAFETY: `steal().reset()` is safe on an initialized software interrupt
    unsafe { SoftwareInterrupt::<0>::steal().reset() }

    let (current_high_regs, next_high_regs) = loop {
        if let Some(res) = SCHEDULER.with_mut(|mut scheduler| {
            #[cfg(feature = "multi-core")]
            scheduler.add_current_thread_to_rq();

            let next_tid = match scheduler.get_next_tid() {
                Some(tid) => tid,
                None => {
                    return None;
                }
            };

            let mut current_high_regs = core::ptr::null();

            if let Some(current_tid_ref) = scheduler.current_tid_mut() {
                if next_tid == *current_tid_ref {
                    return Some((0, 0));
                }
                let current_tid = *current_tid_ref;
                *current_tid_ref = next_tid;
                let current = scheduler.get_unchecked_mut(current_tid);
                current_high_regs = &raw mut current.data;
            } else {
                *scheduler.current_tid_mut() = Some(next_tid);
            }
            let next = scheduler.get_unchecked_mut(next_tid);
            next.data.mstatus = mstatus;

            let next_high_regs = &raw mut next.data;
            trace!("next cleanup: {}", next.data.ra);
            Some((current_high_regs as u32, next_high_regs as u32))
        }) {
            break res;
        } else {
            // Returned None, meaning we should wait for an interrupt

            let mut mstatus_st = esp_hal::riscv::register::mstatus::read();
            mstatus_st.set_mie(true);
            unsafe {
                esp_hal::riscv::register::mstatus::write(mstatus_st);
            }
            trace!("Scheduler wfi");
            Cpu::wfi();
            let mut mstatus_st = esp_hal::riscv::register::mstatus::read();
            mstatus_st.set_mie(false);
            unsafe {
                esp_hal::riscv::register::mstatus::write(mstatus_st);
            }
        }
    };

    trace!(
        "Scheduler result: {:?}-{:?}",
        current_high_regs, next_high_regs
    );

    let mepc = esp_hal::riscv::register::mepc::read();
    trace!("Scheduler end mepc: {}", mepc);
    // The caller expects these two pointers in a0 and a1:
    // a0 = &current.data.high_regs (or 0)
    // a1 = &next.data.high_regs
    (current_high_regs as u64) | (next_high_regs as u64) << 32
}
