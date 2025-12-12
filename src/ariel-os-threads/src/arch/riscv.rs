#![expect(unsafe_code)]

use crate::{Arch, SCHEDULER, Thread, cleanup};
use ariel_os_debug::log::{debug, info, trace};
use core::arch::global_asm;
#[cfg(context = "esp32c6")]
use esp_hal::peripherals::INTPRI as SYSTEM;
#[cfg(context = "esp32c3")]
use esp_hal::peripherals::SYSTEM;
use esp_hal::{
    interrupt::{self},
    peripherals::Interrupt,
    riscv,
    system::Cpu as EspHalCpu,
};
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
        debug!("risscv::schedule()");
        let mstatus_st = esp_hal::riscv::register::mstatus::read();
        let mstatus = mstatus_st.bits();

        debug!(
            "mstatus.mie: {} mstatus.mpie: {} ",
            mstatus_st.mie(),
            mstatus_st.mpie()
        );

        unsafe {
            (&*SYSTEM::PTR)
                .cpu_intr_from_cpu(0)
                .modify(|_, w| w.cpu_intr().set_bit());
        }
        debug!("mstatus: {:#x}", mstatus);
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

        trace!("cleanup addr: {}", cleanup as usize);
        // ra
        thread.data.ra = cleanup as usize;
        // pc
        thread.data.mepc = func as usize;

        thread.stack_lowest = stack_start;
        thread.stack_highest = stack_pos;

        // Safety: This is the place to initialize stack painting.
        unsafe { thread.stack_paint_init(stack_pos) };
    }

    /// Enable and trigger the appropriate software interrupt.
    fn start_threading() {
        debug!("riscv::start_threading");
        interrupt::disable(EspHalCpu::ProCpu, Interrupt::FROM_CPU_INTR0);
        debug!("interrupts disabled");

        Self::schedule();
        debug!("schedule done");

        interrupt::enable_direct(
            Interrupt::FROM_CPU_INTR0,
            esp_hal::interrupt::Priority::Priority3,
            esp_hal::interrupt::CpuInterrupt::Interrupt20,
            FROM_CPU_INTR0,
        )
        .unwrap();
        // Panics if `FROM_CPU_INTR0` is among `esp_hal::interrupt::RESERVED_INTERRUPTS`,
        // which isn't the case.
        let e = interrupt::enable(Interrupt::FROM_CPU_INTR0, interrupt::Priority::min());
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
        // unset mie
        csrc mstatus, 0x8

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

        call {sched}

        // if a0 is null, no need to save
        beqz    a0, restore
        csrr t0, mepc
        sw t0, 32*4(a0)


        // save registers

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


        mret
    "#,
    sched = sym sched
);

/// Probes the runqueue for the next thread and switches context if needed.
///
/// # Safety
///
/// This method might switch the current register state that is saved in the
/// `trap_frame`.
/// It should only be called from inside the trap handler that is responsible for
/// context switching.
unsafe extern "C" fn sched() -> u64 {
    debug!("sched !");
    let mstatus_st = esp_hal::riscv::register::mstatus::read();
    let mstatus = mstatus_st.bits();

    trace!(
        "mstatus.mie: {} mstatus.mpie: {} ",
        mstatus_st.mie(),
        mstatus_st.mpie()
    );

    trace!("mstatus: {:#x}", mstatus);
    unsafe {
        // esp_hal::riscv::register::mstatus::write(
        //     esp_hal::riscv::register::mstatus::Mstatus::from_bits(mstatus & !(1 << 3)),
        // );

        let mstatus_st = esp_hal::riscv::register::mstatus::read();
        let mstatus = mstatus_st.bits();

        trace!(
            "mstatus.mie: {} mstatus.mpie: {} ",
            mstatus_st.mie(),
            mstatus_st.mpie()
        );

        // clear FROM_CPU_INTR0
        (&*SYSTEM::PTR)
            .cpu_intr_from_cpu(0)
            .modify(|_, w| w.cpu_intr().clear_bit());
    }

    let (current_high_regs, next_high_regs) = loop {
        if let Some(res) = SCHEDULER.with_mut(|mut scheduler| {
            #[cfg(feature = "multi-core")]
            scheduler.add_current_thread_to_rq();

            let next_tid = match scheduler.get_next_tid() {
                Some(tid) => tid,
                None => {
                    Cpu::wfi();
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
        }
    };

    trace!("result: {:?}-{:?}", current_high_regs, next_high_regs);

    // The caller expects these two pointers in a0 and a1:
    // a0 = &current.data.high_regs (or 0)
    // a1 = &next.data.high_regs
    (current_high_regs as u64) | (next_high_regs as u64) << 32
}
