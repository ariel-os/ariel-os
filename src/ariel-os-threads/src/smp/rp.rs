#![expect(unsafe_code)]

use crate::arch::{Arch as _, Cpu};

use cortex_m::peripheral::SCB;
use embassy_rp::{
    interrupt,
    interrupt::InterruptExt as _,
    multicore::{Stack, spawn_core1},
    peripherals::CORE1,
};
use rp_pac::SIO;

use super::{CoreId, ISR_STACKSIZE_CORE1, Multicore, StackLimits};

pub struct Chip;

impl Multicore for Chip {
    const CORES: u32 = 2;
    type Stack = Stack<ISR_STACKSIZE_CORE1>;

    fn core_id() -> CoreId {
        CoreId(SIO.cpuid().read() as u8)
    }

    fn startup_other_cores(stack: &'static mut Self::Stack) {
        // Trigger scheduler.
        let start_threading = move || {
            unsafe {
                #[cfg(context = "rp2040")]
                interrupt::SIO_IRQ_PROC1.enable();
                #[cfg(context = "rp235xa")]
                interrupt::SIO_IRQ_FIFO.enable();
            }
            Cpu::start_threading();
            unreachable!()
        };
        unsafe {
            spawn_core1(CORE1::steal(), stack, start_threading);
            #[cfg(context = "rp2040")]
            interrupt::SIO_IRQ_PROC0.enable();
            #[cfg(context = "rp235xa")]
            interrupt::SIO_IRQ_FIFO.enable();
        }
    }

    fn schedule_on_core(id: CoreId) {
        if id == Self::core_id() {
            schedule();
        } else {
            schedule_other_core();
        }
    }
}

fn schedule() {
    if SCB::is_pendsv_pending() {
        // If a scheduling attempt is already pending, there must have been multiple
        // changes in the runqueue at the same time.
        // Trigger the scheduler on the other core as well to make sure that both schedulers
        // have the most recent runqueue state.
        return schedule_other_core();
    }
    crate::schedule()
}

fn schedule_other_core() {
    // Use the FIFO queue between the cores to trigger the scheduler
    // on the other core.
    let sio = SIO;
    // If its already full, no need to send another `SCHEDULE_TOKEN`.
    if !sio.fifo().st().read().rdy() {
        return;
    }
    sio.fifo().wr().write_value(SCHEDULE_TOKEN);
}

const SCHEDULE_TOKEN: u32 = 0x11;

/// Handles FIFO message from other core and triggers scheduler
/// if a [`SCHEDULE_TOKEN`] was received.
///
/// This method is injected into the `embassy_rp` interrupt handler
/// for FIFO messages.
// SAFETY: symbol required by our fork of `embassy_rp`; the function signature matches the expected
// one.
#[unsafe(no_mangle)]
// SAFETY: this function is placed in RAM to improve execution latency.
#[unsafe(link_section = ".data.ram_func")]
#[inline]
fn handle_fifo_token(token: u32) -> bool {
    if token != SCHEDULE_TOKEN {
        return false;
    }
    crate::schedule();
    true
}

impl<const SIZE: usize> StackLimits for Stack<SIZE> {
    fn limits(&self) -> (usize, usize) {
        let lowest = self.mem.as_ptr() as usize;
        let highest = lowest + SIZE;
        (lowest, highest)
    }
}
