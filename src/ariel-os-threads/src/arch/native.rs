use std::cell::Cell;

use std::sync::atomic::{AtomicU32, Ordering};

use crate::{thread::Thread, Arch, ThreadId, ThreadState, SCHEDULER};

pub struct Cpu;

mod critical_section;

static THREAD_BLOCKS: [AtomicU32; crate::THREAD_COUNT] =
    [const { AtomicU32::new(0) }; crate::THREAD_COUNT];

#[derive(Debug)]
pub struct ThreadData {
    thread: Option<std::thread::Thread>,
}

impl ThreadData {
    pub const fn new() -> Self {
        Self { thread: None }
    }
}

impl ThreadData {
    thread_local! {
        pub static ID: Cell<Option<ThreadId>> = Cell::new(None);
    }
}

impl Arch for Cpu {
    type ThreadData = ThreadData;
    const DEFAULT_THREAD_DATA: Self::ThreadData = ThreadData::new();

    fn setup_stack(thread: &mut Thread, _stack: &mut [u8], func: usize, arg: usize) {
        let func: fn(arg: usize) = unsafe { core::mem::transmute(func) };

        let thread_id = thread.tid;

        let handle = std::thread::spawn(move || {
            ThreadData::ID.with(|x| x.set(Some(thread_id)));
            atomic_wait::wait(&THREAD_BLOCKS[usize::from(thread_id)], 0);
            func(arg);
            SCHEDULER.with_mut(|mut scheduler| {
                scheduler.set_state(thread_id, ThreadState::Invalid);
            });
        });

        thread.data.thread = Some(handle.thread().clone());
    }

    fn start_threading() {
        loop {
            SCHEDULER.with(|scheduler| {
                for (n, thread) in scheduler.threads.iter().enumerate() {
                    if thread.state == ThreadState::Running {
                        if THREAD_BLOCKS[n].swap(1, Ordering::Acquire) == 0 {
                            atomic_wait::wake_one(&THREAD_BLOCKS[n]);
                        }
                    }
                }
            });

            std::thread::park();
        }
    }

    fn schedule() {}

    fn wfi() {
        unimplemented!()
    }

    fn set_running(thread_id: ThreadId) {
        let n = usize::from(thread_id);
        if THREAD_BLOCKS[n].swap(1, Ordering::Acquire) == 0 {
            atomic_wait::wake_one(&THREAD_BLOCKS[n]);
        }
    }

    fn set_stopped(thread_id: ThreadId) {
        THREAD_BLOCKS[usize::from(thread_id)].store(0, Ordering::Release);
    }
}
