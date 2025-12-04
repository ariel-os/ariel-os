#![expect(unsafe_code)]

use crate::{Arch as _, Cpu, RunqueueId, ThreadData, ThreadId, thread_flags::ThreadFlags};

/// Main struct for holding thread data.
#[derive(Debug)]
pub struct Thread {
    /// Saved stack pointer after context switch.
    #[allow(
        dead_code,
        reason = "sp is used in context-specific scheduler implementation"
    )]
    /// The thread's current state.
    pub state: ThreadState,
    /// Priority of the thread between 0..[`super::SCHED_PRIO_LEVELS`].
    /// Multiple threads may have the same priority.
    pub prio: RunqueueId,
    /// Id of the thread between 0..[`super::THREAD_COUNT`].
    /// Ids are unique while a thread is alive but reused after a thread finished.
    pub tid: ThreadId,
    /// Flags set for the thread.
    pub flags: ThreadFlags,
    /// Arch-specific thread data.
    #[allow(dead_code)]
    pub(crate) data: ThreadData,
    /// Core affinity of the thread.
    #[cfg(feature = "core-affinity")]
    pub core_affinity: crate::CoreAffinity,

    /// Lowest stack address.
    pub stack_lowest: usize,
    /// Highest stack address.
    pub stack_highest: usize,
}

/// Possible states of a thread
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ThreadState {
    /// No active thread.
    Invalid,
    /// Ready to run.
    ///
    /// This doesn't necessarily mean that the thread is currently running,
    /// but rather that it is in the runqueue.
    Running,
    /// Suspended / paused.
    Parked,
    /// Waiting to acquire a [`super::lock::Lock`].
    LockBlocked,
    /// Waiting for [`ThreadFlags`] to be set.
    FlagBlocked(crate::thread_flags::WaitMode),
    /// Waiting to receive on a [`crate::sync::Channel`], i.e. waiting for the sender.
    ChannelRxBlocked(usize),
    /// Waiting to send on a [`crate::sync::Channel`], i.e. waiting for the receiver.
    ChannelTxBlocked(usize),
}

impl Thread {
    /// Creates an empty [`Thread`] object with [`ThreadState::Invalid`].
    pub const fn default() -> Thread {
        Thread {
            state: ThreadState::Invalid,
            data: Cpu::DEFAULT_THREAD_DATA,
            flags: 0,
            prio: RunqueueId::new(0),
            tid: ThreadId::new(0),
            #[cfg(feature = "core-affinity")]
            core_affinity: crate::CoreAffinity::no_affinity(),
            stack_highest: 0,
            stack_lowest: 0,
        }
    }

    /// Paints a stack.
    ///
    /// # Safety
    /// - must only be called before the stack is active (within `arch::setup_stack()`).
    #[allow(dead_code, reason = "not used in all configurations")]
    pub(crate) unsafe fn stack_paint_init(&mut self, sp: usize) {
        // Byte that's used to paint stacks.
        const STACK_PAINT_COLOR: u8 = 0xCC;

        for pos in self.stack_lowest..sp {
            // SAFETY: Writing to the slice that was passed to `setup_stack()` is fine
            unsafe {
                core::ptr::write_volatile(pos as *mut u8, STACK_PAINT_COLOR);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_type_sizes() {
        // `ThreadData` is arch-specific, and is replaced with a dummy value in tests; its size is
        // non-zero otherwise.
        assert_eq!(size_of::<ThreadData>(), 0);
        assert_eq!(size_of::<Thread>(), size_of::<ThreadData>() + 40);
    }
}
